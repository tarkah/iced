use crate::conversion;
use crate::{Application, Color, Debug, Mode, Point, Size, Viewport};

use std::marker::PhantomData;
use winit::event::WindowEvent;
use winit::window::Window;

#[derive(Debug, Clone)]
pub struct State<A: Application> {
    title: String,
    mode: Mode,
    background_color: Color,
    scale_factor: f64,
    viewport: Viewport,
    cursor_position: winit::dpi::PhysicalPosition<f64>,
    modifiers: winit::event::ModifiersState,
    application: PhantomData<A>,
}

impl<A: Application> State<A> {
    pub fn new(application: &A, window: &Window) -> Self {
        let title = application.title();
        let mode = application.mode();
        let background_color = application.background_color();
        let scale_factor = application.scale_factor();

        let viewport = {
            let physical_size = window.inner_size();

            Viewport::with_physical_size(
                Size::new(physical_size.width, physical_size.height),
                window.scale_factor() * scale_factor,
            )
        };

        Self {
            title,
            mode,
            background_color,
            scale_factor,
            viewport,
            // TODO: Encode cursor availability in the type-system
            cursor_position: winit::dpi::PhysicalPosition::new(-1.0, -1.0),
            modifiers: winit::event::ModifiersState::default(),
            application: PhantomData,
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn physical_size(&self) -> Size<u32> {
        self.viewport.physical_size()
    }

    pub fn logical_size(&self) -> Size<f32> {
        self.viewport.logical_size()
    }

    pub fn scale_factor(&self) -> f64 {
        self.viewport.scale_factor()
    }

    pub fn cursor_position(&self) -> Point {
        conversion::cursor_position(
            self.cursor_position,
            self.viewport.scale_factor(),
        )
    }

    pub fn modifiers(&self) -> winit::event::ModifiersState {
        self.modifiers
    }

    pub fn update(
        &mut self,
        window: &Window,
        event: &WindowEvent<'_>,
        _debug: &mut Debug,
    ) {
        match event {
            WindowEvent::Resized(new_size) => {
                let size = Size::new(new_size.width, new_size.height);

                self.viewport = Viewport::with_physical_size(
                    size,
                    window.scale_factor() * self.scale_factor,
                );
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor: new_scale_factor,
                new_inner_size,
            } => {
                let size =
                    Size::new(new_inner_size.width, new_inner_size.height);

                self.viewport = Viewport::with_physical_size(
                    size,
                    new_scale_factor * self.scale_factor,
                );
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = *position;
            }
            WindowEvent::CursorLeft { .. } => {
                // TODO: Encode cursor availability in the type-system
                self.cursor_position =
                    winit::dpi::PhysicalPosition::new(-1.0, -1.0);
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = *new_modifiers;
            }
            #[cfg(feature = "debug")]
            WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        virtual_keycode: Some(winit::event::VirtualKeyCode::F12),
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => _debug.toggle(),
            _ => {}
        }
    }

    pub fn synchronize(&mut self, application: &A, window: &Window) {
        // Update window title
        let new_title = application.title();

        if self.title != new_title {
            window.set_title(&new_title);

            self.title = new_title;
        }

        // Update window mode
        let new_mode = application.mode();

        if self.mode != new_mode {
            window.set_fullscreen(conversion::fullscreen(
                window.current_monitor(),
                new_mode,
            ));

            self.mode = new_mode;
        }

        // Update background color
        self.background_color = application.background_color();

        // Update scale factor
        let new_scale_factor = application.scale_factor();

        if self.scale_factor != new_scale_factor {
            let size = window.inner_size();

            self.viewport = Viewport::with_physical_size(
                Size::new(size.width, size.height),
                window.scale_factor() * new_scale_factor,
            );

            self.scale_factor = new_scale_factor;
        }
    }
}
