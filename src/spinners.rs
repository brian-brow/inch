// The spinner frames and intervals below come from sindresorhus/cli-spinners
// (https://github.com/sindresorhus/cli-spinners), used under its license:
//
// MIT License
//
// Copyright (c) Sindre Sorhus <sindresorhus@gmail.com> (https://sindresorhus.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! Single-character, single-cell spinner styles, from
//! [cli-spinners](https://github.com/sindresorhus/cli-spinners).
//!
//! Pick one with `Bar::with_spinner(spinners::LINE)`, or walk [`ALL`] to list them.

use std::time::Duration;

/// Frames shown in order, each for `interval`.
#[derive(Clone, Copy, Debug)]
pub struct Spinner {
    pub frames: &'static [char],
    pub interval: Duration,
}

impl Spinner {
    /// The frame to show after `elapsed` time.
    pub fn frame(&self, elapsed: Duration) -> char {
        let step = elapsed.as_millis() / self.interval.as_millis().max(1);
        self.frames[step as usize % self.frames.len()]
    }
}

const fn spinner(interval_ms: u64, frames: &'static [char]) -> Spinner {
    Spinner {
        frames,
        interval: Duration::from_millis(interval_ms),
    }
}

pub const DOTS: Spinner = spinner(80, &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']);
pub const DOTS2: Spinner = spinner(80, &['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷']);
pub const DOTS3: Spinner = spinner(80, &['⠋', '⠙', '⠚', '⠞', '⠖', '⠦', '⠴', '⠲', '⠳', '⠓']);
pub const DOTS4: Spinner = spinner(
    80,
    &[
        '⠄', '⠆', '⠇', '⠋', '⠙', '⠸', '⠰', '⠠', '⠰', '⠸', '⠙', '⠋', '⠇', '⠆',
    ],
);
pub const DOTS5: Spinner = spinner(
    80,
    &[
        '⠋', '⠙', '⠚', '⠒', '⠂', '⠂', '⠒', '⠲', '⠴', '⠦', '⠖', '⠒', '⠐', '⠐', '⠒', '⠓', '⠋',
    ],
);
pub const DOTS6: Spinner = spinner(
    80,
    &[
        '⠁', '⠉', '⠙', '⠚', '⠒', '⠂', '⠂', '⠒', '⠲', '⠴', '⠤', '⠄', '⠄', '⠤', '⠴', '⠲', '⠒', '⠂',
        '⠂', '⠒', '⠚', '⠙', '⠉', '⠁',
    ],
);
pub const DOTS7: Spinner = spinner(
    80,
    &[
        '⠈', '⠉', '⠋', '⠓', '⠒', '⠐', '⠐', '⠒', '⠖', '⠦', '⠤', '⠠', '⠠', '⠤', '⠦', '⠖', '⠒', '⠐',
        '⠐', '⠒', '⠓', '⠋', '⠉', '⠈',
    ],
);
pub const DOTS8: Spinner = spinner(
    80,
    &[
        '⠁', '⠁', '⠉', '⠙', '⠚', '⠒', '⠂', '⠂', '⠒', '⠲', '⠴', '⠤', '⠄', '⠄', '⠤', '⠠', '⠠', '⠤',
        '⠦', '⠖', '⠒', '⠐', '⠐', '⠒', '⠓', '⠋', '⠉', '⠈', '⠈',
    ],
);
pub const DOTS9: Spinner = spinner(80, &['⢹', '⢺', '⢼', '⣸', '⣇', '⡧', '⡗', '⡏']);
pub const DOTS10: Spinner = spinner(80, &['⢄', '⢂', '⢁', '⡁', '⡈', '⡐', '⡠']);
pub const DOTS11: Spinner = spinner(100, &['⠁', '⠂', '⠄', '⡀', '⢀', '⠠', '⠐', '⠈']);
pub const DOTS13: Spinner = spinner(80, &['⣼', '⣹', '⢻', '⠿', '⡟', '⣏', '⣧', '⣶']);
pub const DOTS8_BIT: Spinner = spinner(
    80,
    &[
        '⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇', '⠈', '⠉',
        '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⡈', '⡉', '⡊', '⡋', '⡌', '⡍', '⡎', '⡏', '⠐', '⠑', '⠒', '⠓',
        '⠔', '⠕', '⠖', '⠗', '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗', '⠘', '⠙', '⠚', '⠛', '⠜', '⠝',
        '⠞', '⠟', '⡘', '⡙', '⡚', '⡛', '⡜', '⡝', '⡞', '⡟', '⠠', '⠡', '⠢', '⠣', '⠤', '⠥', '⠦', '⠧',
        '⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧', '⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', '⡨', '⡩',
        '⡪', '⡫', '⡬', '⡭', '⡮', '⡯', '⠰', '⠱', '⠲', '⠳', '⠴', '⠵', '⠶', '⠷', '⡰', '⡱', '⡲', '⡳',
        '⡴', '⡵', '⡶', '⡷', '⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽',
        '⡾', '⡿', '⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇', '⣀', '⣁', '⣂', '⣃', '⣄', '⣅', '⣆', '⣇',
        '⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏', '⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏', '⢐', '⢑',
        '⢒', '⢓', '⢔', '⢕', '⢖', '⢗', '⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗', '⢘', '⢙', '⢚', '⢛',
        '⢜', '⢝', '⢞', '⢟', '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟', '⢠', '⢡', '⢢', '⢣', '⢤', '⢥',
        '⢦', '⢧', '⣠', '⣡', '⣢', '⣣', '⣤', '⣥', '⣦', '⣧', '⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯',
        '⣨', '⣩', '⣪', '⣫', '⣬', '⣭', '⣮', '⣯', '⢰', '⢱', '⢲', '⢳', '⢴', '⢵', '⢶', '⢷', '⣰', '⣱',
        '⣲', '⣳', '⣴', '⣵', '⣶', '⣷', '⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿', '⣸', '⣹', '⣺', '⣻',
        '⣼', '⣽', '⣾', '⣿',
    ],
);
pub const SAND: Spinner = spinner(
    80,
    &[
        '⠁', '⠂', '⠄', '⡀', '⡈', '⡐', '⡠', '⣀', '⣁', '⣂', '⣄', '⣌', '⣔', '⣤', '⣥', '⣦', '⣮', '⣶',
        '⣷', '⣿', '⡿', '⠿', '⢟', '⠟', '⡛', '⠛', '⠫', '⢋', '⠋', '⠍', '⡉', '⠉', '⠑', '⠡', '⢁',
    ],
);
pub const LINE: Spinner = spinner(130, &['-', '\\', '|', '/']);
pub const LINE2: Spinner = spinner(100, &['⠂', '-', '–', '—', '–', '-']);
pub const PIPE: Spinner = spinner(100, &['┤', '┘', '┴', '└', '├', '┌', '┬', '┐']);
pub const STAR: Spinner = spinner(70, &['✶', '✸', '✹', '✺', '✹', '✷']);
pub const STAR2: Spinner = spinner(80, &['+', 'x', '*']);
pub const FLIP: Spinner = spinner(
    70,
    &['_', '_', '_', '-', '`', '`', '\'', '´', '-', '_', '_', '_'],
);
pub const GROW_VERTICAL: Spinner =
    spinner(120, &['▁', '▃', '▄', '▅', '▆', '▇', '▆', '▅', '▄', '▃']);
pub const GROW_HORIZONTAL: Spinner = spinner(
    120,
    &['▏', '▎', '▍', '▌', '▋', '▊', '▉', '▊', '▋', '▌', '▍', '▎'],
);
pub const BALLOON: Spinner = spinner(140, &[' ', '.', 'o', 'O', '@', '*', ' ']);
pub const BALLOON2: Spinner = spinner(120, &['.', 'o', 'O', '°', 'O', 'o', '.']);
pub const NOISE: Spinner = spinner(100, &['▓', '▒', '░']);
pub const BOUNCE: Spinner = spinner(120, &['⠁', '⠂', '⠄', '⠂']);
pub const BOX_BOUNCE: Spinner = spinner(120, &['▖', '▘', '▝', '▗']);
pub const BOX_BOUNCE2: Spinner = spinner(100, &['▌', '▀', '▐', '▄']);
pub const TRIANGLE: Spinner = spinner(50, &['◢', '◣', '◤', '◥']);
pub const ARC: Spinner = spinner(100, &['◜', '◠', '◝', '◞', '◡', '◟']);
pub const CIRCLE: Spinner = spinner(120, &['◡', '⊙', '◠']);
pub const SQUARE_CORNERS: Spinner = spinner(180, &['◰', '◳', '◲', '◱']);
pub const CIRCLE_QUARTERS: Spinner = spinner(120, &['◴', '◷', '◶', '◵']);
pub const CIRCLE_HALVES: Spinner = spinner(50, &['◐', '◓', '◑', '◒']);
pub const SQUISH: Spinner = spinner(100, &['╫', '╪']);
pub const TOGGLE: Spinner = spinner(250, &['⊶', '⊷']);
pub const TOGGLE2: Spinner = spinner(80, &['▫', '▪']);
pub const TOGGLE3: Spinner = spinner(120, &['□', '■']);
pub const TOGGLE4: Spinner = spinner(100, &['■', '□', '▪', '▫']);
pub const TOGGLE5: Spinner = spinner(100, &['▮', '▯']);
pub const TOGGLE6: Spinner = spinner(300, &['ဝ', '၀']);
pub const TOGGLE7: Spinner = spinner(80, &['⦾', '⦿']);
pub const TOGGLE8: Spinner = spinner(100, &['◍', '◌']);
pub const TOGGLE9: Spinner = spinner(100, &['◉', '◎']);
pub const TOGGLE11: Spinner = spinner(50, &['⧇', '⧆']);
pub const TOGGLE12: Spinner = spinner(120, &['☗', '☖']);
pub const TOGGLE13: Spinner = spinner(80, &['=', '*', '-']);
pub const ARROW: Spinner = spinner(100, &['←', '↖', '↑', '↗', '→', '↘', '↓', '↙']);
pub const DQPB: Spinner = spinner(100, &['d', 'q', 'p', 'b']);
pub const LAYER: Spinner = spinner(150, &['-', '=', '≡']);

/// Every spinner with its cli-spinners name.
pub const ALL: &[(&str, Spinner)] = &[
    ("dots", DOTS),
    ("dots2", DOTS2),
    ("dots3", DOTS3),
    ("dots4", DOTS4),
    ("dots5", DOTS5),
    ("dots6", DOTS6),
    ("dots7", DOTS7),
    ("dots8", DOTS8),
    ("dots9", DOTS9),
    ("dots10", DOTS10),
    ("dots11", DOTS11),
    ("dots13", DOTS13),
    ("dots8Bit", DOTS8_BIT),
    ("sand", SAND),
    ("line", LINE),
    ("line2", LINE2),
    ("pipe", PIPE),
    ("star", STAR),
    ("star2", STAR2),
    ("flip", FLIP),
    ("growVertical", GROW_VERTICAL),
    ("growHorizontal", GROW_HORIZONTAL),
    ("balloon", BALLOON),
    ("balloon2", BALLOON2),
    ("noise", NOISE),
    ("bounce", BOUNCE),
    ("boxBounce", BOX_BOUNCE),
    ("boxBounce2", BOX_BOUNCE2),
    ("triangle", TRIANGLE),
    ("arc", ARC),
    ("circle", CIRCLE),
    ("squareCorners", SQUARE_CORNERS),
    ("circleQuarters", CIRCLE_QUARTERS),
    ("circleHalves", CIRCLE_HALVES),
    ("squish", SQUISH),
    ("toggle", TOGGLE),
    ("toggle2", TOGGLE2),
    ("toggle3", TOGGLE3),
    ("toggle4", TOGGLE4),
    ("toggle5", TOGGLE5),
    ("toggle6", TOGGLE6),
    ("toggle7", TOGGLE7),
    ("toggle8", TOGGLE8),
    ("toggle9", TOGGLE9),
    ("toggle11", TOGGLE11),
    ("toggle12", TOGGLE12),
    ("toggle13", TOGGLE13),
    ("arrow", ARROW),
    ("dqpb", DQPB),
    ("layer", LAYER),
];
