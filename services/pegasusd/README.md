# pegasusd

Ponto de entrada inicial do serviço de diretório em userspace (ring 3). Ainda
não abre sockets, não persiste dados e não oferece autenticação; essas funções
dependem das fases de protocolo e segurança do roadmap.

Execute com `cargo run -p pegasusd` após instalar a toolchain indicada na raiz.
