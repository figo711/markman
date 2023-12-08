use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings}, layout::layout,
};

use itertools::Itertools;

#[derive(Default)]
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
}

impl Home {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn render_bottom_bar(&self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let keys = [
      ("Q/Esc", "Quit"),
      ("Tab", "Next Tab"),
      ("↑/k", "Up"),
      ("↓/j", "Down"),
    ];
    let spans = keys
      .iter()
      .flat_map(|(key, desc)| {
        let key = Span::styled(format!(" {} ", key), 
          Style::new().fg(Color::Black).bg(Color::DarkGray));
        let desc = Span::styled(format!(" {} ", desc), 
          Style::new().bg(Color::Black).fg(Color::DarkGray));
        [key, desc]
      })
      .collect_vec();

    let s = Paragraph::new(Line::from(spans))
      .alignment(Alignment::Center)
      .fg(Color::Indexed(236))
      .bg(Color::Indexed(232));

    f.render_widget(s, area);

    Ok(())
  }

  pub fn render_title_bar(&self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let area = layout(area, Direction::Horizontal, vec![0, 45]);

    let app_title = Style::new()
      .fg(Color::White)
      .bg(Color::Rgb(16, 24, 48))
      .add_modifier(Modifier::BOLD);

    let p = Paragraph::new(Span::styled("Markman", app_title));
    f.render_widget(p, area[0]);

    let titles = vec![
      "", 
      " Lists ", 
      " Tags ", 
      " Settings ", 
      " About "
    ];

    let tabs_selected = Style::new()
      .fg(Color::Indexed(255))
      .bg(Color::Rgb(16, 24, 48))
      .add_modifier(Modifier::BOLD)
      .add_modifier(Modifier::REVERSED);

    let t = Tabs::new(titles)
      .style(Style::new().fg(Color::Indexed(244)).bg(Color::Rgb(16, 24, 48)))
      .highlight_style(tabs_selected)
      .select(4)
      .divider("");

    f.render_widget(t, area[1]);

    Ok(())
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {
      },
      Action::Render => {

      },
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let area = layout(area, Direction::Vertical, vec![1, 0, 1]); 
    
    f.render_widget(Paragraph::new("hello world here"), area[1]);
    self.render_title_bar(f, area[0])?;
    self.render_bottom_bar(f, area[2])?;
    Ok(())
  }
}

