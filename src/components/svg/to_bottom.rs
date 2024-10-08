use super::SvgProps;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn ToBottomSvg(props: SvgProps) -> Element {
    rsx!(svg {
        width: "100%",
        height: "100%",
        svg_content: r#"
          <svg width="100%" height="100%" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M30 18H41C41.5523 18 42 18.4477 42 19V41C42 41.5523 41.5523 42 41 42H19C18.4477 42 18 41.5523 18 41V30" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M9.96906 6H6V10.0336" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M9.99705 30H6V26.012" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M26 30H29.9971V26.012" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M26.0023 6H30V9.99785" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M16.0283 6H20.0083" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round"/><path d="M6 16V20.0148" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M30 16V20.0148" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M15.9922 30H19.9996" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        "#
    })
}
