use bitwarden_core::{
    error::Result,
    vault::{Cipher, Collection, Folder},
    Client,
};

use crate::{
    export::{export_organization_vault, export_vault},
    ExportError, ExportFormat,
};

pub struct ClientExporters<'a> {
    client: &'a Client,
}

impl<'a> ClientExporters<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn export_vault(
        &self,
        folders: Vec<Folder>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String, ExportError> {
        export_vault(&self.client, folders, ciphers, format)
    }

    pub async fn export_organization_vault(
        &self,
        collections: Vec<Collection>,
        ciphers: Vec<Cipher>,
        format: ExportFormat,
    ) -> Result<String, ExportError> {
        export_organization_vault(collections, ciphers, format)
    }
}

pub trait ClientExportersExt<'a> {
    fn exporters(&'a self) -> ClientExporters<'a>;
}

impl<'a> ClientExportersExt<'a> for Client {
    fn exporters(&'a self) -> ClientExporters<'a> {
        ClientExporters::new(self)
    }
}
