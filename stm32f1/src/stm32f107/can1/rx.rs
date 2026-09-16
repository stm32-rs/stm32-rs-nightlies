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
    ///0x00 - CAN_RI0R
    #[inline(always)]
    pub const fn rir(&self) -> &RIR {
        &self.rir
    }
    ///0x04 - CAN_RDT0R
    #[inline(always)]
    pub const fn rdtr(&self) -> &RDTR {
        &self.rdtr
    }
    ///0x08 - CAN_RDL0R
    #[inline(always)]
    pub const fn rdlr(&self) -> &RDLR {
        &self.rdlr
    }
    ///0x0c - CAN_RDH0R
    #[inline(always)]
    pub const fn rdhr(&self) -> &RDHR {
        &self.rdhr
    }
}
/**RIR (r) register accessor: CAN_RI0R

You can [`read`](crate::Reg::read) this register and get [`rir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rir`] module*/
pub type RIR = crate::Reg<rir::RIRrs>;
///CAN_RI0R
pub mod rir;
/**RDTR (r) register accessor: CAN_RDT0R

You can [`read`](crate::Reg::read) this register and get [`rdtr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdtr`] module*/
pub type RDTR = crate::Reg<rdtr::RDTRrs>;
///CAN_RDT0R
pub mod rdtr;
/**RDLR (r) register accessor: CAN_RDL0R

You can [`read`](crate::Reg::read) this register and get [`rdlr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdlr`] module*/
pub type RDLR = crate::Reg<rdlr::RDLRrs>;
///CAN_RDL0R
pub mod rdlr;
/**RDHR (r) register accessor: CAN_RDH0R

You can [`read`](crate::Reg::read) this register and get [`rdhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdhr`] module*/
pub type RDHR = crate::Reg<rdhr::RDHRrs>;
///CAN_RDH0R
pub mod rdhr;
