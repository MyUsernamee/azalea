use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateEnabledFeaturesPacket {
    pub features: Vec<ResourceLocation>,
}
