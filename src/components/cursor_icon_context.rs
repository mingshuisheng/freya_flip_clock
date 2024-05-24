use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn CursorIconContextProvider(children: Element) -> Element {
    let cursor_icon = use_signal(|| CursorIcon::Default);

    provide_context(CursorIconContext(cursor_icon));

    rsx!({ children })
}

pub fn use_cursor_icon_context() -> CursorIconContext {
    consume_context::<CursorIconContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct CursorIconContext(pub Signal<CursorIcon>);

impl CursorIconContext {
    pub fn is_default(&self) -> bool {
        *self.0.read() == CursorIcon::Default
    }

    pub fn cursor_icon(&self) -> CursorIcon {
        *self.0.read()
    }

    pub fn set_cursor(&mut self, cursor_icon: CursorIcon) {
        self.0.set(cursor_icon);
    }
}
