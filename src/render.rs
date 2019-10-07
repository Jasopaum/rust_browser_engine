use crate::layout;

extern crate find_folder;
extern crate piston_window;

use piston_window::*;

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

fn render_text(window: &mut PistonWindow, e: &Event) {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
    window.draw_2d(e, |c, g, device| {
        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
            "Hello world!",
            &mut glyphs,
            &c.draw_state,
            c.transform.trans(10.0, 100.0), g
        ).unwrap();
        // Update glyphs before rendering.
        glyphs.factory.encoder.flush(device);
    });
}
