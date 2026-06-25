use crate::camera::{CAM_H, CAM_W};
use crate::codec::yuyv_to_rgb;
use crate::codec::luma_to_char;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

pub struct AsciiFrame<'a> {
    pub frame: &'a [u8],
}

impl<'a> Widget for AsciiFrame<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }
        for row in 0..area.height {
            let src_y = (row as u32 * CAM_H) / area.height as u32;
            for col in 0..area.width {
                let src_x = (col as u32 * CAM_W) / area.width as u32;
                let (r, g, b, luma) = yuyv_to_rgb(self.frame, CAM_W, src_x, src_y);
                let cell = buf.get_mut(area.x + col, area.y + row);
                cell.set_char(luma_to_char(luma));
                cell.set_style(Style::default().fg(Color::Rgb(r, g, b)));
            }
        }
    }
}

pub fn draw(f: &mut Frame, frame_data: &[u8]) {
    let area = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    let video_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " 📷 camascii ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    let inner = video_block.inner(chunks[0]);
    f.render_widget(video_block, chunks[0]);
    f.render_widget(AsciiFrame { frame: frame_data }, inner);

    let key = |label: &'static str| {
        Span::styled(
            format!(" {label} "),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    };
    let desc = |text: &'static str| {
        Span::styled(
            format!(" {text}"),
            Style::default().fg(Color::White),
        )
    };

    let status = Paragraph::new(Line::from(vec![
        Span::raw("  "),
        key("q"),       desc("Quit"),
        Span::raw("   "),
        key("Esc"),     desc("Quit"),
        Span::raw("   "),
        key("Ctrl+C"),  desc("Quit"),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                " Shortcuts ",
                Style::default().fg(Color::DarkGray),
            )),
    );

    f.render_widget(status, chunks[1]);
}