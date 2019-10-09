use crate::layout;
use crate::dom;
use crate::css;

extern crate find_folder;
extern crate piston_window;

use piston_window::*;
use textwrap::wrap_iter;

pub fn render_layout_tree(layout_tree: &layout::LayoutBox) {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |_, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
        });
        render_box(&mut window, &e, &layout_tree);
    }
}

fn render_box(window: &mut PistonWindow, e: &Event, layout_box: &layout::LayoutBox) {
    match layout_box.box_type {
        layout::BoxType::TextNode => render_text(window, e, layout_box),
        _ => render_area(window, e, layout_box),
    }
    for child in &layout_box.children {
        render_box(window, e, child);
    }
}

fn render_area(window: &mut PistonWindow, e: &Event, layout_box: &layout::LayoutBox) {
    window.draw_2d(e, |c, g, _| {
        let white = [1.0, 1.0, 1.0, 1.0];
        let black = [0.0, 0.0, 0.0, 1.0];
        let rect = math::margin_rectangle(
            [
                layout_box.dimensions.content.x as f64,
                layout_box.dimensions.content.y as f64,
                layout_box.dimensions.content.width as f64,
                layout_box.dimensions.content.height as f64,
            ], 0.0
        );
        rectangle(white, rect, c.transform, g);
        Rectangle::new_border(black, 2.0).draw(rect, &c.draw_state, c.transform, g);
    });
}

fn render_text(window: &mut PistonWindow, e: &Event, layout_box: &layout::LayoutBox) {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    let to_write = layout_box.get_text().unwrap();
    let pos_x = layout_box.dimensions.content.x as f64;
    let pos_y = layout_box.dimensions.content.y as f64;

    let default_font_size = css::Value::Length(12.0, css::Unit::Px);
    let font_size = layout_box.styled_node.get_property("font-size")
                                           .unwrap_or(&default_font_size)
                                           .to_px();
    let width_char = font_size / 2.0;
    let nb_chars_per_line = layout_box.dimensions.content.width as usize / width_char as usize;

    for (i, line) in wrap_iter(to_write, nb_chars_per_line).enumerate() {
        window.draw_2d(e, |c, g, device| {
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 12).draw(
                &line,
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(pos_x, pos_y + i as f64 * font_size as f64), g
            ).unwrap();
            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}
