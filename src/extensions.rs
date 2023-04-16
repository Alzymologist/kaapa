use crate::field_content::FieldContent;
use frame_metadata::{RuntimeMetadataV14, SignedExtensionMetadata};
use scale_info::form::PortableForm;
use yew::prelude::*;
use yew::Callback;
use crate::CallConstructorEvent;

pub struct ExtensionContent {
    identifier: String,
    content: FieldContent,
    additional: FieldContent,
}

impl ExtensionContent {
    pub fn new(input: &SignedExtensionMetadata<PortableForm>, metadata: &RuntimeMetadataV14) -> Self {
        ExtensionContent {
            identifier: input.identifier.clone(),
            content: FieldContent::new(metadata.types.resolve(input.ty.id()), Some(&input.identifier), false, metadata),
            additional: FieldContent::new(metadata.types.resolve(input.additional_signed.id()), Some(&input.identifier), false, metadata),
        }
    }

    pub fn get_content_cards(&self, parent: Vec<usize>, callback_original: Callback<CallConstructorEvent>, metadata: &RuntimeMetadataV14) -> Vec<Html> {
        self.content
            .get_cards(parent, callback_original, metadata)
    }
    
    pub fn get_additional_cards(&self, parent: Vec<usize>, callback_original: Callback<CallConstructorEvent>, metadata: &RuntimeMetadataV14) -> Vec<Html> {
        self.additional
            .get_cards(parent, callback_original, metadata)
    }

    pub fn content(&mut self) -> &mut FieldContent {
        &mut self.content
    }

    pub fn additional(&mut self) -> &mut FieldContent {
        &mut self.content
    }

    pub fn encoded_content(&self, metadata: &RuntimeMetadataV14) -> Vec<u8> {
        self.content.encoded(metadata)
    }

    pub fn encoded_additional(&self, metadata: &RuntimeMetadataV14) -> Vec<u8> {
        self.additional.encoded(metadata)
    }
}
