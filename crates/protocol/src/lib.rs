#![forbid(unsafe_code)]
//! Tipos independentes de transporte para contratos do Pegasus DS.

/// Versão do envelope de mensagens internas.
pub const MESSAGE_VERSION: u16 = 1;

/// Identificador opaco de uma requisição, útil para correlação e auditoria.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RequestId(pub u128);

/// Cabeçalho comum dos contratos internos versionados.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MessageHeader {
    /// Versão do schema que codifica o corpo da mensagem.
    pub version: u16,
    /// ID de correlação único dentro do escopo definido pelo transporte.
    pub request_id: RequestId,
}

impl MessageHeader {
    /// Cria cabeçalho na versão atual do contrato.
    #[must_use]
    pub const fn current(request_id: RequestId) -> Self {
        Self {
            version: MESSAGE_VERSION,
            request_id,
        }
    }
}
