use bitflags::bitflags;

fn case_ratio(message: &str) -> f32 {
    let (mut u, mut l) = (0 as f32, 0 as f32);
    for c in message.chars().filter(|c| c.is_alphabetic()) {
        if c.is_uppercase() {
            u += 1.;
        } else {
            l += 1.
        }
    }
    u / l
}

bitflags! {
    struct MessageFlags: u8 {
        // Flags to build from
        const CALM      = 0b0000;
        const YELL      = 0b0010;
        const ASSERTION = 0b0000;
        const QUESTION  = 0b0100;
        // Flags to check against
        const EMPTY_MESSAGE  = 0b0001;
        const CALM_ASSERTION = Self::CALM.bits | Self::ASSERTION.bits;
        const YELL_ASSERTION = Self::YELL.bits | Self::ASSERTION.bits;
        const CALM_QUESTION  = Self::CALM.bits | Self::QUESTION.bits;
        const YELL_QUESTION  = Self::YELL.bits | Self::QUESTION.bits;
    }
}

struct Responses<'a> {
    pub empty: &'a str,
    pub calm_assertion: &'a str,
    pub yell_assertion: &'a str,
    pub calm_question: &'a str,
    pub yell_question: &'a str,
    //pub zalgo: &'a str,
}

const RESPONSES: Responses = Responses {
    empty:          "Fine. Be that way!",
    calm_assertion: "Whatever.",
    yell_assertion: "Whoa, chill out!",
    calm_question:  "Sure.",
    yell_question:  "Calm down, I know what I'm doing!",
    //zalgo:          "b͉̼̻̻̮̤̈́̎̓̈͂̓ͦ͞ǒ͕͖͕̈̔̒̃̒̓b͓͙͚̬̙̎ͫ ͙̗̇ͦ̏̈ͨͯ͜d̘̐̎̎̎̏o̷͛̾̿͌̔e̟͉̠͍͊̇̆͘ͅs̖̯̣͞ ̧͔̩̒n̵̟͉̬͊̈ͮ͊̑ͧo͎̳̹̤̙͐̾͌̎̒ͯt̥̪͈̥͝ ̝̞͍̳̎͐͌ͤ́u͕͙̝̞̎̈́͞nͨ̿͂̌̏̃҉̥̝͕̜̜ͅḓ̗͔̱̜̽̽̿͐̀ėr̖̟̎ͬ͐͋s̶̤̬̟̦̙͕̺̍ͮ̒ͫͪt̙̱̥̬͔̣̑̿̂͊̃̍ͅä̡̬́̂̊̊̏̀n̖̝͕̼͎͑̀̌ͪḏ͇͊͋̇̆́͡",
};

pub fn reply(message: &str) -> &str {
    let mut message_flags = MessageFlags::empty();
    if message.chars().all(|c| c.is_whitespace()) {
        message_flags |= MessageFlags::EMPTY_MESSAGE;
    } else {
        let r = case_ratio(message);
        if r.is_nan() || r < 0.5 {
            message_flags |= MessageFlags::CALM;
        } else {
            message_flags |= MessageFlags::YELL;
        };
        if !message.trim_end().ends_with("?") {
            message_flags |= MessageFlags::ASSERTION;
        } else {
            message_flags |= MessageFlags::QUESTION;
        };
    }
    match message_flags {
        MessageFlags::EMPTY_MESSAGE  => RESPONSES.empty,
        MessageFlags::CALM_ASSERTION => RESPONSES.calm_assertion,
        MessageFlags::YELL_ASSERTION => RESPONSES.yell_assertion,
        MessageFlags::CALM_QUESTION  => RESPONSES.calm_question,
        MessageFlags::YELL_QUESTION  => RESPONSES.yell_question,
        //_                            => RESPONSES.zalgo,
        _ => unreachable!()
    }
}
