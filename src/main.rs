use std::env;
use rand::RngExt; // Importa o gerador de números aleatórios

fn tentar_atualizar() -> Result<(), Box<dyn std::error::Error>> {
    println!("Verificando atualizações...");

    // O env!("CARGO_PKG_VERSION") pega a versão direto do seu Cargo.toml
    let versao_atual = env!("CARGO_PKG_VERSION");

    // Configurando de onde baixar o update
    let status = self_update::backends::github::Update::configure()
        .repo_owner("aarch6660")
        .repo_name("Anny_Repo")
        .bin_name("anny")
        .show_download_progress(true)
        .current_version(versao_atual)
        .build()?
        .update()?;

    if status.updated() {
        println!("Atualizado com sucesso para a versão: {}!", status.version());
    } else {
        println!("Você já está na versão mais recente ({}).", versao_atual);
    }

    Ok(())
}

fn mostrar_boas_vindas() {
    // Array contendo todas as suas artes em "Raw Strings"
    let artes = [
r#"                   .
                 / V\
               / `  /
             <<   |
             /    |
           /      |
         /        |
       /    \  \ /
      (      ) | |
      ________|  _/_  | |
    <__________\______)\__)"#,

r#"                        ,     ,
                        |\---/|
                        /  , , |
                  __.-'|  / \ /
         __ ___.-'        ._O|
      .-'  '        :      _/
     / ,    .        .     |
    :  ;    :        :   _/
    |  |  .'      __:   /
    |  :  /'----'| \  |
    \  |\  |      | /| |
     '.'| /       || \ |
     | /|.'       '.l \\_
snd  || ||             '-'
     '-''-'"#,

r#"                             __
                           .d$$b
                         .' TO$;\
                        /  : TP._;
                       / _.;  :Tb|
                      /   /   ;j$j
                  _.-"       d$$$$
                .' ..       d$$$$;
               /  /P'      d$$$$P. |\
              /   "      .d$$$P' |\^"l
            .'           `T$P^"""""  :
          ._.'      _.'                ;
       `-.-".-'-' ._.       _.-"    .-"
     `.-" _____  ._             .-"
    -(.g$$$$$$$b.             .'
      ""^^T$$$P^)            .(:
        _/  -"  /.'         /:/;
    ._.'-'`-'  ")/         /;/;
 `-.-"..--""   " /         /  ;
.-" ..--""        -'          :
..--""--.-"         (\      .-(\
  ..--""             `-\(\/;`
    _.                      :
                            `-
                            :\
                            ;  bug"#,

r#"     _                  _
    | '-.            .-' |
    | -. '..\\,.//,.' .- |
    |   \  \\\||///  /   |
   /|    )M\/%%%%/\/(  . |\
  (/\  MM\/%/\||/%\\/MM  /\)
  (//M   \%\\\%%//%//  M\\)
(// M________ /\ ________M \\)
 (// M\ \(',)|  |(',)/ /M \\) \\\\  
  (\\ M\.  /,\\//,\  ./M //)
    / MMmm( \\||// )mmMM \  \\
     // MMM\\\||///MMM \\ \\
      \//''\)/||\(/''\\/ \\
      mrf\\( \oo/ )\\\/\
           \'-..-'\/\\
             \\/ \\"#,

r#"                 _.--"""-,
               .'         `\
              /   _         \
 .-""-.       |  (O\.--.-.-/O)          .-""-.
/ O O  \      .\|(_._.__._.__)         /  O O \
|O .-.  \    (   )   0 _ 0   \        /  .-. O|
\ (   )  '.   `-|     (_)     |      .'  (   ) /
 '.'-'     '-./`|             |`\.-'     '-'.'
   \         |  \   \     /   /  |         /
    \        \   '.  '._.'  .'   /        /
     \        '.   `'-----'`   .'        /
      \   .'    '-._        .-'\   '.   /
       |/`          `'''''')    )    `\|
       /                  (    (      ,\
      ;                    \    '-..-'/ ;
      |                     '.       /  |
      |                       `'---'`   |
      ;                                 ;
       \                               /
        `.                           .'
          '-._                   _.-'
    jgs    __/`"  '  - - -  ' "`` \__
         /`            /^\            `\
         \(          .'   '.          )/
          '.__(__.-'        '.__)__).'"#
    ];

    // Gera um número aleatório entre 0 e o tamanho do array (5)
	let indice_aleatorio = rand::rng().random_range(0..artes.len());
    
    // Imprime a arte sorteada
    println!("{}", artes[indice_aleatorio]);
    println!("\n==================================");
    println!("     BEM-VINDO AO ANNY CLI");
    println!("==================================\n");
}

fn main() {
    // 1. Coleta o que o usuário digitou no terminal
    let args: Vec<String> = env::args().collect();

    // 2. Checa se o usuário pediu para atualizar ("anny update" ou "anny u")
    if args.len() > 1 && (args[1] == "update" || args[1] == "u") {
        if let Err(e) = tentar_atualizar() {
            eprintln!("Erro ao tentar atualizar: {}", e);
        }
        return; // Sai do programa após a atualização
    }

    // 3. Se não for update, mostra a arte aleatória e continua o programa normal
    mostrar_boas_vindas();
    println!("Iniciando processos...");
    println!("Olá, mundo! Rodando no Snapdragon!");
}
