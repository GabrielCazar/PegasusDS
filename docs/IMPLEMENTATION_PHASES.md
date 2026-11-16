# Fases de implementação do Pegasus DS

Este roadmap divide a entrega em etapas verificáveis. O estado inicial é
**planejado**; nenhuma capacidade de protocolo, produção ou compatibilidade com
AD DS é declarada como entregue apenas por existir o scaffold.

## Fase 0 — Fundação e governança (em andamento)

- [x] Criar workspace Rust, toolchain fixada em stable e crates sem `unsafe`.
- [x] Definir contrato de contribuição e fonte do kernel CarlIO OS.
- [x] Criar roadmap e painel de progresso.
- [ ] Definir licença final, política de releases, proprietários e CI inicial.
- [ ] Adicionar configuração versionada de `cargo-deny` e SBOM.

**Saída:** builds locais reproduzíveis, governança aprovada e pipeline básico.

## Fase 1 — Domínio de identidade e armazenamento

- [ ] Definir modelo de domínio, identificadores estáveis, grupos e atributos.
- [ ] Especificar autorização, ciclo de vida e invariantes de auditoria.
- [ ] Selecionar armazenamento transacional e desenhar migrações/recuperação.
- [ ] Definir criptografia em repouso, gestão/rotação de chaves e retenção.
- [ ] Criar testes de propriedades e cenários de concorrência.

**Saída:** operações locais determinísticas e persistência recuperável, sem
exposição de credenciais em texto plano.

## Fase 2 — LDAP v3 e API de diretório

- [ ] Especificar subconjunto LDAP suportado, schema, controles e limites.
- [ ] Implementar parser limitado, TLS 1.3 e autorização por operação.
- [ ] Criar suíte de conformidade, interoperabilidade e fuzzing.

**Saída:** consultas e operações documentadas com evidências de conformidade.

## Fase 3 — Autenticação e descoberta

- [ ] Implementar Kerberos v5 e ciclo de vida seguro de tickets.
- [ ] Publicar descoberta DNS/SRV e integrar sincronização NTP monitorada.
- [ ] Definir rotação de chaves, mitigação de replay e recuperação de nós.

**Saída:** autenticação e descoberta interoperáveis, com falha fechada.

## Fase 4 — Administração, políticas e clientes

- [ ] Definir API administrativa autenticada, auditada e de escopo limitado.
- [ ] Criar agente CarlIO em userspace para receber políticas assinadas.
- [ ] Implementar atualização atômica, rollback e comportamento offline explícito.
- [ ] Avaliar SMB 3.x como serviço isolado, conforme requisitos e ameaças.

**Saída:** gerenciamento remoto controlado sem conceder privilégio implícito.

## Fase 5 — Replicação e alta disponibilidade

- [ ] Definir consistência, conflitos, quorum, recuperação e limites de atraso.
- [ ] Proteger replicação entre nós com mTLS e identidades distintas.
- [ ] Executar testes de caos e recuperação de desastre.

**Saída:** operação multi-nó auditável com comportamento de falha documentado.

## Fase 6 — Integração opcional com CarlIO Kernel Bridge

- [ ] Primeiro comprovar que APIs userspace do CarlIO OS não atendem ao caso.
- [ ] Propor ADR e contrato IPC versionado; validar a API na revisão do kernel.
- [ ] Obter aprovação humana explícita antes de qualquer alteração em
  `src/kernel_bridge/`.
- [ ] Limitar `unsafe` à allowlist aprovada e submeter a dois revisores.
- [ ] Testar em VMs CarlIO descartáveis e realizar auditoria de segurança.

**Saída:** ponte mínima, sem lógica de diretório ou acesso direto a hardware.

## Fase 7 — Endurecimento e lançamento

- [ ] Completar testes e2e, fuzzing contínuo, SBOM, assinatura e proveniência.
- [ ] Mapear evidências e lacunas para LGPD e ISO/IEC 27001.
- [ ] Fazer revisão externa de segurança e publicar matriz de compatibilidade.
- [ ] Definir SLOs, resposta a incidentes, backup e ciclo de suporte.

**Saída:** release com escopo de compatibilidade declarado e operação suportável.

## Critérios de avanço

Uma fase só pode ser marcada como concluída quando seus critérios de saída,
documentação e verificações de segurança estiverem registrados em PR. Bloqueios,
dependências externas e waivers devem apontar para ADR ou registro em
`docs/waivers/`. O painel `progress/html/` é uma visualização do roadmap; seus
estados locais no navegador não alteram este registro versionado.
