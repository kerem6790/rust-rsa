use std::io;
use rand::Rng;
use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero, ToPrimitive};
use num_integer::gcd;

fn main() {
    // Kullanıcıdan p ve q'yu al
    let mut input = String::new();
    println!("Enter first prime (p): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let p: u32 = input.trim().parse().expect("Please enter a valid number");
    
    input.clear();
    println!("Enter second prime (q): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let q: u32 = input.trim().parse().expect("Please enter a valid number");
    
    // Modulus N ve Euler Totient Fonksiyonu
    let n = p * q;
    let totient = (p - 1) * (q - 1);
    
    // Public key için e seç (gcd(e, totient) = 1 olacak şekilde rastgele bir sayı seç)
    let mut rng = rand::thread_rng();
    let mut e = rng.gen_range(2..totient);
    while gcd(e, totient) != 1 {
        e = rng.gen_range(2..totient);
    }
    
    // Private key (d) hesapla: e'nin modüler tersini al
    let e_big = e.to_bigint().unwrap();
    let totient_big = totient.to_bigint().unwrap();
    let d = e_big.modinv(&totient_big).unwrap();
    
    println!("Public Key: (e = {}, N = {})", e, n);
    println!("Private Key: (d = {}, N = {})", d, n);

    // Kullanıcıdan mesaj al
    input.clear();
    println!("Enter a message (must be < N): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let message: u32 = input.trim().parse().expect("Please enter a valid number");

    // Mesajı şifrele
    let ciphertext = encrypt(message, e, n);
    println!("Encrypted Message: {}", ciphertext);

    // Şifreyi çöz
    let decrypted_message = decrypt(ciphertext, d.clone(), n);
    println!("Decrypted Message: {}", decrypted_message);
}

// Şifreleme fonksiyonu
fn encrypt(message: u32, e: u32, n: u32) -> u32 {
    let message_big = message.to_bigint().unwrap();
    let e_big = e.to_bigint().unwrap();
    let n_big = n.to_bigint().unwrap();
    let result = message_big.modpow(&e_big, &n_big);
    result.to_u32().unwrap()
}

// Şifre çözme fonksiyonu
fn decrypt(ciphertext: u32, d: BigInt, n: u32) -> u32 {
    let ciphertext_big = ciphertext.to_bigint().unwrap();
    let n_big = n.to_bigint().unwrap();
    let result = ciphertext_big.modpow(&d, &n_big);
    result.to_u32().unwrap()
}
