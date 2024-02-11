struct Pessoa {
    nome: String,
    idade: u32,
}

impl Pessoa {
    fn new(nome: String, idade: u32) -> Self {
        Pessoa { nome, idade }
    }

    fn exibir_info(&self) {
        println!("Nome: {}, Idade: {}", self.nome, self.idade);
    }

    fn envelhecer(&mut self, anos: u32) {
        self.idade += anos;
    }
}

fn main() {
    let mut pessoa = Pessoa::new(String::from("Luann"), 19);
    pessoa.exibir_info();
    pessoa.envelhecer(1);
    pessoa.exibir_info();
}