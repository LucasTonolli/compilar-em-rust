use crate::grammar::Token; // Importa os tokens da gramática

pub struct Lexer {
    input: Vec<char>, // Os caracteres da entrada
    pos: usize, // Posição atual
}

impl Lexer {
    // Cria um novo Lexer
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(), // Converte o input para um vetor de caracteres
            pos: 0,
        }
    }

    // Avança a posição
    fn avancar(&mut self) {
        self.pos += 1;
    }

    // Retorna o próximo caractere (sem avançar)
    fn proximo_char(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    // Retorna o próximo token
    pub fn proximo_token(&mut self) -> Option<Token> {
        while let Some(ch) = self.proximo_char() {
            match ch {
                ' ' | '\n' | '\t' | '\r' => self.avancar(), // Ignora espaços em branco
                '(' => { self.avancar(); return Some(Token::ParentesesEsq); }
                ')' => { self.avancar(); return Some(Token::ParentesesDir); }
                ',' => { self.avancar(); return Some(Token::Virgula); }
                ';' => { self.avancar(); return Some(Token::PontoVirgula); }
                '+' => { self.avancar(); return Some(Token::Soma); }
                '-' => { self.avancar(); return Some(Token::Subtracao); }
                '*' => { self.avancar(); return Some(Token::Multiplicacao); }
                '/' => { self.avancar(); return Some(Token::Divisao); }
                '%' => { self.avancar(); return Some(Token::Modulo); }
                '>' => {
                    self.avancar();
                    if let Some('=') = self.proximo_char() {
                        self.avancar();
                        return Some(Token::MaiorIgual);
                    }
                    return Some(Token::Maior);
                }
                '<' => {
                    self.avancar();
                    if let Some('=') = self.proximo_char() {
                        self.avancar();
                        return Some(Token::MenorIgual);
                    } else if let Some('>') = self.proximo_char() {
                        self.avancar();
                        return Some(Token::Diferente);
                    
                    } else if let Some('-') = self.proximo_char() {
                        self.avancar();
                        return Some(Token::Atribuicao)
                    }
                    return Some(Token::Menor);
                }
                '=' => {
                    self.avancar();
                    if let Some('=') = self.proximo_char() {
                        self.avancar();
                        return Some(Token::Igual);
                    }
                }
                _ if ch.is_ascii_digit() => {
                    return Some(self.numero());
                }
                _ if ch.is_alphanumeric() => {
                    return Some(self.identificador());
                }
                _ => {
                    return Some(self.erro_lexico(ch)); 
                }
            }
        }
        None
    }

    // Função para ler um número
    fn numero(&mut self) -> Token {
        let mut num_str = String::new();
        while let Some(ch) = self.proximo_char() {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                self.avancar();
            } else {
                break;
            }
        }
        Token::Numero(num_str.parse().unwrap())
    }

    // Função para identificar palavras-chave e identificadores
    fn identificador(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(ch) = self.proximo_char() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.avancar();
            } else {
                break;
            }
        }
        

        match ident.as_str() {
            "SHOW" => Token::Show,
            "ACORDES" => Token::Acordes,
            "IMPROVISO" => Token::Improviso,
            "FIM_IMPROVISO" => Token::FimImproviso,
            "SOLO" => Token::Solo,
            "RITMO" => Token::Ritmo,
            "FIM_RITMO" => Token::FimRitmo,
            "REFRAO" => Token::Refrao,
            "ATE"   => Token::Ate,
            "FIM_REFRAO" => Token::FimRefrao,
            "AFINAR" => Token::Afinar,
            "TOCAR" => Token::Tocar,
            "ENCERRAR" => Token::Encerrar,
            _ => Token::Identificador(ident)
        }
    }
    fn erro_lexico(&mut self, ch: char) -> Token {
        self.avancar();  // Avança o ponteiro para o próximo caractere após o erro
        Token::ErroLexico(ch)  // Cria um token de erro com o caractere inválido
    }

}
