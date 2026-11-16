# Pegasus DS

Servidor corporativo de diretório em desenvolvimento, com Core em userspace e
integração controlada com CarlIO OS. O kernel hospedeiro oficial é
[`GabrielCazar/carlio-os`](https://github.com/GabrielCazar/carlio-os).

Este repositório contém o scaffold inicial do workspace Rust e um roadmap. LDAP,
Kerberos, armazenamento persistente, replicação e integração de kernel ainda
não estão implementados; não há compatibilidade de produção declarada.

## Estrutura inicial

- `crates/protocol`: tipos e contratos internos versionados.
- `crates/domain`: tipos básicos e regras puras de identidade.
- `services/pegasusd`: ponto de entrada do serviço em ring 3.
- `docs/IMPLEMENTATION_PHASES.md`: fases, entregáveis e critérios de avanço.
- `progress/html/index.html`: painel interativo de progresso local.
- `AGENTS.md`: contrato técnico e regras de contribuição.

## Requisitos e execução

Instale a toolchain Rust indicada em `rust-toolchain.toml` e execute na raiz:

```sh
cargo run -p pegasusd
```

Para abrir o painel, sirva a raiz do repositório por HTTP (por exemplo,
`python3 -m http.server 8000`) e acesse `/progress/html/`. O painel não precisa de
dependências externas; as marcações são guardadas apenas no `localStorage` do
navegador e não alteram o roadmap oficial.

## Licença

O Pegasus DS é distribuído sob a GNU General Public License, versão 3, apenas
(GPL-3.0-only). Consulte o arquivo [`LICENSE`](LICENSE) para os termos completos.

Leia `AGENTS.md` integralmente antes de contribuir. Mudanças em
`src/kernel_bridge/` exigem aprovação humana explícita.
