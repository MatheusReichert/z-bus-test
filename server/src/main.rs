use std::{error::Error, future::pending};
use zbus::{ConnectionBuilder, dbus_interface};

struct Teste {
    count: u64
}

#[dbus_interface(name = "br.com.Teste")]
impl Teste {
    async fn ola(&mut self, nome: &str) -> String {
        self.count += 1;
        format!("Olá {}!!!, <{}x>", nome, self.count)
    }

    
}

#[async_std::main] //Runtime asyncrono
async fn main() -> Result<(), Box<dyn Error>> {
    let teste = Teste {count : 0};
    let _conn = ConnectionBuilder::session()?
        .name("br.com.Teste")?
        .serve_at("/br/com/Teste", teste)?
        .build()
        .await?;    

    //Segura o processo aberto 
    pending::<()>().await;
    Ok(())
}