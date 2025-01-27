use crate::{clone, config, error};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use std::io;

pub fn run(conf: config::Config) -> Result<String, error::CustomError> {
    let mut terminal = ratatui::init();
    let app_result = App::new(conf).run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Clone)]
pub struct App {
    conf: config::Config,

    selected_repo: String,
    selected_branch: String,
    selected_base_branch: String,

    state: AppState,

    repos_list: ReposList,

    exit: bool,
    ready_to_clone: bool,
}

#[derive(Debug, Clone)]
pub struct ReposList {
    available_repos: Vec<config::Repo>,

    matched_repos: Vec<config::Repo>,
    state: ListState,
}

impl ReposList {
    fn new(conf: &config::Config) -> Self {
        let available_repos: Vec<config::Repo> = conf.repos.clone();

        let matched_repos = available_repos.clone();

        Self {
            available_repos,
            matched_repos,
            state: ListState::default(),
        }
    }
}

#[derive(Debug, Clone)]
enum AppState {
    Repo,
    Branch,
    BaseBranch,
}

impl App {
    fn new(conf: config::Config) -> Self {
        let repos_list = ReposList::new(&conf);

        Self {
            conf,
            selected_repo: String::new(),
            selected_branch: String::new(),
            selected_base_branch: String::new(),
            state: AppState::Repo,
            repos_list: repos_list,
            exit: false,
            ready_to_clone: false,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<String, error::CustomError> {
        let message = String::new();
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        if self.ready_to_clone {
            return clone::clone(
                self.conf,
                self.selected_repo,
                self.selected_branch,
                self.selected_base_branch,
            );
        }
        Ok(message)
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.modifiers.contains(event::KeyModifiers::CONTROL)
            && key_event.code == KeyCode::Char('c')
        {
            self.exit();
            self.ready_to_clone = false;
            return;
        }

        match key_event.code {
            KeyCode::Char(ch) => match self.state {
                AppState::Repo => {
                    self.selected_repo.push(ch);
                    self.determine_matched_repos();
                }
                AppState::Branch => {
                    self.selected_branch.push(ch);
                }
                AppState::BaseBranch => {
                    self.selected_base_branch.push(ch);
                }
            },
            KeyCode::Enter => {
                self.advance_state();
            }
            KeyCode::Backspace => match self.state {
                AppState::Repo => {
                    self.selected_repo.pop();
                }
                AppState::Branch => {
                    self.selected_branch.pop();
                }
                AppState::BaseBranch => {
                    self.selected_base_branch.pop();
                }
            },
            KeyCode::Down | KeyCode::Tab => {
                self.repos_list.state.select_next();
            }
            KeyCode::Up => {
                self.repos_list.state.select_previous();
            }
            KeyCode::Esc => {
                self.repos_list.state.select(None);
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn advance_state(&mut self) {
        self.state = match self.state {
            AppState::Repo => {
                if let Some(i) = self.repos_list.state.selected() {
                    self.selected_repo = self.repos_list.matched_repos[i].name.clone();
                }
                AppState::Branch
            }
            AppState::Branch => {
                if self.selected_branch.is_empty() {
                    self.exit();
                    self.ready_to_clone = true;
                }
                AppState::BaseBranch
            }
            AppState::BaseBranch => {
                self.exit();
                self.ready_to_clone = true;
                AppState::BaseBranch
            }
        }
    }

    fn determine_matched_repos(&mut self) {
        let matcher = SkimMatcherV2::default();
        let selected_repo = self.selected_repo.clone();
        self.repos_list.matched_repos = self
            .repos_list
            .available_repos
            .iter()
            .filter(|r| matcher.fuzzy_match(&r.name, &selected_repo).is_some())
            .cloned()
            .collect();
        self.repos_list.state.select(None);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(5), Constraint::Fill(1)]).areas(area);

        self.render_input(input_area, buf);
        self.render_list(list_area, buf);
    }
}

impl App {
    fn render_input(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from("New Space".bold()))
            .border_set(border::THICK);

        let repo_text = vec!["Repo to Clone: ".into(), self.selected_repo.clone().into()];
        let branch_text = vec![
            "Branch to Checkout (leave blank for default): ".into(),
            self.selected_branch.clone().into(),
        ];
        let base_branch_text = vec![
            "Base Branch (leave blank for default): ".into(),
            self.selected_base_branch.clone().into(),
        ];

        let mut text = vec![Line::from(repo_text)];

        match self.state {
            AppState::Branch => {
                text.push(Line::from(branch_text));
            }
            AppState::BaseBranch => {
                text.push(Line::from(branch_text));
                text.push(Line::from(base_branch_text));
            }
            _ => {}
        }

        Paragraph::new(Text::from(text))
            .block(block.clone())
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from("Spaces".bold()))
            .border_set(border::THICK);

        let items: Vec<ListItem> = self
            .repos_list
            .matched_repos
            .iter()
            .enumerate()
            .map(|(_i, repo)| ListItem::from(Text::raw(repo.name.clone())))
            .collect();

        let list = List::new(items).block(block).highlight_symbol(">");

        StatefulWidget::render(list, area, buf, &mut self.repos_list.state.clone());
    }
}
