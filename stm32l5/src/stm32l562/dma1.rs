#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ch: [CH; 8],
    cselr: CSELR,
}
impl RegisterBlock {
    #[doc = "0x00 - interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x08..0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch1(&self) -> &CH {
        self.ch(0)
    }
    #[doc = "0x1c..0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch2(&self) -> &CH {
        self.ch(1)
    }
    #[doc = "0x30..0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch3(&self) -> &CH {
        self.ch(2)
    }
    #[doc = "0x44..0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch4(&self) -> &CH {
        self.ch(3)
    }
    #[doc = "0x58..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch5(&self) -> &CH {
        self.ch(4)
    }
    #[doc = "0x6c..0x80 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch6(&self) -> &CH {
        self.ch(5)
    }
    #[doc = "0x80..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch7(&self) -> &CH {
        self.ch(6)
    }
    #[doc = "0x94..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
    #[inline(always)]
    pub const fn ch8(&self) -> &CH {
        self.ch(7)
    }
    #[doc = "0xa8 - channel selection register"]
    #[inline(always)]
    pub const fn cselr(&self) -> &CSELR {
        &self.cselr
    }
}
#[doc = "ISR (r) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR?, and CM1AR? registers"]
pub mod ch;
#[doc = "CSELR (rw) register accessor: channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cselr`]
module"]
pub type CSELR = crate::Reg<cselr::CSELRrs>;
#[doc = "channel selection register"]
pub mod cselr;
