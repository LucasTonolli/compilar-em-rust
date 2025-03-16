<G> ::= 'SHOW' <SETLIST> <CMDS> 'ENCERRAR'
<SETLIST> ::= 'ACORDES' <VARS>
<VARS> ::= <VAR> ',' <VARS>
<VARS> ::= <VAR>
<CMDS> ::= <CMD> ';' <CMDS>
<CMDS> ::= <CMD>
<CMD> ::= <CMD_IF>
<CMD> ::= <CMD_LOOP>
<CMD> ::= <CMD_FOR>
<CMD> ::= <CMD_ATRIBUICAO>
<CMD> ::= <CMD_LER>
<CMD> ::= <CMD_TOCAR>
<CMD_IF> ::= 'IMPROVISO' <CONDICAO> <CMDS> 'FIM_IMPROVISO'
<CMD_IF> ::= 'IMPROVISO' <CONDICAO> <CMDS> 'SOLO' <CMDS> 'FIM_IMPROVISO'
<CMD_LOOP> ::= 'RITMO' <CONDICAO> <CMDS> 'FIM_RITMO'
<CMD_FOR> ::= 'REFRAO' <VAR> '<-' <E> 'ATE' <E> <CMDS> 'FIM_REFRAO'
<CMD_ATRIBUICAO> ::= <VAR> '<-' <E>
<CMD_LER> ::= 'AFINAR' '(' <VAR> ')'
<CMD_IMPRIMIR> ::= 'TOCAR' '(' <E> ')'
<CONDICAO> ::= <E> '>' <E>
<CONDICAO> ::= <E> '>=' <E>
<CONDICAO> ::= <E> '<>' <E>
<CONDICAO> ::= <E> '<=' <E>
<CONDICAO> ::= <E> '<' <E>
<CONDICAO> ::= <E> '==' <E>
<E> ::= <E> '+' <T>
<E> ::= <E> '-' <T>
<E> ::= <T>
<T> ::= <T> '\*' <F>
<T> ::= <T> '/' <F>
<T> ::= <T> '%' <F>
<T> ::= <F>
<F> ::= '-' <F>
<F> ::= <X> '\*\*' <F>
<F> ::= <X>
<X> ::= '(' <E> ')'
<X> ::= [0-9]+('.'[0-9]+)
<X> ::= <VAR>
