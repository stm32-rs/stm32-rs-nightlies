#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - channel configuration y register"]
    pub ch0cfgr1: crate::Reg<ch0cfgr1::CH0CFGR1_SPEC>,
    #[doc = "0x04 - channel configuration y register"]
    pub ch0cfgr2: crate::Reg<ch0cfgr2::CH0CFGR2_SPEC>,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub ch0awscdr: crate::Reg<ch0awscdr::CH0AWSCDR_SPEC>,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub ch0wdatr: crate::Reg<ch0wdatr::CH0WDATR_SPEC>,
    #[doc = "0x10 - channel data input register"]
    pub ch0datinr: crate::Reg<ch0datinr::CH0DATINR_SPEC>,
    #[doc = "0x14 - channel y delay register"]
    pub ch0dlyr: crate::Reg<ch0dlyr::CH0DLYR_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - CH1CFGR1"]
    pub ch1cfgr1: crate::Reg<ch1cfgr1::CH1CFGR1_SPEC>,
    #[doc = "0x24 - CH1CFGR2"]
    pub ch1cfgr2: crate::Reg<ch1cfgr2::CH1CFGR2_SPEC>,
    #[doc = "0x28 - CH1AWSCDR"]
    pub ch1awscdr: crate::Reg<ch1awscdr::CH1AWSCDR_SPEC>,
    #[doc = "0x2c - CH1WDATR"]
    pub ch1wdatr: crate::Reg<ch1wdatr::CH1WDATR_SPEC>,
    #[doc = "0x30 - CH1DATINR"]
    pub ch1datinr: crate::Reg<ch1datinr::CH1DATINR_SPEC>,
    #[doc = "0x34 - channel y delay register"]
    pub ch1dlyr: crate::Reg<ch1dlyr::CH1DLYR_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - CH2CFGR1"]
    pub ch2cfgr1: crate::Reg<ch2cfgr1::CH2CFGR1_SPEC>,
    #[doc = "0x44 - CH2CFGR2"]
    pub ch2cfgr2: crate::Reg<ch2cfgr2::CH2CFGR2_SPEC>,
    #[doc = "0x48 - CH2AWSCDR"]
    pub ch2awscdr: crate::Reg<ch2awscdr::CH2AWSCDR_SPEC>,
    #[doc = "0x4c - CH2WDATR"]
    pub ch2wdatr: crate::Reg<ch2wdatr::CH2WDATR_SPEC>,
    #[doc = "0x50 - CH2DATINR"]
    pub ch2datinr: crate::Reg<ch2datinr::CH2DATINR_SPEC>,
    #[doc = "0x54 - channel y delay register"]
    pub ch2dlyr: crate::Reg<ch2dlyr::CH2DLYR_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x60 - CH3CFGR1"]
    pub ch3cfgr1: crate::Reg<ch3cfgr1::CH3CFGR1_SPEC>,
    #[doc = "0x64 - CH3CFGR2"]
    pub ch3cfgr2: crate::Reg<ch3cfgr2::CH3CFGR2_SPEC>,
    #[doc = "0x68 - CH3AWSCDR"]
    pub ch3awscdr: crate::Reg<ch3awscdr::CH3AWSCDR_SPEC>,
    #[doc = "0x6c - CH3WDATR"]
    pub ch3wdatr: crate::Reg<ch3wdatr::CH3WDATR_SPEC>,
    #[doc = "0x70 - CH3DATINR"]
    pub ch3datinr: crate::Reg<ch3datinr::CH3DATINR_SPEC>,
    #[doc = "0x74 - channel y delay register"]
    pub ch3dlyr: crate::Reg<ch3dlyr::CH3DLYR_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - CH4CFGR1"]
    pub ch4cfgr1: crate::Reg<ch4cfgr1::CH4CFGR1_SPEC>,
    #[doc = "0x84 - CH4CFGR2"]
    pub ch4cfgr2: crate::Reg<ch4cfgr2::CH4CFGR2_SPEC>,
    #[doc = "0x88 - CH4AWSCDR"]
    pub ch4awscdr: crate::Reg<ch4awscdr::CH4AWSCDR_SPEC>,
    #[doc = "0x8c - CH4WDATR"]
    pub ch4wdatr: crate::Reg<ch4wdatr::CH4WDATR_SPEC>,
    #[doc = "0x90 - CH4DATINR"]
    pub ch4datinr: crate::Reg<ch4datinr::CH4DATINR_SPEC>,
    #[doc = "0x94 - channel y delay register"]
    pub ch4dlyr: crate::Reg<ch4dlyr::CH4DLYR_SPEC>,
    _reserved30: [u8; 0x08],
    #[doc = "0xa0 - CH5CFGR1"]
    pub ch5cfgr1: crate::Reg<ch5cfgr1::CH5CFGR1_SPEC>,
    #[doc = "0xa4 - CH5CFGR2"]
    pub ch5cfgr2: crate::Reg<ch5cfgr2::CH5CFGR2_SPEC>,
    #[doc = "0xa8 - CH5AWSCDR"]
    pub ch5awscdr: crate::Reg<ch5awscdr::CH5AWSCDR_SPEC>,
    #[doc = "0xac - CH5WDATR"]
    pub ch5wdatr: crate::Reg<ch5wdatr::CH5WDATR_SPEC>,
    #[doc = "0xb0 - CH5DATINR"]
    pub ch5datinr: crate::Reg<ch5datinr::CH5DATINR_SPEC>,
    #[doc = "0xb4 - channel y delay register"]
    pub ch5dlyr: crate::Reg<ch5dlyr::CH5DLYR_SPEC>,
    _reserved36: [u8; 0x08],
    #[doc = "0xc0 - CH6CFGR1"]
    pub ch6cfgr1: crate::Reg<ch6cfgr1::CH6CFGR1_SPEC>,
    #[doc = "0xc4 - CH6CFGR2"]
    pub ch6cfgr2: crate::Reg<ch6cfgr2::CH6CFGR2_SPEC>,
    #[doc = "0xc8 - CH6AWSCDR"]
    pub ch6awscdr: crate::Reg<ch6awscdr::CH6AWSCDR_SPEC>,
    #[doc = "0xcc - CH6WDATR"]
    pub ch6wdatr: crate::Reg<ch6wdatr::CH6WDATR_SPEC>,
    #[doc = "0xd0 - CH6DATINR"]
    pub ch6datinr: crate::Reg<ch6datinr::CH6DATINR_SPEC>,
    #[doc = "0xd4 - channel y delay register"]
    pub ch6dlyr: crate::Reg<ch6dlyr::CH6DLYR_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0xe0 - CH7CFGR1"]
    pub ch7cfgr1: crate::Reg<ch7cfgr1::CH7CFGR1_SPEC>,
    #[doc = "0xe4 - CH7CFGR2"]
    pub ch7cfgr2: crate::Reg<ch7cfgr2::CH7CFGR2_SPEC>,
    #[doc = "0xe8 - CH7AWSCDR"]
    pub ch7awscdr: crate::Reg<ch7awscdr::CH7AWSCDR_SPEC>,
    #[doc = "0xec - CH7WDATR"]
    pub ch7wdatr: crate::Reg<ch7wdatr::CH7WDATR_SPEC>,
    #[doc = "0xf0 - CH7DATINR"]
    pub ch7datinr: crate::Reg<ch7datinr::CH7DATINR_SPEC>,
    #[doc = "0xf4 - channel y delay register"]
    pub ch7dlyr: crate::Reg<ch7dlyr::CH7DLYR_SPEC>,
    _reserved48: [u8; 0x08],
    #[doc = "0x100 - control register 1"]
    pub dfsdm_flt0cr1: crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1_SPEC>,
    #[doc = "0x104 - control register 2"]
    pub dfsdm_flt0cr2: crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2_SPEC>,
    #[doc = "0x108 - interrupt and status register"]
    pub dfsdm_flt0isr: crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISR_SPEC>,
    #[doc = "0x10c - interrupt flag clear register"]
    pub dfsdm_flt0icr: crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICR_SPEC>,
    #[doc = "0x110 - injected channel group selection register"]
    pub dfsdm_flt0jchgr: crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGR_SPEC>,
    #[doc = "0x114 - filter control register"]
    pub dfsdm_flt0fcr: crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCR_SPEC>,
    #[doc = "0x118 - data register for injected group"]
    pub dfsdm_flt0jdatar: crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATAR_SPEC>,
    #[doc = "0x11c - data register for the regular channel"]
    pub dfsdm_flt0rdatar: crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATAR_SPEC>,
    #[doc = "0x120 - analog watchdog high threshold register"]
    pub dfsdm_flt0awhtr: crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTR_SPEC>,
    #[doc = "0x124 - analog watchdog low threshold register"]
    pub dfsdm_flt0awltr: crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTR_SPEC>,
    #[doc = "0x128 - analog watchdog status register"]
    pub dfsdm_flt0awsr: crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSR_SPEC>,
    #[doc = "0x12c - analog watchdog clear flag register"]
    pub dfsdm_flt0awcfr: crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFR_SPEC>,
    #[doc = "0x130 - Extremes detector maximum register"]
    pub dfsdm_flt0exmax: crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAX_SPEC>,
    #[doc = "0x134 - Extremes detector minimum register"]
    pub dfsdm_flt0exmin: crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMIN_SPEC>,
    #[doc = "0x138 - conversion timer register"]
    pub dfsdm_flt0cnvtimr: crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMR_SPEC>,
    _reserved63: [u8; 0x44],
    #[doc = "0x180 - control register 1"]
    pub dfsdm_flt1cr1: crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1_SPEC>,
    #[doc = "0x184 - control register 2"]
    pub dfsdm_flt1cr2: crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2_SPEC>,
    #[doc = "0x188 - interrupt and status register"]
    pub dfsdm_flt1isr: crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISR_SPEC>,
    #[doc = "0x18c - interrupt flag clear register"]
    pub dfsdm_flt1icr: crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICR_SPEC>,
    #[doc = "0x190 - injected channel group selection register"]
    pub dfsdm_flt1chgr: crate::Reg<dfsdm_flt1chgr::DFSDM_FLT1CHGR_SPEC>,
    #[doc = "0x194 - filter control register"]
    pub dfsdm_flt1fcr: crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCR_SPEC>,
    #[doc = "0x198 - data register for injected group"]
    pub dfsdm_flt1jdatar: crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATAR_SPEC>,
    #[doc = "0x19c - data register for the regular channel"]
    pub dfsdm_flt1rdatar: crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATAR_SPEC>,
    #[doc = "0x1a0 - analog watchdog high threshold register"]
    pub dfsdm_flt1awhtr: crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTR_SPEC>,
    #[doc = "0x1a4 - analog watchdog low threshold register"]
    pub dfsdm_flt1awltr: crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTR_SPEC>,
    #[doc = "0x1a8 - analog watchdog status register"]
    pub dfsdm_flt1awsr: crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSR_SPEC>,
    #[doc = "0x1ac - analog watchdog clear flag register"]
    pub dfsdm_flt1awcfr: crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFR_SPEC>,
    #[doc = "0x1b0 - Extremes detector maximum register"]
    pub dfsdm_flt1exmax: crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAX_SPEC>,
    #[doc = "0x1b4 - Extremes detector minimum register"]
    pub dfsdm_flt1exmin: crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMIN_SPEC>,
    #[doc = "0x1b8 - conversion timer register"]
    pub dfsdm_flt1cnvtimr: crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMR_SPEC>,
    _reserved78: [u8; 0x44],
    #[doc = "0x200 - control register 1"]
    pub dfsdm_flt2cr1: crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1_SPEC>,
    #[doc = "0x204 - control register 2"]
    pub dfsdm_flt2cr2: crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2_SPEC>,
    #[doc = "0x208 - interrupt and status register"]
    pub dfsdm_flt2isr: crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISR_SPEC>,
    #[doc = "0x20c - interrupt flag clear register"]
    pub dfsdm_flt2icr: crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICR_SPEC>,
    #[doc = "0x210 - injected channel group selection register"]
    pub dfsdm_flt2jchgr: crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGR_SPEC>,
    #[doc = "0x214 - filter control register"]
    pub dfsdm_flt2fcr: crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCR_SPEC>,
    #[doc = "0x218 - data register for injected group"]
    pub dfsdm_flt2jdatar: crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATAR_SPEC>,
    #[doc = "0x21c - data register for the regular channel"]
    pub dfsdm_flt2rdatar: crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATAR_SPEC>,
    #[doc = "0x220 - analog watchdog high threshold register"]
    pub dfsdm_flt2awhtr: crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTR_SPEC>,
    #[doc = "0x224 - analog watchdog low threshold register"]
    pub dfsdm_flt2awltr: crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTR_SPEC>,
    #[doc = "0x228 - analog watchdog status register"]
    pub dfsdm_flt2awsr: crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSR_SPEC>,
    #[doc = "0x22c - analog watchdog clear flag register"]
    pub dfsdm_flt2awcfr: crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFR_SPEC>,
    #[doc = "0x230 - Extremes detector maximum register"]
    pub dfsdm_flt2exmax: crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAX_SPEC>,
    #[doc = "0x234 - Extremes detector minimum register"]
    pub dfsdm_flt2exmin: crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMIN_SPEC>,
    #[doc = "0x238 - conversion timer register"]
    pub dfsdm_flt2cnvtimr: crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMR_SPEC>,
    _reserved93: [u8; 0x44],
    #[doc = "0x280 - control register 1"]
    pub dfsdm_flt3cr1: crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1_SPEC>,
    #[doc = "0x284 - control register 2"]
    pub dfsdm_flt3cr2: crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2_SPEC>,
    #[doc = "0x288 - interrupt and status register"]
    pub dfsdm_flt3isr: crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISR_SPEC>,
    #[doc = "0x28c - interrupt flag clear register"]
    pub dfsdm_flt3icr: crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICR_SPEC>,
    #[doc = "0x290 - injected channel group selection register"]
    pub dfsdm_flt3jchgr: crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGR_SPEC>,
    #[doc = "0x294 - filter control register"]
    pub dfsdm_flt3fcr: crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCR_SPEC>,
    #[doc = "0x298 - data register for injected group"]
    pub dfsdm_flt3jdatar: crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATAR_SPEC>,
    #[doc = "0x29c - data register for the regular channel"]
    pub dfsdm_flt3rdatar: crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATAR_SPEC>,
    #[doc = "0x2a0 - analog watchdog high threshold register"]
    pub dfsdm_flt3awhtr: crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTR_SPEC>,
    #[doc = "0x2a4 - analog watchdog low threshold register"]
    pub dfsdm_flt3awltr: crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTR_SPEC>,
    #[doc = "0x2a8 - analog watchdog status register"]
    pub dfsdm_flt3awsr: crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSR_SPEC>,
    #[doc = "0x2ac - analog watchdog clear flag register"]
    pub dfsdm_flt3awcfr: crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFR_SPEC>,
    #[doc = "0x2b0 - Extremes detector maximum register"]
    pub dfsdm_flt3exmax: crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAX_SPEC>,
    #[doc = "0x2b4 - Extremes detector minimum register"]
    pub dfsdm_flt3exmin: crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMIN_SPEC>,
    #[doc = "0x2b8 - conversion timer register"]
    pub dfsdm_flt3cnvtimr: crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMR_SPEC>,
}
#[doc = "CH0CFGR1 register accessor: an alias for `Reg<CH0CFGR1_SPEC>`"]
pub type CH0CFGR1 = crate::Reg<ch0cfgr1::CH0CFGR1_SPEC>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr1;
#[doc = "CH0CFGR2 register accessor: an alias for `Reg<CH0CFGR2_SPEC>`"]
pub type CH0CFGR2 = crate::Reg<ch0cfgr2::CH0CFGR2_SPEC>;
#[doc = "channel configuration y register"]
pub mod ch0cfgr2;
#[doc = "CH0AWSCDR register accessor: an alias for `Reg<CH0AWSCDR_SPEC>`"]
pub type CH0AWSCDR = crate::Reg<ch0awscdr::CH0AWSCDR_SPEC>;
#[doc = "analog watchdog and short-circuit detector register"]
pub mod ch0awscdr;
#[doc = "CH0WDATR register accessor: an alias for `Reg<CH0WDATR_SPEC>`"]
pub type CH0WDATR = crate::Reg<ch0wdatr::CH0WDATR_SPEC>;
#[doc = "channel watchdog filter data register"]
pub mod ch0wdatr;
#[doc = "CH0DATINR register accessor: an alias for `Reg<CH0DATINR_SPEC>`"]
pub type CH0DATINR = crate::Reg<ch0datinr::CH0DATINR_SPEC>;
#[doc = "channel data input register"]
pub mod ch0datinr;
#[doc = "CH0DLYR register accessor: an alias for `Reg<CH0DLYR_SPEC>`"]
pub type CH0DLYR = crate::Reg<ch0dlyr::CH0DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch0dlyr;
#[doc = "CH1CFGR1 register accessor: an alias for `Reg<CH1CFGR1_SPEC>`"]
pub type CH1CFGR1 = crate::Reg<ch1cfgr1::CH1CFGR1_SPEC>;
#[doc = "CH1CFGR1"]
pub mod ch1cfgr1;
#[doc = "CH1CFGR2 register accessor: an alias for `Reg<CH1CFGR2_SPEC>`"]
pub type CH1CFGR2 = crate::Reg<ch1cfgr2::CH1CFGR2_SPEC>;
#[doc = "CH1CFGR2"]
pub mod ch1cfgr2;
#[doc = "CH1AWSCDR register accessor: an alias for `Reg<CH1AWSCDR_SPEC>`"]
pub type CH1AWSCDR = crate::Reg<ch1awscdr::CH1AWSCDR_SPEC>;
#[doc = "CH1AWSCDR"]
pub mod ch1awscdr;
#[doc = "CH1WDATR register accessor: an alias for `Reg<CH1WDATR_SPEC>`"]
pub type CH1WDATR = crate::Reg<ch1wdatr::CH1WDATR_SPEC>;
#[doc = "CH1WDATR"]
pub mod ch1wdatr;
#[doc = "CH1DATINR register accessor: an alias for `Reg<CH1DATINR_SPEC>`"]
pub type CH1DATINR = crate::Reg<ch1datinr::CH1DATINR_SPEC>;
#[doc = "CH1DATINR"]
pub mod ch1datinr;
#[doc = "CH1DLYR register accessor: an alias for `Reg<CH1DLYR_SPEC>`"]
pub type CH1DLYR = crate::Reg<ch1dlyr::CH1DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch1dlyr;
#[doc = "CH2CFGR1 register accessor: an alias for `Reg<CH2CFGR1_SPEC>`"]
pub type CH2CFGR1 = crate::Reg<ch2cfgr1::CH2CFGR1_SPEC>;
#[doc = "CH2CFGR1"]
pub mod ch2cfgr1;
#[doc = "CH2CFGR2 register accessor: an alias for `Reg<CH2CFGR2_SPEC>`"]
pub type CH2CFGR2 = crate::Reg<ch2cfgr2::CH2CFGR2_SPEC>;
#[doc = "CH2CFGR2"]
pub mod ch2cfgr2;
#[doc = "CH2AWSCDR register accessor: an alias for `Reg<CH2AWSCDR_SPEC>`"]
pub type CH2AWSCDR = crate::Reg<ch2awscdr::CH2AWSCDR_SPEC>;
#[doc = "CH2AWSCDR"]
pub mod ch2awscdr;
#[doc = "CH2WDATR register accessor: an alias for `Reg<CH2WDATR_SPEC>`"]
pub type CH2WDATR = crate::Reg<ch2wdatr::CH2WDATR_SPEC>;
#[doc = "CH2WDATR"]
pub mod ch2wdatr;
#[doc = "CH2DATINR register accessor: an alias for `Reg<CH2DATINR_SPEC>`"]
pub type CH2DATINR = crate::Reg<ch2datinr::CH2DATINR_SPEC>;
#[doc = "CH2DATINR"]
pub mod ch2datinr;
#[doc = "CH2DLYR register accessor: an alias for `Reg<CH2DLYR_SPEC>`"]
pub type CH2DLYR = crate::Reg<ch2dlyr::CH2DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch2dlyr;
#[doc = "CH3CFGR1 register accessor: an alias for `Reg<CH3CFGR1_SPEC>`"]
pub type CH3CFGR1 = crate::Reg<ch3cfgr1::CH3CFGR1_SPEC>;
#[doc = "CH3CFGR1"]
pub mod ch3cfgr1;
#[doc = "CH3CFGR2 register accessor: an alias for `Reg<CH3CFGR2_SPEC>`"]
pub type CH3CFGR2 = crate::Reg<ch3cfgr2::CH3CFGR2_SPEC>;
#[doc = "CH3CFGR2"]
pub mod ch3cfgr2;
#[doc = "CH3AWSCDR register accessor: an alias for `Reg<CH3AWSCDR_SPEC>`"]
pub type CH3AWSCDR = crate::Reg<ch3awscdr::CH3AWSCDR_SPEC>;
#[doc = "CH3AWSCDR"]
pub mod ch3awscdr;
#[doc = "CH3WDATR register accessor: an alias for `Reg<CH3WDATR_SPEC>`"]
pub type CH3WDATR = crate::Reg<ch3wdatr::CH3WDATR_SPEC>;
#[doc = "CH3WDATR"]
pub mod ch3wdatr;
#[doc = "CH3DATINR register accessor: an alias for `Reg<CH3DATINR_SPEC>`"]
pub type CH3DATINR = crate::Reg<ch3datinr::CH3DATINR_SPEC>;
#[doc = "CH3DATINR"]
pub mod ch3datinr;
#[doc = "CH3DLYR register accessor: an alias for `Reg<CH3DLYR_SPEC>`"]
pub type CH3DLYR = crate::Reg<ch3dlyr::CH3DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch3dlyr;
#[doc = "CH4CFGR1 register accessor: an alias for `Reg<CH4CFGR1_SPEC>`"]
pub type CH4CFGR1 = crate::Reg<ch4cfgr1::CH4CFGR1_SPEC>;
#[doc = "CH4CFGR1"]
pub mod ch4cfgr1;
#[doc = "CH4CFGR2 register accessor: an alias for `Reg<CH4CFGR2_SPEC>`"]
pub type CH4CFGR2 = crate::Reg<ch4cfgr2::CH4CFGR2_SPEC>;
#[doc = "CH4CFGR2"]
pub mod ch4cfgr2;
#[doc = "CH4AWSCDR register accessor: an alias for `Reg<CH4AWSCDR_SPEC>`"]
pub type CH4AWSCDR = crate::Reg<ch4awscdr::CH4AWSCDR_SPEC>;
#[doc = "CH4AWSCDR"]
pub mod ch4awscdr;
#[doc = "CH4WDATR register accessor: an alias for `Reg<CH4WDATR_SPEC>`"]
pub type CH4WDATR = crate::Reg<ch4wdatr::CH4WDATR_SPEC>;
#[doc = "CH4WDATR"]
pub mod ch4wdatr;
#[doc = "CH4DATINR register accessor: an alias for `Reg<CH4DATINR_SPEC>`"]
pub type CH4DATINR = crate::Reg<ch4datinr::CH4DATINR_SPEC>;
#[doc = "CH4DATINR"]
pub mod ch4datinr;
#[doc = "CH4DLYR register accessor: an alias for `Reg<CH4DLYR_SPEC>`"]
pub type CH4DLYR = crate::Reg<ch4dlyr::CH4DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch4dlyr;
#[doc = "CH5CFGR1 register accessor: an alias for `Reg<CH5CFGR1_SPEC>`"]
pub type CH5CFGR1 = crate::Reg<ch5cfgr1::CH5CFGR1_SPEC>;
#[doc = "CH5CFGR1"]
pub mod ch5cfgr1;
#[doc = "CH5CFGR2 register accessor: an alias for `Reg<CH5CFGR2_SPEC>`"]
pub type CH5CFGR2 = crate::Reg<ch5cfgr2::CH5CFGR2_SPEC>;
#[doc = "CH5CFGR2"]
pub mod ch5cfgr2;
#[doc = "CH5AWSCDR register accessor: an alias for `Reg<CH5AWSCDR_SPEC>`"]
pub type CH5AWSCDR = crate::Reg<ch5awscdr::CH5AWSCDR_SPEC>;
#[doc = "CH5AWSCDR"]
pub mod ch5awscdr;
#[doc = "CH5WDATR register accessor: an alias for `Reg<CH5WDATR_SPEC>`"]
pub type CH5WDATR = crate::Reg<ch5wdatr::CH5WDATR_SPEC>;
#[doc = "CH5WDATR"]
pub mod ch5wdatr;
#[doc = "CH5DATINR register accessor: an alias for `Reg<CH5DATINR_SPEC>`"]
pub type CH5DATINR = crate::Reg<ch5datinr::CH5DATINR_SPEC>;
#[doc = "CH5DATINR"]
pub mod ch5datinr;
#[doc = "CH5DLYR register accessor: an alias for `Reg<CH5DLYR_SPEC>`"]
pub type CH5DLYR = crate::Reg<ch5dlyr::CH5DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch5dlyr;
#[doc = "CH6CFGR1 register accessor: an alias for `Reg<CH6CFGR1_SPEC>`"]
pub type CH6CFGR1 = crate::Reg<ch6cfgr1::CH6CFGR1_SPEC>;
#[doc = "CH6CFGR1"]
pub mod ch6cfgr1;
#[doc = "CH6CFGR2 register accessor: an alias for `Reg<CH6CFGR2_SPEC>`"]
pub type CH6CFGR2 = crate::Reg<ch6cfgr2::CH6CFGR2_SPEC>;
#[doc = "CH6CFGR2"]
pub mod ch6cfgr2;
#[doc = "CH6AWSCDR register accessor: an alias for `Reg<CH6AWSCDR_SPEC>`"]
pub type CH6AWSCDR = crate::Reg<ch6awscdr::CH6AWSCDR_SPEC>;
#[doc = "CH6AWSCDR"]
pub mod ch6awscdr;
#[doc = "CH6WDATR register accessor: an alias for `Reg<CH6WDATR_SPEC>`"]
pub type CH6WDATR = crate::Reg<ch6wdatr::CH6WDATR_SPEC>;
#[doc = "CH6WDATR"]
pub mod ch6wdatr;
#[doc = "CH6DATINR register accessor: an alias for `Reg<CH6DATINR_SPEC>`"]
pub type CH6DATINR = crate::Reg<ch6datinr::CH6DATINR_SPEC>;
#[doc = "CH6DATINR"]
pub mod ch6datinr;
#[doc = "CH6DLYR register accessor: an alias for `Reg<CH6DLYR_SPEC>`"]
pub type CH6DLYR = crate::Reg<ch6dlyr::CH6DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch6dlyr;
#[doc = "CH7CFGR1 register accessor: an alias for `Reg<CH7CFGR1_SPEC>`"]
pub type CH7CFGR1 = crate::Reg<ch7cfgr1::CH7CFGR1_SPEC>;
#[doc = "CH7CFGR1"]
pub mod ch7cfgr1;
#[doc = "CH7CFGR2 register accessor: an alias for `Reg<CH7CFGR2_SPEC>`"]
pub type CH7CFGR2 = crate::Reg<ch7cfgr2::CH7CFGR2_SPEC>;
#[doc = "CH7CFGR2"]
pub mod ch7cfgr2;
#[doc = "CH7AWSCDR register accessor: an alias for `Reg<CH7AWSCDR_SPEC>`"]
pub type CH7AWSCDR = crate::Reg<ch7awscdr::CH7AWSCDR_SPEC>;
#[doc = "CH7AWSCDR"]
pub mod ch7awscdr;
#[doc = "CH7WDATR register accessor: an alias for `Reg<CH7WDATR_SPEC>`"]
pub type CH7WDATR = crate::Reg<ch7wdatr::CH7WDATR_SPEC>;
#[doc = "CH7WDATR"]
pub mod ch7wdatr;
#[doc = "CH7DATINR register accessor: an alias for `Reg<CH7DATINR_SPEC>`"]
pub type CH7DATINR = crate::Reg<ch7datinr::CH7DATINR_SPEC>;
#[doc = "CH7DATINR"]
pub mod ch7datinr;
#[doc = "CH7DLYR register accessor: an alias for `Reg<CH7DLYR_SPEC>`"]
pub type CH7DLYR = crate::Reg<ch7dlyr::CH7DLYR_SPEC>;
#[doc = "channel y delay register"]
pub mod ch7dlyr;
#[doc = "DFSDM_FLT0CR1 register accessor: an alias for `Reg<DFSDM_FLT0CR1_SPEC>`"]
pub type DFSDM_FLT0CR1 = crate::Reg<dfsdm_flt0cr1::DFSDM_FLT0CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt0cr1;
#[doc = "DFSDM_FLT0CR2 register accessor: an alias for `Reg<DFSDM_FLT0CR2_SPEC>`"]
pub type DFSDM_FLT0CR2 = crate::Reg<dfsdm_flt0cr2::DFSDM_FLT0CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt0cr2;
#[doc = "DFSDM_FLT0ISR register accessor: an alias for `Reg<DFSDM_FLT0ISR_SPEC>`"]
pub type DFSDM_FLT0ISR = crate::Reg<dfsdm_flt0isr::DFSDM_FLT0ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt0isr;
#[doc = "DFSDM_FLT0ICR register accessor: an alias for `Reg<DFSDM_FLT0ICR_SPEC>`"]
pub type DFSDM_FLT0ICR = crate::Reg<dfsdm_flt0icr::DFSDM_FLT0ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt0icr;
#[doc = "DFSDM_FLT0JCHGR register accessor: an alias for `Reg<DFSDM_FLT0JCHGR_SPEC>`"]
pub type DFSDM_FLT0JCHGR = crate::Reg<dfsdm_flt0jchgr::DFSDM_FLT0JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt0jchgr;
#[doc = "DFSDM_FLT0FCR register accessor: an alias for `Reg<DFSDM_FLT0FCR_SPEC>`"]
pub type DFSDM_FLT0FCR = crate::Reg<dfsdm_flt0fcr::DFSDM_FLT0FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt0fcr;
#[doc = "DFSDM_FLT0JDATAR register accessor: an alias for `Reg<DFSDM_FLT0JDATAR_SPEC>`"]
pub type DFSDM_FLT0JDATAR = crate::Reg<dfsdm_flt0jdatar::DFSDM_FLT0JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt0jdatar;
#[doc = "DFSDM_FLT0RDATAR register accessor: an alias for `Reg<DFSDM_FLT0RDATAR_SPEC>`"]
pub type DFSDM_FLT0RDATAR = crate::Reg<dfsdm_flt0rdatar::DFSDM_FLT0RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt0rdatar;
#[doc = "DFSDM_FLT0AWHTR register accessor: an alias for `Reg<DFSDM_FLT0AWHTR_SPEC>`"]
pub type DFSDM_FLT0AWHTR = crate::Reg<dfsdm_flt0awhtr::DFSDM_FLT0AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt0awhtr;
#[doc = "DFSDM_FLT0AWLTR register accessor: an alias for `Reg<DFSDM_FLT0AWLTR_SPEC>`"]
pub type DFSDM_FLT0AWLTR = crate::Reg<dfsdm_flt0awltr::DFSDM_FLT0AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt0awltr;
#[doc = "DFSDM_FLT0AWSR register accessor: an alias for `Reg<DFSDM_FLT0AWSR_SPEC>`"]
pub type DFSDM_FLT0AWSR = crate::Reg<dfsdm_flt0awsr::DFSDM_FLT0AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt0awsr;
#[doc = "DFSDM_FLT0AWCFR register accessor: an alias for `Reg<DFSDM_FLT0AWCFR_SPEC>`"]
pub type DFSDM_FLT0AWCFR = crate::Reg<dfsdm_flt0awcfr::DFSDM_FLT0AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt0awcfr;
#[doc = "DFSDM_FLT0EXMAX register accessor: an alias for `Reg<DFSDM_FLT0EXMAX_SPEC>`"]
pub type DFSDM_FLT0EXMAX = crate::Reg<dfsdm_flt0exmax::DFSDM_FLT0EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt0exmax;
#[doc = "DFSDM_FLT0EXMIN register accessor: an alias for `Reg<DFSDM_FLT0EXMIN_SPEC>`"]
pub type DFSDM_FLT0EXMIN = crate::Reg<dfsdm_flt0exmin::DFSDM_FLT0EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt0exmin;
#[doc = "DFSDM_FLT0CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT0CNVTIMR_SPEC>`"]
pub type DFSDM_FLT0CNVTIMR = crate::Reg<dfsdm_flt0cnvtimr::DFSDM_FLT0CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt0cnvtimr;
#[doc = "DFSDM_FLT1CR1 register accessor: an alias for `Reg<DFSDM_FLT1CR1_SPEC>`"]
pub type DFSDM_FLT1CR1 = crate::Reg<dfsdm_flt1cr1::DFSDM_FLT1CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt1cr1;
#[doc = "DFSDM_FLT1CR2 register accessor: an alias for `Reg<DFSDM_FLT1CR2_SPEC>`"]
pub type DFSDM_FLT1CR2 = crate::Reg<dfsdm_flt1cr2::DFSDM_FLT1CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt1cr2;
#[doc = "DFSDM_FLT1ISR register accessor: an alias for `Reg<DFSDM_FLT1ISR_SPEC>`"]
pub type DFSDM_FLT1ISR = crate::Reg<dfsdm_flt1isr::DFSDM_FLT1ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt1isr;
#[doc = "DFSDM_FLT1ICR register accessor: an alias for `Reg<DFSDM_FLT1ICR_SPEC>`"]
pub type DFSDM_FLT1ICR = crate::Reg<dfsdm_flt1icr::DFSDM_FLT1ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt1icr;
#[doc = "DFSDM_FLT1CHGR register accessor: an alias for `Reg<DFSDM_FLT1CHGR_SPEC>`"]
pub type DFSDM_FLT1CHGR = crate::Reg<dfsdm_flt1chgr::DFSDM_FLT1CHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt1chgr;
#[doc = "DFSDM_FLT1FCR register accessor: an alias for `Reg<DFSDM_FLT1FCR_SPEC>`"]
pub type DFSDM_FLT1FCR = crate::Reg<dfsdm_flt1fcr::DFSDM_FLT1FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt1fcr;
#[doc = "DFSDM_FLT1JDATAR register accessor: an alias for `Reg<DFSDM_FLT1JDATAR_SPEC>`"]
pub type DFSDM_FLT1JDATAR = crate::Reg<dfsdm_flt1jdatar::DFSDM_FLT1JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt1jdatar;
#[doc = "DFSDM_FLT1RDATAR register accessor: an alias for `Reg<DFSDM_FLT1RDATAR_SPEC>`"]
pub type DFSDM_FLT1RDATAR = crate::Reg<dfsdm_flt1rdatar::DFSDM_FLT1RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt1rdatar;
#[doc = "DFSDM_FLT1AWHTR register accessor: an alias for `Reg<DFSDM_FLT1AWHTR_SPEC>`"]
pub type DFSDM_FLT1AWHTR = crate::Reg<dfsdm_flt1awhtr::DFSDM_FLT1AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt1awhtr;
#[doc = "DFSDM_FLT1AWLTR register accessor: an alias for `Reg<DFSDM_FLT1AWLTR_SPEC>`"]
pub type DFSDM_FLT1AWLTR = crate::Reg<dfsdm_flt1awltr::DFSDM_FLT1AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt1awltr;
#[doc = "DFSDM_FLT1AWSR register accessor: an alias for `Reg<DFSDM_FLT1AWSR_SPEC>`"]
pub type DFSDM_FLT1AWSR = crate::Reg<dfsdm_flt1awsr::DFSDM_FLT1AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt1awsr;
#[doc = "DFSDM_FLT1AWCFR register accessor: an alias for `Reg<DFSDM_FLT1AWCFR_SPEC>`"]
pub type DFSDM_FLT1AWCFR = crate::Reg<dfsdm_flt1awcfr::DFSDM_FLT1AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt1awcfr;
#[doc = "DFSDM_FLT1EXMAX register accessor: an alias for `Reg<DFSDM_FLT1EXMAX_SPEC>`"]
pub type DFSDM_FLT1EXMAX = crate::Reg<dfsdm_flt1exmax::DFSDM_FLT1EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt1exmax;
#[doc = "DFSDM_FLT1EXMIN register accessor: an alias for `Reg<DFSDM_FLT1EXMIN_SPEC>`"]
pub type DFSDM_FLT1EXMIN = crate::Reg<dfsdm_flt1exmin::DFSDM_FLT1EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt1exmin;
#[doc = "DFSDM_FLT1CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT1CNVTIMR_SPEC>`"]
pub type DFSDM_FLT1CNVTIMR = crate::Reg<dfsdm_flt1cnvtimr::DFSDM_FLT1CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt1cnvtimr;
#[doc = "DFSDM_FLT2CR1 register accessor: an alias for `Reg<DFSDM_FLT2CR1_SPEC>`"]
pub type DFSDM_FLT2CR1 = crate::Reg<dfsdm_flt2cr1::DFSDM_FLT2CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt2cr1;
#[doc = "DFSDM_FLT2CR2 register accessor: an alias for `Reg<DFSDM_FLT2CR2_SPEC>`"]
pub type DFSDM_FLT2CR2 = crate::Reg<dfsdm_flt2cr2::DFSDM_FLT2CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt2cr2;
#[doc = "DFSDM_FLT2ISR register accessor: an alias for `Reg<DFSDM_FLT2ISR_SPEC>`"]
pub type DFSDM_FLT2ISR = crate::Reg<dfsdm_flt2isr::DFSDM_FLT2ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt2isr;
#[doc = "DFSDM_FLT2ICR register accessor: an alias for `Reg<DFSDM_FLT2ICR_SPEC>`"]
pub type DFSDM_FLT2ICR = crate::Reg<dfsdm_flt2icr::DFSDM_FLT2ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt2icr;
#[doc = "DFSDM_FLT2JCHGR register accessor: an alias for `Reg<DFSDM_FLT2JCHGR_SPEC>`"]
pub type DFSDM_FLT2JCHGR = crate::Reg<dfsdm_flt2jchgr::DFSDM_FLT2JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt2jchgr;
#[doc = "DFSDM_FLT2FCR register accessor: an alias for `Reg<DFSDM_FLT2FCR_SPEC>`"]
pub type DFSDM_FLT2FCR = crate::Reg<dfsdm_flt2fcr::DFSDM_FLT2FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt2fcr;
#[doc = "DFSDM_FLT2JDATAR register accessor: an alias for `Reg<DFSDM_FLT2JDATAR_SPEC>`"]
pub type DFSDM_FLT2JDATAR = crate::Reg<dfsdm_flt2jdatar::DFSDM_FLT2JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt2jdatar;
#[doc = "DFSDM_FLT2RDATAR register accessor: an alias for `Reg<DFSDM_FLT2RDATAR_SPEC>`"]
pub type DFSDM_FLT2RDATAR = crate::Reg<dfsdm_flt2rdatar::DFSDM_FLT2RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt2rdatar;
#[doc = "DFSDM_FLT2AWHTR register accessor: an alias for `Reg<DFSDM_FLT2AWHTR_SPEC>`"]
pub type DFSDM_FLT2AWHTR = crate::Reg<dfsdm_flt2awhtr::DFSDM_FLT2AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt2awhtr;
#[doc = "DFSDM_FLT2AWLTR register accessor: an alias for `Reg<DFSDM_FLT2AWLTR_SPEC>`"]
pub type DFSDM_FLT2AWLTR = crate::Reg<dfsdm_flt2awltr::DFSDM_FLT2AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt2awltr;
#[doc = "DFSDM_FLT2AWSR register accessor: an alias for `Reg<DFSDM_FLT2AWSR_SPEC>`"]
pub type DFSDM_FLT2AWSR = crate::Reg<dfsdm_flt2awsr::DFSDM_FLT2AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt2awsr;
#[doc = "DFSDM_FLT2AWCFR register accessor: an alias for `Reg<DFSDM_FLT2AWCFR_SPEC>`"]
pub type DFSDM_FLT2AWCFR = crate::Reg<dfsdm_flt2awcfr::DFSDM_FLT2AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt2awcfr;
#[doc = "DFSDM_FLT2EXMAX register accessor: an alias for `Reg<DFSDM_FLT2EXMAX_SPEC>`"]
pub type DFSDM_FLT2EXMAX = crate::Reg<dfsdm_flt2exmax::DFSDM_FLT2EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt2exmax;
#[doc = "DFSDM_FLT2EXMIN register accessor: an alias for `Reg<DFSDM_FLT2EXMIN_SPEC>`"]
pub type DFSDM_FLT2EXMIN = crate::Reg<dfsdm_flt2exmin::DFSDM_FLT2EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt2exmin;
#[doc = "DFSDM_FLT2CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT2CNVTIMR_SPEC>`"]
pub type DFSDM_FLT2CNVTIMR = crate::Reg<dfsdm_flt2cnvtimr::DFSDM_FLT2CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt2cnvtimr;
#[doc = "DFSDM_FLT3CR1 register accessor: an alias for `Reg<DFSDM_FLT3CR1_SPEC>`"]
pub type DFSDM_FLT3CR1 = crate::Reg<dfsdm_flt3cr1::DFSDM_FLT3CR1_SPEC>;
#[doc = "control register 1"]
pub mod dfsdm_flt3cr1;
#[doc = "DFSDM_FLT3CR2 register accessor: an alias for `Reg<DFSDM_FLT3CR2_SPEC>`"]
pub type DFSDM_FLT3CR2 = crate::Reg<dfsdm_flt3cr2::DFSDM_FLT3CR2_SPEC>;
#[doc = "control register 2"]
pub mod dfsdm_flt3cr2;
#[doc = "DFSDM_FLT3ISR register accessor: an alias for `Reg<DFSDM_FLT3ISR_SPEC>`"]
pub type DFSDM_FLT3ISR = crate::Reg<dfsdm_flt3isr::DFSDM_FLT3ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod dfsdm_flt3isr;
#[doc = "DFSDM_FLT3ICR register accessor: an alias for `Reg<DFSDM_FLT3ICR_SPEC>`"]
pub type DFSDM_FLT3ICR = crate::Reg<dfsdm_flt3icr::DFSDM_FLT3ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod dfsdm_flt3icr;
#[doc = "DFSDM_FLT3JCHGR register accessor: an alias for `Reg<DFSDM_FLT3JCHGR_SPEC>`"]
pub type DFSDM_FLT3JCHGR = crate::Reg<dfsdm_flt3jchgr::DFSDM_FLT3JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod dfsdm_flt3jchgr;
#[doc = "DFSDM_FLT3FCR register accessor: an alias for `Reg<DFSDM_FLT3FCR_SPEC>`"]
pub type DFSDM_FLT3FCR = crate::Reg<dfsdm_flt3fcr::DFSDM_FLT3FCR_SPEC>;
#[doc = "filter control register"]
pub mod dfsdm_flt3fcr;
#[doc = "DFSDM_FLT3JDATAR register accessor: an alias for `Reg<DFSDM_FLT3JDATAR_SPEC>`"]
pub type DFSDM_FLT3JDATAR = crate::Reg<dfsdm_flt3jdatar::DFSDM_FLT3JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod dfsdm_flt3jdatar;
#[doc = "DFSDM_FLT3RDATAR register accessor: an alias for `Reg<DFSDM_FLT3RDATAR_SPEC>`"]
pub type DFSDM_FLT3RDATAR = crate::Reg<dfsdm_flt3rdatar::DFSDM_FLT3RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod dfsdm_flt3rdatar;
#[doc = "DFSDM_FLT3AWHTR register accessor: an alias for `Reg<DFSDM_FLT3AWHTR_SPEC>`"]
pub type DFSDM_FLT3AWHTR = crate::Reg<dfsdm_flt3awhtr::DFSDM_FLT3AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm_flt3awhtr;
#[doc = "DFSDM_FLT3AWLTR register accessor: an alias for `Reg<DFSDM_FLT3AWLTR_SPEC>`"]
pub type DFSDM_FLT3AWLTR = crate::Reg<dfsdm_flt3awltr::DFSDM_FLT3AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm_flt3awltr;
#[doc = "DFSDM_FLT3AWSR register accessor: an alias for `Reg<DFSDM_FLT3AWSR_SPEC>`"]
pub type DFSDM_FLT3AWSR = crate::Reg<dfsdm_flt3awsr::DFSDM_FLT3AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod dfsdm_flt3awsr;
#[doc = "DFSDM_FLT3AWCFR register accessor: an alias for `Reg<DFSDM_FLT3AWCFR_SPEC>`"]
pub type DFSDM_FLT3AWCFR = crate::Reg<dfsdm_flt3awcfr::DFSDM_FLT3AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm_flt3awcfr;
#[doc = "DFSDM_FLT3EXMAX register accessor: an alias for `Reg<DFSDM_FLT3EXMAX_SPEC>`"]
pub type DFSDM_FLT3EXMAX = crate::Reg<dfsdm_flt3exmax::DFSDM_FLT3EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod dfsdm_flt3exmax;
#[doc = "DFSDM_FLT3EXMIN register accessor: an alias for `Reg<DFSDM_FLT3EXMIN_SPEC>`"]
pub type DFSDM_FLT3EXMIN = crate::Reg<dfsdm_flt3exmin::DFSDM_FLT3EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod dfsdm_flt3exmin;
#[doc = "DFSDM_FLT3CNVTIMR register accessor: an alias for `Reg<DFSDM_FLT3CNVTIMR_SPEC>`"]
pub type DFSDM_FLT3CNVTIMR = crate::Reg<dfsdm_flt3cnvtimr::DFSDM_FLT3CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod dfsdm_flt3cnvtimr;
