pub mod hosting;

pub mod serving {
    pub fn take_order() -> &'static str {
        "pedido tomado"
    }

    pub fn serve_order() -> &'static str {
        take_payment();
        "pedido servido"
    }

    fn take_payment() {}
}
