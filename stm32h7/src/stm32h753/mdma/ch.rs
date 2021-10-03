#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "MDMA channel x interrupt/status register"]
pub mod isr;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod ifcr;
#[doc = "ESR register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "MDMA Channel x error status register"]
pub mod esr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "This register is used to control the concerned channel."]
pub mod cr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "This register is used to configure the concerned channel."]
pub mod tcr;
#[doc = "BNDTR register accessor: an alias for `Reg<BNDTR_SPEC>`"]
pub type BNDTR = crate::Reg<bndtr::BNDTR_SPEC>;
#[doc = "MDMA Channel x block number of data register"]
pub mod bndtr;
#[doc = "SAR register accessor: an alias for `Reg<SAR_SPEC>`"]
pub type SAR = crate::Reg<sar::SAR_SPEC>;
#[doc = "MDMA channel x source address register"]
pub mod sar;
#[doc = "DAR register accessor: an alias for `Reg<DAR_SPEC>`"]
pub type DAR = crate::Reg<dar::DAR_SPEC>;
#[doc = "MDMA channel x destination address register"]
pub mod dar;
#[doc = "BRUR register accessor: an alias for `Reg<BRUR_SPEC>`"]
pub type BRUR = crate::Reg<brur::BRUR_SPEC>;
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod brur;
#[doc = "LAR register accessor: an alias for `Reg<LAR_SPEC>`"]
pub type LAR = crate::Reg<lar::LAR_SPEC>;
#[doc = "MDMA channel x Link Address register"]
pub mod lar;
#[doc = "TBR register accessor: an alias for `Reg<TBR_SPEC>`"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod tbr;
#[doc = "MAR register accessor: an alias for `Reg<MAR_SPEC>`"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "MDMA channel x Mask address register"]
pub mod mar;
#[doc = "MDR register accessor: an alias for `Reg<MDR_SPEC>`"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "MDMA channel x Mask Data register"]
pub mod mdr;
