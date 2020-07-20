mod lib;

use lib::frequency;

fn main() {
    //let args: Vec<String> = std::env::args().skip(1).collect();

    const LINES: &[&str; 8] = &[
        "O say can you see by the dawn's early light,",
        "What so proudly we hailed at the twilight's last gleaming,",
        "Whose broad stripes and bright stars through the perilous fight,",
        "O'er the ramparts we watched, were so gallantly streaming?",
        "And the rockets' red glare, the bombs bursting in air,",
        "Gave proof through the night that our flag was still there;",
        "O say does that star-spangled banner yet wave,",
        "O'er the land of the free and the home of the brave?",
    ];

    let worker_counts = 3;

    let counts = frequency(LINES, worker_counts);

    //for (c, n) in counts {
    //println!("'{}' {}", c, n);
    //}
    //const LINES: &[&str] = &["abcde", "bcdef", "cdefg"];
}