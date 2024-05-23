use super::SvgProps;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn ToTopSvg(props: SvgProps) -> Element {
    rsx!(svg {
        width: "100%",
        height: "100%",
        svg_content: r#"
          <svg width="100%" height="100%" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M41 18H19C18.4477 18 18 18.4477 18 19V41C18 41.5523 18.4477 42 19 42H41C41.5523 42 42 41.5523 42 41V19C42 18.4477 41.5523 18 41 18Z" fill="none" stroke="{props.stroke_color}" stroke-width="4" stroke-linejoin="round"/>
            <path d="M9.96906 6H6V10.0336" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M9.99705 30H6V26.012" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M26.0023 6H30V10.0152" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M16.0283 6H20.0083" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M6 16C6 18.6536 6 19.9869 6 20" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M30 16C30 18.6765 30 19.3456 30 18.0074" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M15.9922 30H17.9996" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round"/>
          </svg>
        "#
    })
}
