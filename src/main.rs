// src/main.rs
use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Terminal,
};

// 消息结构体
#[derive(Clone)]
struct Message {
    sender: String,
    content: String,
    timestamp: String,
}

// 应用状态
struct App {
    messages: Vec<Message>,
    input: String,
    scroll: usize,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        // 添加一些初始消息
        let mut messages = Vec::new();
        messages.push(Message {
            sender: "系统".to_string(),
            content: "欢迎使用 Rust TUI QQ!".to_string(),
            timestamp: "10:00".to_string(),
        });
        messages.push(Message {
            sender: "Alice".to_string(),
            content: "你好！Rust TUI 很有趣吧？".to_string(),
            timestamp: "10:01".to_string(),
        });
        messages.push(Message {
            sender: "Bob".to_string(),
            content: "是的，性能很好！".to_string(),
            timestamp: "10:02".to_string(),
        });
        messages.push(Message {
            sender: "You".to_string(),
            content: "我正在学习如何制作TUI界面".to_string(),
            timestamp: "10:03".to_string(),
        });
        messages.push(Message {
            sender: "Alice".to_string(),
            content: "这个聊天窗口可以上下滚动查看消息历史".to_string(),
            timestamp: "10:04".to_string(),
        });
        messages.push(Message {
            sender: "Bob".to_string(),
            content: "试试按上下箭头滚动，按回车发送消息".to_string(),
            timestamp: "10:05".to_string(),
        });

        Self {
            messages,
            input: String::new(),
            scroll: 0,
            should_quit: false,
        }
    }

    // 发送消息
    fn send_message(&mut self) {
        if !self.input.trim().is_empty() {
            let msg = Message {
                sender: "You".to_string(),
                content: self.input.clone(),
                timestamp: "now".to_string(),
            };
            self.messages.push(msg);
            self.input.clear();
            self.scroll = 0; // 发送后回到最新消息
        }
    }

    // 上移滚动位置
    fn scroll_up(&mut self) {
        if self.scroll + 1 < self.messages.len() {
            self.scroll += 1;
        }
    }

    // 下移滚动位置
    fn scroll_down(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }
}

fn main() -> io::Result<()> {
    // 初始化终端
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    
    // 创建应用
    let mut app = App::new();
    
    // 主事件循环
    let result = run_app(&mut terminal, &mut app);
    
    // 恢复终端
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    
    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|f| ui(f, app))?;
        
        // 处理键盘事件
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    // 退出
                    KeyCode::Char('q') => app.should_quit = true,
                    // 发送消息
                    KeyCode::Enter => app.send_message(),
                    // 退格删除
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    // 滚动
                    KeyCode::Up => app.scroll_up(),
                    KeyCode::Down => app.scroll_down(),
                    // PageUp/PageDown 快速滚动
                    KeyCode::PageUp => {
                        for _ in 0..5 {
                            app.scroll_up();
                        }
                    }
                    KeyCode::PageDown => {
                        for _ in 0..5 {
                            app.scroll_down();
                        }
                    }
                    // 输入文字
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),    // 标题
            Constraint::Min(10),      // 消息列表
            Constraint::Length(3),    // 输入框
            Constraint::Length(1),    // 状态栏
        ])
        .split(f.size());

    // 1. 标题区域
    let title = Paragraph::new("Rust TUI QQ - 聊天窗口")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // 2. 消息列表区域
    let messages: Vec<ListItem> = app.messages
        .iter()
        .rev()  // 反转，最新消息在底部
        .skip(app.scroll)  // 跳过滚动的位置
        .map(|msg| {
            let content = vec![
                Line::from(vec![
                    Span::styled(
                        format!("[{}] ", msg.timestamp),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(
                        format!("{}: ", msg.sender),
                        Style::default().fg(match msg.sender.as_str() {
                            "You" => Color::Green,
                            "系统" => Color::Red,
                            "Alice" => Color::Magenta,
                            "Bob" => Color::Yellow,
                            _ => Color::Cyan,
                        }).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(&msg.content),
                ])
            ];
            ListItem::new(content)
        })
        .collect();

    let messages_list = List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("消息 (按上下箭头滚动: {}/{})", 
                    app.scroll, 
                    app.messages.len().saturating_sub(1)))
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(messages_list, chunks[1]);

    // 3. 输入区域
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("输入消息 (按Enter发送，q退出)")
        );
    f.render_widget(input, chunks[2]);
    
    // 设置光标位置（关键修改）
    f.set_cursor(
        chunks[2].x + app.input.len() as u16 + 1,
        chunks[2].y + 1,
    );

    // 4. 状态栏
    let status = Paragraph::new(format!(
        "总消息数: {} | 滚动位置: {} | 输入长度: {}",
        app.messages.len(),
        app.scroll,
        app.input.len()
    ))
    .style(Style::default().fg(Color::DarkGray))
    .alignment(Alignment::Center);
    f.render_widget(status, chunks[3]);
}