#[repr(C)]
#[derive(Debug)]
///CAN Transmit cluster
pub struct TX {
    tir: TIR,
    tdtr: TDTR,
    tdlr: TDLR,
    tdhr: TDHR,
}
impl TX {
    ///0x00 - TX mailbox identifier register
    #[inline(always)]
    pub const fn tir(&self) -> &TIR {
        &self.tir
    }
    ///0x04 - mailbox data length control and time stamp register
    #[inline(always)]
    pub const fn tdtr(&self) -> &TDTR {
        &self.tdtr
    }
    ///0x08 - mailbox data low register
    #[inline(always)]
    pub const fn tdlr(&self) -> &TDLR {
        &self.tdlr
    }
    ///0x0c - mailbox data high register
    #[inline(always)]
    pub const fn tdhr(&self) -> &TDHR {
        &self.tdhr
    }
}
/**TIR (rw) register accessor: TX mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`tir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tir`] module*/
pub type TIR = crate::Reg<tir::TIRrs>;
///TX mailbox identifier register
pub mod tir;
/**TDTR (rw) register accessor: mailbox data length control and time stamp register

You can [`read`](crate::Reg::read) this register and get [`tdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdtr`] module*/
pub type TDTR = crate::Reg<tdtr::TDTRrs>;
///mailbox data length control and time stamp register
pub mod tdtr;
/**TDLR (rw) register accessor: mailbox data low register

You can [`read`](crate::Reg::read) this register and get [`tdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdlr`] module*/
pub type TDLR = crate::Reg<tdlr::TDLRrs>;
///mailbox data low register
pub mod tdlr;
/**TDHR (rw) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`tdhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdhr`] module*/
pub type TDHR = crate::Reg<tdhr::TDHRrs>;
///mailbox data high register
pub mod tdhr;
