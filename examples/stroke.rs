/* *

#![allow(unused)]

use ink_stroke_modeler_rs::{
    ModelerError, ModelerInput, ModelerInputEventType, ModelerResult, StrokeModeler,
};
use svg::Node;

fn main() -> anyhow::Result<()> {
    let bounds = Aabb {
        mins: (0.0, 0.0),
        maxs: (300.0, 300.0),
    };

    let input_stroke = vec![
        ModelerInput::new(
            ModelerInputEventType::kDown,
            (90.0, 30.0),
            0.0,
            0.25,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (30.0, 45.0),
            0.02,
            0.3,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (60.0, 240.0),
            0.04,
            0.7,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (105.0, 270.0),
            0.06,
            1.0,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (120.0, 150.0),
            0.10,
            0.6,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (180.0, 30.0),
            0.12,
            0.3,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (240.0, 120.0),
            0.16,
            0.3,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (210.0, 150.0),
            0.18,
            0.9,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (150.0, 210.0),
            0.20,
            0.8,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (210.0, 240.0),
            0.22,
            0.8,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kMove,
            (255.0, 240.0),
            0.24,
            0.7,
            0.0,
            0.0,
        ),
        ModelerInput::new(
            ModelerInputEventType::kUp,
            (270.0, 270.0),
            0.26,
            0.5,
            0.0,
            0.0,
        ),
    ];
    let input_elements = input_stroke
        .iter()
        .map(Element::from_modeler_input)
        .collect::<Vec<Element>>();
    create_svg(
        &input_elements,
        bounds,
        std::path::PathBuf::from("./examples/stroke/input.svg"),
    )?;

    let mut modeler = StrokeModeler::default();

    let result_stroke = input_stroke
        .into_iter()
        .filter_map(|i| {
            modeler
                .update(i)
                .map_err(|e| eprintln!("modeler updated, Err: {e:?}"))
                .ok()
        })
        .flatten()
        .collect::<Vec<ModelerResult>>();

    let result_elements = result_stroke
        .iter()
        .map(Element::from_modeler_result)
        .collect::<Vec<Element>>();
    create_svg(
        &result_elements,
        bounds,
        std::path::PathBuf::from("./examples/stroke/modeled.svg"),
    )?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Element {
    pos: (f32, f32),
    velocity: Option<(f32, f32)>,
    time: f64,
    pressure: f32,
    tilt: f32,
    orientation: f32,
}

impl Element {
    fn from_modeler_input(i: &ModelerInput) -> Self {
        Self {
            pos: i.pos(),
            velocity: None,
            time: i.time(),
            pressure: i.pressure(),
            tilt: i.tilt(),
            orientation: i.orientation(),
        }
    }

    fn from_modeler_result(r: &ModelerResult) -> Self {
        Self {
            pos: r.pos(),
            velocity: Some(r.velocity()),
            time: r.time(),
            pressure: r.pressure(),
            tilt: r.tilt(),
            orientation: r.orientation(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Aabb {
    mins: (f32, f32),
    maxs: (f32, f32),
}

impl Aabb {
    fn new_invalid() -> Self {
        Self {
            mins: (f32::MAX, f32::MAX),
            maxs: (f32::MIN, f32::MIN),
        }
    }

    fn width(&self) -> f32 {
        self.maxs.0 - self.mins.0
    }
    fn height(&self) -> f32 {
        self.maxs.1 - self.mins.1
    }

    fn extend(&mut self, coord: (f32, f32)) {
        self.mins.0 = self.mins.0.min(coord.0);
        self.mins.1 = self.mins.1.min(coord.1);
        self.maxs.0 = self.maxs.0.max(coord.0);
        self.maxs.1 = self.maxs.1.max(coord.1);
    }
}

fn create_svg(
    elements: &[Element],
    bounds: Aabb,
    file: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    let mut doc = svg::Document::new()
        .set("x", bounds.mins.0)
        .set("y", bounds.mins.1)
        .set("width", bounds.width())
        .set("height", bounds.height());

    doc.append(
        svg::node::element::Rectangle::new()
            .set("x", bounds.mins.0)
            .set("y", bounds.mins.1)
            .set("width", bounds.width())
            .set("height", bounds.height())
            .set("fill", "white"),
    );

    for (start, end) in elements.iter().zip(elements.iter().skip(1)) {
        let brightness = 1.0 / (end.pressure + start.pressure) / 2.0;
        doc.append(
            svg::node::element::Line::new()
                .set("x1", start.pos.0)
                .set("y1", start.pos.1)
                .set("x2", end.pos.0)
                .set("y2", end.pos.1)
                .set(
                    "stroke",
                    format!("hsl(200, 100%, {}%", (brightness * 100.0).round()),
                )
                .set("stroke-width", 2.0)
                .set("stroke-linecap", "round"),
        );
    }

    Ok(svg::save(file, &doc)?)
}
*/