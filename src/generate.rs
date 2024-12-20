use rand::Rng;

pub fn generate_random_string(
    length: usize,
    include_number: bool,
    include_symbol: bool,
    include_uppercase: bool,
    include_lowercase: bool,
) -> String {
    let mut rng = rand::thread_rng();

    let mut chars = vec![];

    let uppercase = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase = b"abcdefghijklmnopqrstuvwxyz";
    let number = b"0123456789";
    let symbol = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

    if include_number {
        chars.extend_from_slice(number);
    }

    if include_symbol {
        chars.extend_from_slice(symbol);
    }

    if include_uppercase {
        chars.extend_from_slice(uppercase);
    }

    if include_lowercase {
        chars.extend_from_slice(lowercase);
    }

    let random_string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..chars.len());
            chars[index] as char
        })
        .collect();

    random_string
}
