use openssl::symm::{Cipher, Crypter, Mode};
use base64::decode;
use dotenv::var;

pub fn encrypt_data(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key = decode(var("ENCRYPTION_KEY")?)?;
    let iv = decode(var("ENCRYPTION_IV")?)?;

    // Ensure that key and IV have the correct length
    if key.len() != 32 {
        return Err("Invalid key length. Expected 32 bytes for AES-256.".into());
    }
    if iv.len() != 16 {
        return Err("Invalid IV length. Expected 16 bytes.".into());
    }

    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv))?;
    let mut ciphertext = vec![0; data.len() + cipher.block_size()];
    
    let count = crypter.update(data, &mut ciphertext)?;
    let rest = crypter.finalize(&mut ciphertext[count..])?;
    ciphertext.truncate(count + rest);

    Ok(ciphertext)
}

pub fn decrypt_data(ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key = decode(var("ENCRYPTION_KEY")?)?;
    let iv = decode(var("ENCRYPTION_IV")?)?;

    // Ensure that key and IV have the correct length
    if key.len() != 32 {
        return Err("Invalid key length. Expected 32 bytes for AES-256.".into());
    }
    if iv.len() != 16 {
        return Err("Invalid IV length. Expected 16 bytes.".into());
    }

    let cipher = Cipher::aes_256_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, &key, Some(&iv))?;
    let mut decrypted_data = vec![0; ciphertext.len() + cipher.block_size()];

    let count = decrypter.update(ciphertext, &mut decrypted_data)?;
    let rest = decrypter.finalize(&mut decrypted_data[count..])?;
    decrypted_data.truncate(count + rest);

    Ok(decrypted_data)
}
