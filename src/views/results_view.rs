use crate::components::app::Message;
use crate::components::header::get_header;
use crate::components::state::State;
use iced::widget::{button, column, container, row, scrollable, space, text};
use iced::{Element, Length, color};

/// Formats a byte count as a human-readable string (B, KiB, MiB, GiB).
fn format_size(bytes: u64) -> String {
    const KI_B: u64 = 1024;
    const MI_B: u64 = 1024 * KI_B;
    const GI_B: u64 = 1024 * MI_B;

    if bytes >= GI_B {
        format!("{:.2} GiB", bytes as f64 / GI_B as f64)
    } else if bytes >= MI_B {
        format!("{:.2} MiB", bytes as f64 / MI_B as f64)
    } else if bytes >= KI_B {
        format!("{:.2} KiB", bytes as f64 / KI_B as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Formats a percentage with a `+` prefix when the file grew and `-` when it shrank.
fn format_pct(pct: f64) -> String {
    if pct == 0.0 {
        "0.0%".to_string()
    } else if pct > 0.0 {
        format!("-{:.1}%", pct)
    } else {
        format!("+{:.1}%", pct.abs())
    }
}

/// Returns a thin horizontal divider element.
fn divider<'a>() -> Element<'a, Message> {
    container(space::horizontal().height(1))
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(color!(128, 128, 128, 0.4))),
            ..Default::default()
        })
        .into()
}

/// Builds the compression results view showing before/after sizes per file.
pub fn view(state: &State) -> Element<'_, Message> {
    let lang = state.current_language();

    let header = get_header(lang.compressr_results.clone(), color!(48, 48, 48, 0.8));

    let results = &state.compression_results;

    // Column headers
    let col_headers = row![
        text(lang.file.as_str()).width(Length::FillPortion(4)),
        text(lang.original.as_str()).width(Length::FillPortion(2)),
        text(lang.compressed_size.as_str()).width(Length::FillPortion(2)),
        text(lang.saved.as_str()).width(Length::FillPortion(2)),
    ]
    .spacing(8)
    .padding([4, 8]);

    // Per-file rows
    let file_rows: Vec<Element<'_, Message>> = results
        .iter()
        .map(|r| {
            row![
                text(r.file_name.as_str()).width(Length::FillPortion(4)),
                text(format_size(r.original_size)).width(Length::FillPortion(2)),
                text(format_size(r.compressed_size)).width(Length::FillPortion(2)),
                text(format_pct(r.percent_saved())).width(Length::FillPortion(2)),
            ]
            .spacing(8)
            .padding([2, 8])
            .into()
        })
        .collect();

    let file_list = scrollable(column(file_rows).spacing(4)).height(Length::Fill);

    // Totals row
    let (total_original, total_compressed): (u64, u64) =
        results.iter().fold((0, 0), |(o, c), r| {
            (o + r.original_size, c + r.compressed_size)
        });
    let total_pct = if total_original > 0 {
        let diff = total_original as f64 - total_compressed as f64;
        diff / total_original as f64 * 100.0
    } else {
        0.0
    };

    let files_total_label = lang
        .files_total
        .replace("{count}", &results.len().to_string());

    let totals_row = row![
        text(files_total_label).width(Length::FillPortion(4)),
        text(format_size(total_original)).width(Length::FillPortion(2)),
        text(format_size(total_compressed)).width(Length::FillPortion(2)),
        text(format_pct(total_pct)).width(Length::FillPortion(2)),
    ]
    .spacing(8)
    .padding([4, 8]);

    let close_row = row![
        space::horizontal().width(Length::Fill),
        button(lang.close.as_str())
            .width(Length::Shrink)
            .on_press(Message::CloseResultsView),
    ]
    .padding([8, 8]);

    let content = column![
        col_headers,
        divider(),
        file_list,
        divider(),
        totals_row,
        close_row,
    ]
    .padding([8, 0])
    .spacing(4);

    let together = column![header, content];

    container(together)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
