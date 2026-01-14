#[repr(C)]
#[derive(Debug)]
///CAN Receive cluster
pub struct RX {
    rir: RIR,
    rdtr: RDTR,
    rdlr: RDLR,
    rdhr: RDHR,
}
impl RX {
    ///0x00 - receive FIFO mailbox identifier register
    #[inline(always)]
    pub const fn rir(&self) -> &RIR {
        &self.rir
    }
    ///0x04 - mailbox data high register
    #[inline(always)]
    pub const fn rdtr(&self) -> &RDTR {
        &self.rdtr
    }
    ///0x08 - mailbox data high register
    #[inline(always)]
    pub const fn rdlr(&self) -> &RDLR {
        &self.rdlr
    }
    ///0x0c - receive FIFO mailbox data high register
    #[inline(always)]
    pub const fn rdhr(&self) -> &RDHR {
        &self.rdhr
    }
}
/**RIR (r) register accessor: receive FIFO mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`rir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rir`] module*/
pub type RIR = crate::Reg<rir::RIRrs>;
///receive FIFO mailbox identifier register
pub mod rir;
/**RDTR (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdtr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdtr`] module*/
pub type RDTR = crate::Reg<rdtr::RDTRrs>;
///mailbox data high register
pub mod rdtr;
/**RDLR (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdlr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdlr`] module*/
pub type RDLR = crate::Reg<rdlr::RDLRrs>;
///mailbox data high register
pub mod rdlr;
/**RDHR (r) register accessor: receive FIFO mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdhr`] module*/
pub type RDHR = crate::Reg<rdhr::RDHRrs>;
///receive FIFO mailbox data high register
pub mod rdhr;
