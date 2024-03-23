#[repr(C)]
#[doc = "CAN Receive cluster"]
pub struct RX {
    rir: RIR,
    rdtr: RDTR,
    rdlr: RDLR,
    rdhr: RDHR,
}
impl RX {
    #[doc = "0x00 - receive FIFO mailbox identifier register"]
    #[inline(always)]
    pub const fn rir(&self) -> &RIR {
        &self.rir
    }
    #[doc = "0x04 - receive FIFO mailbox data length control and time stamp register"]
    #[inline(always)]
    pub const fn rdtr(&self) -> &RDTR {
        &self.rdtr
    }
    #[doc = "0x08 - receive FIFO mailbox data low register"]
    #[inline(always)]
    pub const fn rdlr(&self) -> &RDLR {
        &self.rdlr
    }
    #[doc = "0x0c - receive FIFO mailbox data high register"]
    #[inline(always)]
    pub const fn rdhr(&self) -> &RDHR {
        &self.rdhr
    }
}
#[doc = "RIR (r) register accessor: receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rir`]
module"]
pub type RIR = crate::Reg<rir::RIRrs>;
#[doc = "receive FIFO mailbox identifier register"]
pub mod rir;
#[doc = "RDTR (r) register accessor: receive FIFO mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdtr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdtr`]
module"]
pub type RDTR = crate::Reg<rdtr::RDTRrs>;
#[doc = "receive FIFO mailbox data length control and time stamp register"]
pub mod rdtr;
#[doc = "RDLR (r) register accessor: receive FIFO mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdlr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdlr`]
module"]
pub type RDLR = crate::Reg<rdlr::RDLRrs>;
#[doc = "receive FIFO mailbox data low register"]
pub mod rdlr;
#[doc = "RDHR (r) register accessor: receive FIFO mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdhr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdhr`]
module"]
pub type RDHR = crate::Reg<rdhr::RDHRrs>;
#[doc = "receive FIFO mailbox data high register"]
pub mod rdhr;
