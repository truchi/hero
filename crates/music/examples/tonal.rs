#![allow(unused)]

use music::list::*;
use music::*;

fn main() {
    let scales = "/home/romain/perso/music/scales.txt";
    let chords = "/home/romain/perso/music/chords.txt";

    println!("");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("Parsing scales.txt");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("");
    go(scales);
    println!("");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("Parsing chords.txt");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("################################################################################################################################################");
    println!("");
    go(chords);
}

fn go(path: &str) {
    let file = std::fs::read_to_string(path).unwrap();
    let mut dict = Dictionary::default();

    for line in file.lines() {
        if line.starts_with('#') {
            continue;
        }

        let mut split = line.split(", ");
        let notes = split.next().unwrap();
        let names = split.map(Into::into).collect::<Vec<String>>();
        let notes = notes.split(" ").map(parse_interval).collect::<List<_>>();
        let scales = dict.scales_mut(notes.len());
        let mode = Mode::new(names, notes);

        if scales.is_empty() {
            scales.push(Scale::new(mode));
        } else {
            let mut index = None;
            for (scale_index, scale) in scales.into_iter().enumerate() {
                if let Some(mode_index) = scale.first().is_related(&mode) {
                    index = Some((scale_index, mode_index));
                    break;
                }
            }

            if let Some((scale_index, mode_index)) = index {
                scales[scale_index].push(mode_index, mode);
            } else {
                scales.push(Scale::new(mode));
            }
        }
    }

    dict.reduce_symmetries();
    dict.print();
}

#[derive(Default, Debug)]
struct Dictionary {
    two: Vec<Scale>,
    three: Vec<Scale>,
    four: Vec<Scale>,
    five: Vec<Scale>,
    six: Vec<Scale>,
    seven: Vec<Scale>,
    eight: Vec<Scale>,
    nine: Vec<Scale>,
    ten: Vec<Scale>,
    twelve: Vec<Scale>,
    thirteen: Vec<Scale>,
}

impl Dictionary {
    fn scales_mut(&mut self, len: usize) -> &mut Vec<Scale> {
        match len {
            2 => &mut self.two,
            3 => &mut self.three,
            4 => &mut self.four,
            5 => &mut self.five,
            6 => &mut self.six,
            7 => &mut self.seven,
            8 => &mut self.eight,
            9 => &mut self.nine,
            10 => &mut self.ten,
            12 => &mut self.twelve,
            len => panic!("{len}"),
        }
    }

    fn reduce_symmetries(&mut self) {
        let groups = [
            &mut self.two,
            &mut self.three,
            &mut self.four,
            &mut self.five,
            &mut self.six,
            &mut self.seven,
            &mut self.eight,
            &mut self.nine,
            &mut self.ten,
            &mut self.twelve,
        ];

        for scales in groups {
            for scale in scales {
                if let Some(symmetry) = scale.first().symmetry() {
                    for _ in symmetry..scale.first().intervals.len() {
                        scale.modes.pop();
                    }
                }
            }
        }
    }

    fn print(&self) {
        fn print_scales(scales: &[Scale], name: &str) {
            let sep = std::iter::repeat('=').take(130).collect::<String>();
            println!("{sep}");
            println!("                                                {name}");
            println!("{sep}");
            for scale in scales {
                println!("-----------------------------------");
                for mode in scale.modes.as_ref() {
                    if let Some(mode) = mode {
                        println!(
                            "{:?} {:?} ({:?})",
                            mode.intervals,
                            mode.names,
                            mode.symmetry()
                        );
                        // println!("{:?}", mode.relatives);
                    } else {
                        println!("NO MODE");
                    }
                }
                println!("-----------------------------------");
            }
        }

        for (scales, name) in [
            (&self.two, "TWO"),
            (&self.three, "THREE"),
            (&self.four, "FOUR"),
            (&self.five, "FIVE"),
            (&self.six, "SIX"),
            (&self.seven, "SEVEN"),
            (&self.eight, "EIGHT"),
            (&self.nine, "NINE"),
            (&self.ten, "TEN"),
            (&self.twelve, "TWELVE"),
        ] {
            if !scales.is_empty() {
                print_scales(&scales, name);
            }
        }
    }
}

#[derive(Debug)]
struct Scale {
    len: usize,
    modes: List<Option<Mode>>,
}

impl Scale {
    fn new(first: Mode) -> Self {
        let len = first.len;
        let mut modes = List::new();

        modes.push(Some(first));
        for _ in 1..len {
            modes.push(None);
        }

        Self { len, modes }
    }

    fn first(&self) -> &Mode {
        self.modes.as_ref().first().unwrap().as_ref().unwrap()
    }

    fn push(&mut self, index: usize, mode: Mode) {
        self.modes.as_mut()[index] = Some(mode);
    }

    fn is(&self, name: &str) -> bool {
        self.modes
            .as_ref()
            .iter()
            .any(|mode| mode.as_ref().map(|mode| mode.is(name)).unwrap_or(false))
    }
}

#[derive(Debug)]
struct Mode {
    len: usize,
    names: Vec<String>,
    intervals: List<Interval>,
    relatives: List<u8>,
}

impl Mode {
    fn new(names: Vec<String>, mut intervals: List<Interval>) -> Self {
        intervals
            .as_mut()
            .sort_by(|a, b| a.semitones().cmp(&b.semitones()));
        let len = intervals.len();
        let relatives = relative_semitones(&intervals);

        Self {
            len,
            names,
            intervals,
            relatives,
        }
    }

    fn is_related(&self, other: &Self) -> Option<usize> {
        let this = self.relatives.as_ref();
        let other = other.relatives.as_ref();

        if this.len() != other.len() {
            return None;
        }

        for i in 0..this.len() {
            let (this_left, this_right) = this.split_at(i);
            let (other_left, other_right) = other.split_at(other.len() - i);

            if this_left == other_right && this_right == other_left {
                return Some(i);
            }
        }

        None
    }

    fn symmetry(&self) -> Option<usize> {
        let this = self.relatives.as_ref();

        for i in 1..this.len() {
            let (this_left, this_right) = this.split_at(i);
            let (other_left, other_right) = this.split_at(this.len() - i);

            if this_left == other_right && this_right == other_left {
                return Some(i);
            }
        }

        None
    }

    fn is(&self, name: &str) -> bool {
        self.names.iter().any(|n| n == name)
    }
}

fn parse_interval(note: &str) -> Interval {
    let mut note = note.chars();
    let a = note.next().unwrap();
    let b = note.next().unwrap();
    let c = note.next();

    match (a, b, c) {
        ('1', 'P', None) => Interval(Natural, Unisson),
        ('2', 'm', None) => Interval(Flat, Second),
        ('2', 'M', None) => Interval(Natural, Second),
        ('2', 'A', None) => Interval(Sharp, Second),
        ('3', 'm', None) => Interval(Flat, Third),
        ('3', 'M', None) => Interval(Natural, Third),
        ('4', 'd', None) => Interval(Flat, Fourth),
        ('4', 'P', None) => Interval(Natural, Fourth),
        ('4', 'A', None) => Interval(Sharp, Fourth),
        ('5', 'd', None) => Interval(Flat, Fifth),
        ('5', 'P', None) => Interval(Natural, Fifth),
        ('5', 'A', None) => Interval(Sharp, Fifth),
        ('6', 'm', None) => Interval(Flat, Sixth),
        ('6', 'M', None) => Interval(Natural, Sixth),
        ('6', 'A', None) => Interval(Sharp, Sixth),
        ('7', 'd', None) => Interval(DoubleFlat, Seventh),
        ('7', 'm', None) => Interval(Flat, Seventh),
        ('7', 'M', None) => Interval(Natural, Seventh),
        ('9', 'm', None) => Interval(Flat, Second),
        ('9', 'M', None) => Interval(Natural, Second),
        ('9', 'A', None) => Interval(Sharp, Second),
        ('1', '1', Some('d')) => Interval(Flat, Fourth),
        ('1', '1', Some('P')) => Interval(Natural, Fourth),
        ('1', '1', Some('A')) => Interval(Sharp, Fourth),
        ('1', '3', Some('d')) => Interval(DoubleFlat, Seventh),
        ('1', '3', Some('m')) => Interval(Flat, Seventh),
        ('1', '3', Some('M')) => Interval(Natural, Seventh),
        _ => panic!("{a}{b}{}", c.unwrap_or_default()),
    }
}

pub fn relative_semitones(list: &List<Interval>) -> List<u8> {
    let mut semitones = List::new();
    let mut intervals = list.as_ref().into_iter();

    if let Some(mut prev) = intervals.next() {
        assert_eq!(*prev, Interval(Natural, Unisson));

        for interval in intervals {
            semitones.push(interval.semitones() - prev.semitones());
            prev = interval;
        }

        semitones.push(OCTAVE - prev.semitones());
    }

    semitones
}

// pub fn next(&self) -> List<Interval> {
//     let relatives = self.relative_semitones();
//     let mut relatives = relatives.as_ref().into_iter();
//     let mut next = List::new();

//     if let Some(_) = relatives.next() {
//         let mut prev = 0;
//         next.push(Interval(Natural, Unisson));

//         for relative in relatives {
//             prev = prev + relative;
//             let note = CHROMATIC[prev as usize];
//             next.push(note);
//         }
//     }

//     next
// }
