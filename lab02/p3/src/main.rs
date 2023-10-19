fn add_space(s: &mut String, mut n: i32) {
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
}
fn add_str(s1: &mut String, s2: &str) {
    s1.push_str(s2);
}
fn add_integer(s: &mut String, n: i32) {
    let mut n = n;
    let mut num_digits = 0;
    let mut temp = n;
    let mut contor = 0;
    while temp != 0 {
        num_digits += 1;
        temp /= 10;
    }
    if n < 0 {
        s.push('-');
        n = -n;
    }
    let mut i = num_digits - 1;
    while i >= 0 {
        let digit = ((n / 10_i32.pow(i as u32)) % 10) as u8 + b'0';
        if contor % 3 == 0 && contor != 0 {
            s.push('_');
        }
        s.push(digit as char);
        contor += 1;
        i -= 1;
    }
}
fn add_float(s: &mut String, n: f32) {
    let mut parte_intreaga = n as i32;
    let mut parte_fract = (n - parte_intreaga as f32) as f32;
    while parte_intreaga != 0 {
        let cif = parte_intreaga % 10;
        let c = (cif as u8 + '0' as u8) as char;
        s.push(c);
        parte_intreaga = parte_intreaga / 10;
    }
    s.push('.');
    while parte_fract != 0.0 {
        parte_fract = parte_fract * 10.0;
        let cif = parte_fract as i32 % 10;
        let c = (cif as u8 + '0' as u8) as char;
        s.push(c);
        parte_fract = parte_fract - (parte_fract as i32) as f32;
    }
}

fn main() {
    let mut s1 = String::from("");
    let mut s2 = String::from("");
    let mut s3 = String::from("");
    let mut s4 = String::from("");
    add_space(&mut s1, 40);
    add_str(&mut s1, "I");
    add_space(&mut s1, 1);
    add_str(&mut s1, "💚");
    add_space(&mut s3, 40);
    add_str(&mut s3, "RUST.");
    add_space(&mut s2, 4);
    add_str(&mut s2, "Most");
    add_space(&mut s2, 12);
    add_str(&mut s2, "crate");
    add_space(&mut s2, 6);
    add_integer(&mut s2, 306_437_968);
    add_space(&mut s2, 11);
    add_str(&mut s2, "and");
    add_space(&mut s2, 5);
    add_str(&mut s2, "latest");
    add_space(&mut s2, 9);
    add_str(&mut s2, "is");
    add_space(&mut s4, 10);
    add_str(&mut s4, "downloaded");
    add_space(&mut s4, 8);
    add_str(&mut s4, "has");
    add_space(&mut s4, 13);
    add_str(&mut s4, "downloads");
    add_space(&mut s4, 5);
    add_str(&mut s4, "the");
    add_space(&mut s4, 9);
    add_str(&mut s4, "version");
    add_space(&mut s4, 4);
    add_float(&mut s4, 2.038);
    add_str(&mut s4, ".");
    println!("{}", s1);
    println!("{}", s3);
    println!("{}", s2);
    println!("{}", s4);
}
