#[derive(Debug, PartialEq)]
pub enum Token {
    // Palavras-chave
    Show,         // SHOW
    Encerrar,     // ENCERRAR
    Acordes,      // ACORDES
    Improviso,    // IMPROVISO
    FimImproviso, // FIM_IMPROVISO
    Solo,         // SOLO
    Ritmo,        // RITMO
    FimRitmo,     // FIM_RITMO
    Refrao,       // REFRÃO
    Ate,          // ATE
    FimRefrao,    // FIM_REFRÃO
    Afinar,       // AFINAR
    Tocar,        // TOCAR

    // Símbolos e operadores
    Atribuicao,   // <-
    Maior,        // >
    MaiorIgual,   // >=
    Menor,        // <
    MenorIgual,   // <=
    Diferente,    // <>
    Igual,        // ==
    Soma,         // +
    Subtracao,    // -
    Multiplicacao,// *
    Divisao,      // /
    Modulo,       // %

    // Outros
    Numero(f64),  // Números
    Identificador(String), // Variáveis e nomes
    ErroLexico(char),
    ParentesesEsq, // (
    ParentesesDir, // )
    Virgula,       // ,
    PontoVirgula,  // ;
}
