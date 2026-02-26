# Search Perf Rust 🔍🦀

Este repositório contém um experimento prático desenvolvido em **Rust** para analisar a eficiência de algoritmos de busca sequencial em diferentes cenários de uso.

O objetivo é demonstrar como pequenas mudanças na lógica (como a interrupção antecipada) e o tipo de dado (Inteiros vs Strings) impactam o tempo de execução e o custo computacional.

## 🧪 Experimentos Realizados

O programa testa quatro cenários críticos para validar a complexidade $O(n)$:

1. **Melhor Caso:** O elemento alvo está logo no início do vetor.
2. **Caso Médio:** O elemento alvo está exatamente no meio.
3. **Pior Caso:** O elemento alvo está na última posição.
4. **Elemento Inexistente:** O algoritmo percorre todo o vetor e não encontra nada.



## 📊 Funcionalidades Implementadas

* **Busca Sequencial Simples:** Implementação que percorre 100% do vetor, independente de encontrar o alvo.
* **Busca Sequencial com Interrupção:** Implementação otimizada que encerra a execução assim que o alvo é localizado.
* **Contador de Ocorrências:** Algoritmo para busca e contagem de múltiplas instâncias de Strings.
* **Análise de Operações:** O código não mede apenas o tempo (em nanosegundos/microsegundos), mas também conta o número real de iterações realizadas.

## 🚀 Como Executar

Certifique-se de ter o [Rust](https://www.rust-lang.org/) instalado.

1. Clone o repositório:
   ```bash
   git clone https://github.com/Edu136/search-perf-rust.git
