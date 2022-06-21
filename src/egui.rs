use anyhow::Result;
use egui_wgpu::renderer as egui_wgpu;
use winit::{event::{WindowEvent, ElementState, VirtualKeyCode, ModifiersState, MouseButton, KeyboardInput}, dpi::PhysicalPosition};
use crate::graphics::utils;

fn is_cut_command(modifiers: egui::Modifiers, keycode: winit::event::VirtualKeyCode) -> bool {
    return (modifiers.command && keycode == winit::event::VirtualKeyCode::X)
        || (cfg!(target_os = "windows")
        && modifiers.shift
        && keycode == winit::event::VirtualKeyCode::Delete); }
fn is_copy_command(modifiers: egui::Modifiers, keycode: winit::event::VirtualKeyCode) -> bool {
    return (modifiers.command && keycode == winit::event::VirtualKeyCode::C)
        || (cfg!(target_os = "windows")
        && modifiers.ctrl
        && keycode == winit::event::VirtualKeyCode::Insert); }
fn is_paste_command(modifiers: egui::Modifiers, keycode: winit::event::VirtualKeyCode) -> bool {
    return (modifiers.command && keycode == winit::event::VirtualKeyCode::V)
        || (cfg!(target_os = "windows")
        && modifiers.shift
        && keycode == winit::event::VirtualKeyCode::Insert); }

fn translate_keycode(key: VirtualKeyCode) -> Option<egui::Key> {
    use egui::Key;
    use winit::event::VirtualKeyCode::*;

    return Some(match key {
        Down  => Key::ArrowDown,
        Left  => Key::ArrowLeft,
        Right => Key::ArrowRight,
        Up    => Key::ArrowUp,

        Escape => Key::Escape,
        Tab    => Key::Tab,
        Back   => Key::Backspace,
        Return => Key::Enter,
        Space   => Key::Space,

        Insert   => Key::Insert,
        Delete   => Key::Delete,
        Home     => Key::Home,
        End      => Key::End,
        PageUp   => Key::PageUp,
        PageDown => Key::PageDown,

        Key0 | Numpad0 => Key::Num0,
        Key1 | Numpad1 => Key::Num1,
        Key2 | Numpad2 => Key::Num2,
        Key3 | Numpad3 => Key::Num3,
        Key4 | Numpad4 => Key::Num4,
        Key5 | Numpad5 => Key::Num5,
        Key6 | Numpad6 => Key::Num6,
        Key7 | Numpad7 => Key::Num7,
        Key8 | Numpad8 => Key::Num8,
        Key9 | Numpad9 => Key::Num9,

        A => Key::A,
        B => Key::B,
        C => Key::C,
        D => Key::D,
        E => Key::E,
        F => Key::F,
        G => Key::G,
        H => Key::H,
        I => Key::I,
        J => Key::J,
        K => Key::K,
        L => Key::L,
        M => Key::M,
        N => Key::N,
        O => Key::O,
        P => Key::P,
        Q => Key::Q,
        R => Key::R,
        S => Key::S,
        T => Key::T,
        U => Key::U,
        V => Key::V,
        W => Key::W,
        X => Key::X,
        Y => Key::Y,
        Z => Key::Z,
        
        _ => { return None; }
    })
}
fn translate_modifiers(modifiers: ModifiersState) -> egui::Modifiers {
    return egui::Modifiers {
        alt     : modifiers.alt(),
        ctrl    : modifiers.ctrl(),
        shift   : modifiers.shift(),
        mac_cmd : cfg!(target_os = "macos") && modifiers.logo(),
        command : if cfg!(target_os = "macos") { modifiers.logo() } else { modifiers.ctrl() }
    };
}
fn translate_mouse_button(button: MouseButton) -> Option<egui::PointerButton> {
    match button {
        winit::event::MouseButton::Left => Some(egui::PointerButton::Primary),
        winit::event::MouseButton::Right => Some(egui::PointerButton::Secondary),
        winit::event::MouseButton::Middle => Some(egui::PointerButton::Middle),
        winit::event::MouseButton::Other(_) => None,
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area =
           '\u{e000}'   <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}'  <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';

    return !is_in_private_use_area && !chr.is_ascii_control();
}

/// Wrapper around egui making it easy to implement interfaces.
/// Supply egui with event by calling appropriate methods when they occur.
pub struct EGUI {
    pub pps         : f32,
    pub pointer     : (f32, f32),
    pub modfiers    : ModifiersState,
    pub egui_ctx    : egui::Context,
    pub egui_input  : egui::RawInput,
    pub egui_rpass  : egui_wgpu::RenderPass,
    pub screen_desc : egui_wgpu::ScreenDescriptor,
}

impl EGUI {
    pub fn new(device: &wgpu::Device, surface_format: &wgpu::SurfaceConfiguration) -> Result<Self> {
        return Ok(Self {
            pps         : 1.0,
            pointer     : (0.0, 0.0),
            modfiers    : ModifiersState::default(),
            egui_ctx    : egui::Context::default(),
            egui_input  : egui::RawInput::default(),
            egui_rpass  : egui_wgpu::RenderPass::new(device, wgpu::TextureFormat::Bgra8UnormSrgb, 1),
            screen_desc : egui_wgpu::ScreenDescriptor {
                size_in_pixels   : [surface_format.width, surface_format.height],
                pixels_per_point : 1.0
            },
        });
    }

    pub fn render(&mut self, view: &wgpu::TextureView, queue: &wgpu::Queue, device: &wgpu::Device, run_ui: impl FnOnce(&egui::Context)) {
        // Preparation
        // Layout the GUI
        let output = self.egui_ctx.run(self.egui_input.clone(), run_ui);
        
        // Events have been processed, clear buffer
        self.egui_input.events.clear();


        // Rendering
        // Upload needed textures
        for (id, image_delta) in &output.textures_delta.set {
            self.egui_rpass.update_texture(device, queue, *id, image_delta);
        }
        
        // Generate vertices for the GUI and render them
        let paint_jobs = self.egui_ctx.tessellate(output.shapes);
        self.egui_rpass.update_buffers(device, queue, &paint_jobs, &self.screen_desc);
        utils::submit(&queue, device, |encoder| {
            self.egui_rpass.execute(encoder, view, &paint_jobs, &self.screen_desc, None);
        });


        // Cleanup
        // Free the textures
        for id in &output.textures_delta.free {
            self.egui_rpass.free_texture(id);
        }
    }

    // Prevent input from being passed to the next screen
    pub fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput      { input, .. }         => { self.on_key_input(input); }
            WindowEvent::ModifiersChanged   ( state )             => { self.modfiers = state.clone(); }
            WindowEvent::ReceivedCharacter  ( ch )                => { self.on_text_input(*ch) }
            WindowEvent::CursorMoved        { position, .. }      => { self.on_mouse_move(*position) }
            WindowEvent::MouseWheel         { delta, .. }         => { self.on_mouse_wheel(*delta); }
            WindowEvent::MouseInput         { state, button, .. } => { self.on_mouse_input(*state, *button) }
            WindowEvent::CursorLeft         { .. }                => { self.egui_input.events.push(egui::Event::PointerGone); }
            WindowEvent::ScaleFactorChanged { scale_factor, .. }  => { self.egui_input.pixels_per_point = Some(*scale_factor as f32); }

            _ => {}
        }
    }

    // Gracefully handle resizes with egui
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.screen_desc = egui_wgpu::ScreenDescriptor {
            size_in_pixels   : [new_size.width, new_size.height],
            pixels_per_point : self.pps
        };
    }

    // Helpers
    fn on_key_input(&mut self, input: &KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            let key = translate_keycode(keycode);
            let pressed = input.state == ElementState::Pressed;

            if pressed {
                     if is_cut_command   (self.egui_input.modifiers, keycode) { self.egui_input.events.push(egui::Event::Cut); }
                else if is_copy_command  (self.egui_input.modifiers, keycode) { self.egui_input.events.push(egui::Event::Copy); }
                else if is_paste_command (self.egui_input.modifiers, keycode) {
                    // if let Some(contents) = self.clipboard.get() {
                    //     let contents = contents.replace("\r\n", "\n");
                    //     if !contents.is_empty() {
                    //         self.egui_input.events.push(egui::Event::Paste(contents));
                    //     }
                    // }
                }
            }

            if let Some(key) = key {
                let modifiers = translate_modifiers(self.modfiers);
                self.egui_input.events.push(egui::Event::Key { key, pressed, modifiers });
            }
        }
    }
    fn on_text_input(&mut self, ch: char) {
        let is_mac_cmd = cfg!(target_os = "macos") && (self.egui_input.modifiers.ctrl || self.egui_input.modifiers.mac_cmd);
        if is_printable_char(ch) && !is_mac_cmd { self.egui_input.events.push(egui::Event::Text(ch.to_string())); }
    }
    fn on_mouse_move(&mut self, position: PhysicalPosition<f64>) {
        let pos = (position.x as f32, position.y as f32);
        self.egui_input.events.push(egui::Event::PointerMoved(pos.into())); 
        self.pointer = pos;
    }
    fn on_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if let Some(button) = translate_mouse_button(button) {
            let pressed = state == ElementState::Pressed;
            self.egui_input.events.push(egui::Event::PointerButton {
                button,
                pressed,
                pos: self.pointer.into(),
                modifiers: self.egui_input.modifiers,
            });
        }
    }
    fn on_mouse_wheel(&mut self, delta: winit::event::MouseScrollDelta) {
        let mut delta = match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                let points_per_scroll_line = 50.0;
                egui::vec2(x, y) * points_per_scroll_line
            }
            winit::event::MouseScrollDelta::PixelDelta(delta) => {
                egui::vec2(delta.x as f32, delta.y as f32) / self.pps
            }
        };

        delta.x *= -1.0;
        if self.egui_input.modifiers.ctrl || self.egui_input.modifiers.command {
            let factor = (delta.y / 200.0).exp();
            self.egui_input.events.push(egui::Event::Zoom(factor));
        } else if self.egui_input.modifiers.shift {
            self.egui_input.events.push(egui::Event::Scroll(egui::vec2(delta.x + delta.y, 0.0)));
        } else {
            self.egui_input.events.push(egui::Event::Scroll(delta));
        }
    }
}
