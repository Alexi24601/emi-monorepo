//! Login API for GitHub OAuth
use crate::api::oauth::*;

use actix_web::{get, HttpResponse, Responder};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::jwt_cookies::build_login_response;
use crate::model_implementations::*;
use crate::models::*;
use crate::transactions::renormalize_user_emails::{renormalize_user_emails, Emails};
use redis::Client as RedisClient;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::error::Error;
use web_common::api::ApiError;

/// Struct representing the GitHub OAuth2 configuration.
struct GitHubConfig {
    client_id: String,
    client_secret: String,
    provider_id: i16,
}

impl GitHubConfig {
    /// Function to retrieve the GitHub OAuth2 configuration from the environment.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `GitHubConfig` if the environment variables are set, or an error
    /// message if they are not.
    pub fn from_env(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<GitHubConfig, String> {
        dotenvy::dotenv().ok();
        let client_secret = env::var("GITHUB_CLIENT_SECRET");

        if client_secret.is_err() {
            return Err("GITHUB_CLIENT_SECRET not set".to_string());
        }

        // We retrieve the ID for the 'GitHub' provider from the database.
        let provider = LoginProvider::from_provider_name("GitHub", pool);

        if provider.is_err() {
            return Err(provider.err().unwrap().to_string());
        }

        let provider = provider.unwrap();

        let client_id = env::var("GITHUB_CLIENT_ID");

        if client_id.is_err() {
            return Err("GITHUB_CLIENT_ID not set".to_string());
        }

        Ok(GitHubConfig {
            client_id: client_id.unwrap(),
            client_secret: client_secret.unwrap(),
            provider_id: provider.id,
        })
    }
}

#[derive(Deserialize)]
pub struct GitHubOauthToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
struct GithubEmailMetadata {
    email: String,
    verified: bool,
    primary: bool,
    #[serde(rename = "visibility")]
    _visibility: Option<String>,
}

#[get("/github")]
async fn github_oauth_handler(
    query: web::Query<QueryCode>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    redis_client: web::Data<RedisClient>,
) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(ApiError::unauthorized());
    }

    let token_response = get_github_oauth_token(code.as_str(), &pool).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        log::error!("GitHub login failed: {}", message);
        return HttpResponse::BadGateway().json(ApiError::bad_gateway());
    }

    let token_response = token_response.unwrap();

    // We retrieve the GitHub user emails
    let emails_response = get_github_user_emails(token_response.access_token.as_str()).await;

    if emails_response.is_err() {
        let message = emails_response.err().unwrap().to_string();
        log::error!("GitHub mail retrieval failed: {}", message);
        return HttpResponse::BadGateway().json(ApiError::bad_gateway());
    }

    let github_config = GitHubConfig::from_env(&pool).unwrap();

    let user_query = renormalize_user_emails(
        github_config.provider_id,
        emails_response.unwrap(),
        NewUser::default(),
        &pool,
    );

    if user_query.is_err() {
        let message = user_query.err().unwrap().to_string();
        log::error!("Github user insert into database failed: {}", message);
        return HttpResponse::InternalServerError().json(ApiError::internal_server_error());
    }

    let user_id = user_query.unwrap().id();

    build_login_response(user_id, state, &redis_client).await
}

pub async fn get_github_oauth_token(
    authorization_code: &str,
    pool: &web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<GitHubOauthToken, Box<dyn Error>> {
    let github_config = GitHubConfig::from_env(pool)?;

    let root_url = "https://github.com/login/oauth/access_token";

    let client = Client::new();

    let params = [
        ("client_id", github_config.client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", github_config.client_secret.as_str()),
    ];

    let response = client
        .post(root_url)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let oauth_response = response.json::<GitHubOauthToken>().await?;
        Ok(oauth_response)
    } else {
        let message = format!(
            "An error occurred while trying to retrieve the access token: {}, text: {}",
            response.status(),
            response.text().await?
        );
        Err(From::from(message))
    }
}

/// Function to retrieve the emails associated with a GitHub user.
///
/// # Implementative details
/// This function uses the GitHub API to retrieve the emails associated with a user.
/// While there is an email field in the set of informations returned as the user logs in,
/// these emails are optional and the user on GitHub may choose to not display them (in fact
/// this is the default setting). This function retrieves the emails from the GitHub API
/// from the endpoint `/user/emails` and returns them as a `Vec<String>`.
pub async fn get_github_user_emails(authorization_code: &str) -> Result<Emails, Box<dyn Error>> {
    let root_url = "https://api.github.com/user/emails";

    let client = Client::new();

    let response = client
        .get(root_url)
        .header("Accept", "application/json")
        .header("User-Agent", "EarthMetabolomeInitiativePortal")
        .bearer_auth(authorization_code)
        .send()
        .await?;

    if response.status().is_success() {
        let emails = response.json::<Vec<GithubEmailMetadata>>().await;

        if emails.is_err() {
            let message = format!(
                "An error occurred while trying to retrieve the user emails: {}",
                emails.err().unwrap()
            );
            return Err(From::from(message));
        }

        let emails = emails.unwrap();

        let mut primary = String::new();
        let mut email_list = Vec::new();

        for email in emails {
            if !email.verified {
                continue;
            }
            if email.primary {
                primary.clone_from(&email.email);
            }
            email_list.push(email.email);
        }

        // If not primary mail was set, then this was a bad request.
        if primary.is_empty() {
            let message = "No primary email was found in the list of emails from GitHub";
            return Err(From::from(message));
        }

        // If no email was found, then this was a bad request.
        if email_list.is_empty() {
            let message = "No email was found in the list of emails from GitHub";
            return Err(From::from(message));
        }

        Ok(Emails::new(email_list, primary)?)
    } else {
        let message = "An error occurred while trying to retrieve the user emails";
        Err(From::from(message))
    }
}
