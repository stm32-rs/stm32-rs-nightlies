#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dfsdm_chcfg0r1: DFSDM_CHCFG0R1,
    dfsdm_chcfg0r2: DFSDM_CHCFG0R2,
    dfsdm_awscd0r: DFSDM_AWSCD0R,
    dfsdm_chwdat0r: DFSDM_CHWDAT0R,
    dfsdm_chdatin0r: DFSDM_CHDATIN0R,
    _reserved5: [u8; 0x0c],
    dfsdm_chcfg1r1: DFSDM_CHCFG1R1,
    dfsdm_chcfg1r2: DFSDM_CHCFG1R2,
    dfsdm_awscd1r: DFSDM_AWSCD1R,
    dfsdm_chwdat1r: DFSDM_CHWDAT1R,
    dfsdm_chdatin1r: DFSDM_CHDATIN1R,
    _reserved10: [u8; 0x0c],
    dfsdm_chcfg2r1: DFSDM_CHCFG2R1,
    dfsdm_chcfg2r2: DFSDM_CHCFG2R2,
    dfsdm_awscd2r: DFSDM_AWSCD2R,
    dfsdm_chwdat2r: DFSDM_CHWDAT2R,
    dfsdm_chdatin2r: DFSDM_CHDATIN2R,
    _reserved15: [u8; 0x0c],
    dfsdm_chcfg3r1: DFSDM_CHCFG3R1,
    dfsdm_chcfg3r2: DFSDM_CHCFG3R2,
    dfsdm_awscd3r: DFSDM_AWSCD3R,
    dfsdm_chwdat3r: DFSDM_CHWDAT3R,
    dfsdm_chdatin3r: DFSDM_CHDATIN3R,
    _reserved20: [u8; 0x0c],
    dfsdm_chcfg4r1: DFSDM_CHCFG4R1,
    dfsdm_chcfg4r2: DFSDM_CHCFG4R2,
    dfsdm_awscd4r: DFSDM_AWSCD4R,
    dfsdm_chwdat4r: DFSDM_CHWDAT4R,
    dfsdm_chdatin4r: DFSDM_CHDATIN4R,
    _reserved25: [u8; 0x0c],
    dfsdm_chcfg5r1: DFSDM_CHCFG5R1,
    dfsdm_chcfg5r2: DFSDM_CHCFG5R2,
    dfsdm_awscd5r: DFSDM_AWSCD5R,
    dfsdm_chwdat5r: DFSDM_CHWDAT5R,
    dfsdm_chdatin5r: DFSDM_CHDATIN5R,
    _reserved30: [u8; 0x0c],
    dfsdm_chcfg6r1: DFSDM_CHCFG6R1,
    dfsdm_chcfg6r2: DFSDM_CHCFG6R2,
    dfsdm_awscd6r: DFSDM_AWSCD6R,
    dfsdm_chwdat6r: DFSDM_CHWDAT6R,
    dfsdm_chdatin6r: DFSDM_CHDATIN6R,
    _reserved35: [u8; 0x0c],
    dfsdm_chcfg7r1: DFSDM_CHCFG7R1,
    dfsdm_chcfg7r2: DFSDM_CHCFG7R2,
    dfsdm_awscd7r: DFSDM_AWSCD7R,
    dfsdm_chwdat7r: DFSDM_CHWDAT7R,
    dfsdm_chdatin7r: DFSDM_CHDATIN7R,
    _reserved40: [u8; 0x0c],
    dfsdm0_cr1: DFSDM0_CR1,
    dfsdm0_cr2: DFSDM0_CR2,
    dfsdm0_isr: DFSDM0_ISR,
    dfsdm0_icr: DFSDM0_ICR,
    dfsdm0_jchgr: DFSDM0_JCHGR,
    dfsdm0_fcr: DFSDM0_FCR,
    dfsdm0_jdatar: DFSDM0_JDATAR,
    dfsdm0_rdatar: DFSDM0_RDATAR,
    dfsdm0_awhtr: DFSDM0_AWHTR,
    dfsdm0_awltr: DFSDM0_AWLTR,
    dfsdm0_awsr: DFSDM0_AWSR,
    dfsdm0_awcfr: DFSDM0_AWCFR,
    dfsdm0_exmax: DFSDM0_EXMAX,
    dfsdm0_exmin: DFSDM0_EXMIN,
    dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    _reserved55: [u8; 0x44],
    cr1: CR1,
    cr2: CR2,
    isr: ISR,
    icr: ICR,
    jchgr: JCHGR,
    fcr: FCR,
    _reserved_61_jdatar: [u8; 0x04],
    _reserved62: [u8; 0x04],
    awhtr: AWHTR,
    awltr: AWLTR,
    awsr: AWSR,
    awcfr: AWCFR,
    exmax: EXMAX,
    exmin: EXMIN,
    cnvtimr: CNVTIMR,
    _reserved69: [u8; 0x44],
    dfsdm2_cr1: DFSDM2_CR1,
    dfsdm2_cr2: DFSDM2_CR2,
    dfsdm2_isr: DFSDM2_ISR,
    dfsdm2_icr: DFSDM2_ICR,
    dfsdm2_jchgr: DFSDM2_JCHGR,
    dfsdm2_fcr: DFSDM2_FCR,
    _reserved_75_dfsdm2: [u8; 0x04],
    _reserved76: [u8; 0x04],
    dfsdm2_awhtr: DFSDM2_AWHTR,
    dfsdm2_awltr: DFSDM2_AWLTR,
    dfsdm2_awsr: DFSDM2_AWSR,
    dfsdm2_awcfr: DFSDM2_AWCFR,
    dfsdm2_exmax: DFSDM2_EXMAX,
    dfsdm2_exmin: DFSDM2_EXMIN,
    dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    _reserved83: [u8; 0x64],
    dfsdm3_awhtr: DFSDM3_AWHTR,
    dfsdm3_awltr: DFSDM3_AWLTR,
    dfsdm3_awsr: DFSDM3_AWSR,
    dfsdm3_awcfr: DFSDM3_AWCFR,
    dfsdm3_exmax: DFSDM3_EXMAX,
    dfsdm3_exmin: DFSDM3_EXMIN,
    dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
    _reserved90: [u8; 0x54],
    dfsdm3_jchgr: DFSDM3_JCHGR,
    dfsdm3_fcr: DFSDM3_FCR,
    _reserved_92_dfsdm3: [u8; 0x04],
    _reserved93: [u8; 0x64],
    dfsdm3_cr1: DFSDM3_CR1,
    dfsdm3_cr2: DFSDM3_CR2,
    dfsdm3_isr: DFSDM3_ISR,
    dfsdm3_icr: DFSDM3_ICR,
}
impl RegisterBlock {
    #[doc = "0x00 - DFSDM channel configuration 0 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg0r1(&self) -> &DFSDM_CHCFG0R1 {
        &self.dfsdm_chcfg0r1
    }
    #[doc = "0x04 - DFSDM channel configuration 0 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg0r2(&self) -> &DFSDM_CHCFG0R2 {
        &self.dfsdm_chcfg0r2
    }
    #[doc = "0x08 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd0r(&self) -> &DFSDM_AWSCD0R {
        &self.dfsdm_awscd0r
    }
    #[doc = "0x0c - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat0r(&self) -> &DFSDM_CHWDAT0R {
        &self.dfsdm_chwdat0r
    }
    #[doc = "0x10 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin0r(&self) -> &DFSDM_CHDATIN0R {
        &self.dfsdm_chdatin0r
    }
    #[doc = "0x20 - DFSDM channel configuration 1 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg1r1(&self) -> &DFSDM_CHCFG1R1 {
        &self.dfsdm_chcfg1r1
    }
    #[doc = "0x24 - DFSDM channel configuration 1 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg1r2(&self) -> &DFSDM_CHCFG1R2 {
        &self.dfsdm_chcfg1r2
    }
    #[doc = "0x28 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd1r(&self) -> &DFSDM_AWSCD1R {
        &self.dfsdm_awscd1r
    }
    #[doc = "0x2c - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat1r(&self) -> &DFSDM_CHWDAT1R {
        &self.dfsdm_chwdat1r
    }
    #[doc = "0x30 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin1r(&self) -> &DFSDM_CHDATIN1R {
        &self.dfsdm_chdatin1r
    }
    #[doc = "0x40 - DFSDM channel configuration 2 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg2r1(&self) -> &DFSDM_CHCFG2R1 {
        &self.dfsdm_chcfg2r1
    }
    #[doc = "0x44 - DFSDM channel configuration 2 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg2r2(&self) -> &DFSDM_CHCFG2R2 {
        &self.dfsdm_chcfg2r2
    }
    #[doc = "0x48 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd2r(&self) -> &DFSDM_AWSCD2R {
        &self.dfsdm_awscd2r
    }
    #[doc = "0x4c - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat2r(&self) -> &DFSDM_CHWDAT2R {
        &self.dfsdm_chwdat2r
    }
    #[doc = "0x50 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin2r(&self) -> &DFSDM_CHDATIN2R {
        &self.dfsdm_chdatin2r
    }
    #[doc = "0x60 - DFSDM channel configuration 3 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg3r1(&self) -> &DFSDM_CHCFG3R1 {
        &self.dfsdm_chcfg3r1
    }
    #[doc = "0x64 - DFSDM channel configuration 3 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg3r2(&self) -> &DFSDM_CHCFG3R2 {
        &self.dfsdm_chcfg3r2
    }
    #[doc = "0x68 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd3r(&self) -> &DFSDM_AWSCD3R {
        &self.dfsdm_awscd3r
    }
    #[doc = "0x6c - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat3r(&self) -> &DFSDM_CHWDAT3R {
        &self.dfsdm_chwdat3r
    }
    #[doc = "0x70 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin3r(&self) -> &DFSDM_CHDATIN3R {
        &self.dfsdm_chdatin3r
    }
    #[doc = "0x80 - DFSDM channel configuration 4 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg4r1(&self) -> &DFSDM_CHCFG4R1 {
        &self.dfsdm_chcfg4r1
    }
    #[doc = "0x84 - DFSDM channel configuration 4 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg4r2(&self) -> &DFSDM_CHCFG4R2 {
        &self.dfsdm_chcfg4r2
    }
    #[doc = "0x88 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd4r(&self) -> &DFSDM_AWSCD4R {
        &self.dfsdm_awscd4r
    }
    #[doc = "0x8c - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat4r(&self) -> &DFSDM_CHWDAT4R {
        &self.dfsdm_chwdat4r
    }
    #[doc = "0x90 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin4r(&self) -> &DFSDM_CHDATIN4R {
        &self.dfsdm_chdatin4r
    }
    #[doc = "0xa0 - DFSDM channel configuration 5 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg5r1(&self) -> &DFSDM_CHCFG5R1 {
        &self.dfsdm_chcfg5r1
    }
    #[doc = "0xa4 - DFSDM channel configuration 5 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg5r2(&self) -> &DFSDM_CHCFG5R2 {
        &self.dfsdm_chcfg5r2
    }
    #[doc = "0xa8 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd5r(&self) -> &DFSDM_AWSCD5R {
        &self.dfsdm_awscd5r
    }
    #[doc = "0xac - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat5r(&self) -> &DFSDM_CHWDAT5R {
        &self.dfsdm_chwdat5r
    }
    #[doc = "0xb0 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin5r(&self) -> &DFSDM_CHDATIN5R {
        &self.dfsdm_chdatin5r
    }
    #[doc = "0xc0 - DFSDM channel configuration 6 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg6r1(&self) -> &DFSDM_CHCFG6R1 {
        &self.dfsdm_chcfg6r1
    }
    #[doc = "0xc4 - DFSDM channel configuration 6 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg6r2(&self) -> &DFSDM_CHCFG6R2 {
        &self.dfsdm_chcfg6r2
    }
    #[doc = "0xc8 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd6r(&self) -> &DFSDM_AWSCD6R {
        &self.dfsdm_awscd6r
    }
    #[doc = "0xcc - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat6r(&self) -> &DFSDM_CHWDAT6R {
        &self.dfsdm_chwdat6r
    }
    #[doc = "0xd0 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin6r(&self) -> &DFSDM_CHDATIN6R {
        &self.dfsdm_chdatin6r
    }
    #[doc = "0xe0 - DFSDM channel configuration 7 register 1"]
    #[inline(always)]
    pub const fn dfsdm_chcfg7r1(&self) -> &DFSDM_CHCFG7R1 {
        &self.dfsdm_chcfg7r1
    }
    #[doc = "0xe4 - DFSDM channel configuration 7 register 2"]
    #[inline(always)]
    pub const fn dfsdm_chcfg7r2(&self) -> &DFSDM_CHCFG7R2 {
        &self.dfsdm_chcfg7r2
    }
    #[doc = "0xe8 - DFSDM analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn dfsdm_awscd7r(&self) -> &DFSDM_AWSCD7R {
        &self.dfsdm_awscd7r
    }
    #[doc = "0xec - DFSDM channel watchdog filter data register"]
    #[inline(always)]
    pub const fn dfsdm_chwdat7r(&self) -> &DFSDM_CHWDAT7R {
        &self.dfsdm_chwdat7r
    }
    #[doc = "0xf0 - DFSDM channel data input register"]
    #[inline(always)]
    pub const fn dfsdm_chdatin7r(&self) -> &DFSDM_CHDATIN7R {
        &self.dfsdm_chdatin7r
    }
    #[doc = "0x100 - DFSDM control register 1"]
    #[inline(always)]
    pub const fn dfsdm0_cr1(&self) -> &DFSDM0_CR1 {
        &self.dfsdm0_cr1
    }
    #[doc = "0x104 - DFSDM control register 2"]
    #[inline(always)]
    pub const fn dfsdm0_cr2(&self) -> &DFSDM0_CR2 {
        &self.dfsdm0_cr2
    }
    #[doc = "0x108 - DFSDM interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm0_isr(&self) -> &DFSDM0_ISR {
        &self.dfsdm0_isr
    }
    #[doc = "0x10c - DFSDM interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm0_icr(&self) -> &DFSDM0_ICR {
        &self.dfsdm0_icr
    }
    #[doc = "0x110 - DFSDM injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm0_jchgr(&self) -> &DFSDM0_JCHGR {
        &self.dfsdm0_jchgr
    }
    #[doc = "0x114 - DFSDM filter control register"]
    #[inline(always)]
    pub const fn dfsdm0_fcr(&self) -> &DFSDM0_FCR {
        &self.dfsdm0_fcr
    }
    #[doc = "0x118 - DFSDM data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm0_jdatar(&self) -> &DFSDM0_JDATAR {
        &self.dfsdm0_jdatar
    }
    #[doc = "0x11c - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm0_rdatar(&self) -> &DFSDM0_RDATAR {
        &self.dfsdm0_rdatar
    }
    #[doc = "0x120 - DFSDM analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm0_awhtr(&self) -> &DFSDM0_AWHTR {
        &self.dfsdm0_awhtr
    }
    #[doc = "0x124 - DFSDM analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm0_awltr(&self) -> &DFSDM0_AWLTR {
        &self.dfsdm0_awltr
    }
    #[doc = "0x128 - DFSDM analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm0_awsr(&self) -> &DFSDM0_AWSR {
        &self.dfsdm0_awsr
    }
    #[doc = "0x12c - DFSDM analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm0_awcfr(&self) -> &DFSDM0_AWCFR {
        &self.dfsdm0_awcfr
    }
    #[doc = "0x130 - DFSDM Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm0_exmax(&self) -> &DFSDM0_EXMAX {
        &self.dfsdm0_exmax
    }
    #[doc = "0x134 - DFSDM Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm0_exmin(&self) -> &DFSDM0_EXMIN {
        &self.dfsdm0_exmin
    }
    #[doc = "0x138 - DFSDM conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm0_cnvtimr(&self) -> &DFSDM0_CNVTIMR {
        &self.dfsdm0_cnvtimr
    }
    #[doc = "0x180 - DFSDM control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x184 - DFSDM control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x188 - DFSDM interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x18c - DFSDM interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x190 - DFSDM injected channel group selection register"]
    #[inline(always)]
    pub const fn jchgr(&self) -> &JCHGR {
        &self.jchgr
    }
    #[doc = "0x194 - DFSDM filter control register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x198 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub const fn rdatar(&self) -> &RDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - DFSDM data register for injected group"]
    #[inline(always)]
    pub const fn jdatar(&self) -> &JDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x1a0 - DFSDM analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn awhtr(&self) -> &AWHTR {
        &self.awhtr
    }
    #[doc = "0x1a4 - DFSDM analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn awltr(&self) -> &AWLTR {
        &self.awltr
    }
    #[doc = "0x1a8 - DFSDM analog watchdog status register"]
    #[inline(always)]
    pub const fn awsr(&self) -> &AWSR {
        &self.awsr
    }
    #[doc = "0x1ac - DFSDM analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn awcfr(&self) -> &AWCFR {
        &self.awcfr
    }
    #[doc = "0x1b0 - DFSDM Extremes detector maximum register"]
    #[inline(always)]
    pub const fn exmax(&self) -> &EXMAX {
        &self.exmax
    }
    #[doc = "0x1b4 - DFSDM Extremes detector minimum register"]
    #[inline(always)]
    pub const fn exmin(&self) -> &EXMIN {
        &self.exmin
    }
    #[doc = "0x1b8 - DFSDM conversion timer register"]
    #[inline(always)]
    pub const fn cnvtimr(&self) -> &CNVTIMR {
        &self.cnvtimr
    }
    #[doc = "0x200 - DFSDM control register 1"]
    #[inline(always)]
    pub const fn dfsdm2_cr1(&self) -> &DFSDM2_CR1 {
        &self.dfsdm2_cr1
    }
    #[doc = "0x204 - DFSDM control register 2"]
    #[inline(always)]
    pub const fn dfsdm2_cr2(&self) -> &DFSDM2_CR2 {
        &self.dfsdm2_cr2
    }
    #[doc = "0x208 - DFSDM interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm2_isr(&self) -> &DFSDM2_ISR {
        &self.dfsdm2_isr
    }
    #[doc = "0x20c - DFSDM interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm2_icr(&self) -> &DFSDM2_ICR {
        &self.dfsdm2_icr
    }
    #[doc = "0x210 - DFSDM injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm2_jchgr(&self) -> &DFSDM2_JCHGR {
        &self.dfsdm2_jchgr
    }
    #[doc = "0x214 - DFSDM filter control register"]
    #[inline(always)]
    pub const fn dfsdm2_fcr(&self) -> &DFSDM2_FCR {
        &self.dfsdm2_fcr
    }
    #[doc = "0x218 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm2_rdatar(&self) -> &DFSDM2_RDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - DFSDM data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm2_jdatar(&self) -> &DFSDM2_JDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x220 - DFSDM analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm2_awhtr(&self) -> &DFSDM2_AWHTR {
        &self.dfsdm2_awhtr
    }
    #[doc = "0x224 - DFSDM analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm2_awltr(&self) -> &DFSDM2_AWLTR {
        &self.dfsdm2_awltr
    }
    #[doc = "0x228 - DFSDM analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm2_awsr(&self) -> &DFSDM2_AWSR {
        &self.dfsdm2_awsr
    }
    #[doc = "0x22c - DFSDM analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm2_awcfr(&self) -> &DFSDM2_AWCFR {
        &self.dfsdm2_awcfr
    }
    #[doc = "0x230 - DFSDM Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm2_exmax(&self) -> &DFSDM2_EXMAX {
        &self.dfsdm2_exmax
    }
    #[doc = "0x234 - DFSDM Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm2_exmin(&self) -> &DFSDM2_EXMIN {
        &self.dfsdm2_exmin
    }
    #[doc = "0x238 - DFSDM conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm2_cnvtimr(&self) -> &DFSDM2_CNVTIMR {
        &self.dfsdm2_cnvtimr
    }
    #[doc = "0x2a0 - DFSDM analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn dfsdm3_awhtr(&self) -> &DFSDM3_AWHTR {
        &self.dfsdm3_awhtr
    }
    #[doc = "0x2a4 - DFSDM analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn dfsdm3_awltr(&self) -> &DFSDM3_AWLTR {
        &self.dfsdm3_awltr
    }
    #[doc = "0x2a8 - DFSDM analog watchdog status register"]
    #[inline(always)]
    pub const fn dfsdm3_awsr(&self) -> &DFSDM3_AWSR {
        &self.dfsdm3_awsr
    }
    #[doc = "0x2ac - DFSDM analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn dfsdm3_awcfr(&self) -> &DFSDM3_AWCFR {
        &self.dfsdm3_awcfr
    }
    #[doc = "0x2b0 - DFSDM Extremes detector maximum register"]
    #[inline(always)]
    pub const fn dfsdm3_exmax(&self) -> &DFSDM3_EXMAX {
        &self.dfsdm3_exmax
    }
    #[doc = "0x2b4 - DFSDM Extremes detector minimum register"]
    #[inline(always)]
    pub const fn dfsdm3_exmin(&self) -> &DFSDM3_EXMIN {
        &self.dfsdm3_exmin
    }
    #[doc = "0x2b8 - DFSDM conversion timer register"]
    #[inline(always)]
    pub const fn dfsdm3_cnvtimr(&self) -> &DFSDM3_CNVTIMR {
        &self.dfsdm3_cnvtimr
    }
    #[doc = "0x310 - DFSDM injected channel group selection register"]
    #[inline(always)]
    pub const fn dfsdm3_jchgr(&self) -> &DFSDM3_JCHGR {
        &self.dfsdm3_jchgr
    }
    #[doc = "0x314 - DFSDM filter control register"]
    #[inline(always)]
    pub const fn dfsdm3_fcr(&self) -> &DFSDM3_FCR {
        &self.dfsdm3_fcr
    }
    #[doc = "0x318 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub const fn dfsdm3_rdatar(&self) -> &DFSDM3_RDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(792).cast() }
    }
    #[doc = "0x318 - DFSDM data register for injected group"]
    #[inline(always)]
    pub const fn dfsdm3_jdatar(&self) -> &DFSDM3_JDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(792).cast() }
    }
    #[doc = "0x380 - DFSDM control register 1"]
    #[inline(always)]
    pub const fn dfsdm3_cr1(&self) -> &DFSDM3_CR1 {
        &self.dfsdm3_cr1
    }
    #[doc = "0x384 - DFSDM control register 2"]
    #[inline(always)]
    pub const fn dfsdm3_cr2(&self) -> &DFSDM3_CR2 {
        &self.dfsdm3_cr2
    }
    #[doc = "0x388 - DFSDM interrupt and status register"]
    #[inline(always)]
    pub const fn dfsdm3_isr(&self) -> &DFSDM3_ISR {
        &self.dfsdm3_isr
    }
    #[doc = "0x38c - DFSDM interrupt flag clear register"]
    #[inline(always)]
    pub const fn dfsdm3_icr(&self) -> &DFSDM3_ICR {
        &self.dfsdm3_icr
    }
}
#[doc = "DFSDM_CHCFG0R1 (rw) register accessor: DFSDM channel configuration 0 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg0r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg0r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg0r1`]
module"]
pub type DFSDM_CHCFG0R1 = crate::Reg<dfsdm_chcfg0r1::DFSDM_CHCFG0R1rs>;
#[doc = "DFSDM channel configuration 0 register 1"]
pub mod dfsdm_chcfg0r1;
#[doc = "DFSDM_CHCFG1R1 (rw) register accessor: DFSDM channel configuration 1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg1r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg1r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg1r1`]
module"]
pub type DFSDM_CHCFG1R1 = crate::Reg<dfsdm_chcfg1r1::DFSDM_CHCFG1R1rs>;
#[doc = "DFSDM channel configuration 1 register 1"]
pub mod dfsdm_chcfg1r1;
#[doc = "DFSDM_CHCFG2R1 (rw) register accessor: DFSDM channel configuration 2 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg2r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg2r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg2r1`]
module"]
pub type DFSDM_CHCFG2R1 = crate::Reg<dfsdm_chcfg2r1::DFSDM_CHCFG2R1rs>;
#[doc = "DFSDM channel configuration 2 register 1"]
pub mod dfsdm_chcfg2r1;
#[doc = "DFSDM_CHCFG3R1 (rw) register accessor: DFSDM channel configuration 3 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg3r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg3r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg3r1`]
module"]
pub type DFSDM_CHCFG3R1 = crate::Reg<dfsdm_chcfg3r1::DFSDM_CHCFG3R1rs>;
#[doc = "DFSDM channel configuration 3 register 1"]
pub mod dfsdm_chcfg3r1;
#[doc = "DFSDM_CHCFG4R1 (rw) register accessor: DFSDM channel configuration 4 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg4r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg4r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg4r1`]
module"]
pub type DFSDM_CHCFG4R1 = crate::Reg<dfsdm_chcfg4r1::DFSDM_CHCFG4R1rs>;
#[doc = "DFSDM channel configuration 4 register 1"]
pub mod dfsdm_chcfg4r1;
#[doc = "DFSDM_CHCFG5R1 (rw) register accessor: DFSDM channel configuration 5 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg5r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg5r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg5r1`]
module"]
pub type DFSDM_CHCFG5R1 = crate::Reg<dfsdm_chcfg5r1::DFSDM_CHCFG5R1rs>;
#[doc = "DFSDM channel configuration 5 register 1"]
pub mod dfsdm_chcfg5r1;
#[doc = "DFSDM_CHCFG6R1 (rw) register accessor: DFSDM channel configuration 6 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg6r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg6r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg6r1`]
module"]
pub type DFSDM_CHCFG6R1 = crate::Reg<dfsdm_chcfg6r1::DFSDM_CHCFG6R1rs>;
#[doc = "DFSDM channel configuration 6 register 1"]
pub mod dfsdm_chcfg6r1;
#[doc = "DFSDM_CHCFG7R1 (rw) register accessor: DFSDM channel configuration 7 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg7r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg7r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg7r1`]
module"]
pub type DFSDM_CHCFG7R1 = crate::Reg<dfsdm_chcfg7r1::DFSDM_CHCFG7R1rs>;
#[doc = "DFSDM channel configuration 7 register 1"]
pub mod dfsdm_chcfg7r1;
#[doc = "DFSDM_CHCFG0R2 (rw) register accessor: DFSDM channel configuration 0 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg0r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg0r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg0r2`]
module"]
pub type DFSDM_CHCFG0R2 = crate::Reg<dfsdm_chcfg0r2::DFSDM_CHCFG0R2rs>;
#[doc = "DFSDM channel configuration 0 register 2"]
pub mod dfsdm_chcfg0r2;
#[doc = "DFSDM_CHCFG1R2 (rw) register accessor: DFSDM channel configuration 1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg1r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg1r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg1r2`]
module"]
pub type DFSDM_CHCFG1R2 = crate::Reg<dfsdm_chcfg1r2::DFSDM_CHCFG1R2rs>;
#[doc = "DFSDM channel configuration 1 register 2"]
pub mod dfsdm_chcfg1r2;
#[doc = "DFSDM_CHCFG2R2 (rw) register accessor: DFSDM channel configuration 2 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg2r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg2r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg2r2`]
module"]
pub type DFSDM_CHCFG2R2 = crate::Reg<dfsdm_chcfg2r2::DFSDM_CHCFG2R2rs>;
#[doc = "DFSDM channel configuration 2 register 2"]
pub mod dfsdm_chcfg2r2;
#[doc = "DFSDM_CHCFG3R2 (rw) register accessor: DFSDM channel configuration 3 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg3r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg3r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg3r2`]
module"]
pub type DFSDM_CHCFG3R2 = crate::Reg<dfsdm_chcfg3r2::DFSDM_CHCFG3R2rs>;
#[doc = "DFSDM channel configuration 3 register 2"]
pub mod dfsdm_chcfg3r2;
#[doc = "DFSDM_CHCFG4R2 (rw) register accessor: DFSDM channel configuration 4 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg4r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg4r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg4r2`]
module"]
pub type DFSDM_CHCFG4R2 = crate::Reg<dfsdm_chcfg4r2::DFSDM_CHCFG4R2rs>;
#[doc = "DFSDM channel configuration 4 register 2"]
pub mod dfsdm_chcfg4r2;
#[doc = "DFSDM_CHCFG5R2 (rw) register accessor: DFSDM channel configuration 5 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg5r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg5r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg5r2`]
module"]
pub type DFSDM_CHCFG5R2 = crate::Reg<dfsdm_chcfg5r2::DFSDM_CHCFG5R2rs>;
#[doc = "DFSDM channel configuration 5 register 2"]
pub mod dfsdm_chcfg5r2;
#[doc = "DFSDM_CHCFG6R2 (rw) register accessor: DFSDM channel configuration 6 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg6r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg6r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg6r2`]
module"]
pub type DFSDM_CHCFG6R2 = crate::Reg<dfsdm_chcfg6r2::DFSDM_CHCFG6R2rs>;
#[doc = "DFSDM channel configuration 6 register 2"]
pub mod dfsdm_chcfg6r2;
#[doc = "DFSDM_CHCFG7R2 (rw) register accessor: DFSDM channel configuration 7 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chcfg7r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chcfg7r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chcfg7r2`]
module"]
pub type DFSDM_CHCFG7R2 = crate::Reg<dfsdm_chcfg7r2::DFSDM_CHCFG7R2rs>;
#[doc = "DFSDM channel configuration 7 register 2"]
pub mod dfsdm_chcfg7r2;
#[doc = "DFSDM_AWSCD0R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd0r`]
module"]
pub type DFSDM_AWSCD0R = crate::Reg<dfsdm_awscd0r::DFSDM_AWSCD0Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd0r;
#[doc = "DFSDM_AWSCD1R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd1r`]
module"]
pub type DFSDM_AWSCD1R = crate::Reg<dfsdm_awscd1r::DFSDM_AWSCD1Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd1r;
#[doc = "DFSDM_AWSCD2R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd2r`]
module"]
pub type DFSDM_AWSCD2R = crate::Reg<dfsdm_awscd2r::DFSDM_AWSCD2Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd2r;
#[doc = "DFSDM_AWSCD3R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd3r`]
module"]
pub type DFSDM_AWSCD3R = crate::Reg<dfsdm_awscd3r::DFSDM_AWSCD3Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd3r;
#[doc = "DFSDM_AWSCD4R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd4r`]
module"]
pub type DFSDM_AWSCD4R = crate::Reg<dfsdm_awscd4r::DFSDM_AWSCD4Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd4r;
#[doc = "DFSDM_AWSCD5R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd5r`]
module"]
pub type DFSDM_AWSCD5R = crate::Reg<dfsdm_awscd5r::DFSDM_AWSCD5Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd5r;
#[doc = "DFSDM_AWSCD6R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd6r`]
module"]
pub type DFSDM_AWSCD6R = crate::Reg<dfsdm_awscd6r::DFSDM_AWSCD6Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd6r;
#[doc = "DFSDM_AWSCD7R (rw) register accessor: DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_awscd7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_awscd7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_awscd7r`]
module"]
pub type DFSDM_AWSCD7R = crate::Reg<dfsdm_awscd7r::DFSDM_AWSCD7Rrs>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd7r;
#[doc = "DFSDM_CHWDAT0R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat0r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat0r`]
module"]
pub type DFSDM_CHWDAT0R = crate::Reg<dfsdm_chwdat0r::DFSDM_CHWDAT0Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat0r;
#[doc = "DFSDM_CHWDAT1R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat1r`]
module"]
pub type DFSDM_CHWDAT1R = crate::Reg<dfsdm_chwdat1r::DFSDM_CHWDAT1Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat1r;
#[doc = "DFSDM_CHWDAT2R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat2r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat2r`]
module"]
pub type DFSDM_CHWDAT2R = crate::Reg<dfsdm_chwdat2r::DFSDM_CHWDAT2Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat2r;
#[doc = "DFSDM_CHWDAT3R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat3r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat3r`]
module"]
pub type DFSDM_CHWDAT3R = crate::Reg<dfsdm_chwdat3r::DFSDM_CHWDAT3Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat3r;
#[doc = "DFSDM_CHWDAT4R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat4r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat4r`]
module"]
pub type DFSDM_CHWDAT4R = crate::Reg<dfsdm_chwdat4r::DFSDM_CHWDAT4Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat4r;
#[doc = "DFSDM_CHWDAT5R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat5r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat5r`]
module"]
pub type DFSDM_CHWDAT5R = crate::Reg<dfsdm_chwdat5r::DFSDM_CHWDAT5Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat5r;
#[doc = "DFSDM_CHWDAT6R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat6r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat6r`]
module"]
pub type DFSDM_CHWDAT6R = crate::Reg<dfsdm_chwdat6r::DFSDM_CHWDAT6Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat6r;
#[doc = "DFSDM_CHWDAT7R (r) register accessor: DFSDM channel watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chwdat7r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chwdat7r`]
module"]
pub type DFSDM_CHWDAT7R = crate::Reg<dfsdm_chwdat7r::DFSDM_CHWDAT7Rrs>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat7r;
#[doc = "DFSDM_CHDATIN0R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin0r`]
module"]
pub type DFSDM_CHDATIN0R = crate::Reg<dfsdm_chdatin0r::DFSDM_CHDATIN0Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin0r;
#[doc = "DFSDM_CHDATIN1R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin1r`]
module"]
pub type DFSDM_CHDATIN1R = crate::Reg<dfsdm_chdatin1r::DFSDM_CHDATIN1Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin1r;
#[doc = "DFSDM_CHDATIN2R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin2r`]
module"]
pub type DFSDM_CHDATIN2R = crate::Reg<dfsdm_chdatin2r::DFSDM_CHDATIN2Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin2r;
#[doc = "DFSDM_CHDATIN3R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin3r`]
module"]
pub type DFSDM_CHDATIN3R = crate::Reg<dfsdm_chdatin3r::DFSDM_CHDATIN3Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin3r;
#[doc = "DFSDM_CHDATIN4R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin4r`]
module"]
pub type DFSDM_CHDATIN4R = crate::Reg<dfsdm_chdatin4r::DFSDM_CHDATIN4Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin4r;
#[doc = "DFSDM_CHDATIN5R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin5r`]
module"]
pub type DFSDM_CHDATIN5R = crate::Reg<dfsdm_chdatin5r::DFSDM_CHDATIN5Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin5r;
#[doc = "DFSDM_CHDATIN6R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin6r`]
module"]
pub type DFSDM_CHDATIN6R = crate::Reg<dfsdm_chdatin6r::DFSDM_CHDATIN6Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin6r;
#[doc = "DFSDM_CHDATIN7R (rw) register accessor: DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm_chdatin7r`]
module"]
pub type DFSDM_CHDATIN7R = crate::Reg<dfsdm_chdatin7r::DFSDM_CHDATIN7Rrs>;
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin7r;
#[doc = "DFSDM0_CR1 (rw) register accessor: DFSDM control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_cr1`]
module"]
pub type DFSDM0_CR1 = crate::Reg<dfsdm0_cr1::DFSDM0_CR1rs>;
#[doc = "DFSDM control register 1"]
pub mod dfsdm0_cr1;
#[doc = "CR1 (rw) register accessor: DFSDM control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "DFSDM control register 1"]
pub mod cr1;
#[doc = "DFSDM2_CR1 (rw) register accessor: DFSDM control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_cr1`]
module"]
pub type DFSDM2_CR1 = crate::Reg<dfsdm2_cr1::DFSDM2_CR1rs>;
#[doc = "DFSDM control register 1"]
pub mod dfsdm2_cr1;
#[doc = "DFSDM3_CR1 (rw) register accessor: DFSDM control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_cr1`]
module"]
pub type DFSDM3_CR1 = crate::Reg<dfsdm3_cr1::DFSDM3_CR1rs>;
#[doc = "DFSDM control register 1"]
pub mod dfsdm3_cr1;
#[doc = "DFSDM0_CR2 (rw) register accessor: DFSDM control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_cr2`]
module"]
pub type DFSDM0_CR2 = crate::Reg<dfsdm0_cr2::DFSDM0_CR2rs>;
#[doc = "DFSDM control register 2"]
pub mod dfsdm0_cr2;
#[doc = "CR2 (rw) register accessor: DFSDM control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "DFSDM control register 2"]
pub mod cr2;
#[doc = "DFSDM2_CR2 (rw) register accessor: DFSDM control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_cr2`]
module"]
pub type DFSDM2_CR2 = crate::Reg<dfsdm2_cr2::DFSDM2_CR2rs>;
#[doc = "DFSDM control register 2"]
pub mod dfsdm2_cr2;
#[doc = "DFSDM3_CR2 (rw) register accessor: DFSDM control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_cr2`]
module"]
pub type DFSDM3_CR2 = crate::Reg<dfsdm3_cr2::DFSDM3_CR2rs>;
#[doc = "DFSDM control register 2"]
pub mod dfsdm3_cr2;
#[doc = "DFSDM0_ISR (r) register accessor: DFSDM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_isr`]
module"]
pub type DFSDM0_ISR = crate::Reg<dfsdm0_isr::DFSDM0_ISRrs>;
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm0_isr;
#[doc = "ISR (r) register accessor: DFSDM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "DFSDM interrupt and status register"]
pub mod isr;
#[doc = "DFSDM2_ISR (r) register accessor: DFSDM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_isr`]
module"]
pub type DFSDM2_ISR = crate::Reg<dfsdm2_isr::DFSDM2_ISRrs>;
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm2_isr;
#[doc = "DFSDM3_ISR (r) register accessor: DFSDM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_isr`]
module"]
pub type DFSDM3_ISR = crate::Reg<dfsdm3_isr::DFSDM3_ISRrs>;
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm3_isr;
#[doc = "DFSDM0_ICR (rw) register accessor: DFSDM interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_icr`]
module"]
pub type DFSDM0_ICR = crate::Reg<dfsdm0_icr::DFSDM0_ICRrs>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm0_icr;
#[doc = "ICR (rw) register accessor: DFSDM interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod icr;
#[doc = "DFSDM2_ICR (rw) register accessor: DFSDM interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_icr`]
module"]
pub type DFSDM2_ICR = crate::Reg<dfsdm2_icr::DFSDM2_ICRrs>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm2_icr;
#[doc = "DFSDM3_ICR (rw) register accessor: DFSDM interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_icr`]
module"]
pub type DFSDM3_ICR = crate::Reg<dfsdm3_icr::DFSDM3_ICRrs>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm3_icr;
#[doc = "DFSDM0_JCHGR (rw) register accessor: DFSDM injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_jchgr`]
module"]
pub type DFSDM0_JCHGR = crate::Reg<dfsdm0_jchgr::DFSDM0_JCHGRrs>;
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm0_jchgr;
#[doc = "JCHGR (rw) register accessor: DFSDM injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jchgr`]
module"]
pub type JCHGR = crate::Reg<jchgr::JCHGRrs>;
#[doc = "DFSDM injected channel group selection register"]
pub mod jchgr;
#[doc = "DFSDM2_JCHGR (rw) register accessor: DFSDM injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_jchgr`]
module"]
pub type DFSDM2_JCHGR = crate::Reg<dfsdm2_jchgr::DFSDM2_JCHGRrs>;
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm2_jchgr;
#[doc = "DFSDM3_JCHGR (rw) register accessor: DFSDM injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_jchgr`]
module"]
pub type DFSDM3_JCHGR = crate::Reg<dfsdm3_jchgr::DFSDM3_JCHGRrs>;
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm3_jchgr;
#[doc = "DFSDM0_FCR (rw) register accessor: DFSDM filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_fcr`]
module"]
pub type DFSDM0_FCR = crate::Reg<dfsdm0_fcr::DFSDM0_FCRrs>;
#[doc = "DFSDM filter control register"]
pub mod dfsdm0_fcr;
#[doc = "FCR (rw) register accessor: DFSDM filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "DFSDM filter control register"]
pub mod fcr;
#[doc = "DFSDM2_FCR (rw) register accessor: DFSDM filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_fcr`]
module"]
pub type DFSDM2_FCR = crate::Reg<dfsdm2_fcr::DFSDM2_FCRrs>;
#[doc = "DFSDM filter control register"]
pub mod dfsdm2_fcr;
#[doc = "DFSDM3_FCR (rw) register accessor: DFSDM filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_fcr`]
module"]
pub type DFSDM3_FCR = crate::Reg<dfsdm3_fcr::DFSDM3_FCRrs>;
#[doc = "DFSDM filter control register"]
pub mod dfsdm3_fcr;
#[doc = "DFSDM0_JDATAR (r) register accessor: DFSDM data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_jdatar`]
module"]
pub type DFSDM0_JDATAR = crate::Reg<dfsdm0_jdatar::DFSDM0_JDATARrs>;
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm0_jdatar;
#[doc = "JDATAR (r) register accessor: DFSDM data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdatar`]
module"]
pub type JDATAR = crate::Reg<jdatar::JDATARrs>;
#[doc = "DFSDM data register for injected group"]
pub mod jdatar;
#[doc = "DFSDM2_JDATAR (r) register accessor: DFSDM data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_jdatar`]
module"]
pub type DFSDM2_JDATAR = crate::Reg<dfsdm2_jdatar::DFSDM2_JDATARrs>;
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm2_jdatar;
#[doc = "DFSDM3_JDATAR (r) register accessor: DFSDM data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_jdatar`]
module"]
pub type DFSDM3_JDATAR = crate::Reg<dfsdm3_jdatar::DFSDM3_JDATARrs>;
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm3_jdatar;
#[doc = "DFSDM0_RDATAR (r) register accessor: DFSDM data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_rdatar`]
module"]
pub type DFSDM0_RDATAR = crate::Reg<dfsdm0_rdatar::DFSDM0_RDATARrs>;
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm0_rdatar;
#[doc = "RDATAR (r) register accessor: DFSDM data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdatar`]
module"]
pub type RDATAR = crate::Reg<rdatar::RDATARrs>;
#[doc = "DFSDM data register for the regular channel"]
pub mod rdatar;
#[doc = "DFSDM2_RDATAR (r) register accessor: DFSDM data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_rdatar`]
module"]
pub type DFSDM2_RDATAR = crate::Reg<dfsdm2_rdatar::DFSDM2_RDATARrs>;
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm2_rdatar;
#[doc = "DFSDM3_RDATAR (r) register accessor: DFSDM data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_rdatar`]
module"]
pub type DFSDM3_RDATAR = crate::Reg<dfsdm3_rdatar::DFSDM3_RDATARrs>;
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm3_rdatar;
#[doc = "DFSDM0_AWHTR (rw) register accessor: DFSDM analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_awhtr`]
module"]
pub type DFSDM0_AWHTR = crate::Reg<dfsdm0_awhtr::DFSDM0_AWHTRrs>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm0_awhtr;
#[doc = "AWHTR (rw) register accessor: DFSDM analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awhtr`]
module"]
pub type AWHTR = crate::Reg<awhtr::AWHTRrs>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod awhtr;
#[doc = "DFSDM2_AWHTR (rw) register accessor: DFSDM analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_awhtr`]
module"]
pub type DFSDM2_AWHTR = crate::Reg<dfsdm2_awhtr::DFSDM2_AWHTRrs>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm2_awhtr;
#[doc = "DFSDM3_AWHTR (rw) register accessor: DFSDM analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_awhtr`]
module"]
pub type DFSDM3_AWHTR = crate::Reg<dfsdm3_awhtr::DFSDM3_AWHTRrs>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm3_awhtr;
#[doc = "DFSDM0_AWLTR (rw) register accessor: DFSDM analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_awltr`]
module"]
pub type DFSDM0_AWLTR = crate::Reg<dfsdm0_awltr::DFSDM0_AWLTRrs>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm0_awltr;
#[doc = "AWLTR (rw) register accessor: DFSDM analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awltr`]
module"]
pub type AWLTR = crate::Reg<awltr::AWLTRrs>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod awltr;
#[doc = "DFSDM2_AWLTR (rw) register accessor: DFSDM analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_awltr`]
module"]
pub type DFSDM2_AWLTR = crate::Reg<dfsdm2_awltr::DFSDM2_AWLTRrs>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm2_awltr;
#[doc = "DFSDM3_AWLTR (rw) register accessor: DFSDM analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_awltr`]
module"]
pub type DFSDM3_AWLTR = crate::Reg<dfsdm3_awltr::DFSDM3_AWLTRrs>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm3_awltr;
#[doc = "DFSDM0_AWSR (r) register accessor: DFSDM analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_awsr`]
module"]
pub type DFSDM0_AWSR = crate::Reg<dfsdm0_awsr::DFSDM0_AWSRrs>;
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm0_awsr;
#[doc = "AWSR (r) register accessor: DFSDM analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awsr`]
module"]
pub type AWSR = crate::Reg<awsr::AWSRrs>;
#[doc = "DFSDM analog watchdog status register"]
pub mod awsr;
#[doc = "DFSDM2_AWSR (r) register accessor: DFSDM analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_awsr`]
module"]
pub type DFSDM2_AWSR = crate::Reg<dfsdm2_awsr::DFSDM2_AWSRrs>;
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm2_awsr;
#[doc = "DFSDM3_AWSR (r) register accessor: DFSDM analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_awsr`]
module"]
pub type DFSDM3_AWSR = crate::Reg<dfsdm3_awsr::DFSDM3_AWSRrs>;
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm3_awsr;
#[doc = "DFSDM0_AWCFR (rw) register accessor: DFSDM analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm0_awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_awcfr`]
module"]
pub type DFSDM0_AWCFR = crate::Reg<dfsdm0_awcfr::DFSDM0_AWCFRrs>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm0_awcfr;
#[doc = "AWCFR (rw) register accessor: DFSDM analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awcfr`]
module"]
pub type AWCFR = crate::Reg<awcfr::AWCFRrs>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod awcfr;
#[doc = "DFSDM2_AWCFR (rw) register accessor: DFSDM analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm2_awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_awcfr`]
module"]
pub type DFSDM2_AWCFR = crate::Reg<dfsdm2_awcfr::DFSDM2_AWCFRrs>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm2_awcfr;
#[doc = "DFSDM3_AWCFR (rw) register accessor: DFSDM analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm3_awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_awcfr`]
module"]
pub type DFSDM3_AWCFR = crate::Reg<dfsdm3_awcfr::DFSDM3_AWCFRrs>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm3_awcfr;
#[doc = "DFSDM0_EXMAX (r) register accessor: DFSDM Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_exmax`]
module"]
pub type DFSDM0_EXMAX = crate::Reg<dfsdm0_exmax::DFSDM0_EXMAXrs>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm0_exmax;
#[doc = "EXMAX (r) register accessor: DFSDM Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exmax`]
module"]
pub type EXMAX = crate::Reg<exmax::EXMAXrs>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod exmax;
#[doc = "DFSDM2_EXMAX (r) register accessor: DFSDM Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_exmax`]
module"]
pub type DFSDM2_EXMAX = crate::Reg<dfsdm2_exmax::DFSDM2_EXMAXrs>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm2_exmax;
#[doc = "DFSDM3_EXMAX (r) register accessor: DFSDM Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_exmax`]
module"]
pub type DFSDM3_EXMAX = crate::Reg<dfsdm3_exmax::DFSDM3_EXMAXrs>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm3_exmax;
#[doc = "DFSDM0_EXMIN (r) register accessor: DFSDM Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_exmin`]
module"]
pub type DFSDM0_EXMIN = crate::Reg<dfsdm0_exmin::DFSDM0_EXMINrs>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm0_exmin;
#[doc = "EXMIN (r) register accessor: DFSDM Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exmin`]
module"]
pub type EXMIN = crate::Reg<exmin::EXMINrs>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod exmin;
#[doc = "DFSDM2_EXMIN (r) register accessor: DFSDM Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_exmin`]
module"]
pub type DFSDM2_EXMIN = crate::Reg<dfsdm2_exmin::DFSDM2_EXMINrs>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm2_exmin;
#[doc = "DFSDM3_EXMIN (r) register accessor: DFSDM Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_exmin`]
module"]
pub type DFSDM3_EXMIN = crate::Reg<dfsdm3_exmin::DFSDM3_EXMINrs>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm3_exmin;
#[doc = "DFSDM0_CNVTIMR (r) register accessor: DFSDM conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm0_cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm0_cnvtimr`]
module"]
pub type DFSDM0_CNVTIMR = crate::Reg<dfsdm0_cnvtimr::DFSDM0_CNVTIMRrs>;
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm0_cnvtimr;
#[doc = "CNVTIMR (r) register accessor: DFSDM conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnvtimr`]
module"]
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMRrs>;
#[doc = "DFSDM conversion timer register"]
pub mod cnvtimr;
#[doc = "DFSDM2_CNVTIMR (r) register accessor: DFSDM conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm2_cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm2_cnvtimr`]
module"]
pub type DFSDM2_CNVTIMR = crate::Reg<dfsdm2_cnvtimr::DFSDM2_CNVTIMRrs>;
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm2_cnvtimr;
#[doc = "DFSDM3_CNVTIMR (r) register accessor: DFSDM conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm3_cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfsdm3_cnvtimr`]
module"]
pub type DFSDM3_CNVTIMR = crate::Reg<dfsdm3_cnvtimr::DFSDM3_CNVTIMRrs>;
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm3_cnvtimr;
