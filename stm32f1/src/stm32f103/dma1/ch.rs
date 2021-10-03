#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod cr;
#[doc = "NDTR register accessor: an alias for `Reg<NDTR_SPEC>`"]
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
#[doc = "DMA channel 1 number of data register"]
pub mod ndtr;
#[doc = "PAR register accessor: an alias for `Reg<PAR_SPEC>`"]
pub type PAR = crate::Reg<par::PAR_SPEC>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod par;
#[doc = "MAR register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "DMA channel 1 memory address register"]
pub mod mar;
