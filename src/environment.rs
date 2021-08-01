use crate::storage::{Storage, StorageMock};

use tui::{backend::Backend, layout::Rect, terminal::CompletedFrame, Frame};

use std::{io, marker::PhantomData};

pub struct Environment<SM, S, T, B>
where
    SM: SignalManager,
    S: Storage,
    T: TerminalKind<B>,
    B: Backend,
{
    pub signal_manager: SM,
    pub storage: S,
    pub terminal: T,
    _phantom: PhantomData<B>,
}

impl<T, B> Environment<SignalManagerMock, StorageMock, T, B>
where
    T: TerminalKind<B>,
    B: Backend,
{
    pub fn with_terminal(terminal: T) -> Self {
        Self {
            signal_manager: SignalManagerMock {},
            storage: StorageMock {},
            terminal,
            _phantom: PhantomData,
        }
    }
}

pub trait SignalManager {}

pub struct SignalManagerMock {}

impl SignalManagerMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl SignalManager for SignalManagerMock {}

pub trait TerminalKind<B> {
    fn size(&mut self) -> Rect;

    fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<B>),
        B: Backend;
}

struct TerminalMock {
    size: Rect,
}

impl<B: Backend> TerminalKind<B> for TerminalMock {
    fn draw<F>(&mut self, _f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<B>),
    {
        Err(io::Error::new(io::ErrorKind::Other, "unimplemented"))
    }

    fn size(&mut self) -> Rect {
        self.size
    }
}

impl<B: Backend> TerminalKind<B> for tui::Terminal<B> {
    fn size(&mut self) -> Rect {
        self.get_frame().size()
    }

    fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame<B>),
    {
        self.draw(f)
    }
}
