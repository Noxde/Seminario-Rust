mod streaming_rust;
mod fecha;


fn main() {
    let mut plataforma = streaming_rust::Plataforma::new();
    plataforma.crear_usuario(200, None, streaming_rust::MedioDePago::Cripto);

}
