use zbus::{Connection, Result, dbus_proxy};

#[dbus_proxy(
    interface = "br.com.Teste",
    default_service = "br.com.Teste",
    default_path = "/br/com/Teste"
)]
trait Teste {
    async fn ola(&self, name: &str) -> Result<String>;
}

#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    let proxy = TesteProxy::new(&connection).await?;
    loop{
        let reply = proxy.ola("Matheus").await?;
        println!("{reply}");
        
    }
    

    Ok(())
}