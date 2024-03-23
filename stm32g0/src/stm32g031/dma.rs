#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ch: [CH; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - DMA interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x08..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x08..0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub const fn ch1(&self) -> &CH {
        self.ch(0)
    }
    #[doc = "0x1c..0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub const fn ch2(&self) -> &CH {
        self.ch(1)
    }
    #[doc = "0x30..0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub const fn ch3(&self) -> &CH {
        self.ch(2)
    }
    #[doc = "0x44..0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub const fn ch4(&self) -> &CH {
        self.ch(3)
    }
    #[doc = "0x58..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub const fn ch5(&self) -> &CH {
        self.ch(4)
    }
}
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
#[doc = "ISR (r) register accessor: DMA interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: DMA interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
