use crate::window::WindowHandle;
use crate::WinHandler;

pub enum WinHandlerWrapper {
    Uninitialized(Box<dyn 'static + FnOnce(&WindowHandle) -> Box<dyn WinHandler>>),
    Initialized(Box<dyn WinHandler>),
    RunningConstructor,
}

impl WinHandlerWrapper {
    /// Creates a new `WinHandlerWrapper` from some constructor.
    pub fn create<H: 'static + WinHandler, F: 'static + FnOnce(&WindowHandle) -> H>(
        constructor: F,
    ) -> Self {
        Self::Uninitialized(Box::new(|handler| Box::new(constructor(handler))))
    }

    /// Initializes the `WinHandler` with the specified `WindowHandle`. If this method
    /// has already been previously called, this function is a no-op.
    pub fn connect(&mut self, handle: &WindowHandle) {
        let this = std::mem::replace(self, Self::RunningConstructor);
        let handler = match this {
            Self::Uninitialized(constructor) => constructor(handle),
            Self::Initialized(handler) => handler,
            Self::RunningConstructor => unreachable!(),
        };
        *self = Self::Initialized(handler);
    }

    pub fn get_mut(&mut self) -> &mut dyn WinHandler {
        // TODO: track_caller and better errors here
        match self {
            Self::Initialized(handler) => &mut **handler,
            Self::Uninitialized(_) => {
                panic!("get_mut was called before the win handler wrapper was initialized");
            }
            Self::RunningConstructor => unreachable!(),
        }
    }
}
