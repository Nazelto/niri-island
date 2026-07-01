use std::{cell::RefCell, rc::Rc, sync::mpsc::Receiver, time::Duration};

use gtk::{
    DrawingArea,
    glib::{ControlFlow, SourceId},
    prelude::WidgetExt,
};

use crate::audio::SpectrumFrame;

const FRAME_INTERVAL: Duration = Duration::from_millis(16);

fn spawn_task<F>(interval: Duration, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    gtk::glib::timeout_add_local(interval, func)
}

pub fn spectrum_task(
    spectrums: Vec<DrawingArea>,
    spectrum_values: Rc<RefCell<Vec<f64>>>,
    rx: Receiver<SpectrumFrame>,
) {
    spawn_task(FRAME_INTERVAL, move || {
        while let Ok(spectrum_frame) = rx.try_recv() {
            *spectrum_values.borrow_mut() = spectrum_frame.bars;
            for spectrum in &spectrums {
                spectrum.queue_draw();
            }
        }
        ControlFlow::Continue
    });
}

pub fn source_task(title: gtk::Label, subtext: gtk::Label, fallback: String) {
    title.set_text(&String::from(crate::app::App::Default));
    subtext.set_text(&fallback);
    spawn_task(Duration::from_secs(1), move || {
        let (app_name, label) = crate::media::current_source(|source| {
            let app_name: String = source.app.into();
            (app_name, source.label)
        })
        .unwrap_or_else(|| (String::from(crate::app::App::Default), fallback.clone()));

        title.set_text(&app_name);
        subtext.set_text(&label);
        gtk::glib::ControlFlow::Continue
    });
}
