use crate::grammar::Token;
use crate::lexical::Lexer;

pub struct Parse {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parse {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
        };
        parser.avancar().ok();
        parser
    }

    fn avancar(&mut self) -> Result<(), String> {
        match self.lexer.proximo_token() {
            Some(Token::ErroLexico(ch)) => Err(format!("Erro léxico encontrado: {}", ch)),
            Some(token) => {
                self.current_token = Some(token);
                Ok(())
            }
            None => {
                self.current_token = None;
                Ok(())
            }
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.programa()
    }

    fn programa(&mut self) -> Result<(), String> {
        if let Some(Token::Show) = self.current_token {
            self.avancar()?;
            self.setlist()?;
            self.cmds()?;

            if let Some(Token::Encerrar) = self.current_token {
                self.avancar()?;
                Ok(())
            } else {
                Err("Erro de sintaxe: esperado 'ENCERRAR'".to_string())
            }
        } else {
            Err("Erro de sintaxe: esperado 'SHOW'".to_string())
        }
    }

    fn setlist(&mut self) -> Result<(), String> {
        if let Some(Token::Acordes) = self.current_token {
            self.avancar()?;
            self.vars()?;
            Ok(())
        } else {
            Err("Erro de sintaxe: esperado 'ACORDES'".to_string())
        }
    }

    fn vars(&mut self) -> Result<(), String> {
        self.var()?;
        while matches!(self.current_token, Some(Token::Virgula)) {
            self.avancar()?;
            self.var()?;
        }
        Ok(())
    }

    fn var(&mut self) -> Result<(), String> {
        if let Some(Token::Identificador(_)) = &self.current_token {
            self.avancar()?;
            Ok(())
        } else {
            Err("Erro de sintaxe: esperado identificador".to_string())
        }
    }

    fn cmds(&mut self) -> Result<(), String> {
        while let Some(token) = &self.current_token {
            println!("token: {:?}", token);
            match token {
                Token::Improviso => self.cmd_improviso()?,
                Token::Ritmo => self.cmd_ritmo()?,
                Token::Refrao => self.cmd_refrao()?,
                Token::Identificador(_) => self.cmd_atribuicao()?,
                Token::Afinar => self.cmd_afinar()?,
                Token::Tocar => self.cmd_tocar()?,
                Token::FimImproviso => break,
                Token::Solo => break,
                Token::FimRitmo => break,
                Token::FimRefrao => break,
                Token::Encerrar => break,
                _ => return Err(format!("Erro de sintaxe: comando inesperado {:?}", self.current_token)),
            }

            if let Some(Token::PontoVirgula) = self.current_token {
                self.avancar()?;
            } else {
                return Err("Erro de sintaxe: esperado ';' entre comandos".to_string());
            }
        }
        Ok(())
    }

    fn cmd_improviso(&mut self) -> Result<(), String> {
        self.avancar()?;
        println!("token 1, {:?}", self.current_token );
        self.condicao()?;
        self.cmds()?;

        println!("token 2, {:?}", self.current_token );

        if let Some(Token::FimImproviso) = self.current_token {
            println!("token FIM, {:?}", self.current_token );
            self.avancar()?;
            Ok(())
        } else if let Some(Token::Solo) = self.current_token {
        
            self.avancar()?;
            self.cmds()?;
            if let Some(Token::FimImproviso) = self.current_token {
                self.avancar()?;
                Ok(())
            } else {
                Err("Erro de sintaxe: esperado 'FIM_IMPROVISO'".to_string())
            }
        } else {
            Err("Erro de sintaxe: esperado 'FIM_IMPROVISO' ou 'SOLO'".to_string())
        }
    }

    fn cmd_ritmo(&mut self) -> Result<(), String> {
        self.avancar()?;
        self.condicao()?;
        self.cmds()?;

        if let Some(Token::FimRitmo) = self.current_token {
            self.avancar()?;
            Ok(())
        } else {
            Err("Erro de sintaxe: esperado 'FIM_RITMO'".to_string())
        }
    }

    fn cmd_refrao(&mut self) -> Result<(), String> {
        self.avancar()?;
        self.var()?;
        println!("TOKEN refrao {:?}", self.current_token);
        if let Some(Token::Atribuicao) = self.current_token {
            self.avancar()?;
            self.expressao()?;
        } else {
            return Err("Erro de sintaxe: esperado '<-'".to_string());
        }

        if let Some(Token::Ate) = self.current_token {
            println!("TOKEN ATE {:?}", self.current_token);
            self.avancar()?;
            self.condicao()?;
            println!("TOKEN ATE FIM {:?}", self.current_token);

        } else {
            return Err("Erro de sintaxe: esperado 'ATE'".to_string());
        }

        self.cmds()?;

        if let Some(Token::FimRefrao) = self.current_token {
            self.avancar()?;
            Ok(())
        } else {
            Err("Erro de sintaxe: esperado 'FIM_REFRÃO'".to_string())
        }
    }

    fn cmd_atribuicao(&mut self) -> Result<(), String> {
        self.var()?;

        if let Some(Token::Atribuicao) = self.current_token {
            self.avancar()?;
            self.expressao()?;
            Ok(())
        } else {
            Err("Erro de sintaxe: esperado '<-' na atribuição".to_string())
        }
    }

    fn cmd_afinar(&mut self) -> Result<(), String> {
        self.avancar()?;

        if let Some(Token::ParentesesEsq) = self.current_token {
            self.avancar()?;
            self.var()?;

            if let Some(Token::ParentesesDir) = self.current_token {
                self.avancar()?;
                Ok(())
            } else {
                Err("Erro de sintaxe: esperado ')'".to_string())
            }
        } else {
            Err("Erro de sintaxe: esperado '('".to_string())
        }
    }

    fn cmd_tocar(&mut self) -> Result<(), String> {
        self.avancar()?;

        if let Some(Token::ParentesesEsq) = self.current_token {
            self.avancar()?;
            self.expressao()?;

            if let Some(Token::ParentesesDir) = self.current_token {
                self.avancar()?;
                Ok(())
            } else {
                Err("Erro de sintaxe: esperado ')'".to_string())
            }
        } else {
            Err("Erro de sintaxe: esperado '('".to_string())
        }
    }
    //Função para operadores de comparação
    fn condicao(&mut self) -> Result<(), String> {
        self.expressao()?;
        if let Some(Token::Igual | Token::Diferente | Token::Maior | Token::Menor | Token::MaiorIgual | Token::MenorIgual) = self.current_token {
            self.avancar()?;
            self.expressao()?;
            Ok(())
        } else {
            Err("Erro de sintaxe: esperado operador de comparação".to_string())
        }
    }

    fn expressao(&mut self) -> Result<(), String> {
        self.termo()?;
       

        while matches!(self.current_token, Some(Token::Soma | Token::Subtracao | Token::Multiplicacao | Token::Divisao)) {
            self.avancar()?;
            self.termo()?;
        }
            

        Ok(())
    }

    fn termo(&mut self) -> Result<(), String> {
        self.fator()?;
        while matches!(self.current_token, Some(Token::Multiplicacao)) {
            self.avancar()?;
            self.fator()?;
        }
        Ok(())
    }

    fn fator(&mut self) -> Result<(), String> {
        match &self.current_token {
            Some(Token::Numero(_)) | Some(Token::Identificador(_)) => {
                self.avancar()?;
                Ok(())
            }
            Some(Token::ParentesesEsq) => {
                self.avancar()?;
                self.expressao()?;
                self.avancar()
            }
            _ => Err("Erro de sintaxe: esperado número, variável ou expressão entre parênteses".to_string()),
        }
    }
}
