use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::*;
use yew::prelude::*;

impl RowToBadge for NestedTeam {
    fn badge_title(&self) -> String {
        self.inner.name.clone()
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }

    fn children(&self, props: &super::BadgeProps<Self>) -> Option<yew::prelude::Html> {
        Some(html! {
            <>
                <Badge<NestedTeamState> badge={self.state.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
                <Badge<NestedUser> badge={self.updated_by.clone()} onclick={props.onclick.clone()} li={true} query={props.query.clone()} size={BadgeSize::Small} />
            </>
        })
    }
}
