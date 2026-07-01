use anyhow::{Context, bail};
use gstreamer::{
    self as gst, Pipeline,
    glib::object::Cast,
    prelude::{ElementExt, GstObjectExt},
};
use std::sync::mpsc::Sender;
#[derive(Debug, Clone)]
// 一帧的频谱容器
pub struct SpectrumFrame {
    pub bars: Vec<f64>,
}

pub static BAR_COUNT: i32 = 24;
static THRESHOLD_DB: f64 = -80.0;

// 音频线程
pub fn spawn_spectrum(tx: Sender<SpectrumFrame>, source: Option<String>) {
    std::thread::spawn(move || {
        if let Err(err) = run_spectrum(tx, source.as_deref()) {
            eprintln!("audio spectrum failed: {err}");
        }
    });
}

fn run_spectrum(tx: Sender<SpectrumFrame>, source: Option<&str>) -> anyhow::Result<()> {
    gst::init()?;
    // 音频设备
    let source_device = source
        .map(|source| format!("device={source}"))
        .unwrap_or_default();
    let pipeline_text = format!(
        "pulsesrc {source_device} \
        ! audioconvert \
        ! audioresample \
        ! spectrum bands={BAR_COUNT} threshold={THRESHOLD_DB} post-messages=true interval=30000000 \
        ! fakesink sync=false
        "
    );
    let pipeline = gst::parse::launch(&pipeline_text)?
        .downcast::<Pipeline>()
        .map_err(|_| anyhow::anyhow!("failed downcast to pipeline"))?;
    pipeline.set_state(gst::State::Playing)?;
    // 获得总线
    let bus = pipeline.bus().context("pipeline has no bus")?;

    //从总线读取信息
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Element(element) => {
                let Some(structure) = element.structure() else {
                    continue;
                };
                if structure.name() != "spectrum" {
                    continue;
                }
                let Ok(magnitude) = structure.get::<gst::List>("magnitude") else {
                    continue;
                };
                let bars = magnitude
                    .iter()
                    .filter_map(|value| {
                        value
                            .get::<f64>()
                            .ok()
                            .or_else(|| value.get::<f32>().ok().map(f64::from))
                    })
                    .map(db_to_unit)
                    .collect();
                let _ = tx.send(SpectrumFrame { bars });
            }
            gst::MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                bail!(
                    "gstreamer error from {:?}: {}",
                    err.src().map(|src| src.path_string()),
                    err.error()
                );
            }
            gst::MessageView::Eos(_) => break,
            _ => {}
        }
    }
    pipeline.set_state(gst::State::Null)?;
    Ok(())
}

// 归一化
fn db_to_unit(db: f64) -> f64 {
    ((db - THRESHOLD_DB) / -THRESHOLD_DB).clamp(0.0, 1.0)
}
