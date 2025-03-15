mod grammar;
mod lexical;
mod sintatic;

use lexical::Lexer;

fn main() {
    let codigo = "SHOW # ACORDES xablau, y; IMPROVYSO x > 10 TOCAR(xi); FIM_IMPROVISO; ENCERRAR";
    let mut lexer = Lexer::new(codigo);

    loop {
        match lexer.proximo_token() {
            Some(token) => {
                println!("{:?}", token); // Imprime o token gerado
            }
            None => break, // Fim da entrada
        }
    }
    // let mut parser = Parse::new(lexer);

    // match parser.parse() {
    //     Ok(()) => println!("Código válido!"),
    //     Err(err) => println!("Erro: {}", err),
    // }

}