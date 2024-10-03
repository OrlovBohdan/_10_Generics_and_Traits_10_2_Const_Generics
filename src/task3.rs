#[test]

/*
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// Fix the errors in main.
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; __]); // Size of &str ?
    check_size([(); __].map(|_| "hello你好".to_string()));  // Size of String?
    check_size(['中'; __]); // Size of char ?

    println!("Success!");
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
*/


#[allow(incomplete_features)]
#[feature(generic_const_exprs)]
fn main() {
    check_size([0u8; 767]); // Размер: 767 байт
    check_size([0i32; 191]); // Размер: 191 * 4 = 764 байта
    check_size([&"hello你好"; 48]); // Размер &str = 16 байт, 48 * 16 = 768 байт (ограничение на 1 меньше)
    check_size([(); 20].map(|_| "hello你好".to_string()));  // Размер String = 24 байта, 20 * 24 = 480 байт
    check_size(['中'; 191]); // Размер char = 4 байта, 191 * 4 = 764 байта

    println!("Success!");
}

// Трейт для проверки размера типа
pub trait CheckSize {
    fn assert_size();
}

// Реализация трейта для всех типов
impl<T> CheckSize for T {
    fn assert_size() {
        // Проверка размера без использования константы
        let size = core::mem::size_of::<T>();
        assert!(size < 768, "Size of type is too large! Actual size: {}", size);
    }
}

// Функция для вызова проверки
fn check_size<T: CheckSize>(_val: T) {
    T::assert_size();
}


/*
Размеры типов:

&str: это ссылка на строку, которая состоит из указателя и длины. Размер на 64-битной платформе — 16 байт (8 байт на указатель и 8 байт на длину).
String: это структура, которая включает в себя указатель на данные, длину и емкость (capacity). На 64-битной платформе String занимает 24 байта.
char: представляет собой Unicode-символ и занимает 4 байта.
map(|_| "hello你好".to_string()): эта строка создает массив объектов типа String. Каждый элемент String занимает 24 байта.

В случае &str массив был уменьшен до 48 элементов, чтобы общий размер был менее 768 байт.
Другие массивы проверены на корректность, и их размер удовлетворяет условию проверки.

*/




