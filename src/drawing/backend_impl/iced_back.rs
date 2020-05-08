use crate::drawing::backend::{BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind};
use crate::style::text_anchor::{HPos, VPos};
use crate::style::{Color, FontTransform, RGBAColor, TextStyle};

use std::rc::Rc;
use iced::canvas::{self, Canvas, Frame, Path, Stroke, Text, Program, Cache, State};
use iced::{Point, Rectangle, Size, VerticalAlignment, HorizontalAlignment};
use iced_native::{layout, Widget, Clipboard};
use iced_wgpu::Renderer;
use iced_core::Length;

use std::marker::PhantomData;
/// The backend that is drawing on the HTML canvas
/// TODO: Support double buffering
#[derive(Debug, Default, Clone)]
pub struct IcedCanvasBackend {
    pub canvas: canvas::Cache,
    pub geom: Vec<canvas::Geometry>,
    pub bounds: Rectangle
}

// impl<Message, P: Program<Message>> IcedCanvasBackend<Message, P> {
//     fn get_canvas_size<T: Widget<Message, Renderer>>(t: &T) -> (u32, u32) {
//         (t.width() as u32, t.height() as u32)
//     }
// }

pub struct IcedCanvasError(String);

impl std::fmt::Display for IcedCanvasError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return write!(fmt, "Iced Canvas Error: {}", self.0);
    }
}

impl std::fmt::Debug for IcedCanvasError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return write!(fmt, "Iced IcedCanvasError({})", self.0);
    }
}

// impl From<JsValue> for DrawingErrorKind<IcedCanvasError> {
//     fn from(e: JsValue) -> DrawingErrorKind<IcedCanvasError> {
//         DrawingErrorKind::DrawingError(IcedCanvasError(
//             JSON::stringify(&e)
//                 .map(|s| Into::<String>::into(&s))
//                 .unwrap_or_else(|_| "Unknown".to_string()),
//         ))
//     }
// }

impl std::error::Error for IcedCanvasError {}

impl IcedCanvasBackend {
    fn init_backend(canvas: canvas::Cache) -> Option<Self> {
        Some(IcedCanvasBackend { canvas, geom: Vec::new(), bounds: Rectangle::with_size(Size::new(100.0, 100.0))})
    }

    /// Create a new drawing backend backed with an HTML5 canvas object with given Id
    /// - `elem_id` The element id for the canvas
    /// - Return either some drawing backend has been created, or none in error case
    pub fn new(canvas: canvas::Cache) -> Option<Self> {;
        Self::init_backend(canvas)
    }

    /// Create a new drawing backend backend with a HTML5 canvas object passed in
    /// - `canvas` The object we want to use as backend
    /// - Return either the drawing backend or None for error
    pub fn with_canvas_object(canvas: canvas::Cache) -> Option<Self> {
        Self::init_backend(canvas)
    }
}

fn coord_to_point(be: BackendCoord) -> Point {
    Point { x: be.0 as f32, y: be.1 as f32}
}

fn color_convert(be: &impl BackendStyle) -> iced::Color {
    let c = be.as_color();
    color_main(c)
}

fn color_main(b: RGBAColor) -> iced::Color {
    let rgb = b.rgb();
    iced::Color { r: rgb.0 as f32, g: rgb.1 as f32, b: rgb.2 as f32, a: b.alpha() as f32}
}



impl DrawingBackend for IcedCanvasBackend
{
    type ErrorType = IcedCanvasError;

    fn get_size(&self) -> (u32, u32) {
        // Getting just canvas.width gives poor results on HighDPI screens.
        // IcedCanvasBackend::get_canvas_size(&self.canvas)
        // let state = json!{format!("{:?}", self.canvas)};
        println!("{:?}", self.bounds);
        (self.bounds.width as u32, self.bounds.height as u32)
        // (0,0)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<IcedCanvasError>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<IcedCanvasError>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        style: &RGBAColor,
    ) -> Result<(), DrawingErrorKind<IcedCanvasError>> {
        if style.alpha() == 0.0 {
            return Ok(());
        }
        let size = self.get_size();
        let a = self.canvas.draw(Size::new(size.0 as f32, size.1 as f32), |frame| {
            let pixel = Path::rectangle(Point::new(point.0 as f32, point.1 as f32), Size::new(1.0, 1.0));
            frame.fill(&pixel, color_main(style.clone()))
        });
        self.geom.push(a);
        println!("{:?}", self.geom);
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.as_color().alpha() == 0.0 {
            return Ok(());
        }
        let size = self.get_size();
        let a = self.canvas.draw(Size::new(size.0 as f32, size.1 as f32), |frame| {
            let line = Path::line(coord_to_point(from), coord_to_point(to));
            frame.stroke(
                &line,
                Stroke {
                    width: style.stroke_width() as f32,
                    color: color_convert(style),
                    ..Stroke::default()
                })
        });
        self.geom.push(a);
        println!("{:?}", self.geom);
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.as_color().alpha() == 0.0 {
            return Ok(());
        }
        let size = self.get_size();
        let a = self.canvas.draw(Size::new(size.0 as f32, size.1 as f32), |frame| {
            let width = bottom_right.0 - upper_left.0;
            let height = bottom_right.1 - upper_left.1;
            let size = Size::new(width as f32, height as f32);
            let rect = Path::rectangle(coord_to_point(upper_left), size);
            if fill {
                frame.fill(&rect, color_convert(style));
            } else {
                frame.stroke(&rect, Stroke {
                    width: style.stroke_width() as f32,
                    color: color_convert(style),
                    ..Stroke::default()
                });
            }
        });
        self.geom.push(a);
        println!("{:?}", self.geom);
        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.as_color().alpha() == 0.0 {
            return Ok(());
        }
        let mut pa = path.into_iter();
        let finished_path = Path::new(|pat| {
            if let Some(start) = pa.next() {
                pat.move_to(Point::new(start.0 as f32, start.1 as f32));
                for p in pa {
                    pat.line_to(Point::new(p.0 as f32, p.1 as f32));
                }
            }
        });
        let size = self.get_size();
        let a = self.canvas.draw(Size::new(size.0 as f32, size.1 as f32), |frame| {
            frame.stroke(&finished_path, Stroke {
                width: style.stroke_width() as f32,
                color: color_convert(style),
                ..Stroke::default()
            });
        });
        self.geom.push(a);
        println!("{:?}", self.geom);
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.as_color().alpha() == 0.0 {
            return Ok(());
        }
        let mut pa = path.into_iter();
        let finished_path = Path::new(|pat| {
            if let Some(start) = pa.next() {
                pat.move_to(Point::new(start.0 as f32, start.1 as f32));
                for p in pa {
                    pat.line_to(Point::new(p.0 as f32, p.1 as f32));
                }
            }
            pat.close()
        });
        let size = self.get_size();
        let a = self.canvas.draw(Size::new(size.0 as f32, size.1 as f32), |frame| {
            frame.fill(&finished_path, color_convert(style));
        });
        self.geom.push(a);
        println!("{:?}", self.geom);
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.as_color().alpha() == 0.0 {
            return Ok(());
        }

        let size = self.get_size();
        let a = self.canvas.draw(Size::new(size.0 as f32, size.1 as f32), |frame| {
            let circ = Path::circle(
                coord_to_point(center),
                radius as f32
            );
            if fill {
                frame.fill(&circ, color_convert(style));
            } else {
                frame.stroke(&circ, Stroke {
                    width: style.stroke_width() as f32,
                    color: color_convert(style),
                    ..Stroke::default()
                });
            }
        });
        self.geom.push(a);
        println!("{:?}", self.geom);
        Ok(())
    }

    fn draw_text(
        &mut self,
        text: &str,
        style: &TextStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let font = &style.font;
        let color = &style.color;
        if color.alpha() == 0.0 {
            return Ok(());
        }

        let (mut x, mut y) = (pos.0, pos.1);

        // rotations not yet supported by iced
        let degree = 0.0;
        //  match font.get_transform() {
        //     FontTransform::None => 0.0,
        //     FontTransform::Rotate90 => 90.0,
        //     FontTransform::Rotate180 => 180.0,
        //     FontTransform::Rotate270 => 270.0,
        // } / 180.0
        //     * std::f64::consts::PI;

        // if degree != 0.0 {
        //     self.context.save();
        //     self.context.translate(f64::from(x), f64::from(y))?;
        //     self.context.rotate(degree)?;
        //     x = 0;
        //     y = 0;
        // }

        let text_baseline = match style.pos.v_pos {
            VPos::Top => VerticalAlignment::Top,
            VPos::Center => VerticalAlignment::Center,
            VPos::Bottom => VerticalAlignment::Bottom,
        };
        let text_align = match style.pos.h_pos {
            HPos::Left => HorizontalAlignment::Left,
            HPos::Right => HorizontalAlignment::Right,
            HPos::Center => HorizontalAlignment::Center,
        };
        let mut t = Text::from(text);
        t.vertical_alignment = text_baseline;
        t.horizontal_alignment = text_align;
        t.color = color_main(color.clone());
        // external fonts probably dont work because bytes?
        // t.font = Font {
        //     Font::External {
        //         name: font.get_name(),
        //         bytes: &[u8; 0],
        //     }
        // }
        t.size = font.get_size() as f32;

        let size = self.get_size();
        let a = self.canvas.draw(Size::new(size.0 as f32, size.1 as f32), |frame| {
            frame.fill_text(t.clone());
        });
        self.geom.push(a);
        println!("{:?}", self.geom);
        Ok(())
    }
}
