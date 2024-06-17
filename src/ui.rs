use ratatui::{
    layout::{Layout, Constraint},
    text::{Line, Span, Text},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation},
    style::{Color},
};
use std::{fs, io::BufRead};

use crate::app::App;

pub fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]).split(f.size());

    let scrollbar_chunk = chunks[0];
    let stack_chunk = chunks[1];
    let code_chunk = chunks[2];

    let active_stack = app.view.active_stack();

    let mut stack_filename_lines: Vec<Line> = Vec::new();

    for (i, frame) in active_stack.frames.iter().enumerate() {
        if let Some(file) = &frame.file {
            if i == app.view.active_frame_index {
                stack_filename_lines.push(Line::from(Span::styled(file.clone(), Color::Red)));

                if app.view.expand_code {
                    match frame.line {
                        Some(line) => {
                            let file = fs::File::open(file).expect("Could not open file");
                            let reader = std::io::BufReader::new(file);
                            let slice_around = 2;
                            let start = if line + 1 > slice_around { line - slice_around - 1 } else { 0 };
                            let lines = reader.lines().skip(start).take(slice_around * 2 + 1).filter_map(Result::ok).collect::<Vec<String>>();
                            for (i, line) in lines.iter().enumerate() {
                                let line = if i == slice_around {
                                    Line::from(Span::styled(line.clone(), Color::Green))
                                } else {
                                    Line::from(Span::styled(line.clone(), Color::Yellow))
                                };
                                stack_filename_lines.push(line);
                            }
                        }
                        None => {
                            stack_filename_lines.push(Line::from(Span::styled("No line information", Color::Red)));
                        }
                    }
                }

            } else {
                stack_filename_lines.push(Line::from(Span::styled(file.clone(), Color::White)));
            }
        }
    }

    let filename_paragraph = Paragraph::new(Text::from(stack_filename_lines));

    let stack_x_text: Vec<Line> = app
        .view
        .profile
        .stacks
        .iter()
        .enumerate()
        .map(|(i, stack)| {
            if i == app.view.active_stack_index {
                if app.view.active_frame_index == 0 {
                    let selected = Span::styled("x", Color::Green);
                    let rest = Span::styled("x".repeat(stack.frames.len() - 1), Color::Yellow);
                    Line::from(vec![selected, rest])
                } else {
                    let pre = Span::styled("x".repeat(app.view.active_frame_index), Color::Yellow);
                    let selected = Span::styled("x", Color::Green);
                    let post = Span::styled("x".repeat(stack.frames.len() - app.view.active_frame_index - 1), Color::Yellow);
                    Line::from(vec![pre, selected, post])
                }
            } else {
                Line::from(Span::styled("x".repeat(stack.frames.len()), Color::White))
            }
        })
        .collect();

    let vertical_size: usize = stack_x_text.len();

    app.view.scrollbar_state = app.view.scrollbar_state.content_length(vertical_size);

    let paragraph = Paragraph::new(stack_x_text).scroll((app.view.vertical_scroll as u16, 0));
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalLeft)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        scrollbar_chunk,
        &mut app.view.scrollbar_state,
    );
    f.render_widget(paragraph, stack_chunk);
    f.render_widget(filename_paragraph, code_chunk);

}
