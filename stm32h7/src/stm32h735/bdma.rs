#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ch: [CH; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
}
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "Interrupt flag clear register"]
pub mod ifcr;
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers"]
pub mod ch;
