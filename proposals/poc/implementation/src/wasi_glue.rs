use std::convert::TryInto;

use crate::types as guest_types;
use crate::WasiCryptoCtx;

impl crate::wasi_ephemeral_crypto_common::WasiEphemeralCryptoCommon for WasiCryptoCtx {
    // --- options

    fn options_open(
        &self,
        options_type: guest_types::AlgorithmType,
    ) -> Result<guest_types::Options, guest_types::CryptoErrno> {
        Ok(self.ctx.options_open(options_type.into())?.into())
    }

    fn options_close(
        &self,
        options_handle: guest_types::Options,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.options_close(options_handle.into())?.into())
    }

    fn options_set(
        &self,
        options_handle: guest_types::Options,
        name_str: &wiggle::GuestPtr<'_, str>,
        value_ptr: &wiggle::GuestPtr<'_, u8>,
        value_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let name_str: &str = &*name_str.as_str()?;
        let value: &[u8] = { &*value_ptr.as_array(value_len).as_slice()? };
        Ok(self
            .ctx
            .options_set(options_handle.into(), name_str, value)?
            .into())
    }

    fn options_set_guest_buffer(
        &self,
        options_handle: guest_types::Options,
        name_str: &wiggle::GuestPtr<'_, str>,
        buffer_ptr: &wiggle::GuestPtr<'_, u8>,
        buffer_len: guest_types::Size,
    ) -> Result<(), guest_types::CryptoErrno> {
        let name_str: &str = &*name_str.as_str()?;
        let buffer: &'static mut [u8] =
            unsafe { std::mem::transmute(&mut *buffer_ptr.as_array(buffer_len).as_slice()?) };
        Ok(self
            .ctx
            .options_set_guest_buffer(options_handle.into(), name_str, buffer)?
            .into())
    }

    fn options_set_u64(
        &self,
        options_handle: guest_types::Options,
        name_str: &wiggle::GuestPtr<'_, str>,
        value: u64,
    ) -> Result<(), guest_types::CryptoErrno> {
        let name_str: &str = &*name_str.as_str()?;
        Ok(self
            .ctx
            .options_set_u64(options_handle.into(), name_str, value)?
            .into())
    }

    // --- array

    fn array_output_len(
        &self,
        array_output_handle: guest_types::ArrayOutput,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        Ok(self
            .ctx
            .array_output_len(array_output_handle.into())?
            .try_into()?)
    }

    fn array_output_pull(
        &self,
        array_output_handle: guest_types::ArrayOutput,
        buf_ptr: &wiggle::GuestPtr<'_, u8>,
        buf_len: guest_types::Size,
    ) -> Result<guest_types::Size, guest_types::CryptoErrno> {
        let buf: &mut [u8] = { &mut *buf_ptr.as_array(buf_len).as_slice()? };
        Ok(self
            .ctx
            .array_output_pull(array_output_handle.into(), buf)?
            .try_into()?)
    }

    // --- secrets_manager

    fn secrets_manager_open(
        &self,
        options_handle: &guest_types::OptOptions,
    ) -> Result<guest_types::SecretsManager, guest_types::CryptoErrno> {
        let options_handle = match *options_handle {
            guest_types::OptOptions::Some(options_handle) => Some(options_handle),
            guest_types::OptOptions::None => None,
        };
        Ok(self
            .ctx
            .secrets_manager_open(options_handle.map(Into::into))?
            .into())
    }

    fn secrets_manager_close(
        &self,
        secrets_manager_handle: guest_types::SecretsManager,
    ) -> Result<(), guest_types::CryptoErrno> {
        Ok(self.ctx.secrets_manager_close(secrets_manager_handle.into())?)
    }

    fn secrets_manager_invalidate(
        &self,
        secrets_manager_handle: guest_types::SecretsManager,
        key_id_ptr: &wiggle::GuestPtr<'_, u8>,
        key_id_len: guest_types::Size,
        key_version: guest_types::Version,
    ) -> Result<(), guest_types::CryptoErrno> {
        let key_id: &[u8] = { &*key_id_ptr.as_array(key_id_len).as_slice()? };
        Ok(self
            .ctx
            .secrets_manager_invalidate(secrets_manager_handle.into(), key_id, key_version.into())?
            .into())
    }
}
