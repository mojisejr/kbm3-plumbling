use crate::log;
use printpdf::*;

pub fn draw_bill_split_line() -> Line {
    log::log_debug("Drawing bill split line");
    let y = 114.0;
    Line {
        points: vec![
            (Point::new(Mm(0.0), Mm(y)), false),
            (Point::new(Mm(148.0), Mm(y)), false),
        ],
        is_closed: false,
    }
}

pub fn draw_line(y: Mm, start: Mm, end: Mm) -> Line {
    log::log_debug(&format!(
        "Drawing horizontal line at y={}, from {} to {}",
        y.0, start.0, end.0
    ));
    Line {
        points: vec![(Point::new(start, y), false), (Point::new(end, y), false)],
        is_closed: false,
    }
}

pub fn draw_vetical_line(x: Mm, start: Mm, end: Mm) -> Line {
    log::log_debug(&format!(
        "Drawing vertical line at x={}, from {} to {}",
        x.0, start.0, end.0
    ));
    Line {
        points: vec![(Point::new(x, start), false), (Point::new(x, end), false)],
        is_closed: false,
    }
}
