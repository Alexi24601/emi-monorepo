//! Submodule to test the codegen of diesel tables.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of diesel schema for tables.
async fn test_codegen_tables_attribute_traits() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables_attribute_traits")
            .await
            .unwrap();

    let outcome = Codegen::default()
        .set_output_directory("tests/codegen_tables_attribute_traits".as_ref())
        .enable_attribute_trait()
        .beautify()
        .generate(&mut conn, &database_name, None);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_attribute_traits");

    std::fs::remove_dir_all("tests/codegen_tables_attribute_traits").unwrap();
}
