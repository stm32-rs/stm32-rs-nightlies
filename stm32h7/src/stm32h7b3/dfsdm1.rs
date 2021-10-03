#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x18 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch0: CH,
    _reserved1: [u8; 0x08],
    #[doc = "0x20..0x38 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch1: CH,
    _reserved2: [u8; 0x08],
    #[doc = "0x40..0x58 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch2: CH,
    _reserved3: [u8; 0x08],
    #[doc = "0x60..0x78 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch3: CH,
    _reserved4: [u8; 0x08],
    #[doc = "0x80..0x98 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch4: CH,
    _reserved5: [u8; 0x08],
    #[doc = "0xa0..0xb8 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch5: CH,
    _reserved6: [u8; 0x08],
    #[doc = "0xc0..0xd8 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch6: CH,
    _reserved7: [u8; 0x08],
    #[doc = "0xe0..0xf8 - Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
    pub ch7: CH,
    _reserved8: [u8; 0x08],
    #[doc = "0x100..0x13c - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt0: FLT,
    _reserved9: [u8; 0x44],
    #[doc = "0x180..0x1bc - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt1: FLT,
    _reserved10: [u8; 0x44],
    #[doc = "0x200..0x23c - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt2: FLT,
    _reserved11: [u8; 0x44],
    #[doc = "0x280..0x2bc - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt3: FLT,
    _reserved12: [u8; 0x44],
    #[doc = "0x300..0x33c - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt4: FLT,
    _reserved13: [u8; 0x44],
    #[doc = "0x380..0x3bc - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt5: FLT,
    _reserved14: [u8; 0x44],
    #[doc = "0x400..0x43c - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt6: FLT,
    _reserved15: [u8; 0x44],
    #[doc = "0x480..0x4bc - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt7: FLT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DFSDM channel 0 configuration register"]
    pub cfgr1: crate::Reg<self::ch::cfgr1::CFGR1_SPEC>,
    #[doc = "0x04 - DFSDM channel 0 configuration register"]
    pub cfgr2: crate::Reg<self::ch::cfgr2::CFGR2_SPEC>,
    #[doc = "0x08 - DFSDM channel 0 analog watchdog and short-circuit detector register"]
    pub awscdr: crate::Reg<self::ch::awscdr::AWSCDR_SPEC>,
    #[doc = "0x0c - DFSDM channel 0 watchdog filter data register"]
    pub wdatr: crate::Reg<self::ch::wdatr::WDATR_SPEC>,
    #[doc = "0x10 - DFSDM channel 0 data input register"]
    pub datinr: crate::Reg<self::ch::datinr::DATINR_SPEC>,
    #[doc = "0x14 - "]
    pub dlyr: crate::Reg<self::ch::dlyr::DLYR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR"]
pub mod ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct FLT {
    #[doc = "0x00 - "]
    pub cr1: crate::Reg<self::flt::cr1::CR1_SPEC>,
    #[doc = "0x04 - "]
    pub cr2: crate::Reg<self::flt::cr2::CR2_SPEC>,
    #[doc = "0x08 - "]
    pub isr: crate::Reg<self::flt::isr::ISR_SPEC>,
    #[doc = "0x0c - "]
    pub icr: crate::Reg<self::flt::icr::ICR_SPEC>,
    #[doc = "0x10 - "]
    pub jchgr: crate::Reg<self::flt::jchgr::JCHGR_SPEC>,
    #[doc = "0x14 - "]
    pub fcr: crate::Reg<self::flt::fcr::FCR_SPEC>,
    #[doc = "0x18 - "]
    pub jdatar: crate::Reg<self::flt::jdatar::JDATAR_SPEC>,
    #[doc = "0x1c - "]
    pub rdatar: crate::Reg<self::flt::rdatar::RDATAR_SPEC>,
    #[doc = "0x20 - "]
    pub awhtr: crate::Reg<self::flt::awhtr::AWHTR_SPEC>,
    #[doc = "0x24 - "]
    pub awltr: crate::Reg<self::flt::awltr::AWLTR_SPEC>,
    #[doc = "0x28 - "]
    pub awsr: crate::Reg<self::flt::awsr::AWSR_SPEC>,
    #[doc = "0x2c - "]
    pub awcfr: crate::Reg<self::flt::awcfr::AWCFR_SPEC>,
    #[doc = "0x30 - "]
    pub exmax: crate::Reg<self::flt::exmax::EXMAX_SPEC>,
    #[doc = "0x34 - "]
    pub exmin: crate::Reg<self::flt::exmin::EXMIN_SPEC>,
    #[doc = "0x38 - "]
    pub fltcnvtimr: crate::Reg<self::flt::fltcnvtimr::FLTCNVTIMR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
pub mod flt;
