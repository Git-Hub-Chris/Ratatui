#![warn(clippy::missing_const_for_fn)]

mod alignment;
mod constraint;
mod direction;
mod flex;
#[allow(clippy::module_inception)]
mod layout;
mod margin;
mod position;
mod rect;
mod size;

pub use alignment::Alignment;
pub use constraint::Constraint;
pub use direction::Direction;
pub use flex::Flex;
pub use layout::Layout;
pub use margin::Margin;
pub use position::Position;
pub use rect::*;

///
/// When the layout is computed, the result is cached in a thread-local cache, so that subsequent
/// calls with the same parameters are faster. The cache is a simple HashMap, and grows
/// indefinitely. (See <https://github.com/ratatui-org/ratatui/issues/402> for more information)
///
/// # Constructors
///
/// There are four ways to create a new layout:
///
/// - [`Layout::default`]: create a new layout with default values
/// - [`Layout::new`]: create a new layout with a given direction and constraints
/// - [`Layout::vertical`]: create a new vertical layout with the given constraints
/// - [`Layout::horizontal`]: create a new horizontal layout with the given constraints
///
/// # Setters
///
/// There are several setters to modify the layout:
///
/// - [`Layout::direction`]: set the direction of the layout
/// - [`Layout::constraints`]: set the constraints of the layout
/// - [`Layout::margin`]: set the margin of the layout
/// - [`Layout::horizontal_margin`]: set the horizontal margin of the layout
/// - [`Layout::vertical_margin`]: set the vertical margin of the layout
/// - [`Layout::segment_size`]: set the way the space is distributed when the constraints are
///   satisfied
///
/// # Example
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// fn render(frame: &mut Frame, area: Rect) {
///     let layout = Layout::new(
///         Direction::Vertical,
///         [Constraint::Length(5), Constraint::Min(0)],
///     )
///     .split(Rect::new(0, 0, 10, 10));
///     frame.render_widget(Paragraph::new("foo"), layout[0]);
///     frame.render_widget(Paragraph::new("bar"), layout[1]);
/// }
/// ```
///
/// The [`layout.rs` example](https://github.com/ratatui-org/ratatui/blob/main/examples/layout.rs)
/// shows the effect of combining constraints:
///
/// ![layout
/// example](https://camo.githubusercontent.com/77d22f3313b782a81e5e033ef82814bb48d786d2598699c27f8e757ccee62021/68747470733a2f2f7668732e636861726d2e73682f7668732d315a4e6f4e4c4e6c4c746b4a58706767396e435635652e676966)
///
/// [`cassowary-rs`]: https://crates.io/crates/cassowary
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
    margin: Margin,
    /// option for segment size preferences
    segment_size: SegmentSize,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Margin {
    pub horizontal: u16,
    pub vertical: u16,
}


#[derive(Copy, Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash)]
pub enum SegmentSize {
    /// prefer equal chunks if other constraints are all satisfied
    EvenDistribution,

    /// the last chunk is expanded to fill the remaining space
    #[default]
    LastTakesRemainder,

    /// extra space is not distributed
    None,
}

/// A container used by the solver inside split
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Element {
    start: Variable,
    end: Variable,
}

impl Layout {
    pub const DEFAULT_CACHE_SIZE: usize = 16;
    /// Creates a new layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// AsRef<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// Default values for the other fields are:
    ///
    /// - `margin`: 0, 0
    /// - `segment_size`: SegmentSize::LastTakesRemainder
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::new(
    ///     Direction::Horizontal,
    ///     [Constraint::Length(5), Constraint::Min(0)],
    /// );
    ///
    /// let layout = Layout::new(
    ///     Direction::Vertical,
    ///     [1, 2, 3].iter().map(|&c| Constraint::Length(c)),
    /// );
    /// ```
    pub fn new<I>(direction: Direction, constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: AsRef<Constraint>,
    {
        Layout {
            direction,
            margin: Margin::new(0, 0),
            constraints: constraints.into_iter().map(|c| *c.as_ref()).collect(),
            segment_size: SegmentSize::LastTakesRemainder,
        }
    }

    /// Creates a new vertical layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// AsRef<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::vertical([Constraint::Length(5), Constraint::Min(0)]);
    /// ```
    pub fn vertical<I>(constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        Layout::new(Direction::Vertical, constraints.into_iter().map(Into::into))
    }

    /// Creates a new horizontal layout with default values.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// AsRef<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::horizontal([Constraint::Length(5), Constraint::Min(0)]);
    /// ```
    pub fn horizontal<I>(constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        Layout::new(
            Direction::Horizontal,
            constraints.into_iter().map(Into::into),
        )
    }

    /// Initialize an empty cache with a custom size. The cache is keyed on the layout and area, so
    /// that subsequent calls with the same parameters are faster. The cache is a LruCache, and
    /// grows until `cache_size` is reached.
    ///
    /// Returns true if the cell's value was set by this call.
    /// Returns false if the cell's value was not set by this call, this means that another thread
    /// has set this value or that the cache size is already initialized.
    ///
    /// Note that a custom cache size will be set only if this function:
    /// * is called before [Layout::split()] otherwise, the cache size is
    ///   [`Self::DEFAULT_CACHE_SIZE`].
    /// * is called for the first time, subsequent calls do not modify the cache size.
    pub fn init_cache(cache_size: usize) -> bool {
        LAYOUT_CACHE
            .with(|c| {
                c.set(RefCell::new(LruCache::new(
                    NonZeroUsize::new(cache_size).unwrap(),
                )))
            })
            .is_ok()
    }

    /// Set the direction of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 5, 10), Rect::new(5, 0, 5, 10)]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 0, 10, 5), Rect::new(0, 5, 10, 5)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn direction(mut self, direction: Direction) -> Layout {
        self.direction = direction;
        self
    }

    /// Sets the constraints of the layout.
    ///
    /// The `constraints` parameter accepts any type that implements `IntoIterator<Item =
    /// AsRef<Constraint>>`. This includes arrays, slices, vectors, iterators, etc.
    ///
    /// Note that the constraints are applied to the whole area that is to be split, so using
    /// percentages and ratios with the other constraints may not have the desired effect of
    /// splitting the area up. (e.g. splitting 100 into [min 20, 50%, 50%], may not result in
    /// [20, 40, 40] but rather an indeterminate result between [20, 50, 30] and [20, 30, 50]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([
    ///         Constraint::Percentage(20),
    ///         Constraint::Ratio(1, 5),
    ///         Constraint::Length(2),
    ///         Constraint::Min(2),
    ///         Constraint::Max(2),
    ///     ])
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(
    ///     layout[..],
    ///     [
    ///         Rect::new(0, 0, 10, 2),
    ///         Rect::new(0, 2, 10, 2),
    ///         Rect::new(0, 4, 10, 2),
    ///         Rect::new(0, 6, 10, 2),
    ///         Rect::new(0, 8, 10, 2),
    ///     ]
    /// );
    ///
    /// let layout = Layout::default().constraints([Constraint::Min(0)]);
    /// let layout = Layout::default().constraints(&[Constraint::Min(0)]);
    /// let layout = Layout::default().constraints(vec![Constraint::Min(0)]);
    /// let layout = Layout::default().constraints([Constraint::Min(0)].iter().filter(|_| true));
    /// let layout = Layout::default().constraints([1, 2, 3].iter().map(|&c| Constraint::Length(c)));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn constraints<I>(mut self, constraints: I) -> Layout
    where
        I: IntoIterator,
        I::Item: AsRef<Constraint>,
    {
        self.constraints = constraints.into_iter().map(|c| *c.as_ref()).collect();
        self
    }

    /// Set the margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 2, 6, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn margin(mut self, margin: u16) -> Layout {
        self.margin = Margin {
            horizontal: margin,
            vertical: margin,
        };
        self
    }

    /// Set the horizontal margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .horizontal_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(2, 0, 6, 10)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn horizontal_margin(mut self, horizontal: u16) -> Layout {
        self.margin.horizontal = horizontal;
        self
    }

    /// Set the vertical margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .vertical_margin(2)
    ///     .split(Rect::new(0, 0, 10, 10));
    /// assert_eq!(layout[..], [Rect::new(0, 2, 10, 6)]);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn vertical_margin(mut self, vertical: u16) -> Layout {
        self.margin.vertical = vertical;
        self
    }

