// This is the main function
// main関数です

// Unlike C/C++, there's no restriction on the order of function definitions
// C/C++とは違い、関数の定義を行う順番に制限はない。
fn main() {
    // Variables can be type annotated.
    // 変数に型を指定
    let logical: bool = true;
    println!("{}",logical);

    let an_integer   = 5i32;
    println!("{}",an_integer);

    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    println!("{}",inferred_type);


    let mut mutable = 12; // Mutable `i32`
                          // ミュータブルな `i32`.
    mutable = 21i64;

    println!("{}",mutable);

    // // Error! The type of a variable can't be changed.
    // // エラー！ ミュータブルな変数でも型は不変
    // mutable = true;

    // // Variables can be overwritten with shadowing.
    // let mutable = true;
    let mutable = true;
}
