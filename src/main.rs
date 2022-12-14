extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;
use std::cell::RefCell;

use std::rc::Rc;

use gio::prelude::*;
use gtk::prelude::*;

use cairo::{Context, Extend, Filter, Format, ImageSurface, SurfacePattern};
use gdk::enums::key;
use gdk::{EventMask, ModifierType};
use gtk::{Application, ApplicationWindow, DrawingArea};

fn main() {
    let application = Application::new(Some("com.capital-ex.crisp"), Default::default())
        .expect("failed to initialize GTK");


    application.connect_activate(|app| {
        let texture_0875 = create_surface(vec![
            0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  
            0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,
        ]);

        let texture_0750 = create_surface(vec![
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00, 
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,
        ]);

        let texture_0625 = create_surface(vec![
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00, 
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,
        ]);

        let texture_0500 = create_surface(vec![
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF, 
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,
        ]);

        let texture_0375 = create_surface(vec![
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF, 
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00, 
            0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,
        ]);

        let texture_0250 = create_surface(vec![
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF, 
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00, 
            0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,
        ]);

        let texture_0125 = create_surface(vec![
            0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF, 
            0xFF, 0xFF, 0xFF, 0xFF,  0x00, 0x00, 0x00, 0x00,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF,
        ]);

        let window = ApplicationWindow::new(app);
        let drawing_area = DrawingArea::new();

        window.set_title("C R I S P");
        window.set_default_size(512, 512);

        let mut events = drawing_area.get_events();
        events.insert(EventMask::POINTER_MOTION_MASK);
        events.insert(EventMask::BUTTON_MOTION_MASK);
        events.insert(EventMask::BUTTON_PRESS_MASK);
        events.insert(EventMask::TABLET_PAD_MASK);
        drawing_area.set_events(events);

        let context = Rc::new(RefCell::new(Context::new(
            &ImageSurface::create(Format::Rgb24, 512, 512).unwrap(),
        )));
        context.borrow().set_source_rgb(1.0, 1.0, 1.0);
        context.borrow().rectangle(0.0, 0.0, 512.0, 512.0);
        context.borrow().fill();
        context.borrow().set_source_rgb(0.0, 0.0, 0.0);
        let old_pos = Rc::new(RefCell::new((0.0, 0.0)));
        let pen_size = Rc::new(RefCell::new(8.0));

        window.connect_key_press_event({

            let _context = context.clone();
            let _pen_size = pen_size.clone();
            move |_widget, event| {
                match event.get_keyval() {
                    key::_1 => _context.borrow().set_source_rgb(0.0, 0.0, 0.0),
                    key::_2 => _context.borrow().set_source(&texture_0875),
                    key::_3 => _context.borrow().set_source(&texture_0750),
                    key::_4 => _context.borrow().set_source(&texture_0625),
                    key::_5 => _context.borrow().set_source(&texture_0500),
                    key::_6 => _context.borrow().set_source(&texture_0375),
                    key::_7 => _context.borrow().set_source(&texture_0250),
                    key::_8 => _context.borrow().set_source(&texture_0125),
                    key::_9 => _context.borrow().set_source_rgb(1.0, 1.0, 1.0),
                    key::Shift_L => { 
                        let size = *_pen_size.borrow();
                        if size > 1.0 {
                            *_pen_size.borrow_mut() = 1.0;
                        } else {
                            *_pen_size.borrow_mut() = 8.0;
                        }
                    }
                    _ => {}
                }

                Inhibit(false)
            }
        });

        drawing_area.connect_motion_notify_event({
            let _context = context.clone();
            let _old_pos = old_pos.clone();
            let _pen_size = pen_size.clone();
            move |widget, event| {
                if event.get_state().contains(ModifierType::BUTTON1_MASK) {
                    let (old_x, old_y) = *_old_pos.borrow();
                    let (x, y) = event.get_position();
                    let dx = x - old_x;
                    let dy = y - old_y;
                    let __context = _context.borrow();
                    let size = *_pen_size.borrow();

                    if dx.abs() > size || dy.abs() > size{
                        if dx.abs() > dy.abs() {
                            let step_x = dx.abs() / size;
                            let step_y = dy.abs() / step_x;
                            let sign_y = if dy >= 0.0 { 1.0 } else { -1.0 };
                            let sign_x = if dx >= 0.0 { 1.0 } else { -1.0 };
                           
                            for i in 0 .. step_x.abs().ceil() as i64 * 2 {
                                let n = i as f64;
                                __context.rectangle(old_x + size / 2.0 * n * sign_x, old_y + step_y / 2.0 * n * sign_y, size, size);
                                __context.fill();
                            }
                        } else {
                            let step_y = dy.abs() / size;
                            let step_x = dx.abs() / step_y;
                            let sign_y = if dy >= 0.0 { 1.0 } else { -1.0 };
                            let sign_x = if dx >= 0.0 { 1.0 } else { -1.0 };
                           
                            for i in 0 .. step_y.abs().ceil() as i64 * 2{
                                let n = i as f64;
                                __context.rectangle(old_x + step_x / 2.0 * n * sign_x, old_y + size / 2.0 * n * sign_y, size, size);
                                __context.fill();
                            }
                        }

                    }

                    __context.rectangle(x, y, size, size);
                    __context.fill();

                    
                    
                    widget.queue_draw();
                }
                *_old_pos.borrow_mut() = event.get_position();
                Inhibit(false)
            }
        });

        drawing_area.connect_draw({
            let _context = context.clone();
            let _old_pos = old_pos.clone();
            move |widget, context| {
                context.set_source_surface(&_context.borrow().get_target(), 0.0, 0.0);
                context.rectangle(
                    0.0,
                    0.0,
                    widget.get_allocated_width().into(),
                    widget.get_allocated_height().into(),
                );
                context.fill();
                Inhibit(false)
            }
        });

        window.add(&drawing_area);
        window.show_all();
    });

    application.run(&[]);
}

fn create_surface(data: Vec<u8>) -> SurfacePattern {
    let surface = SurfacePattern::create(
        &ImageSurface::create_for_data(data,Format::Rgb24, 4, 4, Format::Rgb24.stride_for_width(4).unwrap()).unwrap()
    );
    surface.set_extend(Extend::Repeat);
    surface.set_filter(Filter::Nearest);
    surface
}
