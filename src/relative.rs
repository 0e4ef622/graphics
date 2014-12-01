use internal::ColorComponent;
use context::{ Transform, GetTransform, SetTransform };
use context::{ ViewTransform, GetViewTransform, SetViewTransform };
use can::{
    CanColor,
    CanRectangle,
    CanSourceRectangle,
};
use has::{
    HasColor,
    HasRectangle,
    HasSourceRectangle,
};
use vecmath::{
    get_scale,
    hsv,
    identity,
    margin_rectangle,
    multiply,
    orient,
    relative_rectangle,
    relative_source_rectangle,
    rotate_radians,
    scale,
    shear,
    translate,
    Matrix2d,
    Scalar,
    Vec2d,
};
use radians::Radians;

/// Implemented by contexts that contains color.
pub trait RelativeColor: HasColor + CanColor {
    /// Multiplies with red, green, blue and alpha values.
    #[inline(always)]
    fn mul_rgba(
        &self,
        r: ColorComponent,
        g: ColorComponent,
        b: ColorComponent,
        a: ColorComponent
    ) -> Self {
        let color = self.get_color();
        self.color([color[0] * r, color[1] * g, color[2] * b, color[3] * a])
    }

    /// Mixes the current color with white.
    ///
    /// 0 is black and 1 is white.
    #[inline(always)]
    fn tint(&self, f: ColorComponent) -> Self {
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Mixes the current color with black.
    ///
    /// 0 is white and 1 is black.
    #[inline(always)]
    fn shade(&self, f: ColorComponent) -> Self {
        let f = 1.0 - f;
        self.mul_rgba(f, f, f, 1.0)
    }

    /// Rotates hue by degrees.
    #[inline(always)]
    fn hue_deg(&self, angle: ColorComponent) -> Self {
        let pi: ColorComponent = Radians::_180();
        self.hue_rad(angle * pi / 180.0)
    }

    /// Rotates hue by radians.
    #[inline(always)]
    fn hue_rad(&self, angle: ColorComponent) -> Self {
        self.color(hsv(self.get_color(), angle, 1.0, 1.0))
    }
}

impl<T: HasColor + CanColor> RelativeColor for T {}

/// Should be implemented by contexts that have rectangle information.
pub trait RelativeRectangle: HasRectangle + CanRectangle {
    /// Shrinks the current rectangle equally by all sides.
    #[inline(always)]
    fn margin(&self, m: Scalar) -> Self {
        self.rectangle(margin_rectangle(self.get_rectangle(), m))
    }

    /// Expands the current rectangle equally by all sides.
    #[inline(always)]
    fn expand(&self, m: Scalar) -> Self {
        self.margin(-m)
    }

    /// Moves to a relative rectangle using the current rectangle as tile.
    #[inline(always)]
    fn rel(&self, x: Scalar, y: Scalar) -> Self {
        self.rectangle(relative_rectangle(self.get_rectangle(), [x, y]))
    }
}

impl<T: HasRectangle + CanRectangle> RelativeRectangle for T {}

/// Should be implemented by contexts that
/// have source rectangle information.
pub trait RelativeSourceRectangle: HasSourceRectangle + CanSourceRectangle {
    /// Adds a source rectangle.
    #[inline(always)]
    fn src_rect(&self, x: i32, y: i32, w: i32, h: i32) -> Self {
        self.source_rectangle([x, y, w, h])
    }

    /// Moves to a relative source rectangle using
    /// the current source rectangle as tile.
    #[inline(always)]
    fn src_rel(&self, x: i32, y: i32) -> Self {
        self.source_rectangle(
            relative_source_rectangle(self.get_source_rectangle(), x, y)
        )
    }

    /// Flips the source rectangle horizontally.
    #[inline(always)]
    fn src_flip_h(&self) -> Self {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1],
            -source_rect[2],
            source_rect[3]
        ])
    }

    /// Flips the source rectangle vertically.
    #[inline(always)]
    fn src_flip_v(&self) -> Self {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0],
            source_rect[1] + source_rect[3],
            source_rect[2],
            -source_rect[3]
        ])
    }

    /// Flips the source rectangle horizontally and vertically.
    #[inline(always)]
    fn src_flip_hv(&self) -> Self {
        let source_rect = self.get_source_rectangle();
        self.source_rectangle([
            source_rect[0] + source_rect[2],
            source_rect[1] + source_rect[3],
            -source_rect[2],
            -source_rect[3]
        ])
    }
}

impl<T: HasSourceRectangle
      + CanSourceRectangle,
> RelativeSourceRectangle for T {}

/// Implemented by contexts that can transform.
pub trait RelativeTransform: GetTransform + SetTransform + Clone {
    /// Appends transform to the current one.
    #[inline(always)]
    fn append_transform(&self, transform: Matrix2d) -> Self {
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_transform(Transform(multiply(mat, transform)));
        res
    }

    /// Prepends transform to the current one.
    #[inline(always)]
    fn prepend_transform(&self, transform: Matrix2d) -> Self {
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_transform(Transform(multiply(transform, mat)));
        res
    }

    /// Translate x an y in local coordinates.
    #[inline(always)]
    fn trans(&self, x: Scalar, y: Scalar) -> Self {
        let trans = translate([x, y]);
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_transform(Transform(multiply(mat, trans)));
        res
    }

    /// Rotates degrees in local coordinates.
    #[inline(always)]
    fn rot_deg(&self, angle: Scalar) -> Self {
        let pi: Scalar = Radians::_180();
        self.rot_rad(angle * pi / 180.0)
    }

    /// Rotate radians in local coordinates.
    #[inline(always)]
    fn rot_rad(&self, angle: Scalar) -> Self {
        let rot = rotate_radians(angle);
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_transform(Transform(multiply(mat, rot)));
        res
    }

    /// Orients x axis to look at point locally.
    ///
    /// Leaves x axis unchanged if the point to
    /// look at is the origin.
    #[inline(always)]
    fn orient(&self, x: Scalar, y: Scalar) -> Self {
        let orient = orient(x, y);
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_transform(Transform(multiply(mat, orient)));
        res
    }

    /// Scales in local coordinates.
    #[inline(always)]
    fn scale(&self, sx: Scalar, sy: Scalar) -> Self {
        let scale = scale(sx, sy);
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_transform(Transform(multiply(mat, scale)));
        res
    }

    /// Scales in both directions in local coordinates.
    #[inline(always)]
    fn zoom(&self, s: Scalar) -> Self {
        self.scale(s, s)
    }

    /// Flips vertically in local coordinates.
    #[inline(always)]
    fn flip_v(&self) -> Self {
        self.scale(1.0, -1.0)
    }

    /// Flips horizontally in local coordinates.
    #[inline(always)]
    fn flip_h(&self) -> Self {
        self.scale(-1.0, 1.0)
    }

    /// Flips horizontally and vertically in local coordinates.
    #[inline(always)]
    fn flip_hv(&self) -> Self {
        self.scale(-1.0, -1.0)
    }

    /// Shears in local coordinates.
    #[inline(always)]
    fn shear(&self, v: Vec2d) -> Self {
        let shear = shear(v);
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_transform(Transform(multiply(mat, shear)));
        res
    }
}

impl<T: GetTransform + SetTransform + Clone> RelativeTransform for T {}

/// Should be implemented by contexts that
/// draws something relative to view.
pub trait RelativeViewTransform:
    GetViewTransform + SetViewTransform
  + GetTransform + SetTransform
  + Clone
{
    /// Moves the current transform to the view coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the upper left corner
    /// with the x axis pointing to the right
    /// and the y axis pointing down.
    #[inline(always)]
    fn view(&self) -> Self {
        let mut res = self.clone();
        let ViewTransform(mat) = self.get_view_transform();
        res.set_transform(Transform(mat));
        res
    }

    /// Moves the current transform to the default coordinate system.
    ///
    /// This is usually [0.0, 0.0] in the center
    /// with the x axis pointing to the right
    /// and the y axis pointing up.
    #[inline(always)]
    fn reset(&self) -> Self {
        let mut res = self.clone();
        res.set_transform(Transform(identity()));
        res
    }

    /// Stores the current transform as new view.
    #[inline(always)]
    fn store_view(&self) -> Self {
        let mut res = self.clone();
        let Transform(mat) = self.get_transform();
        res.set_view_transform(ViewTransform(mat));
        res
    }

    /// Computes the current view size.
    #[inline(always)]
    fn get_view_size(&self) -> (Scalar, Scalar) {
        let ViewTransform(mat) = self.get_view_transform();
        let scale = get_scale(mat);
        (2.0 / scale[0], 2.0 / scale[1])
    }
}

impl<
    T: GetViewTransform
     + GetTransform
     + SetViewTransform
     + SetTransform
     + Clone
> RelativeViewTransform for T {}

