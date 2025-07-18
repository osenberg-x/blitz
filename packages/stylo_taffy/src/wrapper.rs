use crate::convert;
use std::ops::Deref;
use style::properties::ComputedValues;
use taffy::prelude::FromLength;

/// A wrapper struct for anything that `Deref`s to a [`stylo::ComputedValues`](ComputedValues) (can be pointed to by an `&` reference, [`Arc`](std::sync::Arc),
/// [`Ref`](std::cell::Ref), etc). It implements [`taffy`]'s [layout traits](taffy::traits) and can used with Taffy's [layout algorithms](taffy::compute).
pub struct TaffyStyloStyle<T: Deref<Target = ComputedValues>>(pub T);

// Deref<stylo::ComputedValues> impl
impl<T: Deref<Target = ComputedValues>> From<T> for TaffyStyloStyle<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

// Into<taffy::Style> impl
impl<T: Deref<Target = ComputedValues>> From<TaffyStyloStyle<T>> for taffy::Style {
    fn from(value: TaffyStyloStyle<T>) -> Self {
        convert::to_taffy_style(&value.0)
    }
}

// CoreStyle impl
impl<T: Deref<Target = ComputedValues>> taffy::CoreStyle for TaffyStyloStyle<T> {
    #[inline]
    fn box_generation_mode(&self) -> taffy::BoxGenerationMode {
        convert::box_generation_mode(self.0.get_box().display)
    }

    #[inline]
    fn is_block(&self) -> bool {
        convert::is_block(self.0.get_box().display)
    }

    #[inline]
    fn box_sizing(&self) -> taffy::BoxSizing {
        convert::box_sizing(self.0.get_position().box_sizing)
    }

    #[inline]
    fn overflow(&self) -> taffy::Point<taffy::Overflow> {
        let box_styles = self.0.get_box();
        taffy::Point {
            x: convert::overflow(box_styles.overflow_x),
            y: convert::overflow(box_styles.overflow_y),
        }
    }

    #[inline]
    fn scrollbar_width(&self) -> f32 {
        0.0
    }

    #[inline]
    fn position(&self) -> taffy::Position {
        convert::position(self.0.get_box().position)
    }

    #[inline]
    fn inset(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        let position_styles = self.0.get_position();
        taffy::Rect {
            left: convert::inset(&position_styles.left),
            right: convert::inset(&position_styles.right),
            top: convert::inset(&position_styles.top),
            bottom: convert::inset(&position_styles.bottom),
        }
    }

    #[inline]
    fn size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: convert::dimension(&position_styles.width),
            height: convert::dimension(&position_styles.height),
        }
    }

    #[inline]
    fn min_size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: convert::dimension(&position_styles.min_width),
            height: convert::dimension(&position_styles.min_height),
        }
    }

    #[inline]
    fn max_size(&self) -> taffy::Size<taffy::Dimension> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: convert::max_size_dimension(&position_styles.max_width),
            height: convert::max_size_dimension(&position_styles.max_height),
        }
    }

    #[inline]
    fn aspect_ratio(&self) -> Option<f32> {
        convert::aspect_ratio(self.0.get_position().aspect_ratio)
    }

    #[inline]
    fn margin(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        let margin_styles = self.0.get_margin();
        taffy::Rect {
            left: convert::margin(&margin_styles.margin_left),
            right: convert::margin(&margin_styles.margin_right),
            top: convert::margin(&margin_styles.margin_top),
            bottom: convert::margin(&margin_styles.margin_bottom),
        }
    }

    #[inline]
    fn padding(&self) -> taffy::Rect<taffy::LengthPercentage> {
        let padding_styles = self.0.get_padding();
        taffy::Rect {
            left: convert::length_percentage(&padding_styles.padding_left.0),
            right: convert::length_percentage(&padding_styles.padding_right.0),
            top: convert::length_percentage(&padding_styles.padding_top.0),
            bottom: convert::length_percentage(&padding_styles.padding_bottom.0),
        }
    }

    #[inline]
    fn border(&self) -> taffy::Rect<taffy::LengthPercentage> {
        let border_styles = self.0.get_border();
        taffy::Rect {
            left: taffy::LengthPercentage::from_length(border_styles.border_left_width.to_f32_px()),
            right: taffy::LengthPercentage::from_length(
                border_styles.border_right_width.to_f32_px(),
            ),
            top: taffy::LengthPercentage::from_length(border_styles.border_top_width.to_f32_px()),
            bottom: taffy::LengthPercentage::from_length(
                border_styles.border_bottom_width.to_f32_px(),
            ),
        }
    }
}

// BlockContainerStyle impl
#[cfg(feature = "block")]
impl<T: Deref<Target = ComputedValues>> taffy::BlockContainerStyle for TaffyStyloStyle<T> {
    #[inline]
    fn text_align(&self) -> taffy::TextAlign {
        convert::text_align(self.0.clone_text_align())
    }
}

// BlockItemStyle impl
#[cfg(feature = "block")]
impl<T: Deref<Target = ComputedValues>> taffy::BlockItemStyle for TaffyStyloStyle<T> {
    #[inline]
    fn is_table(&self) -> bool {
        convert::is_table(self.0.clone_display())
    }
}

// FlexboxContainerStyle impl
#[cfg(feature = "flexbox")]
impl<T: Deref<Target = ComputedValues>> taffy::FlexboxContainerStyle for TaffyStyloStyle<T> {
    #[inline]
    fn flex_direction(&self) -> taffy::FlexDirection {
        convert::flex_direction(self.0.get_position().flex_direction)
    }

    #[inline]
    fn flex_wrap(&self) -> taffy::FlexWrap {
        convert::flex_wrap(self.0.get_position().flex_wrap)
    }

    #[inline]
    fn gap(&self) -> taffy::Size<taffy::LengthPercentage> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: convert::gap(&position_styles.column_gap),
            height: convert::gap(&position_styles.row_gap),
        }
    }

    #[inline]
    fn align_content(&self) -> Option<taffy::AlignContent> {
        convert::content_alignment(self.0.get_position().align_content.0)
    }

    #[inline]
    fn align_items(&self) -> Option<taffy::AlignItems> {
        convert::item_alignment(self.0.get_position().align_items.0)
    }

    #[inline]
    fn justify_content(&self) -> Option<taffy::JustifyContent> {
        convert::content_alignment(self.0.get_position().justify_content.0)
    }
}

// FlexboxItemStyle impl
#[cfg(feature = "flexbox")]
impl<T: Deref<Target = ComputedValues>> taffy::FlexboxItemStyle for TaffyStyloStyle<T> {
    #[inline]
    fn flex_basis(&self) -> taffy::Dimension {
        convert::flex_basis(&self.0.get_position().flex_basis)
    }

    #[inline]
    fn flex_grow(&self) -> f32 {
        self.0.get_position().flex_grow.0
    }

    #[inline]
    fn flex_shrink(&self) -> f32 {
        self.0.get_position().flex_shrink.0
    }

    #[inline]
    fn align_self(&self) -> Option<taffy::AlignSelf> {
        convert::item_alignment(self.0.get_position().align_self.0.0)
    }
}

// GridContainerStyle impl
#[cfg(feature = "grid")]
impl<T: Deref<Target = ComputedValues>> taffy::GridContainerStyle for TaffyStyloStyle<T> {
    type TemplateTrackList<'a>
        = Vec<taffy::TrackSizingFunction>
    where
        Self: 'a;
    type AutoTrackList<'a>
        = Vec<taffy::NonRepeatedTrackSizingFunction>
    where
        Self: 'a;

    #[inline]
    fn grid_template_rows(&self) -> Self::TemplateTrackList<'_> {
        convert::grid_template_tracks(&self.0.get_position().grid_template_rows)
    }

    #[inline]
    fn grid_template_columns(&self) -> Self::TemplateTrackList<'_> {
        convert::grid_template_tracks(&self.0.get_position().grid_template_columns)
    }

    #[inline]
    fn grid_auto_rows(&self) -> Self::AutoTrackList<'_> {
        convert::grid_auto_tracks(&self.0.get_position().grid_auto_rows)
    }

    #[inline]
    fn grid_auto_columns(&self) -> Self::AutoTrackList<'_> {
        convert::grid_auto_tracks(&self.0.get_position().grid_auto_columns)
    }

    #[inline]
    fn grid_auto_flow(&self) -> taffy::GridAutoFlow {
        convert::grid_auto_flow(self.0.get_position().grid_auto_flow)
    }

    #[inline]
    fn gap(&self) -> taffy::Size<taffy::LengthPercentage> {
        let position_styles = self.0.get_position();
        taffy::Size {
            width: convert::gap(&position_styles.column_gap),
            height: convert::gap(&position_styles.row_gap),
        }
    }

    #[inline]
    fn align_content(&self) -> Option<taffy::AlignContent> {
        convert::content_alignment(self.0.get_position().align_content.0)
    }

    #[inline]
    fn justify_content(&self) -> Option<taffy::JustifyContent> {
        convert::content_alignment(self.0.get_position().justify_content.0)
    }

    #[inline]
    fn align_items(&self) -> Option<taffy::AlignItems> {
        convert::item_alignment(self.0.get_position().align_items.0)
    }

    #[inline]
    fn justify_items(&self) -> Option<taffy::AlignItems> {
        convert::item_alignment(self.0.get_position().justify_items.computed.0)
    }
}

// GridItemStyle impl
#[cfg(feature = "grid")]
impl<T: Deref<Target = ComputedValues>> taffy::GridItemStyle for TaffyStyloStyle<T> {
    #[inline]
    fn grid_row(&self) -> taffy::Line<taffy::GridPlacement> {
        let position_styles = self.0.get_position();
        taffy::Line {
            start: convert::grid_line(&position_styles.grid_row_start),
            end: convert::grid_line(&position_styles.grid_row_end),
        }
    }

    #[inline]
    fn grid_column(&self) -> taffy::Line<taffy::GridPlacement> {
        let position_styles = self.0.get_position();
        taffy::Line {
            start: convert::grid_line(&position_styles.grid_column_start),
            end: convert::grid_line(&position_styles.grid_column_end),
        }
    }

    #[inline]
    fn align_self(&self) -> Option<taffy::AlignSelf> {
        convert::item_alignment(self.0.get_position().align_self.0.0)
    }

    #[inline]
    fn justify_self(&self) -> Option<taffy::AlignSelf> {
        convert::item_alignment(self.0.get_position().justify_self.0.0)
    }
}
