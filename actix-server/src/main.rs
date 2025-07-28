fn main() {}

// use solana_sdk::{ pubkey::Pubkey, signature::Keypair, signer::{ SeedDerivable, Signer } };

// fn main() {
//     use const_crypto::ed25519;

//     //pub const POOL_AUTHORITY_PREFIX: &[u8] = b"pool_authority";
//     const POOL_AUTHORITY_AND_BUMP: ([u8; 32], u8) = ed25519::derive_program_address(
//         &[b"pool_authority"],
//         &Pubkey::from_str_const("smthzQ3PhuKkbpRVkRcUKjiHFgyfUqjmuo68L3dnsKF").to_bytes()
//     );

//     //const METEORA_CONFIG:

//     pub const ID: Pubkey = Pubkey::new_from_array(POOL_AUTHORITY_AND_BUMP.0);
//     pub const BUMP: u8 = POOL_AUTHORITY_AND_BUMP.1;

//     // let token_a_vault_address1 = ed25519::derive_program_address(
//     //     seeds,
//     //     program
//     // );

//     println!("ID: {}", ID);
// }
// smthzQ3PhuKkbpRVkRcUKjiHFgyfUqjmuo68L3dnsKF
//AKdY87CAiQCLVF52ZfBAWxtQeFrWZ5XKwHPPanbCgY1M
// G3YvJfzMpXr9HZicvKok9erisP12w6NVnkS28QA2KRbv
// use solana_sdk::{signature::{Keypair, Signer}, signer::SeedDerivable};
// use bs58;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Вставьте ваш Base58 seed
//     let base58_seed = "3UmC5EktZqTchpV1Kx6UozASLmnohRYFFELf7Y4uhUrJAjDtGk7uwLt49JsE2xY5WLvU3zihKrwPmPfSrp5qgKYf"; // Замените на полный Base58 seed

//     // Декодируем Base58 seed в массив байтов
//     let seed = bs58::decode(base58_seed)
//         .into_vec()
//         .map_err(|e| format!("Ошибка декодирования Base58 seed: {}", e))?;

//     // Проверяем длину seed (должно быть 32 байта для seed)
//     // if seed.len() != 32 {
//     //     return Err(format!(
//     //         "Неверная длина seed. Ожидается 32 байта, получено {} байт.",
//     //         seed.len()
//     //     )
//     //     .into());
//     // }

//     // Приводим seed к массиву [u8; 32]
//     let seed_array: [u8; 32] = seed
//         .try_into()
//         .map_err(|_| "Не удалось преобразовать seed в [u8; 32]")?;

//     // Создаём ключевую пару из seed
//     let keypair = Keypair::from_seed(&seed_array)
//         .map_err(|e| format!("Ошибка создания ключевой пары: {}", e))?;

//     // Выводим секретный ключ (массив) и публичный ключ
//     let secret_key = keypair.to_bytes(); // Полный массив ключевой пары (64 байта: 32 seed + 32 pubkey)
//     println!("Секретный ключ (массив): {:?}", secret_key);
//     println!("Публичный ключ: {}", keypair.pubkey());

//     // Сохраняем ключевую пару в файл (опционально)
//     // let secret_key_json = serde_json::to_string(&secret_key)?;
//     // std::fs::write("recovered_keypair.json", secret_key_json)?;
//     // println!("Ключевая пара сохранена в recovered_keypair.json");

//     Ok(())
// }
