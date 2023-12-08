use std::rc::Rc;

use itertools::Itertools;
use ratatui::layout::{Rect, Direction, Constraint, Layout};


/// From: https://github.com/ratatui-org/ratatui/blob/main/examples/demo2/root.rs#L78
/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect_vec();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}