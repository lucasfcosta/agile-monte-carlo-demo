extern crate rand;

use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::collections::HashMap;

const TOTAL_RUNS: i32 = 1_000_000;
const STORIES_TARGET: i32 = 50;
const TEN_DAY_THROUGHPUTS: [i32; 10] = [1, 2, 0, 1, 1, 2, 3, 1, 2, 1];

fn create_histogram(data: HashMap<i64, i32>) {
    let root_area = BitMapBackend::new("histogram.png", (600, 400)).into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let max_freq: i32 = *data.values().max().unwrap();

    let min_timestamp = *data.keys().min().unwrap();
    let max_timestamp = *data.keys().max().unwrap();
    let min_date: DateTime<Local> = Local.timestamp(min_timestamp, 0);
    let max_date: DateTime<Local> = Local.timestamp(max_timestamp, 0);

    let labels = data
        .keys()
        .enumerate()
        .map(|(_, e)| Local.timestamp(*e, 0))
        .collect();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 80)
        .margin_top(10)
        .margin_right(10)
        .build_cartesian_2d((min_date..max_date).with_key_points(labels), 0..max_freq)
        .unwrap();

    let pos = Pos::new(HPos::Right, VPos::Bottom);
    let style = TextStyle::from(("sans-serif", 12).into_font())
        .transform(FontTransform::Rotate90)
        .pos(pos);

    let label_formatter =
        |l: &DateTime<Local>| l.format("                    %Y/%m/%d").to_string();

    ctx.configure_mesh()
        .x_labels(10)
        .x_label_formatter(&label_formatter)
        .x_label_style(style)
        .draw()
        .unwrap();

    let bars = data.into_iter().map(|(k, freq)| {
        let mut bar = Rectangle::new(
            [(Local.timestamp(k, 0), 0), (Local.timestamp(k, 0), freq)],
            BLUE.filled(),
        );
        bar.set_margin(0, 0, 6, 6);
        bar
    });

    ctx.draw_series(bars).unwrap();
}

fn main() {
    let mut rng = thread_rng();
    let throughput = Uniform::from(0..TEN_DAY_THROUGHPUTS.len());

    let mut outcomes: HashMap<i64, i32> = HashMap::new();

    let one_day = Duration::days(1);

    let start_date: DateTime<Local> = Local::now();

    for _ in 0..TOTAL_RUNS {
        let mut current_date = start_date;
        let mut stories_completed = 0;

        while stories_completed < STORIES_TARGET {
            let random_index = throughput.sample(&mut rng);
            stories_completed += TEN_DAY_THROUGHPUTS[random_index];
            current_date = current_date + one_day;
        }

        let count = outcomes.entry(current_date.timestamp()).or_insert(0);
        *count += 1;
    }

    println!("Total Simulations: {}", TOTAL_RUNS);
    create_histogram(outcomes);
}
