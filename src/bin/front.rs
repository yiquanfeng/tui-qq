use std::{io::{self, stdout}};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{
    enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen
    },
    ExecutableCommand,
};
use ratatui::{
    Terminal, backend::CrosstermBackend, prelude::*, widgets::{Block, Borders, Paragraph}
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;


    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut shouldstop = false;
    

    let mut input_msg = String::new();

    while !shouldstop {
        // 借用 input_msg 进行绘制
        terminal.draw(|frame| ui_painting(frame, &input_msg))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    // 退出
                    KeyCode::Char('q') => shouldstop = true,
                    KeyCode::Backspace => {
                        input_msg.pop();
                    }
                    KeyCode::Char(c) => {
                        input_msg.push(c);
                    }
                    KeyCode::Enter => {
                        println!("发送: {}", input_msg);
                        input_msg.clear();
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui_painting(frame: &mut Frame, msgs: &str) {
    let area = frame.size();

    let app_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(area);

    let msg_list_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(app_layout[0]);

    let chat_window_layout = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints([
        Constraint::Length(3),    // 标题
        Constraint::Min(10),      // 消息列表
        Constraint::Length(3),    // 输入框
        Constraint::Length(1),    // 状态栏
    ])
    .split(app_layout[1]);

    // 左侧条状群组列表
    let groups_list = Paragraph::new("Groups List")
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("群组列表")
        );

    
    // 右侧下方输入框
    let input = Paragraph::new(msgs)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("输入消息 (按Enter发送，q退出)")
        );

    frame.render_widget(groups_list, msg_list_layout[0]);

    frame.render_widget(Paragraph::new("Title"), chat_window_layout[0]);
    frame.render_widget(Paragraph::new("Messages"), chat_window_layout[1]);
    // question
    // terminal小到一定程度，会导致输入框和状态栏无法显示
    frame.render_widget(input, chat_window_layout[2]);
    frame.render_widget(Paragraph::new("Status Bar"), chat_window_layout[3]);

    frame.set_cursor(0, 0);
}