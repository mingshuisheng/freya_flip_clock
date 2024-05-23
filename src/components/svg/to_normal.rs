use super::SvgProps;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn ToNormalSvg(props: SvgProps) -> Element {
    rsx!(svg {
        width: "100%",
        height: "100%",
        svg_content: r#"
          <svg width="100%" height="100%" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M14 21H5V5H21V14" stroke="{props.stroke_color}" stroke-width="4" stroke-linejoin="round"/>
            <path d="M32 27H43V43H27V32" stroke="{props.stroke_color}" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M14 32V14H32V32H14Z" fill="none" stroke="{props.stroke_color}" stroke-width="4" stroke-linejoin="round"/>
          </svg>
        "#
    })
}
