#![forbid(unsafe_code)]
//! Tipos iniciais do domínio de identidades Pegasus.

use core::fmt;

/// Nome canônico local de uma identidade, sem credenciais ou atributos pessoais.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct AccountName(String);

/// Erros ao criar um nome de conta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountNameError {
    /// O nome está vazio ou contém apenas espaços ASCII.
    Empty,
    /// O nome excede o limite de bytes definido para a fase inicial.
    TooLong,
    /// O nome contém controle ASCII ou separadores de caminho.
    InvalidCharacter,
}

impl AccountName {
    /// Cria um nome após validações sintáticas conservadoras.
    ///
    /// Esta validação não substitui normalização Unicode nem as regras finais
    /// de identificadores LDAP, que serão definidas na fase de protocolo.
    pub fn parse(value: impl Into<String>) -> Result<Self, AccountNameError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(AccountNameError::Empty);
        }
        if value.len() > 128 {
            return Err(AccountNameError::TooLong);
        }
        if value
            .chars()
            .any(|ch| ch.is_ascii_control() || ch == '/' || ch == '\\')
        {
            return Err(AccountNameError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    /// Retorna o valor original já validado sintaticamente.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AccountName").field(&self.0).finish()
    }
}
