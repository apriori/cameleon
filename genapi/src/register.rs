use super::{
    interface::IRegister,
    node_base::{NodeAttributeBase, NodeBase},
    register_base::RegisterBase,
    store::{CacheStore, NodeStore, ValueStore},
    Device, GenApiResult, ValueCtxt,
};

#[derive(Debug, Clone)]
pub struct RegisterNode {
    pub(crate) attr_base: NodeAttributeBase,
    pub(crate) register_base: RegisterBase,
}

impl RegisterNode {
    #[must_use]
    pub fn node_base(&self) -> NodeBase {
        let elem_base = &self.register_base.elem_base;
        NodeBase::new(&self.attr_base, elem_base)
    }

    #[must_use]
    pub fn register_base(&self) -> &RegisterBase {
        &self.register_base
    }
}

impl IRegister for RegisterNode {
    fn read<T: ValueStore, U: CacheStore>(
        &self,
        buf: &mut [u8],
        device: impl Device,
        store: impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        self.register_base().read(buf, device, store, cx)
    }

    fn write<T: ValueStore, U: CacheStore>(
        &self,
        buf: &[u8],
        device: impl Device,
        store: impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        self.register_base().write(buf, device, store, cx)
    }

    fn address<T: ValueStore, U: CacheStore>(
        &self,
        device: impl Device,
        store: impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        self.register_base().address(device, store, cx)
    }

    fn length<T: ValueStore, U: CacheStore>(
        &self,
        device: impl Device,
        store: impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        self.register_base().length(device, store, cx)
    }
}
