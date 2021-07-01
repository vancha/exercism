enum Messagetype {
 QUESTION,
 YELLING,
 YELLINGQUESTION,
 MEANINGLESSADDRESSING,
 OTHER,
}

fn get_type(message:&str)->Messagetype {
    match message.contains("?") {
        true => {
            match message.chars().filter(|c| c.is_lowercase()).count() {
            0 => {
                Messagetype::YELLINGQUESTION
            },
            _ => {
                Messagetype::QUESTION
            },
            }
        },
        false => {
            match message.chars().filter(|c| c.is_lowercase()).count() {
                0 => {
                    Messagetype::YELLING
                },
                _ => match message.chars().filter(|c| c.is_ascii_alphabetic()).count() { //match message.len() {
                    0 => {
                        Messagetype::MEANINGLESSADDRESSING
                    },
                    _ => {
                        Messagetype::OTHER
                    },
                }
            }
        },
    }
}

pub fn reply(message: &str) -> &str {
    match get_type(message) {
        Messagetype::QUESTION => {
            "Sure."
        },
        Messagetype::YELLING => {
            "Whoa, chill out!"
        },
        Messagetype::YELLINGQUESTION => {
            "Calm down, I know what I'm doing!"
        },
        Messagetype::MEANINGLESSADDRESSING => {
            "Fine. Be that way!"
        },
        Messagetype::OTHER => {
            "Whatever."
        }
    }
}
