use std::{cell::RefCell, rc::Rc};

use gtk::prelude::{DrawingAreaExt, DrawingAreaExtManual, WidgetExt};

pub fn spectrum(
    spectrum_values: Rc<RefCell<Vec<f64>>>,
    width: i32,
    height: i32,
) -> gtk::DrawingArea {
    let spectrum = gtk::DrawingArea::new();
    spectrum.set_widget_name("spectrum");
    spectrum.set_content_width(width);
    spectrum.set_content_height(height);

    spectrum.set_draw_func(move |_, ctx, width, height| {
        let values = spectrum_values.borrow();
        let gap = 3.0;
        let count = values.len().max(1) as f64;

        let bar_width = ((width as f64 - gap * (count - 1.0)) / count).max(2.0);
        ctx.set_source_rgba(0.54, 0.71, 0.98, 0.95);
        for (index, value) in values.iter().enumerate() {
            let value = (*value).clamp(0.0, 1.0);
            let bar_height = (height as f64 * value).max(2.0);
            let x = index as f64 * (bar_width + gap);
            let y = height as f64 - bar_height;
            ctx.rectangle(x, y, bar_width, bar_height);
            let _ = ctx.fill();
        }
    });
    spectrum
}
