async fn pow(num: i8, exp: i8) -> i32 {
    let mut potencia: i32 = 1;
    let mut i = 1;
    while i <= exp {
        potencia *= num as i32;
        i+= 1;
    }
    println!("{:?}", potencia);
    return potencia;

}

#[tokio::main]
async fn main() {
    let a: i8 = 3;
    let b: i8 = 3;
    let potencia = pow(a, b).await;
}
