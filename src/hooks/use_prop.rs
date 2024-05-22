use freya::prelude::*;

pub fn use_prop<T: Clone + PartialEq>(value: T) -> Signal<T> {
    let mut s = use_signal(|| value.clone());
    if s() != value {
        s.set(value);
    }
    s
}

pub fn use_prop_with_option_default<T: Clone + PartialEq>(
    value: Option<T>,
    default_value: T,
) -> Signal<T> {
    let mut s = use_signal(|| value.clone().unwrap_or(default_value.clone()));
    // if s() != value {
    //     s.set(value);
    // }

    if let Some(value) = value {
        s.set(value);
    } else {
        s.set(default_value);
    }

    s
}
