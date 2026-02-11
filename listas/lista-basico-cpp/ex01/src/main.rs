/**
 * @exercise Lista Básico C++ - Exercício 1
 * @title Olá Mundo Personalizado
 * @description Modifique o programa "Olá, mundo!" para pedir e ler o nome do usuário e exibir uma saudação personalizada.
 * @input stdin
 * @output stdout
 * @timeout 1000
 * @test name="Nome simples" input="João" expected="Olá, João!"
 * @test name="Nome composto" input="Maria Silva" expected="Olá, Maria Silva!"
 * @test name="Nome vazio" input="" expected="Olá, !"
 */

use std::io;

fn main() {
    let mut name = String::new();
    
    println!("Qual o seu nome?");
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim_end_matches(['\n', '\r']);

    
    println!("Olá, {}!", name);
}
