use music::list::List;
use music::*;

pub fn main() {
    let a_flat = Note(A, Flat);
    let flat_second = Interval(Flat, Second);
    dbg!(a_flat);
    dbg!(flat_second.semitones());
    let mut list = List::new();
    list.push(A);
    dbg!(list.as_ref());

    // let mode = Mode {
    //     root: Note(A, Natural),
    //     intervals: List::from([
    //         Interval(Natural, Unisson),
    //         Interval(Natural, Second),
    //         Interval(Natural, Third),
    //         Interval(Natural, Fourth),
    //         Interval(Natural, Fifth),
    //         Interval(Natural, Sixth),
    //         Interval(Natural, Seventh),
    //     ]),
    // };

    // dbg!(mode);

    for mode in dictionary::diatonic().as_ref() {
        dbg!(mode);
    }
}
