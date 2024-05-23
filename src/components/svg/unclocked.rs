use freya::prelude::*;

use super::SvgProps;

#[allow(non_snake_case)]
#[component]
pub fn UnLockedSvg(props: SvgProps) -> Element {
    rsx!(svg {
        width: "100%",
        height: "100%",
        svg_content: r#"
          <svg width="100%" height="100%" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="7" y="22.0476" width="34" height="22" rx="2" fill="none" stroke="{props.stroke_color}" stroke-width="4" stroke-linejoin="round"/>
            <path d="M14 22V14.0047C13.9948 8.87022 17.9227 4.56718 23.0859 4.05117C28.249 3.53516 32.9673 6.97408 34 12.0059" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M24 30V36" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        "#
    })
}
