#[test]

/*
struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1.0, 2.0, 3.0],
        },
        Array {
            data: [1, 2]
        }
    ];

    println!("Success!");
}
*/


fn main() {
    let _arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];

    println!("Success!");
}

#[allow(dead_code)]
struct Array<T, const N: usize> {
    data : [T; N]
}

/*
Приведено все массивы к типу Array<i32, 3>, чтобы соответствовать требованиям типа для массивов одного размера и одного типа элементов.
*/