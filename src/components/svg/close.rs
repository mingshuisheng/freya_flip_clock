use super::SvgProps;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn CloseSvg(props: SvgProps) -> Element {
    rsx!(svg {
        width: "100%",
        height: "100%",
        svg_content: r#"
          <svg width="100%" height="100%" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M8 8L40 40" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" />
            <path d="M8 40L40 8" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        "#
    })
}
