#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDMA Global Interrupt/Status Register"]
    pub gisr0: crate::Reg<gisr0::GISR0_SPEC>,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40..0x78 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch0: CH,
    _reserved2: [u8; 0x08],
    #[doc = "0x80..0xb8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch1: CH,
    _reserved3: [u8; 0x08],
    #[doc = "0xc0..0xf8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch2: CH,
    _reserved4: [u8; 0x08],
    #[doc = "0x100..0x138 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch3: CH,
    _reserved5: [u8; 0x08],
    #[doc = "0x140..0x178 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch4: CH,
    _reserved6: [u8; 0x08],
    #[doc = "0x180..0x1b8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch5: CH,
    _reserved7: [u8; 0x08],
    #[doc = "0x1c0..0x1f8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch6: CH,
    _reserved8: [u8; 0x08],
    #[doc = "0x200..0x238 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch7: CH,
    _reserved9: [u8; 0x08],
    #[doc = "0x240..0x278 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch8: CH,
    _reserved10: [u8; 0x08],
    #[doc = "0x280..0x2b8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch9: CH,
    _reserved11: [u8; 0x08],
    #[doc = "0x2c0..0x2f8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch10: CH,
    _reserved12: [u8; 0x08],
    #[doc = "0x300..0x338 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch11: CH,
    _reserved13: [u8; 0x08],
    #[doc = "0x340..0x378 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch12: CH,
    _reserved14: [u8; 0x08],
    #[doc = "0x380..0x3b8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch13: CH,
    _reserved15: [u8; 0x08],
    #[doc = "0x3c0..0x3f8 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch14: CH,
    _reserved16: [u8; 0x08],
    #[doc = "0x400..0x438 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch15: CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - MDMA channel x interrupt/status register"]
    pub isr: crate::Reg<self::ch::isr::ISR_SPEC>,
    #[doc = "0x04 - MDMA channel x interrupt flag clear register"]
    pub ifcr: crate::Reg<self::ch::ifcr::IFCR_SPEC>,
    #[doc = "0x08 - MDMA Channel x error status register"]
    pub esr: crate::Reg<self::ch::esr::ESR_SPEC>,
    #[doc = "0x0c - This register is used to control the concerned channel."]
    pub cr: crate::Reg<self::ch::cr::CR_SPEC>,
    #[doc = "0x10 - This register is used to configure the concerned channel."]
    pub tcr: crate::Reg<self::ch::tcr::TCR_SPEC>,
    #[doc = "0x14 - MDMA Channel x block number of data register"]
    pub bndtr: crate::Reg<self::ch::bndtr::BNDTR_SPEC>,
    #[doc = "0x18 - MDMA channel x source address register"]
    pub sar: crate::Reg<self::ch::sar::SAR_SPEC>,
    #[doc = "0x1c - MDMA channel x destination address register"]
    pub dar: crate::Reg<self::ch::dar::DAR_SPEC>,
    #[doc = "0x20 - MDMA channel x Block Repeat address Update register"]
    pub brur: crate::Reg<self::ch::brur::BRUR_SPEC>,
    #[doc = "0x24 - MDMA channel x Link Address register"]
    pub lar: crate::Reg<self::ch::lar::LAR_SPEC>,
    #[doc = "0x28 - MDMA channel x Trigger and Bus selection Register"]
    pub tbr: crate::Reg<self::ch::tbr::TBR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - MDMA channel x Mask address register"]
    pub mar: crate::Reg<self::ch::mar::MAR_SPEC>,
    #[doc = "0x34 - MDMA channel x Mask Data register"]
    pub mdr: crate::Reg<self::ch::mdr::MDR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
pub mod ch;
#[doc = "GISR0 register accessor: an alias for `Reg<GISR0_SPEC>`"]
pub type GISR0 = crate::Reg<gisr0::GISR0_SPEC>;
#[doc = "MDMA Global Interrupt/Status Register"]
pub mod gisr0;
