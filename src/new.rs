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

    existing_spaces_list: ExistingSpacesList,

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

#[derive(Debug, Clone, Default)]
pub struct ExistingSpacesList {
    matched_spaces: Vec<String>,
    state: ListState,
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
            exit: false,
            ready_to_clone: false,
            repos_list,
            existing_spaces_list: ExistingSpacesList::default(),
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
                    self.determine_matched_spaces();
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
            KeyCode::Down | KeyCode::Tab => match self.state {
                AppState::Repo => {
                    self.repos_list.state.select_next();
                }
                AppState::Branch => {
                    self.existing_spaces_list.state.select_next();
                }
                _ => {}
            },
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
                self.determine_matched_spaces();
                AppState::Branch
            }
            AppState::Branch => {
                if let Some(i) = self.existing_spaces_list.state.selected() {
                    self.selected_branch = self.get_selected_branch(i);
                    self.exit();
                    self.ready_to_clone = true;
                }
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

    fn determine_matched_spaces(&mut self) {
        let matcher = SkimMatcherV2::default();
        let selected_branch = self.selected_branch.clone();

        let selected_repo = self.selected_repo.clone();
        let mut split_repo = selected_repo.split('/').collect::<Vec<&str>>();

        let repo = split_repo.pop().unwrap().replace(".git", "");
        let owner = split_repo.pop().unwrap();

        let default = Vec::new();
        let spaces = self.conf.current_spaces.get(owner).unwrap_or(&default);

        let matched_input = format!("{}-{}", repo, selected_branch);
        self.existing_spaces_list.matched_spaces = spaces
            .iter()
            .filter(|s| matcher.fuzzy_match(s, &matched_input).is_some())
            .cloned()
            .collect();

        self.existing_spaces_list.state.select(None);
    }

    fn get_selected_branch(&self, i: usize) -> String {
        let selected_repo = self.selected_repo.clone();
        let repo_name = selected_repo
            .split('/')
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .replace(".git", "");
        let repo_name = format!("{}-", repo_name);
        self.existing_spaces_list.matched_spaces[i]
            .clone()
            .replace(&repo_name, "")
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
        match self.state {
            AppState::Repo => {
                self.render_repos_list(area, buf);
            }
            AppState::Branch => {
                self.render_existing_spaces_list(area, buf);
            }
            _ => {}
        }
    }

    fn render_repos_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from("Repos".bold()))
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

    fn render_existing_spaces_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from("Current Spaces".bold()))
            .border_set(border::THICK);

        let items: Vec<ListItem> = self
            .existing_spaces_list
            .matched_spaces
            .iter()
            .enumerate()
            .map(|(_i, space)| ListItem::from(Text::raw(space.clone())))
            .collect();

        let list = List::new(items).block(block).highlight_symbol(">");

        StatefulWidget::render(
            list,
            area,
            buf,
            &mut self.existing_spaces_list.state.clone(),
        );
    }
}
