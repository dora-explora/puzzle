use std::io::Result;
use std::cmp::max;
use ratatui::{
    DefaultTerminal, 
    Frame, 
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, 
    layout::{Constraint, Direction, Layout}, 
    style::{Stylize, Color},
    text::{Line, Span, Text, ToSpan},
    widgets::{Block, BorderType, Paragraph, Widget}
};

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

pub struct App {
    grid: [[char; 40]; 13],
    running: bool,
}

impl App {
    pub fn new() -> App {
        return App { 
            grid: [[' '; 40]; 13],
            running: true 
        };
    }

    fn quit(&mut self) {
        self.running = false;
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        return Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let titlebar = Paragraph::new("test")
                .block(Block::bordered().border_type(BorderType::Rounded))
                .centered();
        let sidebar = self.render_sidebar();
        let grid = self.render_grid();
        let vlayout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(27),
            ]).split(frame.area());
        let hlayout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(81),
                Constraint::Fill(1)
            ]).split(vlayout[1]);
        frame.render_widget(
            titlebar,
            vlayout[0],
        );
        frame.render_widget(
            grid,
            hlayout[0],
        );
        frame.render_widget(
            sidebar,
            hlayout[1],
        );
    }

    fn render_sidebar(&self) -> impl Widget {
        return Block::bordered().border_type(BorderType::Rounded);
    }

    fn render_grid(&self) -> impl Widget {
        let mut lines: Vec<Line> = vec![];
        for y in 0..13 {
            let mut line: Vec<Span> = vec![];
            for x in 0..40 {
                line.push(self.grid[y][x].to_span());
                line.push("│".dark_gray());
            }
            lines.push(Line::raw("").spans(line));
            lines.push(Line::styled("─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─", Color::DarkGray))
        }
        let text = Text::from(lines);
        return Paragraph::new(text).block(Block::bordered());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keyevent(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn handle_keyevent(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }
}