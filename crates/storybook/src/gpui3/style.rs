use refineable::Refineable;

pub use super::taffy::style::{
    AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
    Overflow, Position,
};
use super::{
    rems, AbsoluteLength, Bounds, DefiniteLength, Edges, EdgesRefinement, Hsla, Length, Pixels,
    Point, PointRefinement, Rems, SharedString, Size, SizeRefinement, ViewContext, WindowContext,
};
pub use gpui2::style::{FontStyle, FontWeight};

#[derive(Clone, Debug)]
pub struct FontSize(f32);

#[derive(Clone, Refineable, Debug)]
#[refineable(debug)]
pub struct Style {
    /// What layout strategy should be used?
    pub display: Display,

    // Overflow properties
    /// How children overflowing their container should affect layout
    #[refineable]
    pub overflow: Point<Overflow>,
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    pub scrollbar_width: f32,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position: Position,
    /// How should the position of this element be tweaked relative to the layout defined?
    #[refineable]
    pub inset: Edges<Length>,

    // Size properies
    /// Sets the initial size of the item
    #[refineable]
    pub size: Size<Length>,
    /// Controls the minimum size of the item
    #[refineable]
    pub min_size: Size<Length>,
    /// Controls the maximum size of the item
    #[refineable]
    pub max_size: Size<Length>,
    /// Sets the preferred aspect ratio for the item. The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    // Spacing Properties
    /// How large should the margin be on each side?
    #[refineable]
    pub margin: Edges<Length>,
    /// How large should the padding be on each side?
    #[refineable]
    pub padding: Edges<DefiniteLength>,
    /// How large should the border be on each side?
    #[refineable]
    pub border_widths: Edges<AbsoluteLength>,

    // Alignment properties
    /// How this node's children aligned in the cross/block axis?
    pub align_items: Option<AlignItems>,
    /// How this node should be aligned in the cross/block axis. Falls back to the parents [`AlignItems`] if not set
    pub align_self: Option<AlignSelf>,
    /// How should content contained within this item be aligned in the cross/block axis
    pub align_content: Option<AlignContent>,
    /// How should contained within this item be aligned in the main/inline axis
    pub justify_content: Option<JustifyContent>,
    /// How large should the gaps between items in a flex container be?
    #[refineable]
    pub gap: Size<DefiniteLength>,

    // Flexbox properies
    /// Which direction does the main axis flow in?
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: FlexWrap,
    /// Sets the initial main axis size of the item
    pub flex_basis: Length,
    /// The relative rate at which this item grows when it is expanding to fill space, 0.0 is the default value, and this value must be positive.
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space, 1.0 is the default value, and this value must be positive.
    pub flex_shrink: f32,

    /// The fill color of this element
    pub fill: Option<Fill>,

    /// The border color of this element
    pub border_color: Option<Hsla>,

    /// The radius of the corners of this element
    #[refineable]
    pub corner_radii: CornerRadii,

    /// The color of text within this element. Cascades to children unless overridden.
    pub text_color: Option<Hsla>,

    /// The font size in rems.
    pub font_size: Option<Rems>,

    pub font_family: Option<SharedString>,

    pub font_weight: Option<FontWeight>,

    pub font_style: Option<FontStyle>,
}

#[derive(Refineable, Clone, Debug)]
#[refineable(debug)]
pub struct TextStyle {
    pub color: Hsla,
    pub font_family: SharedString,
    pub font_size: Rems,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub underline: Underline,
}

#[derive(Refineable, Clone, Default, Debug)]
#[refineable(debug)]
pub struct Underline {
    pub origin: Point<Pixels>,
    pub width: Pixels,
    pub thickness: Pixels,
    pub color: Hsla,
    pub squiggly: bool,
}

impl Style {
    pub fn text_style(&self, cx: &WindowContext) -> Option<TextStyleRefinement> {
        if self.text_color.is_none()
            && self.font_size.is_none()
            && self.font_family.is_none()
            && self.font_weight.is_none()
            && self.font_style.is_none()
        {
            return None;
        }

        Some(TextStyleRefinement {
            color: self.text_color,
            font_family: self.font_family.clone(),
            font_size: self.font_size,
            font_weight: self.font_weight,
            font_style: self.font_style,
            underline: None,
        })
    }

    /// Paints the background of an element styled with this style.
    pub fn paint_background<V: 'static>(&self, bounds: Bounds<Pixels>, cx: &mut ViewContext<V>) {
        let rem_size = cx.rem_size();
        if let Some(color) = self.fill.as_ref().and_then(Fill::color) {
            todo!();
        }
    }

    /// Paints the foreground of an element styled with this style.
    pub fn paint_foreground<V: 'static>(&self, bounds: Bounds<Pixels>, cx: &mut ViewContext<V>) {
        let rem_size = cx.rem_size();

        if let Some(color) = self.border_color {
            let border = self.border_widths.to_pixels(rem_size);
            if !border.is_empty() {
                todo!();
            }
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            display: Display::Block,
            overflow: Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            },
            scrollbar_width: 0.0,
            position: Position::Relative,
            inset: Edges::auto(),
            margin: Edges::<Length>::zero(),
            padding: Edges::<DefiniteLength>::zero(),
            border_widths: Edges::<AbsoluteLength>::zero(),
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: None,
            gap: Size::zero(),
            // Aligment
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
            // Flexbox
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            fill: None,
            border_color: None,
            corner_radii: CornerRadii::default(),
            text_color: None,
            font_size: Some(rems(1.)),
            font_family: None,
            font_weight: None,
            font_style: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Fill {
    Color(Hsla),
}

impl Fill {
    pub fn color(&self) -> Option<Hsla> {
        match self {
            Fill::Color(color) => Some(*color),
        }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Hsla::default())
    }
}

impl From<Hsla> for Fill {
    fn from(color: Hsla) -> Self {
        Self::Color(color)
    }
}

#[derive(Clone, Refineable, Default, Debug)]
#[refineable(debug)]
pub struct CornerRadii {
    top_left: AbsoluteLength,
    top_right: AbsoluteLength,
    bottom_left: AbsoluteLength,
    bottom_right: AbsoluteLength,
}
