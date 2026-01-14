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
    ///0x00 - CAN_TI0R
    #[inline(always)]
    pub const fn tir(&self) -> &TIR {
        &self.tir
    }
    ///0x04 - CAN_TDT0R
    #[inline(always)]
    pub const fn tdtr(&self) -> &TDTR {
        &self.tdtr
    }
    ///0x08 - CAN_TDL0R
    #[inline(always)]
    pub const fn tdlr(&self) -> &TDLR {
        &self.tdlr
    }
    ///0x0c - CAN_TDH0R
    #[inline(always)]
    pub const fn tdhr(&self) -> &TDHR {
        &self.tdhr
    }
}
/**TIR (rw) register accessor: CAN_TI0R

You can [`read`](crate::Reg::read) this register and get [`tir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tir`] module*/
pub type TIR = crate::Reg<tir::TIRrs>;
///CAN_TI0R
pub mod tir;
/**TDTR (rw) register accessor: CAN_TDT0R

You can [`read`](crate::Reg::read) this register and get [`tdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdtr`] module*/
pub type TDTR = crate::Reg<tdtr::TDTRrs>;
///CAN_TDT0R
pub mod tdtr;
/**TDLR (rw) register accessor: CAN_TDL0R

You can [`read`](crate::Reg::read) this register and get [`tdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdlr`] module*/
pub type TDLR = crate::Reg<tdlr::TDLRrs>;
///CAN_TDL0R
pub mod tdlr;
/**TDHR (rw) register accessor: CAN_TDH0R

You can [`read`](crate::Reg::read) this register and get [`tdhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdhr`] module*/
pub type TDHR = crate::Reg<tdhr::TDHRrs>;
///CAN_TDH0R
pub mod tdhr;
