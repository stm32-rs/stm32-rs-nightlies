#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    pub power: crate::Reg<power::POWER_SPEC>,
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCR)"]
    pub clkcr: crate::Reg<clkcr::CLKCR_SPEC>,
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    pub arg: crate::Reg<arg::ARG_SPEC>,
    #[doc = "0x0c - SDIO command register (SDIO_CMD)"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x10 - SDIO command register"]
    pub respcmd: crate::Reg<respcmd::RESPCMD_SPEC>,
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    pub respi1: crate::Reg<respi1::RESPI1_SPEC>,
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    pub resp2: crate::Reg<resp::RESP_SPEC>,
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS2"]
    pub resp3: crate::Reg<resp::RESP_SPEC>,
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS2"]
    pub resp4: crate::Reg<resp::RESP_SPEC>,
    #[doc = "0x24 - Bits 31:0 = DATATIME: Data timeout period"]
    pub dtimer: crate::Reg<dtimer::DTIMER_SPEC>,
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    pub dlen: crate::Reg<dlen::DLEN_SPEC>,
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    pub dctrl: crate::Reg<dctrl::DCTRL_SPEC>,
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    pub dcount: crate::Reg<dcount::DCOUNT_SPEC>,
    #[doc = "0x34 - SDIO status register (SDIO_STA)"]
    pub sta: crate::Reg<sta::STA_SPEC>,
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_ICR)"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x3c - SDIO mask register (SDIO_MASK)"]
    pub mask: crate::Reg<mask::MASK_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x48 - Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
    pub fifocnt: crate::Reg<fifocnt::FIFOCNT_SPEC>,
    _reserved17: [u8; 0x34],
    #[doc = "0x80 - bits 31:0 = FIFOData: Receive and transmit FIFO data"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
}
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod power;
#[doc = "CLKCR register accessor: an alias for `Reg<CLKCR_SPEC>`"]
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
#[doc = "SDI clock control register (SDIO_CLKCR)"]
pub mod clkcr;
#[doc = "ARG register accessor: an alias for `Reg<ARG_SPEC>`"]
pub type ARG = crate::Reg<arg::ARG_SPEC>;
#[doc = "Bits 31:0 = : Command argument"]
pub mod arg;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SDIO command register (SDIO_CMD)"]
pub mod cmd;
#[doc = "RESPCMD register accessor: an alias for `Reg<RESPCMD_SPEC>`"]
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
#[doc = "SDIO command register"]
pub mod respcmd;
#[doc = "RESPI1 register accessor: an alias for `Reg<RESPI1_SPEC>`"]
pub type RESPI1 = crate::Reg<respi1::RESPI1_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod respi1;
#[doc = "RESP register accessor: an alias for `Reg<RESP_SPEC>`"]
pub type RESP = crate::Reg<resp::RESP_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod resp;
#[doc = "DTIMER register accessor: an alias for `Reg<DTIMER_SPEC>`"]
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
#[doc = "Bits 31:0 = DATATIME: Data timeout period"]
pub mod dtimer;
#[doc = "DLEN register accessor: an alias for `Reg<DLEN_SPEC>`"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dlen;
#[doc = "DCTRL register accessor: an alias for `Reg<DCTRL_SPEC>`"]
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dctrl;
#[doc = "DCOUNT register accessor: an alias for `Reg<DCOUNT_SPEC>`"]
pub type DCOUNT = crate::Reg<dcount::DCOUNT_SPEC>;
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dcount;
#[doc = "STA register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "SDIO status register (SDIO_STA)"]
pub mod sta;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "SDIO interrupt clear register (SDIO_ICR)"]
pub mod icr;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "SDIO mask register (SDIO_MASK)"]
pub mod mask;
#[doc = "FIFOCNT register accessor: an alias for `Reg<FIFOCNT_SPEC>`"]
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod fifocnt;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data"]
pub mod fifo;
