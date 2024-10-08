use freya::prelude::*;

use super::SvgProps;

#[allow(non_snake_case)]
#[component]
pub fn LockedSvg(props: SvgProps) -> Element {
    rsx!(svg {
        width: "100%",
        height: "100%",
        svg_content: r#"
          <svg width="100%" height="100%" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="6" y="22" width="36" height="22" rx="2" fill="none" stroke="{props.stroke_color}" stroke-width="4" stroke-linejoin="round"/>
            <path d="M14 22V14C14 8.47715 18.4772 4 24 4C29.5228 4 34 8.47715 34 14V22" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M24 30V36" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        "#
    })
}
