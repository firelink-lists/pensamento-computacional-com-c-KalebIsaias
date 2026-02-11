/**
 * @exercise Lista Básico Rust - Exercício 2
 * @title Contador de Pares e Ímpares
 * @description Escreva um programa que percorra os números de 1 a 50, verifique se é par ou ímpar, conte quantos pares e ímpares existem e exiba os resultados.
 * @input stdin (sem entrada)
 * @output stdout
 * @timeout 1000
 * @test name="Saída esperada" input="" expected="Pares: 25\nÍmpares: 25"
 */

fn main() {
    let mut pares = 0;
    let mut impares = 0;

    for n in 1..51 {
        if n % 2 == 0 {
            pares += 1
        } else {
            impares += 1
        }
    }
    
    println!("Pares: {}", pares);
    println!("Ímpares: {}", impares);
}
