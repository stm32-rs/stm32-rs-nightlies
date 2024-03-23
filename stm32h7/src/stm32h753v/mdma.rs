#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gisr0: GISR0,
    _reserved1: [u8; 0x3c],
    ch: [CH; 16],
}
impl RegisterBlock {
    #[doc = "0x00 - MDMA Global Interrupt/Status Register"]
    #[inline(always)]
    pub const fn gisr0(&self) -> &GISR0 {
        &self.gisr0
    }
    #[doc = "0x40..0x440 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x440 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
}
#[doc = "GISR0 (r) register accessor: MDMA Global Interrupt/Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gisr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gisr0`]
module"]
pub type GISR0 = crate::Reg<gisr0::GISR0rs>;
#[doc = "MDMA Global Interrupt/Status Register"]
pub mod gisr0;
#[doc = "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
pub mod ch;
