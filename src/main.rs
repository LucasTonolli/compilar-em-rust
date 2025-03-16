mod grammar;
mod lexical;
mod sintatic;

use lexical::Lexer;
use sintatic::Parse;
use std::env;
use std::fs;


fn main() {
    // Obtém os argumentos da linha de comando
    let args: Vec<String> = env::args().collect();
    
    // Verifica se foi passado um nome de arquivo
    if args.len() < 2 {
        eprintln!("Uso: {} <arquivo>", args[0]);
        return;
    }

    // Tenta ler o conteúdo do arquivo
    let code = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo: {}", err);
            return;
        }
    };

    println!("Código-fonte:\n{}", code);

    // Cria o lexer
    let lexer = Lexer::new(&code); // `code` agora é um `String`, então a referência `&code` é válida.

    // Cria o parser
    let mut parser = Parse::new(lexer);

    // Executa a análise
    match parser.parse() {
        Ok(()) => println!("Código válido!"),
        Err(err) => eprintln!("Erro: {}", err),
    }
}
