use webrtc::{api::APIBuilder, peer_connection::{configuration::RTCConfiguration}};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    
    let api = APIBuilder::new().build();

    let args = resolve_args();
    if args.is_empty() {
        // user n definiu args
        return Ok(());
    }

    let mode = &args[1];
    println!("Selected mode: {}", mode);

    
    
    // preciso configurar endereco dos servidores STUN e TURN (quando maquinas estao em redes diferentes)
    let config = RTCConfiguration::default();


    // coracao do proojeto, gerencia a conexao (criptografia, etc...)
    let _peer_connection = api.new_peer_connection(config).await?;
    Ok(())
}


fn resolve_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Por favor, especifique o modo de execução:");
        println!("  cargo run -- transmitir");
        println!("  cargo run -- receber");
        return Vec::new();
    }
    args
}
