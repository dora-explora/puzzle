use std::io::Result;
use ratatui::{
    DefaultTerminal, 
    Frame, 
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, 
    layout::{Constraint, Direction as LayoutDirection, Layout}, 
    style::{Stylize, Color},
    text::{Line, Span, Text, ToSpan},
    widgets::{Block, BorderType, Paragraph, Widget}
};

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    return result;
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy)]
struct Stepper {
    x: usize,
    y: usize,
    d: Direction
}
impl Stepper {
    fn new(x: usize, y: usize, d: Direction) -> Stepper {
        return Stepper { x, y, d };
    }
}

#[derive(Clone, Copy)]
struct Mirror {
    x: usize,
    y: usize,
    d: bool,
}
impl Mirror {
    fn new(x: usize, y: usize, d: bool) -> Mirror {
        return Mirror { x, y, d };
    }
}

struct App {
    player: Stepper,
    alive: bool,
    steppers: Vec<Stepper>,
    mirrors: Vec<Mirror>,
    running: bool,
}

impl App {
    fn new() -> App {
        let player = Stepper::new(0, 12, Direction::Up);
        let steppers: Vec<Stepper> = vec![Stepper::new(5, 5, Direction::Left), Stepper::new(3, 11, Direction::Up)];
        let mirrors: Vec<Mirror> = vec![
            Mirror::new(0, 5, true),
            Mirror::new(3, 5, true),
            Mirror::new(3, 2, false),
            Mirror::new(0, 2, false),
            Mirror::new(0, 0, true),
            Mirror::new(3, 0, false),
            ];
        return App { 
            player,
            alive: true,
            steppers,
            mirrors,
            running: true 
        };
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        terminal.clear()?;
        let size = terminal.size()?;
        if size.width < 100 || size.height < 30 {
            panic!("Please keep the terminal size at or above 100x30");
        }
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        return Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let titlebar = Paragraph::new("Level x: Title │ Time: 0")
                .block(Block::bordered().border_type(BorderType::Rounded))
                .centered();
        let sidebar = self.render_sidebar();
        let grid = self.render_grid();

        let outvlayout = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(30),
                Constraint::Fill(1),
            ]).split(frame.area());
        let outhlayout = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(100),
                Constraint::Fill(1),
            ]).split(outvlayout[1]);
        let vlayout = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(27),
            ]).split(outhlayout[1]);
        let hlayout = Layout::default()
            .direction(LayoutDirection::Horizontal)
            .constraints([
                Constraint::Length(81),
                Constraint::Length(19)
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
            'row: for x in 0..40 {
                line.push("│".dark_gray());
                if self.alive && self.player.y == y && self.player.x == x {
                    let arrow = match self.player.d {
                        Direction::Up => "⬆".to_span().green(),
                        Direction::Down => "⬇".to_span().green(),
                        Direction::Left => "⬅".to_span().green(),
                        Direction::Right => "⮕".to_span().green(),
                    };
                    line.push(arrow);
                    continue 'row;
                }
                for stepper in &self.steppers {
                    if stepper.y == y && stepper.x == x {
                        let arrow = match stepper.d {
                            Direction::Up => '⬆'.to_span().red(),
                            Direction::Down => '⬇'.to_span().red(),
                            Direction::Left => '⬅'.to_span().red(),
                            Direction::Right => '⮕'.to_span().red(),
                        };
                        line.push(arrow);
                        continue 'row;
                    }
                }
                for mirror in &self.mirrors {
                    if mirror.y == y && mirror.x == x {
                        let slash = match mirror.d {
                            false => '\\'.to_span(),
                            true => '/'.to_span(),
                        };
                        line.push(slash);
                        continue 'row;
                    }
                }
                line.push(Span::raw(" "));
            }
            line.remove(0);
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
            Event::Resize(width, height) => self.handle_resize(width, height),
            _ => {}
        }
        Ok(())
    }

    fn handle_keyevent(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Enter) => self.tick(),
            _ => {}
        }
    }

    fn tick(&mut self) {
        self.player = self.step(&self.player);
        let mut removals: Vec<usize> = vec![];
        for i in 0..self.steppers.len() {
            if self.steppers[i].x == self.player.x && self.steppers[i].y == self.player.y {
                self.alive = false;
                break;
            }
            self.steppers[i] = self.step(&self.steppers[i]);
            if self.steppers[i].x == self.player.x && self.steppers[i].y == self.player.y {
                self.alive = false;
                break;
            }
            for j in 0..self.steppers.len() {
                if j == i { continue; };
                if self.steppers[i].x == self.steppers[j].x && self.steppers[i].y == self.steppers[j].y {
                    removals.push(i);
                    removals.push(j);
                }
            }
        }
        removals.sort();
        removals.dedup();
        for i in 0..removals.len() {
            self.steppers.remove(removals[i] - i);
        }
    }

    fn step(&self, stepper: &Stepper) -> Stepper {
        let mut x = stepper.x;
        let mut y = stepper.y;
        let mut d = stepper.d;
        match d {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        for mirror in &self.mirrors {
            if mirror.x == x && mirror.y == y {
                match mirror.d {
                    false => match d {
                        Direction::Up => { x -= 1; d = Direction::Left },
                        Direction::Down => { x += 1; d = Direction::Right },
                        Direction::Left => { y -= 1; d = Direction::Up },
                        Direction::Right => { y += 1; d = Direction::Down },
                    },
                    true => match d {
                        Direction::Up => { x += 1; d = Direction::Right },
                        Direction::Down => { x -= 1; d = Direction::Left },
                        Direction::Left => { y += 1; d = Direction::Down },
                        Direction::Right => { y -= 1; d = Direction::Up },
                    },
                }
            }
        }
        return Stepper { x, y, d };
    }

    fn handle_resize(&mut self, width: u16, height: u16) {
        if width < 100 || height < 30 {
            panic!("Please keep the terminal size at or above 100x30");
        }
    }
}