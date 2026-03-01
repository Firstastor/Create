use bevy::{
    asset::{io::Reader, *},
    prelude::*,
};
use serde::Deserialize;

#[derive(TypePath)]
pub struct TomlLoader<A> {
    _marker: std::marker::PhantomData<A>,
}

impl<A> AssetLoader for TomlLoader<A>
where
    for<'de> A: Asset + Deserialize<'de>,
{
    type Asset = A;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        toml::from_slice::<A>(&bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Fail to parse toml"))
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
