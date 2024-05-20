use super::RowToSearchableBadge;
use crate::traits::format_match::FormatMatch;
use web_common::database::NestedDocumentFormat;
use yew::prelude::*;

impl RowToSearchableBadge for NestedDocumentFormat {
    fn to_datalist_badge(&self, query: &str) -> Html {
        html! {
            <div>
                <p>
                <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                    <span>{self.inner.extension.format_match(query)}</span>
                    <span>{self.inner.mime_type.format_match(query)}</span>
                </p>
                <p>{self.inner.description.format_match(query)}</p>
            </div>
        }
    }

    fn to_selected_datalist_badge(&self) -> Html {
        html! {
            <div>
                <p>
                <i class={format!("fas fa-{} {}", self.icon.name, self.color.name)}></i>
                    <span>{self.inner.extension.clone()}</span>
                </p>
                <p>{self.inner.description.clone()}</p>
            </div>
        }
    }
    fn similarity_score(&self, query: &str) -> isize {
        self.inner.extension.similarity_score(query)
            + self.inner.mime_type.similarity_score(query)
            + self.inner.description.similarity_score(query)
    }
    fn primary_color_class(&self) -> &str {
        &self.color.name
    }
    fn description(&self) -> &str {
        &self.inner.description
    }
}
