pub mod fb_conn {
    use rsfbclient::{FbError, DynLink, Connection};
    use rsfbclient_native::NativeFbClient;

    pub type FbConn = Connection<NativeFbClient<DynLink>>;

    pub fn new_conn() -> Result<FbConn, FbError> {
        // DB Connection
        Ok(
            rsfbclient::builder_native()
                .with_dyn_link()
                .with_remote()
                .host("localhost")
                .db_name("C:\\Tarefas\\db\\TAREFAS.FDB")
                .connect()?
        )
    }
}