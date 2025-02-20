//! Simple 2D library that support SVG path parsing/generation/manipulation and rasterization.
//!
//! Main features:
//!  - SVG path parsing and generation
//!  - Anit-aliased rendering
//!  - Path offsetting
//!
#![deny(warnings)]

mod color;
mod curve;
mod ellipse;
mod geometry;
mod grad;
mod image;
mod path;
mod rasterize;
mod scene;
mod svg;
pub mod utils;

pub use crate::rasterize::{
    ActiveEdgeIter, ActiveEdgeRasterizer, Paint, Pixel, Rasterizer, SignedDifferenceRasterizer,
    Size, Units,
};
pub use color::{linear_to_srgb, srgb_to_linear, Color, ColorU8, LinColor};
pub use curve::{
    Cubic, Curve, CurveExtremities, CurveFlattenIter, CurveRoots, Line, Quad, Segment,
};
pub use ellipse::EllipArc;
pub use geometry::{scalar_fmt, Align, BBox, Point, Scalar, Transform, EPSILON, EPSILON_SQRT, PI};
pub use grad::{GradLinear, GradRadial, GradSpread, GradStop, GradStops};
pub use image::{
    Image, ImageIter, ImageMut, ImageMutIter, ImageMutRef, ImageOwned, ImageRef, Shape,
};
pub use path::{
    FillRule, LineCap, LineJoin, Path, PathBuilder, StrokeStyle, SubPath, DEFAULT_FLATNESS,
};
pub use scene::{Layer, Scene};
pub use svg::{SvgPathCmd, SvgPathParser, SvgPathParserError};
