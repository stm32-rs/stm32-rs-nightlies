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
    #[doc = "0x14 - DFSDM channel y delay register"]
    pub ch0dlyr: crate::Reg<ch0dlyr::CH0DLYR_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - CHCFG1R1"]
    pub ch1cfgr1: crate::Reg<ch1cfgr1::CH1CFGR1_SPEC>,
    #[doc = "0x24 - CHCFG1R2"]
    pub ch1cfgr2: crate::Reg<ch1cfgr2::CH1CFGR2_SPEC>,
    #[doc = "0x28 - AWSCD1R"]
    pub ch1awscdr: crate::Reg<ch1awscdr::CH1AWSCDR_SPEC>,
    #[doc = "0x2c - CHWDAT1R"]
    pub ch1wdatr: crate::Reg<ch1wdatr::CH1WDATR_SPEC>,
    #[doc = "0x30 - CHDATIN1R"]
    pub ch1datinr: crate::Reg<ch1datinr::CH1DATINR_SPEC>,
    #[doc = "0x34 - DFSDM channel y delay register"]
    pub ch1dlyr: crate::Reg<ch1dlyr::CH1DLYR_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - CHCFG2R1"]
    pub ch2cfgr1: crate::Reg<ch2cfgr1::CH2CFGR1_SPEC>,
    #[doc = "0x44 - CHCFG2R2"]
    pub ch2cfgr2: crate::Reg<ch2cfgr2::CH2CFGR2_SPEC>,
    #[doc = "0x48 - AWSCD2R"]
    pub ch2awscdr: crate::Reg<ch2awscdr::CH2AWSCDR_SPEC>,
    #[doc = "0x4c - CHWDAT2R"]
    pub ch2wdatr: crate::Reg<ch2wdatr::CH2WDATR_SPEC>,
    #[doc = "0x50 - CHDATIN2R"]
    pub ch2datinr: crate::Reg<ch2datinr::CH2DATINR_SPEC>,
    #[doc = "0x54 - DFSDM channel y delay register"]
    pub ch2dlyr: crate::Reg<ch2dlyr::CH2DLYR_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x60 - CHCFG3R1"]
    pub ch3cfgr1: crate::Reg<ch3cfgr1::CH3CFGR1_SPEC>,
    #[doc = "0x64 - CHCFG3R2"]
    pub ch3cfgr2: crate::Reg<ch3cfgr2::CH3CFGR2_SPEC>,
    #[doc = "0x68 - AWSCD3R"]
    pub ch3awscdr: crate::Reg<ch3awscdr::CH3AWSCDR_SPEC>,
    #[doc = "0x6c - CHWDAT3R"]
    pub ch3wdatr: crate::Reg<ch3wdatr::CH3WDATR_SPEC>,
    #[doc = "0x70 - CHDATIN3R"]
    pub ch3datinr: crate::Reg<ch3datinr::CH3DATINR_SPEC>,
    #[doc = "0x74 - DFSDM channel y delay register"]
    pub ch3dlyr: crate::Reg<ch3dlyr::CH3DLYR_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x80 - CHCFG4R1"]
    pub ch4cfgr1: crate::Reg<ch4cfgr1::CH4CFGR1_SPEC>,
    #[doc = "0x84 - CHCFG4R2"]
    pub ch4cfgr2: crate::Reg<ch4cfgr2::CH4CFGR2_SPEC>,
    #[doc = "0x88 - AWSCD4R"]
    pub ch4awscdr: crate::Reg<ch4awscdr::CH4AWSCDR_SPEC>,
    #[doc = "0x8c - CHWDAT4R"]
    pub ch4wdatr: crate::Reg<ch4wdatr::CH4WDATR_SPEC>,
    #[doc = "0x90 - CHDATIN4R"]
    pub ch4datinr: crate::Reg<ch4datinr::CH4DATINR_SPEC>,
    #[doc = "0x94 - DFSDM channel y delay register"]
    pub ch4dlyr: crate::Reg<ch4dlyr::CH4DLYR_SPEC>,
    _reserved30: [u8; 0x08],
    #[doc = "0xa0 - CHCFG5R1"]
    pub ch5cfgr1: crate::Reg<ch5cfgr1::CH5CFGR1_SPEC>,
    #[doc = "0xa4 - CHCFG5R2"]
    pub ch5cfgr2: crate::Reg<ch5cfgr2::CH5CFGR2_SPEC>,
    #[doc = "0xa8 - AWSCD5R"]
    pub ch5awscdr: crate::Reg<ch5awscdr::CH5AWSCDR_SPEC>,
    #[doc = "0xac - CHWDAT5R"]
    pub ch5wdatr: crate::Reg<ch5wdatr::CH5WDATR_SPEC>,
    #[doc = "0xb0 - CHDATIN5R"]
    pub ch5datinr: crate::Reg<ch5datinr::CH5DATINR_SPEC>,
    #[doc = "0xb4 - DFSDM channel y delay register"]
    pub ch5dlyr: crate::Reg<ch5dlyr::CH5DLYR_SPEC>,
    _reserved36: [u8; 0x08],
    #[doc = "0xc0 - CHCFG6R1"]
    pub ch6cfgr1: crate::Reg<ch6cfgr1::CH6CFGR1_SPEC>,
    #[doc = "0xc4 - CH6CFGR2"]
    pub ch6cfgr2: crate::Reg<ch6cfgr2::CH6CFGR2_SPEC>,
    #[doc = "0xc8 - AWSCD6R"]
    pub ch6awscdr: crate::Reg<ch6awscdr::CH6AWSCDR_SPEC>,
    #[doc = "0xcc - CHWDAT6R"]
    pub ch6wdatr: crate::Reg<ch6wdatr::CH6WDATR_SPEC>,
    #[doc = "0xd0 - CHDATIN6R"]
    pub ch6datinr: crate::Reg<ch6datinr::CH6DATINR_SPEC>,
    #[doc = "0xd4 - DFSDM channel y delay register"]
    pub ch6dlyr: crate::Reg<ch6dlyr::CH6DLYR_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0xe0 - CHCFG7R1"]
    pub ch7cfgr1: crate::Reg<ch7cfgr1::CH7CFGR1_SPEC>,
    #[doc = "0xe4 - CHCFG7R2"]
    pub ch7cfgr2: crate::Reg<ch7cfgr2::CH7CFGR2_SPEC>,
    #[doc = "0xe8 - AWSCD7R"]
    pub ch7awscdr: crate::Reg<ch7awscdr::CH7AWSCDR_SPEC>,
    #[doc = "0xec - CHWDAT7R"]
    pub ch7wdatr: crate::Reg<ch7wdatr::CH7WDATR_SPEC>,
    #[doc = "0xf0 - CHDATIN7R"]
    pub ch7datinr: crate::Reg<ch7datinr::CH7DATINR_SPEC>,
    #[doc = "0xf4 - DFSDM channel y delay register"]
    pub ch7dlyr: crate::Reg<ch7dlyr::CH7DLYR_SPEC>,
    _reserved48: [u8; 0x08],
    #[doc = "0x100 - control register 1"]
    pub flt0cr1: crate::Reg<flt0cr1::FLT0CR1_SPEC>,
    #[doc = "0x104 - control register 2"]
    pub flt0cr2: crate::Reg<flt0cr2::FLT0CR2_SPEC>,
    #[doc = "0x108 - interrupt and status register"]
    pub flt0isr: crate::Reg<flt0isr::FLT0ISR_SPEC>,
    #[doc = "0x10c - interrupt flag clear register"]
    pub flt0icr: crate::Reg<flt0icr::FLT0ICR_SPEC>,
    #[doc = "0x110 - injected channel group selection register"]
    pub flt0jchgr: crate::Reg<flt0jchgr::FLT0JCHGR_SPEC>,
    #[doc = "0x114 - filter control register"]
    pub flt0fcr: crate::Reg<flt0fcr::FLT0FCR_SPEC>,
    #[doc = "0x118 - data register for injected group"]
    pub flt0jdatar: crate::Reg<flt0jdatar::FLT0JDATAR_SPEC>,
    #[doc = "0x11c - data register for the regular channel"]
    pub flt0rdatar: crate::Reg<flt0rdatar::FLT0RDATAR_SPEC>,
    #[doc = "0x120 - analog watchdog high threshold register"]
    pub flt0awhtr: crate::Reg<flt0awhtr::FLT0AWHTR_SPEC>,
    #[doc = "0x124 - analog watchdog low threshold register"]
    pub flt0awltr: crate::Reg<flt0awltr::FLT0AWLTR_SPEC>,
    #[doc = "0x128 - analog watchdog status register"]
    pub flt0awsr: crate::Reg<flt0awsr::FLT0AWSR_SPEC>,
    #[doc = "0x12c - analog watchdog clear flag register"]
    pub flt0awcfr: crate::Reg<flt0awcfr::FLT0AWCFR_SPEC>,
    #[doc = "0x130 - Extremes detector maximum register"]
    pub flt0exmax: crate::Reg<flt0exmax::FLT0EXMAX_SPEC>,
    #[doc = "0x134 - Extremes detector minimum register"]
    pub flt0exmin: crate::Reg<flt0exmin::FLT0EXMIN_SPEC>,
    #[doc = "0x138 - conversion timer register"]
    pub flt0cnvtimr: crate::Reg<flt0cnvtimr::FLT0CNVTIMR_SPEC>,
    _reserved63: [u8; 0x44],
    #[doc = "0x180 - control register 1"]
    pub flt1cr1: crate::Reg<flt1cr1::FLT1CR1_SPEC>,
    #[doc = "0x184 - control register 2"]
    pub flt1cr2: crate::Reg<flt1cr2::FLT1CR2_SPEC>,
    #[doc = "0x188 - interrupt and status register"]
    pub flt1isr: crate::Reg<flt1isr::FLT1ISR_SPEC>,
    #[doc = "0x18c - interrupt flag clear register"]
    pub flt1icr: crate::Reg<flt1icr::FLT1ICR_SPEC>,
    #[doc = "0x190 - injected channel group selection register"]
    pub flt1jchgr: crate::Reg<flt1jchgr::FLT1JCHGR_SPEC>,
    #[doc = "0x194 - filter control register"]
    pub flt1fcr: crate::Reg<flt1fcr::FLT1FCR_SPEC>,
    #[doc = "0x198 - data register for injected group"]
    pub flt1jdatar: crate::Reg<flt1jdatar::FLT1JDATAR_SPEC>,
    #[doc = "0x19c - data register for the regular channel"]
    pub flt1rdatar: crate::Reg<flt1rdatar::FLT1RDATAR_SPEC>,
    _reserved71: [u8; 0x04],
    #[doc = "0x1a4 - analog watchdog low threshold register"]
    pub flt1awltr: crate::Reg<flt1awltr::FLT1AWLTR_SPEC>,
    #[doc = "0x1a8 - analog watchdog status register"]
    pub flt1awsr: crate::Reg<flt1awsr::FLT1AWSR_SPEC>,
    _reserved_73_flt: [u8; 0x04],
    #[doc = "0x1b0 - Extremes detector maximum register"]
    pub flt1exmax: crate::Reg<flt1exmax::FLT1EXMAX_SPEC>,
    #[doc = "0x1b4 - Extremes detector minimum register"]
    pub flt1exmin: crate::Reg<flt1exmin::FLT1EXMIN_SPEC>,
    #[doc = "0x1b8 - conversion timer register"]
    pub flt1cnvtimr: crate::Reg<flt1cnvtimr::FLT1CNVTIMR_SPEC>,
    _reserved77: [u8; 0x44],
    #[doc = "0x200 - control register 1"]
    pub flt2cr1: crate::Reg<flt2cr1::FLT2CR1_SPEC>,
    #[doc = "0x204 - control register 2"]
    pub flt2cr2: crate::Reg<flt2cr2::FLT2CR2_SPEC>,
    #[doc = "0x208 - interrupt and status register"]
    pub flt2isr: crate::Reg<flt2isr::FLT2ISR_SPEC>,
    #[doc = "0x20c - interrupt flag clear register"]
    pub flt2icr: crate::Reg<flt2icr::FLT2ICR_SPEC>,
    #[doc = "0x210 - injected channel group selection register"]
    pub flt2jchgr: crate::Reg<flt2jchgr::FLT2JCHGR_SPEC>,
    #[doc = "0x214 - filter control register"]
    pub flt2fcr: crate::Reg<flt2fcr::FLT2FCR_SPEC>,
    #[doc = "0x218 - data register for injected group"]
    pub flt2jdatar: crate::Reg<flt2jdatar::FLT2JDATAR_SPEC>,
    #[doc = "0x21c - data register for the regular channel"]
    pub flt2rdatar: crate::Reg<flt2rdatar::FLT2RDATAR_SPEC>,
    #[doc = "0x220 - analog watchdog high threshold register"]
    pub flt2awhtr: crate::Reg<flt2awhtr::FLT2AWHTR_SPEC>,
    #[doc = "0x224 - analog watchdog low threshold register"]
    pub flt2awltr: crate::Reg<flt2awltr::FLT2AWLTR_SPEC>,
    #[doc = "0x228 - analog watchdog status register"]
    pub flt2awsr: crate::Reg<flt2awsr::FLT2AWSR_SPEC>,
    #[doc = "0x22c - analog watchdog clear flag register"]
    pub flt2awcfr: crate::Reg<flt2awcfr::FLT2AWCFR_SPEC>,
    #[doc = "0x230 - Extremes detector maximum register"]
    pub flt2exmax: crate::Reg<flt2exmax::FLT2EXMAX_SPEC>,
    #[doc = "0x234 - Extremes detector minimum register"]
    pub flt2exmin: crate::Reg<flt2exmin::FLT2EXMIN_SPEC>,
    #[doc = "0x238 - conversion timer register"]
    pub flt2cnvtimr: crate::Reg<flt2cnvtimr::FLT2CNVTIMR_SPEC>,
    _reserved92: [u8; 0x44],
    #[doc = "0x280 - control register 1"]
    pub flt3cr1: crate::Reg<flt3cr1::FLT3CR1_SPEC>,
    #[doc = "0x284 - control register 2"]
    pub flt3cr2: crate::Reg<flt3cr2::FLT3CR2_SPEC>,
    #[doc = "0x288 - interrupt and status register"]
    pub flt3isr: crate::Reg<flt3isr::FLT3ISR_SPEC>,
    #[doc = "0x28c - interrupt flag clear register"]
    pub flt3icr: crate::Reg<flt3icr::FLT3ICR_SPEC>,
    #[doc = "0x290 - injected channel group selection register"]
    pub flt3jchgr: crate::Reg<flt3jchgr::FLT3JCHGR_SPEC>,
    #[doc = "0x294 - filter control register"]
    pub flt3fcr: crate::Reg<flt3fcr::FLT3FCR_SPEC>,
    #[doc = "0x298 - data register for injected group"]
    pub flt3jdatar: crate::Reg<flt3jdatar::FLT3JDATAR_SPEC>,
    #[doc = "0x29c - data register for the regular channel"]
    pub flt3rdatar: crate::Reg<flt3rdatar::FLT3RDATAR_SPEC>,
    #[doc = "0x2a0 - analog watchdog high threshold register"]
    pub flt3awhtr: crate::Reg<flt3awhtr::FLT3AWHTR_SPEC>,
    #[doc = "0x2a4 - analog watchdog low threshold register"]
    pub flt3awltr: crate::Reg<flt3awltr::FLT3AWLTR_SPEC>,
    #[doc = "0x2a8 - analog watchdog status register"]
    pub flt3awsr: crate::Reg<flt3awsr::FLT3AWSR_SPEC>,
    #[doc = "0x2ac - analog watchdog clear flag register"]
    pub flt3awcfr: crate::Reg<flt3awcfr::FLT3AWCFR_SPEC>,
    #[doc = "0x2b0 - Extremes detector maximum register"]
    pub flt3exmax: crate::Reg<flt3exmax::FLT3EXMAX_SPEC>,
    #[doc = "0x2b4 - Extremes detector minimum register"]
    pub flt3exmin: crate::Reg<flt3exmin::FLT3EXMIN_SPEC>,
    #[doc = "0x2b8 - conversion timer register"]
    pub flt3cnvtimr: crate::Reg<flt3cnvtimr::FLT3CNVTIMR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x1ac - analog watchdog clear flag register"]
    #[inline(always)]
    pub fn flt1awcfr(&self) -> &crate::Reg<flt1awcfr::FLT1AWCFR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(428usize)
                as *const crate::Reg<flt1awcfr::FLT1AWCFR_SPEC>)
        }
    }
    #[doc = "0x1ac - analog watchdog high threshold register"]
    #[inline(always)]
    pub fn flt1awhtr(&self) -> &crate::Reg<flt1awhtr::FLT1AWHTR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(428usize)
                as *const crate::Reg<flt1awhtr::FLT1AWHTR_SPEC>)
        }
    }
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
#[doc = "CH1CFGR1 register accessor: an alias for `Reg<CH1CFGR1_SPEC>`"]
pub type CH1CFGR1 = crate::Reg<ch1cfgr1::CH1CFGR1_SPEC>;
#[doc = "CHCFG1R1"]
pub mod ch1cfgr1;
#[doc = "CH1CFGR2 register accessor: an alias for `Reg<CH1CFGR2_SPEC>`"]
pub type CH1CFGR2 = crate::Reg<ch1cfgr2::CH1CFGR2_SPEC>;
#[doc = "CHCFG1R2"]
pub mod ch1cfgr2;
#[doc = "CH1AWSCDR register accessor: an alias for `Reg<CH1AWSCDR_SPEC>`"]
pub type CH1AWSCDR = crate::Reg<ch1awscdr::CH1AWSCDR_SPEC>;
#[doc = "AWSCD1R"]
pub mod ch1awscdr;
#[doc = "CH1WDATR register accessor: an alias for `Reg<CH1WDATR_SPEC>`"]
pub type CH1WDATR = crate::Reg<ch1wdatr::CH1WDATR_SPEC>;
#[doc = "CHWDAT1R"]
pub mod ch1wdatr;
#[doc = "CH1DATINR register accessor: an alias for `Reg<CH1DATINR_SPEC>`"]
pub type CH1DATINR = crate::Reg<ch1datinr::CH1DATINR_SPEC>;
#[doc = "CHDATIN1R"]
pub mod ch1datinr;
#[doc = "CH2CFGR1 register accessor: an alias for `Reg<CH2CFGR1_SPEC>`"]
pub type CH2CFGR1 = crate::Reg<ch2cfgr1::CH2CFGR1_SPEC>;
#[doc = "CHCFG2R1"]
pub mod ch2cfgr1;
#[doc = "CH2CFGR2 register accessor: an alias for `Reg<CH2CFGR2_SPEC>`"]
pub type CH2CFGR2 = crate::Reg<ch2cfgr2::CH2CFGR2_SPEC>;
#[doc = "CHCFG2R2"]
pub mod ch2cfgr2;
#[doc = "CH2AWSCDR register accessor: an alias for `Reg<CH2AWSCDR_SPEC>`"]
pub type CH2AWSCDR = crate::Reg<ch2awscdr::CH2AWSCDR_SPEC>;
#[doc = "AWSCD2R"]
pub mod ch2awscdr;
#[doc = "CH2WDATR register accessor: an alias for `Reg<CH2WDATR_SPEC>`"]
pub type CH2WDATR = crate::Reg<ch2wdatr::CH2WDATR_SPEC>;
#[doc = "CHWDAT2R"]
pub mod ch2wdatr;
#[doc = "CH2DATINR register accessor: an alias for `Reg<CH2DATINR_SPEC>`"]
pub type CH2DATINR = crate::Reg<ch2datinr::CH2DATINR_SPEC>;
#[doc = "CHDATIN2R"]
pub mod ch2datinr;
#[doc = "CH3CFGR1 register accessor: an alias for `Reg<CH3CFGR1_SPEC>`"]
pub type CH3CFGR1 = crate::Reg<ch3cfgr1::CH3CFGR1_SPEC>;
#[doc = "CHCFG3R1"]
pub mod ch3cfgr1;
#[doc = "CH3CFGR2 register accessor: an alias for `Reg<CH3CFGR2_SPEC>`"]
pub type CH3CFGR2 = crate::Reg<ch3cfgr2::CH3CFGR2_SPEC>;
#[doc = "CHCFG3R2"]
pub mod ch3cfgr2;
#[doc = "CH3AWSCDR register accessor: an alias for `Reg<CH3AWSCDR_SPEC>`"]
pub type CH3AWSCDR = crate::Reg<ch3awscdr::CH3AWSCDR_SPEC>;
#[doc = "AWSCD3R"]
pub mod ch3awscdr;
#[doc = "CH3WDATR register accessor: an alias for `Reg<CH3WDATR_SPEC>`"]
pub type CH3WDATR = crate::Reg<ch3wdatr::CH3WDATR_SPEC>;
#[doc = "CHWDAT3R"]
pub mod ch3wdatr;
#[doc = "CH3DATINR register accessor: an alias for `Reg<CH3DATINR_SPEC>`"]
pub type CH3DATINR = crate::Reg<ch3datinr::CH3DATINR_SPEC>;
#[doc = "CHDATIN3R"]
pub mod ch3datinr;
#[doc = "CH4CFGR1 register accessor: an alias for `Reg<CH4CFGR1_SPEC>`"]
pub type CH4CFGR1 = crate::Reg<ch4cfgr1::CH4CFGR1_SPEC>;
#[doc = "CHCFG4R1"]
pub mod ch4cfgr1;
#[doc = "CH4CFGR2 register accessor: an alias for `Reg<CH4CFGR2_SPEC>`"]
pub type CH4CFGR2 = crate::Reg<ch4cfgr2::CH4CFGR2_SPEC>;
#[doc = "CHCFG4R2"]
pub mod ch4cfgr2;
#[doc = "CH4AWSCDR register accessor: an alias for `Reg<CH4AWSCDR_SPEC>`"]
pub type CH4AWSCDR = crate::Reg<ch4awscdr::CH4AWSCDR_SPEC>;
#[doc = "AWSCD4R"]
pub mod ch4awscdr;
#[doc = "CH4WDATR register accessor: an alias for `Reg<CH4WDATR_SPEC>`"]
pub type CH4WDATR = crate::Reg<ch4wdatr::CH4WDATR_SPEC>;
#[doc = "CHWDAT4R"]
pub mod ch4wdatr;
#[doc = "CH4DATINR register accessor: an alias for `Reg<CH4DATINR_SPEC>`"]
pub type CH4DATINR = crate::Reg<ch4datinr::CH4DATINR_SPEC>;
#[doc = "CHDATIN4R"]
pub mod ch4datinr;
#[doc = "CH5CFGR1 register accessor: an alias for `Reg<CH5CFGR1_SPEC>`"]
pub type CH5CFGR1 = crate::Reg<ch5cfgr1::CH5CFGR1_SPEC>;
#[doc = "CHCFG5R1"]
pub mod ch5cfgr1;
#[doc = "CH5CFGR2 register accessor: an alias for `Reg<CH5CFGR2_SPEC>`"]
pub type CH5CFGR2 = crate::Reg<ch5cfgr2::CH5CFGR2_SPEC>;
#[doc = "CHCFG5R2"]
pub mod ch5cfgr2;
#[doc = "CH5AWSCDR register accessor: an alias for `Reg<CH5AWSCDR_SPEC>`"]
pub type CH5AWSCDR = crate::Reg<ch5awscdr::CH5AWSCDR_SPEC>;
#[doc = "AWSCD5R"]
pub mod ch5awscdr;
#[doc = "CH5WDATR register accessor: an alias for `Reg<CH5WDATR_SPEC>`"]
pub type CH5WDATR = crate::Reg<ch5wdatr::CH5WDATR_SPEC>;
#[doc = "CHWDAT5R"]
pub mod ch5wdatr;
#[doc = "CH5DATINR register accessor: an alias for `Reg<CH5DATINR_SPEC>`"]
pub type CH5DATINR = crate::Reg<ch5datinr::CH5DATINR_SPEC>;
#[doc = "CHDATIN5R"]
pub mod ch5datinr;
#[doc = "CH6CFGR1 register accessor: an alias for `Reg<CH6CFGR1_SPEC>`"]
pub type CH6CFGR1 = crate::Reg<ch6cfgr1::CH6CFGR1_SPEC>;
#[doc = "CHCFG6R1"]
pub mod ch6cfgr1;
#[doc = "CH6CFGR2 register accessor: an alias for `Reg<CH6CFGR2_SPEC>`"]
pub type CH6CFGR2 = crate::Reg<ch6cfgr2::CH6CFGR2_SPEC>;
#[doc = "CH6CFGR2"]
pub mod ch6cfgr2;
#[doc = "CH6AWSCDR register accessor: an alias for `Reg<CH6AWSCDR_SPEC>`"]
pub type CH6AWSCDR = crate::Reg<ch6awscdr::CH6AWSCDR_SPEC>;
#[doc = "AWSCD6R"]
pub mod ch6awscdr;
#[doc = "CH6WDATR register accessor: an alias for `Reg<CH6WDATR_SPEC>`"]
pub type CH6WDATR = crate::Reg<ch6wdatr::CH6WDATR_SPEC>;
#[doc = "CHWDAT6R"]
pub mod ch6wdatr;
#[doc = "CH6DATINR register accessor: an alias for `Reg<CH6DATINR_SPEC>`"]
pub type CH6DATINR = crate::Reg<ch6datinr::CH6DATINR_SPEC>;
#[doc = "CHDATIN6R"]
pub mod ch6datinr;
#[doc = "CH7CFGR1 register accessor: an alias for `Reg<CH7CFGR1_SPEC>`"]
pub type CH7CFGR1 = crate::Reg<ch7cfgr1::CH7CFGR1_SPEC>;
#[doc = "CHCFG7R1"]
pub mod ch7cfgr1;
#[doc = "CH7CFGR2 register accessor: an alias for `Reg<CH7CFGR2_SPEC>`"]
pub type CH7CFGR2 = crate::Reg<ch7cfgr2::CH7CFGR2_SPEC>;
#[doc = "CHCFG7R2"]
pub mod ch7cfgr2;
#[doc = "CH7AWSCDR register accessor: an alias for `Reg<CH7AWSCDR_SPEC>`"]
pub type CH7AWSCDR = crate::Reg<ch7awscdr::CH7AWSCDR_SPEC>;
#[doc = "AWSCD7R"]
pub mod ch7awscdr;
#[doc = "CH7WDATR register accessor: an alias for `Reg<CH7WDATR_SPEC>`"]
pub type CH7WDATR = crate::Reg<ch7wdatr::CH7WDATR_SPEC>;
#[doc = "CHWDAT7R"]
pub mod ch7wdatr;
#[doc = "CH7DATINR register accessor: an alias for `Reg<CH7DATINR_SPEC>`"]
pub type CH7DATINR = crate::Reg<ch7datinr::CH7DATINR_SPEC>;
#[doc = "CHDATIN7R"]
pub mod ch7datinr;
#[doc = "FLT0CR1 register accessor: an alias for `Reg<FLT0CR1_SPEC>`"]
pub type FLT0CR1 = crate::Reg<flt0cr1::FLT0CR1_SPEC>;
#[doc = "control register 1"]
pub mod flt0cr1;
#[doc = "FLT0CR2 register accessor: an alias for `Reg<FLT0CR2_SPEC>`"]
pub type FLT0CR2 = crate::Reg<flt0cr2::FLT0CR2_SPEC>;
#[doc = "control register 2"]
pub mod flt0cr2;
#[doc = "FLT0ISR register accessor: an alias for `Reg<FLT0ISR_SPEC>`"]
pub type FLT0ISR = crate::Reg<flt0isr::FLT0ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod flt0isr;
#[doc = "FLT0ICR register accessor: an alias for `Reg<FLT0ICR_SPEC>`"]
pub type FLT0ICR = crate::Reg<flt0icr::FLT0ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod flt0icr;
#[doc = "FLT0JCHGR register accessor: an alias for `Reg<FLT0JCHGR_SPEC>`"]
pub type FLT0JCHGR = crate::Reg<flt0jchgr::FLT0JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod flt0jchgr;
#[doc = "FLT0FCR register accessor: an alias for `Reg<FLT0FCR_SPEC>`"]
pub type FLT0FCR = crate::Reg<flt0fcr::FLT0FCR_SPEC>;
#[doc = "filter control register"]
pub mod flt0fcr;
#[doc = "FLT0JDATAR register accessor: an alias for `Reg<FLT0JDATAR_SPEC>`"]
pub type FLT0JDATAR = crate::Reg<flt0jdatar::FLT0JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod flt0jdatar;
#[doc = "FLT0RDATAR register accessor: an alias for `Reg<FLT0RDATAR_SPEC>`"]
pub type FLT0RDATAR = crate::Reg<flt0rdatar::FLT0RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod flt0rdatar;
#[doc = "FLT0AWHTR register accessor: an alias for `Reg<FLT0AWHTR_SPEC>`"]
pub type FLT0AWHTR = crate::Reg<flt0awhtr::FLT0AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod flt0awhtr;
#[doc = "FLT0AWLTR register accessor: an alias for `Reg<FLT0AWLTR_SPEC>`"]
pub type FLT0AWLTR = crate::Reg<flt0awltr::FLT0AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod flt0awltr;
#[doc = "FLT0AWSR register accessor: an alias for `Reg<FLT0AWSR_SPEC>`"]
pub type FLT0AWSR = crate::Reg<flt0awsr::FLT0AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod flt0awsr;
#[doc = "FLT0AWCFR register accessor: an alias for `Reg<FLT0AWCFR_SPEC>`"]
pub type FLT0AWCFR = crate::Reg<flt0awcfr::FLT0AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod flt0awcfr;
#[doc = "FLT0EXMAX register accessor: an alias for `Reg<FLT0EXMAX_SPEC>`"]
pub type FLT0EXMAX = crate::Reg<flt0exmax::FLT0EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod flt0exmax;
#[doc = "FLT0EXMIN register accessor: an alias for `Reg<FLT0EXMIN_SPEC>`"]
pub type FLT0EXMIN = crate::Reg<flt0exmin::FLT0EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod flt0exmin;
#[doc = "FLT0CNVTIMR register accessor: an alias for `Reg<FLT0CNVTIMR_SPEC>`"]
pub type FLT0CNVTIMR = crate::Reg<flt0cnvtimr::FLT0CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod flt0cnvtimr;
#[doc = "FLT1CR1 register accessor: an alias for `Reg<FLT1CR1_SPEC>`"]
pub type FLT1CR1 = crate::Reg<flt1cr1::FLT1CR1_SPEC>;
#[doc = "control register 1"]
pub mod flt1cr1;
#[doc = "FLT1CR2 register accessor: an alias for `Reg<FLT1CR2_SPEC>`"]
pub type FLT1CR2 = crate::Reg<flt1cr2::FLT1CR2_SPEC>;
#[doc = "control register 2"]
pub mod flt1cr2;
#[doc = "FLT1ISR register accessor: an alias for `Reg<FLT1ISR_SPEC>`"]
pub type FLT1ISR = crate::Reg<flt1isr::FLT1ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod flt1isr;
#[doc = "FLT1ICR register accessor: an alias for `Reg<FLT1ICR_SPEC>`"]
pub type FLT1ICR = crate::Reg<flt1icr::FLT1ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod flt1icr;
#[doc = "FLT1JCHGR register accessor: an alias for `Reg<FLT1JCHGR_SPEC>`"]
pub type FLT1JCHGR = crate::Reg<flt1jchgr::FLT1JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod flt1jchgr;
#[doc = "FLT1FCR register accessor: an alias for `Reg<FLT1FCR_SPEC>`"]
pub type FLT1FCR = crate::Reg<flt1fcr::FLT1FCR_SPEC>;
#[doc = "filter control register"]
pub mod flt1fcr;
#[doc = "FLT1JDATAR register accessor: an alias for `Reg<FLT1JDATAR_SPEC>`"]
pub type FLT1JDATAR = crate::Reg<flt1jdatar::FLT1JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod flt1jdatar;
#[doc = "FLT1RDATAR register accessor: an alias for `Reg<FLT1RDATAR_SPEC>`"]
pub type FLT1RDATAR = crate::Reg<flt1rdatar::FLT1RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod flt1rdatar;
#[doc = "FLT1AWHTR register accessor: an alias for `Reg<FLT1AWHTR_SPEC>`"]
pub type FLT1AWHTR = crate::Reg<flt1awhtr::FLT1AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod flt1awhtr;
#[doc = "FLT1AWLTR register accessor: an alias for `Reg<FLT1AWLTR_SPEC>`"]
pub type FLT1AWLTR = crate::Reg<flt1awltr::FLT1AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod flt1awltr;
#[doc = "FLT1AWSR register accessor: an alias for `Reg<FLT1AWSR_SPEC>`"]
pub type FLT1AWSR = crate::Reg<flt1awsr::FLT1AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod flt1awsr;
#[doc = "FLT1AWCFR register accessor: an alias for `Reg<FLT1AWCFR_SPEC>`"]
pub type FLT1AWCFR = crate::Reg<flt1awcfr::FLT1AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod flt1awcfr;
#[doc = "FLT1EXMAX register accessor: an alias for `Reg<FLT1EXMAX_SPEC>`"]
pub type FLT1EXMAX = crate::Reg<flt1exmax::FLT1EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod flt1exmax;
#[doc = "FLT1EXMIN register accessor: an alias for `Reg<FLT1EXMIN_SPEC>`"]
pub type FLT1EXMIN = crate::Reg<flt1exmin::FLT1EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod flt1exmin;
#[doc = "FLT1CNVTIMR register accessor: an alias for `Reg<FLT1CNVTIMR_SPEC>`"]
pub type FLT1CNVTIMR = crate::Reg<flt1cnvtimr::FLT1CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod flt1cnvtimr;
#[doc = "FLT2CR1 register accessor: an alias for `Reg<FLT2CR1_SPEC>`"]
pub type FLT2CR1 = crate::Reg<flt2cr1::FLT2CR1_SPEC>;
#[doc = "control register 1"]
pub mod flt2cr1;
#[doc = "FLT2CR2 register accessor: an alias for `Reg<FLT2CR2_SPEC>`"]
pub type FLT2CR2 = crate::Reg<flt2cr2::FLT2CR2_SPEC>;
#[doc = "control register 2"]
pub mod flt2cr2;
#[doc = "FLT2ISR register accessor: an alias for `Reg<FLT2ISR_SPEC>`"]
pub type FLT2ISR = crate::Reg<flt2isr::FLT2ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod flt2isr;
#[doc = "FLT2ICR register accessor: an alias for `Reg<FLT2ICR_SPEC>`"]
pub type FLT2ICR = crate::Reg<flt2icr::FLT2ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod flt2icr;
#[doc = "FLT2JCHGR register accessor: an alias for `Reg<FLT2JCHGR_SPEC>`"]
pub type FLT2JCHGR = crate::Reg<flt2jchgr::FLT2JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod flt2jchgr;
#[doc = "FLT2FCR register accessor: an alias for `Reg<FLT2FCR_SPEC>`"]
pub type FLT2FCR = crate::Reg<flt2fcr::FLT2FCR_SPEC>;
#[doc = "filter control register"]
pub mod flt2fcr;
#[doc = "FLT2JDATAR register accessor: an alias for `Reg<FLT2JDATAR_SPEC>`"]
pub type FLT2JDATAR = crate::Reg<flt2jdatar::FLT2JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod flt2jdatar;
#[doc = "FLT2RDATAR register accessor: an alias for `Reg<FLT2RDATAR_SPEC>`"]
pub type FLT2RDATAR = crate::Reg<flt2rdatar::FLT2RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod flt2rdatar;
#[doc = "FLT2AWHTR register accessor: an alias for `Reg<FLT2AWHTR_SPEC>`"]
pub type FLT2AWHTR = crate::Reg<flt2awhtr::FLT2AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod flt2awhtr;
#[doc = "FLT2AWLTR register accessor: an alias for `Reg<FLT2AWLTR_SPEC>`"]
pub type FLT2AWLTR = crate::Reg<flt2awltr::FLT2AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod flt2awltr;
#[doc = "FLT2AWSR register accessor: an alias for `Reg<FLT2AWSR_SPEC>`"]
pub type FLT2AWSR = crate::Reg<flt2awsr::FLT2AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod flt2awsr;
#[doc = "FLT2AWCFR register accessor: an alias for `Reg<FLT2AWCFR_SPEC>`"]
pub type FLT2AWCFR = crate::Reg<flt2awcfr::FLT2AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod flt2awcfr;
#[doc = "FLT2EXMAX register accessor: an alias for `Reg<FLT2EXMAX_SPEC>`"]
pub type FLT2EXMAX = crate::Reg<flt2exmax::FLT2EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod flt2exmax;
#[doc = "FLT2EXMIN register accessor: an alias for `Reg<FLT2EXMIN_SPEC>`"]
pub type FLT2EXMIN = crate::Reg<flt2exmin::FLT2EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod flt2exmin;
#[doc = "FLT2CNVTIMR register accessor: an alias for `Reg<FLT2CNVTIMR_SPEC>`"]
pub type FLT2CNVTIMR = crate::Reg<flt2cnvtimr::FLT2CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod flt2cnvtimr;
#[doc = "FLT3CR1 register accessor: an alias for `Reg<FLT3CR1_SPEC>`"]
pub type FLT3CR1 = crate::Reg<flt3cr1::FLT3CR1_SPEC>;
#[doc = "control register 1"]
pub mod flt3cr1;
#[doc = "FLT3CR2 register accessor: an alias for `Reg<FLT3CR2_SPEC>`"]
pub type FLT3CR2 = crate::Reg<flt3cr2::FLT3CR2_SPEC>;
#[doc = "control register 2"]
pub mod flt3cr2;
#[doc = "FLT3ISR register accessor: an alias for `Reg<FLT3ISR_SPEC>`"]
pub type FLT3ISR = crate::Reg<flt3isr::FLT3ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod flt3isr;
#[doc = "FLT3ICR register accessor: an alias for `Reg<FLT3ICR_SPEC>`"]
pub type FLT3ICR = crate::Reg<flt3icr::FLT3ICR_SPEC>;
#[doc = "interrupt flag clear register"]
pub mod flt3icr;
#[doc = "FLT3JCHGR register accessor: an alias for `Reg<FLT3JCHGR_SPEC>`"]
pub type FLT3JCHGR = crate::Reg<flt3jchgr::FLT3JCHGR_SPEC>;
#[doc = "injected channel group selection register"]
pub mod flt3jchgr;
#[doc = "FLT3FCR register accessor: an alias for `Reg<FLT3FCR_SPEC>`"]
pub type FLT3FCR = crate::Reg<flt3fcr::FLT3FCR_SPEC>;
#[doc = "filter control register"]
pub mod flt3fcr;
#[doc = "FLT3JDATAR register accessor: an alias for `Reg<FLT3JDATAR_SPEC>`"]
pub type FLT3JDATAR = crate::Reg<flt3jdatar::FLT3JDATAR_SPEC>;
#[doc = "data register for injected group"]
pub mod flt3jdatar;
#[doc = "FLT3RDATAR register accessor: an alias for `Reg<FLT3RDATAR_SPEC>`"]
pub type FLT3RDATAR = crate::Reg<flt3rdatar::FLT3RDATAR_SPEC>;
#[doc = "data register for the regular channel"]
pub mod flt3rdatar;
#[doc = "FLT3AWHTR register accessor: an alias for `Reg<FLT3AWHTR_SPEC>`"]
pub type FLT3AWHTR = crate::Reg<flt3awhtr::FLT3AWHTR_SPEC>;
#[doc = "analog watchdog high threshold register"]
pub mod flt3awhtr;
#[doc = "FLT3AWLTR register accessor: an alias for `Reg<FLT3AWLTR_SPEC>`"]
pub type FLT3AWLTR = crate::Reg<flt3awltr::FLT3AWLTR_SPEC>;
#[doc = "analog watchdog low threshold register"]
pub mod flt3awltr;
#[doc = "FLT3AWSR register accessor: an alias for `Reg<FLT3AWSR_SPEC>`"]
pub type FLT3AWSR = crate::Reg<flt3awsr::FLT3AWSR_SPEC>;
#[doc = "analog watchdog status register"]
pub mod flt3awsr;
#[doc = "FLT3AWCFR register accessor: an alias for `Reg<FLT3AWCFR_SPEC>`"]
pub type FLT3AWCFR = crate::Reg<flt3awcfr::FLT3AWCFR_SPEC>;
#[doc = "analog watchdog clear flag register"]
pub mod flt3awcfr;
#[doc = "FLT3EXMAX register accessor: an alias for `Reg<FLT3EXMAX_SPEC>`"]
pub type FLT3EXMAX = crate::Reg<flt3exmax::FLT3EXMAX_SPEC>;
#[doc = "Extremes detector maximum register"]
pub mod flt3exmax;
#[doc = "FLT3EXMIN register accessor: an alias for `Reg<FLT3EXMIN_SPEC>`"]
pub type FLT3EXMIN = crate::Reg<flt3exmin::FLT3EXMIN_SPEC>;
#[doc = "Extremes detector minimum register"]
pub mod flt3exmin;
#[doc = "FLT3CNVTIMR register accessor: an alias for `Reg<FLT3CNVTIMR_SPEC>`"]
pub type FLT3CNVTIMR = crate::Reg<flt3cnvtimr::FLT3CNVTIMR_SPEC>;
#[doc = "conversion timer register"]
pub mod flt3cnvtimr;
#[doc = "CH0DLYR register accessor: an alias for `Reg<CH0DLYR_SPEC>`"]
pub type CH0DLYR = crate::Reg<ch0dlyr::CH0DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch0dlyr;
#[doc = "CH1DLYR register accessor: an alias for `Reg<CH1DLYR_SPEC>`"]
pub type CH1DLYR = crate::Reg<ch1dlyr::CH1DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch1dlyr;
#[doc = "CH2DLYR register accessor: an alias for `Reg<CH2DLYR_SPEC>`"]
pub type CH2DLYR = crate::Reg<ch2dlyr::CH2DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch2dlyr;
#[doc = "CH3DLYR register accessor: an alias for `Reg<CH3DLYR_SPEC>`"]
pub type CH3DLYR = crate::Reg<ch3dlyr::CH3DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch3dlyr;
#[doc = "CH4DLYR register accessor: an alias for `Reg<CH4DLYR_SPEC>`"]
pub type CH4DLYR = crate::Reg<ch4dlyr::CH4DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch4dlyr;
#[doc = "CH5DLYR register accessor: an alias for `Reg<CH5DLYR_SPEC>`"]
pub type CH5DLYR = crate::Reg<ch5dlyr::CH5DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch5dlyr;
#[doc = "CH6DLYR register accessor: an alias for `Reg<CH6DLYR_SPEC>`"]
pub type CH6DLYR = crate::Reg<ch6dlyr::CH6DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch6dlyr;
#[doc = "CH7DLYR register accessor: an alias for `Reg<CH7DLYR_SPEC>`"]
pub type CH7DLYR = crate::Reg<ch7dlyr::CH7DLYR_SPEC>;
#[doc = "DFSDM channel y delay register"]
pub mod ch7dlyr;
