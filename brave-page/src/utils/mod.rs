use crate::utils::common::get_page_attr;
use actix_web::guard::{Guard, GuardContext};
use brave_config::theme::PageAttr;
use brave_config::GLOBAL_CONFIG;

pub(crate) mod common;

pub(crate) struct StaticGuard;

pub(crate) struct SingleGuard;

impl Guard for StaticGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        get_guard_state(ctx)
    }
}

impl Guard for SingleGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        !get_guard_state(ctx)
    }
}

fn get_guard_state(ctx: &GuardContext<'_>) -> bool {
    let path = ctx.head().uri.path();

    let name_path = path.replace(&GLOBAL_CONFIG.interface.blog_scope, "");
    let name = name_path.split("/").collect::<Vec<&str>>();

    match get_page_attr(name.get(2).unwrap()) {
        PageAttr::Static => true,
        PageAttr::Single => false,
    }
}
