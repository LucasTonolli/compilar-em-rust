mod grammar;
mod lexical;
mod sintatic;

use lexical::Lexer;
use sintatic::Parse;
fn main() {
    let codigo = "
    SHOW
      ACORDES x,y
      x <- (10 * 2);
      
      REFRAO x <- 10 + 2 ATE x >= 48
        TOCAR(x);
      FIM_REFRAO;
     
    ENCERRAR";
    let lexer = Lexer::new(codigo);

    // loop {
    //     match lexer.proximo_token() {
    //         Some(token) => {
    //             println!("{:?}", token); // Imprime o token gerado
    //         }
    //         None => break, // Fim da entrada
    //     }
    // }
    let mut parser = Parse::new(lexer);

    match parser.parse() {
        Ok(()) => println!("Código válido!"),
        Err(err) => println!("Erro: {}", err),
    }

}