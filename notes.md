##Tarefas

- [✔️] Analisador léico (Acho)
- [✔️] Analisador sintático

# 📌 TODO: Implementação do Analisador Sintático 🎵

## ✅ Estrutura Básica do Parser

- [✔️] Criar a estrutura `Parse`
- [✔️] Criar o método `avancar()` para consumir tokens
- [✔️] Criar o método `parse()` para iniciar a análise

## 🎼 Análise Sintática

### 🌟 Estrutura Principal

- [✔️] `programa()`: Valida o bloco principal `SHOW ... ENCERRAR`
- [✔️] `setlist()`: Valida a seção `ACORDES`
- [✔️] `vars()`: Analisa as variáveis declaradas
- [✔️] `var()`: Analisa um identificador

### 🎵 Comandos

- [✔️] `cmds()`: Processa a sequência de comandos
- [✔️] `cmd_improviso()`: Valida `IMPROVISO ... SOLO ... FIM_IMPROVISO`
- [✔️] `cmd_ritmo()`: Valida `RITMO ... FIM_RITMO`
- [✔️] `cmd_refrao()`: Valida `REFRÃO ... ATE ... FIM_REFRÃO`
- [✔️] `cmd_atribuicao()`: Valida atribuições `<-`
- [✔️] `cmd_afinar()`: Valida `AFINAR(var)`
- [✔️] `cmd_tocar()`: Valida `TOCAR(ep)`

### 🔢 Epressões e Condições

- [✔️] `condicao()`: Valida comparações (`ep op ep`)
- [✔️] `expressao()`: Analisa epressões matemáticas (`termo + termo`)
- [✔️] `termo()`: Analisa multiplicações e divisões (`fator * fator`)
- [✔️] `fator()`: Valida números, variáveis e epressões entre `()`

---

## 🚀 Próximos Passos

- [ ] Ler um arquivo
