#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDMA Global Interrupt/Status Register"]
    pub gisr0: GISR0,
    _reserved1: [u8; 60usize],
    #[doc = "0x40 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch0: CH,
    _reserved2: [u8; 8usize],
    #[doc = "0x80 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch1: CH,
    _reserved3: [u8; 8usize],
    #[doc = "0xc0 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch2: CH,
    _reserved4: [u8; 8usize],
    #[doc = "0x100 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch3: CH,
    _reserved5: [u8; 8usize],
    #[doc = "0x140 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch4: CH,
    _reserved6: [u8; 8usize],
    #[doc = "0x180 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch5: CH,
    _reserved7: [u8; 8usize],
    #[doc = "0x1c0 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch6: CH,
    _reserved8: [u8; 8usize],
    #[doc = "0x200 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch7: CH,
    _reserved9: [u8; 8usize],
    #[doc = "0x240 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch8: CH,
    _reserved10: [u8; 8usize],
    #[doc = "0x280 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch9: CH,
    _reserved11: [u8; 8usize],
    #[doc = "0x2c0 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch10: CH,
    _reserved12: [u8; 8usize],
    #[doc = "0x300 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch11: CH,
    _reserved13: [u8; 8usize],
    #[doc = "0x340 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch12: CH,
    _reserved14: [u8; 8usize],
    #[doc = "0x380 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch13: CH,
    _reserved15: [u8; 8usize],
    #[doc = "0x3c0 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch14: CH,
    _reserved16: [u8; 8usize],
    #[doc = "0x400 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
    pub ch15: CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - MDMA channel x interrupt/status register"]
    pub isr: self::ch::ISR,
    #[doc = "0x04 - MDMA channel x interrupt flag clear register"]
    pub ifcr: self::ch::IFCR,
    #[doc = "0x08 - MDMA Channel x error status register"]
    pub esr: self::ch::ESR,
    #[doc = "0x0c - This register is used to control the concerned channel."]
    pub cr: self::ch::CR,
    #[doc = "0x10 - This register is used to configure the concerned channel."]
    pub tcr: self::ch::TCR,
    #[doc = "0x14 - MDMA Channel x block number of data register"]
    pub bndtr: self::ch::BNDTR,
    #[doc = "0x18 - MDMA channel x source address register"]
    pub sar: self::ch::SAR,
    #[doc = "0x1c - MDMA channel x destination address register"]
    pub dar: self::ch::DAR,
    #[doc = "0x20 - MDMA channel x Block Repeat address Update register"]
    pub brur: self::ch::BRUR,
    #[doc = "0x24 - MDMA channel x Link Address register"]
    pub lar: self::ch::LAR,
    #[doc = "0x28 - MDMA channel x Trigger and Bus selection Register"]
    pub tbr: self::ch::TBR,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - MDMA channel x Mask address register"]
    pub mar: self::ch::MAR,
    #[doc = "0x34 - MDMA channel x Mask Data register"]
    pub mdr: self::ch::MDR,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers"]
pub mod ch;
#[doc = "MDMA Global Interrupt/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gisr0](gisr0) module"]
pub type GISR0 = crate::Reg<u32, _GISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GISR0;
#[doc = "`read()` method returns [gisr0::R](gisr0::R) reader structure"]
impl crate::Readable for GISR0 {}
#[doc = "MDMA Global Interrupt/Status Register"]
pub mod gisr0;
