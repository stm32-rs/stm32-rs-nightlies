#[repr(C)]
#[doc = "CAN Transmit cluster"]
pub struct TX {
    tir: TIR,
    tdtr: TDTR,
    tdlr: TDLR,
    tdhr: TDHR,
}
impl TX {
    #[doc = "0x00 - TX mailbox identifier register"]
    #[inline(always)]
    pub const fn tir(&self) -> &TIR {
        &self.tir
    }
    #[doc = "0x04 - mailbox data length control and time stamp register"]
    #[inline(always)]
    pub const fn tdtr(&self) -> &TDTR {
        &self.tdtr
    }
    #[doc = "0x08 - mailbox data low register"]
    #[inline(always)]
    pub const fn tdlr(&self) -> &TDLR {
        &self.tdlr
    }
    #[doc = "0x0c - mailbox data high register"]
    #[inline(always)]
    pub const fn tdhr(&self) -> &TDHR {
        &self.tdhr
    }
}
#[doc = "TIR (rw) register accessor: TX mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tir`]
module"]
pub type TIR = crate::Reg<tir::TIRrs>;
#[doc = "TX mailbox identifier register"]
pub mod tir;
#[doc = "TDTR (rw) register accessor: mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdtr`]
module"]
pub type TDTR = crate::Reg<tdtr::TDTRrs>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdtr;
#[doc = "TDLR (rw) register accessor: mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdlr`]
module"]
pub type TDLR = crate::Reg<tdlr::TDLRrs>;
#[doc = "mailbox data low register"]
pub mod tdlr;
#[doc = "TDHR (rw) register accessor: mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdhr`]
module"]
pub type TDHR = crate::Reg<tdhr::TDHRrs>;
#[doc = "mailbox data high register"]
pub mod tdhr;
