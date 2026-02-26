use std::time::Instant;

/// Busca sequencial simples - sempre percorre todo o vetor
fn busca_sequencial_simples(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;
    let mut resultado = None;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i);
        }
    }

    (resultado, operacoes)
}

/// Busca sequencial com interrupção antecipada
fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;
    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes);
        }
    }

    (None, operacoes)
}

/// Gera um vetor com valores de 1 até n
fn gerar_vetor(tamanho: usize) -> Vec<i32> {
    (1..=tamanho as i32).collect()
}

fn gerar_vetor_strings(tamanho: usize) -> Vec<String>{
    let palavras = vec!["Rust", "Python" , "Java", "C++" , "C#", "Go"];
    let mut vetor = Vec::with_capacity(tamanho);

    for i in 0..tamanho{
        vetor.push(palavras[i % palavras.len()].to_string());
    }

    vetor
}

fn contar_ocorrencias(vetor: &[String], alvo: &str) -> (usize, usize , Vec<usize>) {
    let mut operacoes = 0;
    let mut contador = 0;
    let mut posicoes_encontradas = Vec::new();

    for (indice, item) in vetor.iter().enumerate(){
        operacoes += 1;
        if item == alvo {
            posicoes_encontradas.push(indice);
            contador += 1;
        }
    }

    (contador, operacoes, posicoes_encontradas)
}

fn executar_experimento_contagem(tamanho: usize, alvo: &str) {
    let vetor = gerar_vetor_strings(tamanho);

    println!("\n{}", "=".repeat(60));
    println!("Tamanho do vetor: {}", tamanho);
    println!("Palavra procurada: \"{}\"", alvo);
    println!("{}", "-".repeat(60));

    let inicio = Instant::now();
    let (total_encontrado, ops , posicoes_encontradas) = contar_ocorrencias(&vetor, alvo);

    if !posicoes_encontradas.is_empty() {
        println!("Foi encontrado nas posições: {:?}", posicoes_encontradas);
    } else {
        println!("Nenhum valor foi encontrado.");
    }
   
    let tempo = inicio.elapsed();

    println!("📌 RESULTADO DA CONTAGEM:");
    println!(" Vezes que apareceu: {}", total_encontrado);
    println!(" Operações realizadas: {}", ops);
    println!(" Tempo decorrido: {:?}", tempo);
    println!("{}", "=".repeat(60));
}


/// Executa experimento comparativo entre os dois algoritmos
fn executar_experimento(tamanho: usize, alvo: i32) {
    let vetor = gerar_vetor(tamanho);

    println!("\n{}", "=".repeat(60));
    println!("Tamanho do vetor: {}", tamanho);
    println!("Elemento procurado: {}", alvo);
    println!("{}", "-".repeat(60));

    // Busca Sequencial Simples
    let inicio = Instant::now();
    let (resultado1, ops1) = busca_sequencial_simples(&vetor, alvo);
    let tempo1 = inicio.elapsed();

    println!("\n📌 BUSCA SEQUENCIAL SIMPLES:");
    println!(" Resultado: {:?}", resultado1);
    println!(" Operações: {}", ops1);
    println!(" Tempo: {:?}", tempo1);

    // Busca Sequencial com Interrupção
    let inicio = Instant::now();
    let (resultado2, ops2) = busca_sequencial_interrompida(&vetor, alvo);
    let tempo2 = inicio.elapsed();

    println!("\n📌 BUSCA SEQUENCIAL COM INTERRUPÇÃO:");
    println!(" Resultado: {:?}", resultado2);
    println!(" Operações: {}", ops2);
    println!(" Tempo: {:?}", tempo2);

    // Análise Comparativa
    println!("\n📊 ANÁLISE COMPARATIVA:");
    println!(" Diferença de operações: {} operações", ops1.saturating_sub(ops2));
    
    if tempo1 > tempo2 {
        println!(" Algoritmo com interrupção foi mais rápido");
    } else if tempo2 > tempo1 {
        println!(" Algoritmo simples foi mais rápido (provavelmente devido à variação)");
    } else {
        println!(" Tempos praticamente iguais");
    }
    println!("{}", "=".repeat(60));
}

fn main() {
    println!("\n🔬 EXPERIMENTO: COMPARAÇÃO DE ALGORITMOS DE BUSCA\n");

    println!("\n🎯 CENÁRIO 1: Elemento no início (melhor caso para interrupção)");
    executar_experimento(1_000, 5);
    executar_experimento(100_000, 5);
    executar_experimento(1_000_000, 5);

    println!("\n\n🎯 CENÁRIO 2: Elemento no meio do vetor");
    executar_experimento(1_000, 500);
    executar_experimento(100_000, 50_000);
    executar_experimento(1_000_000, 500_000);

    println!("\n\n🎯 CENÁRIO 3: Elemento no final (pior caso)");
    executar_experimento(1_000, 1_000);
    executar_experimento(100_000, 100_000);
    executar_experimento(1_000_000, 1_000_000);

    println!("\n\n🎯 CENÁRIO 4: Elemento não existe no vetor");
    executar_experimento(1_000, -1);
    executar_experimento(100_000, -1);
    executar_experimento(1_000_000, -1);

    println!("\n\n✅ Experimento concluído!\n");

    println!("\n🔬 EXPERIMENTO: CONTAGEM DE APARIÇÕES\n");

    executar_experimento_contagem(100, "Rust");
    executar_experimento_contagem(100, "Java");
    executar_experimento_contagem(100, "JavaScript");

     println!("\n\n✅ Experimento concluído!\n");
}
