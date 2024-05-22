use super::num::Num;
use freya::prelude::*;

#[derive(Props, Clone, PartialEq, Debug)]
pub struct NumGroupProps {
    num: u32,
    max_num: u32,
}

#[allow(non_snake_case)]
#[component]
pub fn NumGroup(props: NumGroupProps) -> Element {
    rsx!(
      rect {
        direction: "horizontal",
        width: "30%",
        height: "100%",
        rect {
            width: "47.619%",
            height: "100%",
            position: "relative",
            color: "white",
            background: "transparent",
            overflow: "none",
            Num {
              num: props.num / 10,
              max: props.max_num / 10,
            }
          }
          rect {width: "4.7619%"}
          rect {
            width: "47.619%",
            height: "100%",
            position: "relative",
            color: "white",
            background: "transparent",
            overflow: "none",
            Num {
              num: props.num % 10,
              max: 9
            }
         }
      }
    )
}
