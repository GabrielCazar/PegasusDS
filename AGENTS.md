# Contrato técnico do Pegasus DS

Este documento define requisitos normativos para projeto, implementação, revisão e operação do Pegasus DS. Aplica-se a contribuições humanas e automatizadas. Os termos **DEVE**, **NÃO DEVE**, **OBRIGATÓRIO**, **PROIBIDO** e **PODE** expressam requisitos; exceções só são válidas pelo processo da seção 12.

## 1. Visão geral do projeto

### 1.1 Missão e escopo

O Pegasus DS tem como missão fornecer serviços corporativos de diretório, identidade, autenticação, descoberta e políticas para domínios de máquinas CarlIO OS. O objetivo de compatibilidade é interoperar com ecossistemas empresariais que usam LDAP v3, Kerberos v5, DNS, SMB e NTP, com comportamento documentado e testado por protocolo.

O escopo inclui o serviço de diretório e sua API, armazenamento de identidades e políticas, autenticação e autorização, replicação entre nós, auditoria, administração e integração controlada com o CarlIO OS. A integração com o sistema operacional pode aplicar políticas e administrar dispositivos por interfaces explícitas e versionadas.

O Pegasus Core é um serviço de **userspace**. A presença de uma ponte no kernel não transfere ao Pegasus autoridade irrestrita de ring 0: código de domínio, protocolos, administração e políticas permanecem em ring 3. Código de kernel limita-se a primitivas mínimas do CarlIO OS que não possam ser fornecidas com segurança em userspace. Nenhuma funcionalidade pode justificar execução de lógica de diretório, LDAP, Kerberos ou SMB em ring 0.

### 1.2 Não escopo

O Pegasus DS NÃO DEVE substituir o gerenciador de dispositivos do CarlIO OS, acessar hardware diretamente, contornar autenticação/autorização do sistema, nem conceder controle remoto arbitrário por pertencer a um domínio. Não é objetivo declarar compatibilidade total com AD DS sem testes de interoperabilidade por recurso e versão. Não é permitido enfraquecer isolamento, políticas do kernel ou segurança do cliente para simular compatibilidade.

### 1.3 Diagrama conceitual

```text
                    plano de controle autenticado e auditado
+-------------------------+       +-------------------------------+
| Pegasus DS               | LDAP  | Clientes gerenciados           |
| Pegasus Core (ring 3)    |<----->| CarlIO OS (ring 3)             |
| diretório, políticas,    | Kerb  | agente de domínio, serviços    |
| auditoria e replicação   | DNS   | locais e aplicações            |
+------------+------------+       +---------------+---------------+
             | IPC autenticado e versionado         | API do SO
=============|======================================|================
             v                                      v
+-------------------------+       +-------------------------------+
| Pegasus Kernel Bridge    |<----->| CarlIO OS Device Manager       |
| superfície mínima ring 0 |       | drivers e dispositivos          |
+-------------------------+       +-------------------------------+
             | interfaces estáveis do kernel CarlIO OS
             v
+------------------------------------------------------------------+
| CarlIO OS: isolamento, identidade local, autorização e hardware  |
+------------------------------------------------------------------+
```

As setas representam canais sujeitos a autenticação, autorização, validação de entradas, limites de recursos e auditoria. A comunicação entre o servidor e os clientes ocorre por protocolos de rede; o IPC local liga componentes no mesmo host.

### 1.4 Fonte do kernel

O repositório oficial do CarlIO OS, que contém o kernel hospedeiro e suas interfaces, é [`GabrielCazar/carlio-os`](https://github.com/GabrielCazar/carlio-os). Este é o projeto de origem para consultar código, contratos e documentação do kernel. O Pegasus DS NÃO DEVE manter uma cópia divergente do kernel nem presumir APIs que não estejam presentes e documentadas nessa fonte. Cada integração DEVE registrar a revisão ou versão do CarlIO OS contra a qual foi desenvolvida e validada; atualizações DEVEM passar por revisão de compatibilidade e segurança.

## 2. Princípios invioláveis

### 2.1 Segurança e autoridade

- Cada requisição DEVE ser autenticada e autorizada no contexto do recurso e da ação solicitados (zero trust).
- Componentes e identidades DEVEM operar com o menor privilégio necessário; credenciais de serviço DEVEM ser distintas e rotacionáveis.
- Autorização deve falhar fechada quando política, identidade, assinatura ou canal não puder ser validado.
- Associação ao domínio NÃO DEVE equivaler a privilégio administrativo local ou remoto.
- Dados recebidos de rede, disco, kernel, IPC ou configuração DEVEM ser tratados como não confiáveis e validados antes do uso.

### 2.2 Compatibilidade verificável

LDAP v3, Kerberos v5, DNS (incluindo registros SRV de descoberta), SMB 3.x e NTP são protocolos-alvo. Cada recurso declarado compatível DEVE apontar para versões, extensões, limites conhecidos e testes de conformidade/interoperabilidade. Não se deve anunciar suporte total baseado apenas em parsing ou em testes unitários.

### 2.3 Rust seguro e auditabilidade

Crates DEVEM usar `#![forbid(unsafe_code)]` sempre que tecnicamente possível. `unsafe` só PODE existir na fronteira de integração explicitamente aprovada e listada em `unsafe-allowlist.toml`; cada bloco exige comentário SAFETY que descreva a invariante, os limites e como ela é mantida. Mudanças na allowlist seguem revisão de ring 0.

Operações de identidade, alteração de política, emissão/revogação de credenciais, replicação e administração DEVEM ser determinísticas quanto aos dados de entrada e registrar resultado auditável, sujeito às regras de privacidade e retenção. Segredos e material de autenticação NÃO DEVEM aparecer em logs.

## 3. Arquitetura do sistema

### 3.1 Camadas

| Camada | Responsabilidade | Limite de confiança |
|---|---|---|
| Pegasus Core (userspace/ring 3) | Diretório, protocolos, políticas, replicação, administração e auditoria | Não acessa hardware nem memória de kernel diretamente |
| Pegasus Kernel Bridge (ring 0 ↔ ring 3) | Adaptador mínimo para operações do kernel que não possam ser expostas por APIs gerais seguras | Valida identidade, autorização, tamanho, versão e estado; não implementa lógica de diretório |
| CarlIO OS Device Manager | Descoberta, ciclo de vida e operação de dispositivos por drivers do sistema | Única rota para interação com dispositivos |
| Clientes gerenciados | Agente local, aplicação de políticas permitidas e comunicação com o domínio | Não confia implicitamente em comandos do servidor; valida assinatura, escopo e frescor |

### 3.2 Acoplamento e falhas

O Pegasus NÃO DEVE manipular hardware diretamente. Toda interação com dispositivo ocorre pelas interfaces suportadas do CarlIO OS Device Manager, mediadas pela API de sistema ou pelo Kernel Bridge quando estritamente necessário. O Core NÃO DEVE depender de detalhes internos ou layout privado do kernel; contratos públicos têm versão e política de compatibilidade.

Falha, indisponibilidade ou reinício do Pegasus não pode corromper o kernel nem deixar autorização elevada persistente. A indisponibilidade do domínio DEVE seguir política explícita de cache e expiração; não se pode transformar falha de rede em autorização irrestrita. Operações administrativas remotas DEVEM ser limitadas por escopo, prazo, nonce/controle contra replay e trilha de auditoria.

### 3.3 Contratos de IPC

Mensagens entre ring 3 e ring 0 DEVEM usar schemas explícitos, tamanho limitado, inteiros com largura definida, versão de protocolo, identificador de requisição e códigos de erro estáveis. Parsers DEVEM rejeitar campos desconhecidos quando possam alterar segurança, além de rejeitar truncamento, overflow, enumerações inválidas, mensagens excessivas e versões incompatíveis.

O transporte DEVE autenticar o processo chamador usando mecanismos do CarlIO OS e verificar autorização por operação. Buffers compartilhados, se suportados pelo kernel, DEVEM ter ownership, tamanho, ciclo de vida e permissões definidos; não podem conter ponteiros dereferenciados sem validação. Mudanças incompatíveis exigem nova versão, plano de migração e testes de compatibilidade. Schemas e decisões ficam versionados no repositório.

## 4. Integração com CarlIO OS (ring 0)

Todo acesso específico ao kernel pelo Pegasus DEVE passar por API dedicada do driver `pegasus_kmod`, cuja assinatura, versão, operações permitidas e esquema de mensagens sejam auditáveis. O driver não é um canal genérico de execução: comandos são enumerados, tipados, limitados e autorizados. Preferir syscalls e serviços genéricos e estáveis do CarlIO OS quando atenderem ao caso.

As APIs e os contratos utilizados DEVEM ser conferidos no repositório-fonte [`GabrielCazar/carlio-os`](https://github.com/GabrielCazar/carlio-os). O Pegasus DS NÃO DEVE inventar ou documentar como existente uma API do kernel sem verificar sua implementação e documentação na revisão fixada. Diferenças necessárias devem ser propostas no projeto CarlIO OS por fluxo próprio e só podem ser consumidas após integração versionada.

`unsafe` de integração só é permitido em módulos incluídos em `unsafe-allowlist.toml`. A allowlist DEVE registrar caminho/crate, finalidade, responsável funcional, invariantes e revisão. Nenhum agente pode ampliar a lista sem as aprovações previstas na seção 11.

Toda syscall sensível originada pelo serviço DEVE gerar evento estruturado com ID do domínio, identidade solicitante, identidade efetiva, operação, recurso, resultado, instante confiável e justificativa vinculada à requisição. Campos ausentes devem ser explícitos; segredos, tokens, chaves e dados pessoais desnecessários são proibidos. A trilha deve ser protegida contra alteração não autorizada e ter política de retenção.

São proibidos:

- mapear (`mmap`) regiões de kernel sem validação de faixa, permissões, alinhamento, ownership e ciclo de vida definidos pelo CarlIO OS;
- chamadas diretas a hardware fora do CarlIO OS Device Manager;
- compartilhar memória entre processos sem autenticação, autorização e isolamento de buffers;
- aceitar ponteiros de userspace como confiáveis ou permitir que a requisição escolha endereço de kernel;
- expor operações genéricas de leitura/escrita de memória, execução arbitrária ou escalada de privilégio pela ponte.

## 5. Padrões de código Rust

### 5.1 Toolchain e workspace

O repositório DEVE fixar a toolchain em `rust-toolchain.toml`. Rust stable é o padrão; nightly só PODE ser usado em crates de kernel que dependam de recursos instáveis, com justificativa, versão fixada e isolamento da dependência. O workspace DEVE separar protocolo, domínio, armazenamento, serviços, cliente e ponte de kernel em crates com fronteiras explícitas.

Features DEVEM ser aditivas e documentadas. Uma feature não pode desativar autenticação, validação, auditoria ou isolamento em builds de produção. Builds de produção DEVEM declarar perfil, target e features efetivas.

### 5.2 Qualidade e estilo

Contribuições DEVEM passar por `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings` (ou conjunto equivalente documentado para o target) e verificações de tipos. Lints só podem ser suprimidos no menor escopo necessário, com justificativa junto à supressão; não se pode desativar uma checagem para obter aprovação.

Nomes e APIs devem seguir convenções Rust. APIs públicas DEVEM documentar invariantes, erros, segurança, compatibilidade e exemplos quando aplicável. Erros devem preservar contexto útil sem incluir segredos. Panics não devem ser usados para tratar entradas externas, falhas de rede ou erros operacionais recuperáveis.

### 5.3 Dependências

Adicionar dependência requer justificativa técnica, análise de manutenção, superfície de ataque, licença e alternativas. Preferem-se licenças Apache-2.0 ou MIT; qualquer outra licença exige análise e aprovação registrada. Dependências abandonadas, sem origem verificável ou incompatíveis com a política de licenças são proibidas. O CI DEVE executar `cargo-deny` com configuração versionada.

## 6. Segurança e conformidade

### 6.1 Modelo de ameaças STRIDE

| Categoria | Exemplos no Pegasus DS | Controles mínimos |
|---|---|---|
| Spoofing (falsificação) | Cliente, nó ou administrador falso | mTLS, Kerberos, validação de cadeia e identidade por requisição |
| Tampering (alteração) | Política, replicação ou log adulterado | Assinaturas, integridade, autorização, trilha protegida |
| Repudiation (negação) | Operação administrativa sem autoria comprovável | Auditoria estruturada, identidade efetiva, relógio e proteção de logs |
| Information disclosure (exposição) | Dump de credenciais, LDAP ou IPC | Minimização, TLS, controle de acesso, criptografia em repouso |
| Denial of service (negação de serviço) | Parser caro, floods ou esgotamento de recursos | Limites, timeouts, quotas, backpressure, fuzzing e isolamento |
| Elevation of privilege (elevação) | Confusão de identidade ou abuso do Kernel Bridge | Menor privilégio, autorização por operação, validação e revisão de ring 0 |

O threat model DEVE ser revisado para mudanças de protocolo, trust boundary, armazenamento de credenciais, replicação ou kernel bridge.

### 6.2 Criptografia e segredos

Comunicação externa DEVE usar TLS 1.3. Comunicação entre nós do domínio DEVE usar mTLS, identidades de serviço únicas e rotação de certificados. Algoritmos e parâmetros devem seguir recomendações criptográficas atuais e ser substituíveis sem redesenhar o protocolo.

Dados sensíveis em repouso DEVEM ser criptografados com chaves geridas por HSM ou mecanismo de gestão de chaves equivalente, com separação de funções, rotação e recuperação documentadas. Credenciais NÃO DEVEM ser armazenadas em texto plano em nenhuma camada. Senhas devem usar derivação de chave apropriada para autenticação, com salt individual e parâmetros versionados; chaves e tickets devem ter proteção e expiração adequadas. Segredos não podem ser incluídos em código, logs, argumentos de processo ou fixtures reais.

### 6.3 LGPD e ISO 27001

O projeto tem como metas apoiar conformidade com a LGPD e controles aplicáveis da ISO/IEC 27001. Isso não constitui certificação nem parecer jurídico. Cada fluxo de dados pessoais DEVE documentar finalidade, minimização, controle de acesso, retenção, descarte e mecanismo para atender solicitações aplicáveis. Um mapa de controles em `docs/compliance/` DEVE relacionar controles relevantes a políticas, implementação e evidências, com lacunas identificadas; não se deve declarar conformidade sem avaliação competente e evidências.

## 7. Protocolos e padrões corporativos

| Protocolo/padrão | Referência de base | Finalidade e requisito |
|---|---|---|
| LDAP v3 | RFC 4511 | Protocolo primário de consulta e operações de diretório; controles de acesso e transporte seguro são obrigatórios |
| Kerberos v5 | RFC 4120 | Autenticação de rede com tickets, proteção contra replay e política de ciclo de vida |
| DNS | RFC 1035 e RFCs aplicáveis | Resolução e localização de serviços; registros SRV para descoberta de serviços do domínio |
| SMB 3.x | Especificações SMB aplicáveis | Compartilhamento de recursos; negociação segura, autenticação e autorização por recurso |
| NTP | RFC 5905 e perfil adotado | Sincronização de tempo necessária à validação de tickets e auditoria; origem e deriva monitoradas |

Cada implementação DEVE ter suíte de conformidade por protocolo, testes negativos, limites e testes de interoperabilidade com implementações independentes quando disponíveis. Requisitos exatos de extensões, cipher suites, opções e versões DEVEM ser registrados em `docs/protocols/`; nomes de protocolo não bastam para afirmar compatibilidade. Parsers DEVEM ser robustos a mensagens malformadas, grandes, truncadas e adversariais.

## 8. Estratégia de testes

### 8.1 Pirâmide de testes

- Testes unitários DEVEM cobrir regras de domínio, autorização, validação, serialização e tratamento de falhas (`cargo test`).
- Testes de integração DEVEM exercitar serviços e dependências isoladas; testcontainers PODE ser usado quando suportado pelo ambiente e sem dados reais.
- Testes end-to-end DEVEM rodar em VMs CarlIO descartáveis para fluxos de domínio e integração com kernel.
- Testes de protocolo DEVEM incluir casos válidos, inválidos, compatibilidade de versão, limites e interoperabilidade.
- Testes de caos DEVEM cobrir perda, atraso e reordenação de rede, indisponibilidade de nós, reinício do serviço/kernel e falha de persistência sem permitir estado de autorização inconsistente.

### 8.2 Fuzzing, cobertura e segurança

Parsers de protocolos, mensagens IPC e formatos persistidos DEVEM ter targets de fuzzing `cargo-fuzz`, corpus versionado sem segredos e execução contínua limitada no CI, além de campanhas mais longas periódicas. Bugs encontrados viram regressões reproduzíveis.

Camadas de domínio DEVEM manter no mínimo 80% de cobertura de linhas medida por ferramenta e configuração documentadas. Código `unsafe` DEVE ter 100% de cobertura de linhas e branches quando mensurável, mais revisão manual de cada caminho e testes de propriedades/invariantes. Se uma métrica não puder ser medida tecnicamente, a exceção deve ser registrada com justificativa e evidência alternativa; cobertura não substitui revisão de segurança.

Mudanças de kernel, autenticação, criptografia, autorização, armazenamento de credenciais ou protocolo DEVEM incluir testes de falha e regressão específicos. Não se pode usar apenas um teste feliz como evidência de segurança.

## 9. CI/CD e qualidade de entrega

O pipeline mínimo DEVE executar, na ordem lógica apropriada: formatação, Clippy com warnings como erros, testes, auditoria de dependências com cargo-deny, verificações de segurança aplicáveis e build reproduzível. Builds de release DEVEM registrar toolchain, target, features e origem do código.

Artefatos entregues DEVEM ser versionados e acompanhados por SBOM gerado automaticamente. Assinatura, proveniência e retenção dos artefatos DEVEM permitir verificar origem e integridade. Nenhuma chave de assinatura pode residir no repositório ou em logs do CI.

Pull requests DEVEM receber revisão de pelo menos um engenheiro ou agente responsável diferente do autor. Mudanças que afetem ring 0, Kernel Bridge, `unsafe` ou fronteiras de privilégio exigem dois revisores qualificados, além dos gates automatizados. Falha de gate não pode ser contornada por desativação ou edição oportunista da configuração.

## 10. Documentação e decisões

Toda crate DEVE possuir `README.md` descrevendo propósito, limites, uso, features, segurança e testes. APIs públicas DEVEM ter documentação `///`. Decisões arquiteturais com impacto em compatibilidade, segurança, armazenamento, protocolo, integração com kernel ou operação DEVEM ser registradas como ADRs em `docs/adr/`, incluindo contexto, decisão, alternativas e consequências.

Diagramas de arquitetura e fluxo DEVEM ser versionados em `docs/diagrams/` em Mermaid. Contratos de protocolo, operação, resposta a incidentes e recuperação devem acompanhar a funcionalidade correspondente. Mudanças relevantes DEVEM atualizar `CHANGELOG.md`.

## 11. Regras para agentes de IA e revisão

Todo agente DEVE ler este arquivo integralmente antes de contribuir, inspecionar o estado do repositório e identificar instruções locais adicionais. O agente DEVE propor alterações em PR com intenção, arquivos afetados, riscos, validações executadas e referências às seções deste contrato que fundamentam decisões relevantes.

Agentes NÃO DEVEM modificar arquivos em `src/kernel_bridge/` sem aprovação explícita de um mantenedor humano, mesmo em modo autônomo. Essa aprovação é necessária antes de editar esses arquivos; testes ou leitura não concedem autorização para mudanças. A revisão de dois revisores da seção 9 continua obrigatória.

Agentes NÃO DEVEM adicionar dependências sem justificativa no PR e verificação de licença/manutenção. NÃO DEVEM desabilitar lints, testes, fuzzing ou checagens de segurança para “fazer passar”. DEVEM seguir Conventional Commits e registrar mudanças em `CHANGELOG.md`. DEVEM declarar testes não executados e limitações observadas, sem afirmar suporte ou segurança não demonstrados.

## 12. Governança, estrutura e exceções

### 12.1 Estrutura canônica

```text
.
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── unsafe-allowlist.toml
├── CHANGELOG.md
├── crates/
│   ├── protocol/       # schemas e tipos de protocolo compartilhados
│   ├── domain/         # regras puras de identidade e política
│   ├── storage/        # persistência e migrações
├── services/           # processos e endpoints em userspace
├── clients/            # agente e ferramentas administrativas
├── src/kernel_bridge/  # integração mínima com CarlIO OS, sujeita à seção 11
├── docs/
│   ├── adr/
│   ├── compliance/
│   ├── diagrams/
│   ├── protocols/
│   └── waivers/
└── tests/              # integração, conformidade e end-to-end
```

Pastas podem ser introduzidas conforme necessidade, desde que responsabilidades e fronteiras sejam documentadas. A estrutura não implica que todos os componentes já estejam implementados.

### 12.2 Exceções (waivers)

Exceção a requisito deste documento exige solicitação revisável em `docs/waivers/`, com seção afetada, escopo exato, motivo, risco, controles compensatórios, responsável, aprovadores, data de expiração e plano de remoção. Exceções de ring 0, `unsafe`, autenticação, criptografia ou auditoria exigem aprovação de mantenedor de segurança e responsável pelo CarlIO OS. Nenhum waiver pode autorizar armazenamento de credenciais em texto plano, acesso direto a hardware, execução arbitrária via Kernel Bridge ou supressão de aprovação humana requerida pela seção 11.

Waivers são temporários, específicos e revisados antes do vencimento. Um waiver não altera este contrato para outros arquivos ou contribuições.

### 12.3 Revisão periódica

Este documento DEVE ser revisto a cada release minor e quando mudança de ameaça, protocolo, integração de kernel ou obrigação regulatória o exigir. A revisão deve registrar data, responsáveis, alterações e waivers vencidos. Atualizações ao contrato exigem revisão de mantenedor e changelog.

## 13. Como um agente deve começar

Antes de qualquer alteração, o agente DEVE:

1. Ler integralmente este `AGENTS.md` e localizar instruções adicionais aplicáveis no repositório.
2. Verificar branch, estado do working tree e alterações existentes para não sobrescrever trabalho alheio.
3. Identificar a camada afetada e os limites de confiança, em especial se houver interação com ring 0.
4. Consultar ADRs, contratos de protocolo, waivers e testes relacionados antes de escolher uma implementação.
5. Definir a menor mudança compatível com segurança, manutenção e versões suportadas.
6. Implementar testes e documentação requeridos, executando os gates aplicáveis sem desativá-los.
7. Preparar PR/relato com intenção, arquivos, riscos, evidências, limitações e seções deste contrato relevantes.
