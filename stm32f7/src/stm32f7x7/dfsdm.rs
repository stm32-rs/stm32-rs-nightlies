#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DFSDM channel configuration 0 register 1"]
    pub dfsdm_chcfg0r1: DFSDM_CHCFG0R1,
    #[doc = "0x04 - DFSDM channel configuration 0 register 2"]
    pub dfsdm_chcfg0r2: DFSDM_CHCFG0R2,
    #[doc = "0x08 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd0r: DFSDM_AWSCD0R,
    #[doc = "0x0c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat0r: DFSDM_CHWDAT0R,
    #[doc = "0x10 - DFSDM channel data input register"]
    pub dfsdm_chdatin0r: DFSDM_CHDATIN0R,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - DFSDM channel configuration 1 register 1"]
    pub dfsdm_chcfg1r1: DFSDM_CHCFG1R1,
    #[doc = "0x24 - DFSDM channel configuration 1 register 2"]
    pub dfsdm_chcfg1r2: DFSDM_CHCFG1R2,
    #[doc = "0x28 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd1r: DFSDM_AWSCD1R,
    #[doc = "0x2c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat1r: DFSDM_CHWDAT1R,
    #[doc = "0x30 - DFSDM channel data input register"]
    pub dfsdm_chdatin1r: DFSDM_CHDATIN1R,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - DFSDM channel configuration 2 register 1"]
    pub dfsdm_chcfg2r1: DFSDM_CHCFG2R1,
    #[doc = "0x44 - DFSDM channel configuration 2 register 2"]
    pub dfsdm_chcfg2r2: DFSDM_CHCFG2R2,
    #[doc = "0x48 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd2r: DFSDM_AWSCD2R,
    #[doc = "0x4c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat2r: DFSDM_CHWDAT2R,
    #[doc = "0x50 - DFSDM channel data input register"]
    pub dfsdm_chdatin2r: DFSDM_CHDATIN2R,
    _reserved15: [u8; 12usize],
    #[doc = "0x60 - DFSDM channel configuration 3 register 1"]
    pub dfsdm_chcfg3r1: DFSDM_CHCFG3R1,
    #[doc = "0x64 - DFSDM channel configuration 3 register 2"]
    pub dfsdm_chcfg3r2: DFSDM_CHCFG3R2,
    #[doc = "0x68 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd3r: DFSDM_AWSCD3R,
    #[doc = "0x6c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat3r: DFSDM_CHWDAT3R,
    #[doc = "0x70 - DFSDM channel data input register"]
    pub dfsdm_chdatin3r: DFSDM_CHDATIN3R,
    _reserved20: [u8; 12usize],
    #[doc = "0x80 - DFSDM channel configuration 4 register 1"]
    pub dfsdm_chcfg4r1: DFSDM_CHCFG4R1,
    #[doc = "0x84 - DFSDM channel configuration 4 register 2"]
    pub dfsdm_chcfg4r2: DFSDM_CHCFG4R2,
    #[doc = "0x88 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd4r: DFSDM_AWSCD4R,
    #[doc = "0x8c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat4r: DFSDM_CHWDAT4R,
    #[doc = "0x90 - DFSDM channel data input register"]
    pub dfsdm_chdatin4r: DFSDM_CHDATIN4R,
    _reserved25: [u8; 12usize],
    #[doc = "0xa0 - DFSDM channel configuration 5 register 1"]
    pub dfsdm_chcfg5r1: DFSDM_CHCFG5R1,
    #[doc = "0xa4 - DFSDM channel configuration 5 register 2"]
    pub dfsdm_chcfg5r2: DFSDM_CHCFG5R2,
    #[doc = "0xa8 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd5r: DFSDM_AWSCD5R,
    #[doc = "0xac - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat5r: DFSDM_CHWDAT5R,
    #[doc = "0xb0 - DFSDM channel data input register"]
    pub dfsdm_chdatin5r: DFSDM_CHDATIN5R,
    _reserved30: [u8; 12usize],
    #[doc = "0xc0 - DFSDM channel configuration 6 register 1"]
    pub dfsdm_chcfg6r1: DFSDM_CHCFG6R1,
    #[doc = "0xc4 - DFSDM channel configuration 6 register 2"]
    pub dfsdm_chcfg6r2: DFSDM_CHCFG6R2,
    #[doc = "0xc8 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd6r: DFSDM_AWSCD6R,
    #[doc = "0xcc - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat6r: DFSDM_CHWDAT6R,
    #[doc = "0xd0 - DFSDM channel data input register"]
    pub dfsdm_chdatin6r: DFSDM_CHDATIN6R,
    _reserved35: [u8; 12usize],
    #[doc = "0xe0 - DFSDM channel configuration 7 register 1"]
    pub dfsdm_chcfg7r1: DFSDM_CHCFG7R1,
    #[doc = "0xe4 - DFSDM channel configuration 7 register 2"]
    pub dfsdm_chcfg7r2: DFSDM_CHCFG7R2,
    #[doc = "0xe8 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd7r: DFSDM_AWSCD7R,
    #[doc = "0xec - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat7r: DFSDM_CHWDAT7R,
    #[doc = "0xf0 - DFSDM channel data input register"]
    pub dfsdm_chdatin7r: DFSDM_CHDATIN7R,
    _reserved40: [u8; 12usize],
    #[doc = "0x100 - DFSDM control register 1"]
    pub dfsdm0_cr1: DFSDM0_CR1,
    #[doc = "0x104 - DFSDM control register 2"]
    pub dfsdm0_cr2: DFSDM0_CR2,
    #[doc = "0x108 - DFSDM interrupt and status register"]
    pub dfsdm0_isr: DFSDM0_ISR,
    #[doc = "0x10c - DFSDM interrupt flag clear register"]
    pub dfsdm0_icr: DFSDM0_ICR,
    #[doc = "0x110 - DFSDM injected channel group selection register"]
    pub dfsdm0_jchgr: DFSDM0_JCHGR,
    #[doc = "0x114 - DFSDM filter control register"]
    pub dfsdm0_fcr: DFSDM0_FCR,
    #[doc = "0x118 - DFSDM data register for injected group"]
    pub dfsdm0_jdatar: DFSDM0_JDATAR,
    #[doc = "0x11c - DFSDM data register for the regular channel"]
    pub dfsdm0_rdatar: DFSDM0_RDATAR,
    #[doc = "0x120 - DFSDM analog watchdog high threshold register"]
    pub dfsdm0_awhtr: DFSDM0_AWHTR,
    #[doc = "0x124 - DFSDM analog watchdog low threshold register"]
    pub dfsdm0_awltr: DFSDM0_AWLTR,
    #[doc = "0x128 - DFSDM analog watchdog status register"]
    pub dfsdm0_awsr: DFSDM0_AWSR,
    #[doc = "0x12c - DFSDM analog watchdog clear flag register"]
    pub dfsdm0_awcfr: DFSDM0_AWCFR,
    #[doc = "0x130 - DFSDM Extremes detector maximum register"]
    pub dfsdm0_exmax: DFSDM0_EXMAX,
    #[doc = "0x134 - DFSDM Extremes detector minimum register"]
    pub dfsdm0_exmin: DFSDM0_EXMIN,
    #[doc = "0x138 - DFSDM conversion timer register"]
    pub dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    _reserved55: [u8; 68usize],
    #[doc = "0x180 - DFSDM control register 1"]
    pub dfsdm1_cr1: DFSDM1_CR1,
    #[doc = "0x184 - DFSDM control register 2"]
    pub dfsdm1_cr2: DFSDM1_CR2,
    #[doc = "0x188 - DFSDM interrupt and status register"]
    pub dfsdm1_isr: DFSDM1_ISR,
    #[doc = "0x18c - DFSDM interrupt flag clear register"]
    pub dfsdm1_icr: DFSDM1_ICR,
    #[doc = "0x190 - DFSDM injected channel group selection register"]
    pub dfsdm1_jchgr: DFSDM1_JCHGR,
    #[doc = "0x194 - DFSDM filter control register"]
    pub dfsdm1_fcr: DFSDM1_FCR,
    _reserved_61_dfsdm1: [u8; 4usize],
    _reserved62: [u8; 4usize],
    #[doc = "0x1a0 - DFSDM analog watchdog high threshold register"]
    pub dfsdm1_awhtr: DFSDM1_AWHTR,
    #[doc = "0x1a4 - DFSDM analog watchdog low threshold register"]
    pub dfsdm1_awltr: DFSDM1_AWLTR,
    #[doc = "0x1a8 - DFSDM analog watchdog status register"]
    pub dfsdm1_awsr: DFSDM1_AWSR,
    #[doc = "0x1ac - DFSDM analog watchdog clear flag register"]
    pub dfsdm1_awcfr: DFSDM1_AWCFR,
    #[doc = "0x1b0 - DFSDM Extremes detector maximum register"]
    pub dfsdm1_exmax: DFSDM1_EXMAX,
    #[doc = "0x1b4 - DFSDM Extremes detector minimum register"]
    pub dfsdm1_exmin: DFSDM1_EXMIN,
    #[doc = "0x1b8 - DFSDM conversion timer register"]
    pub dfsdm1_cnvtimr: DFSDM1_CNVTIMR,
    _reserved69: [u8; 68usize],
    #[doc = "0x200 - DFSDM control register 1"]
    pub dfsdm2_cr1: DFSDM2_CR1,
    #[doc = "0x204 - DFSDM control register 2"]
    pub dfsdm2_cr2: DFSDM2_CR2,
    #[doc = "0x208 - DFSDM interrupt and status register"]
    pub dfsdm2_isr: DFSDM2_ISR,
    #[doc = "0x20c - DFSDM interrupt flag clear register"]
    pub dfsdm2_icr: DFSDM2_ICR,
    #[doc = "0x210 - DFSDM injected channel group selection register"]
    pub dfsdm2_jchgr: DFSDM2_JCHGR,
    #[doc = "0x214 - DFSDM filter control register"]
    pub dfsdm2_fcr: DFSDM2_FCR,
    _reserved_75_dfsdm2: [u8; 4usize],
    _reserved76: [u8; 4usize],
    #[doc = "0x220 - DFSDM analog watchdog high threshold register"]
    pub dfsdm2_awhtr: DFSDM2_AWHTR,
    #[doc = "0x224 - DFSDM analog watchdog low threshold register"]
    pub dfsdm2_awltr: DFSDM2_AWLTR,
    #[doc = "0x228 - DFSDM analog watchdog status register"]
    pub dfsdm2_awsr: DFSDM2_AWSR,
    #[doc = "0x22c - DFSDM analog watchdog clear flag register"]
    pub dfsdm2_awcfr: DFSDM2_AWCFR,
    #[doc = "0x230 - DFSDM Extremes detector maximum register"]
    pub dfsdm2_exmax: DFSDM2_EXMAX,
    #[doc = "0x234 - DFSDM Extremes detector minimum register"]
    pub dfsdm2_exmin: DFSDM2_EXMIN,
    #[doc = "0x238 - DFSDM conversion timer register"]
    pub dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    _reserved83: [u8; 100usize],
    #[doc = "0x2a0 - DFSDM analog watchdog high threshold register"]
    pub dfsdm3_awhtr: DFSDM3_AWHTR,
    #[doc = "0x2a4 - DFSDM analog watchdog low threshold register"]
    pub dfsdm3_awltr: DFSDM3_AWLTR,
    #[doc = "0x2a8 - DFSDM analog watchdog status register"]
    pub dfsdm3_awsr: DFSDM3_AWSR,
    #[doc = "0x2ac - DFSDM analog watchdog clear flag register"]
    pub dfsdm3_awcfr: DFSDM3_AWCFR,
    #[doc = "0x2b0 - DFSDM Extremes detector maximum register"]
    pub dfsdm3_exmax: DFSDM3_EXMAX,
    #[doc = "0x2b4 - DFSDM Extremes detector minimum register"]
    pub dfsdm3_exmin: DFSDM3_EXMIN,
    #[doc = "0x2b8 - DFSDM conversion timer register"]
    pub dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
    _reserved90: [u8; 84usize],
    #[doc = "0x310 - DFSDM injected channel group selection register"]
    pub dfsdm3_jchgr: DFSDM3_JCHGR,
    #[doc = "0x314 - DFSDM filter control register"]
    pub dfsdm3_fcr: DFSDM3_FCR,
    _reserved_92_dfsdm3: [u8; 4usize],
    _reserved93: [u8; 100usize],
    #[doc = "0x380 - DFSDM control register 1"]
    pub dfsdm3_cr1: DFSDM3_CR1,
    #[doc = "0x384 - DFSDM control register 2"]
    pub dfsdm3_cr2: DFSDM3_CR2,
    #[doc = "0x388 - DFSDM interrupt and status register"]
    pub dfsdm3_isr: DFSDM3_ISR,
    #[doc = "0x38c - DFSDM interrupt flag clear register"]
    pub dfsdm3_icr: DFSDM3_ICR,
}
impl RegisterBlock {
    #[doc = "0x198 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub fn dfsdm1_rdatar(&self) -> &DFSDM1_RDATAR {
        unsafe { &*(((self as *const Self) as *const u8).add(408usize) as *const DFSDM1_RDATAR) }
    }
    #[doc = "0x198 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub fn dfsdm1_rdatar_mut(&self) -> &mut DFSDM1_RDATAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(408usize) as *mut DFSDM1_RDATAR) }
    }
    #[doc = "0x198 - DFSDM data register for injected group"]
    #[inline(always)]
    pub fn dfsdm1_jdatar(&self) -> &DFSDM1_JDATAR {
        unsafe { &*(((self as *const Self) as *const u8).add(408usize) as *const DFSDM1_JDATAR) }
    }
    #[doc = "0x198 - DFSDM data register for injected group"]
    #[inline(always)]
    pub fn dfsdm1_jdatar_mut(&self) -> &mut DFSDM1_JDATAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(408usize) as *mut DFSDM1_JDATAR) }
    }
    #[doc = "0x218 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub fn dfsdm2_rdatar(&self) -> &DFSDM2_RDATAR {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const DFSDM2_RDATAR) }
    }
    #[doc = "0x218 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub fn dfsdm2_rdatar_mut(&self) -> &mut DFSDM2_RDATAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut DFSDM2_RDATAR) }
    }
    #[doc = "0x218 - DFSDM data register for injected group"]
    #[inline(always)]
    pub fn dfsdm2_jdatar(&self) -> &DFSDM2_JDATAR {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const DFSDM2_JDATAR) }
    }
    #[doc = "0x218 - DFSDM data register for injected group"]
    #[inline(always)]
    pub fn dfsdm2_jdatar_mut(&self) -> &mut DFSDM2_JDATAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut DFSDM2_JDATAR) }
    }
    #[doc = "0x318 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub fn dfsdm3_rdatar(&self) -> &DFSDM3_RDATAR {
        unsafe { &*(((self as *const Self) as *const u8).add(792usize) as *const DFSDM3_RDATAR) }
    }
    #[doc = "0x318 - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub fn dfsdm3_rdatar_mut(&self) -> &mut DFSDM3_RDATAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(792usize) as *mut DFSDM3_RDATAR) }
    }
    #[doc = "0x318 - DFSDM data register for injected group"]
    #[inline(always)]
    pub fn dfsdm3_jdatar(&self) -> &DFSDM3_JDATAR {
        unsafe { &*(((self as *const Self) as *const u8).add(792usize) as *const DFSDM3_JDATAR) }
    }
    #[doc = "0x318 - DFSDM data register for injected group"]
    #[inline(always)]
    pub fn dfsdm3_jdatar_mut(&self) -> &mut DFSDM3_JDATAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(792usize) as *mut DFSDM3_JDATAR) }
    }
}
#[doc = "DFSDM channel configuration 0 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg0r1](dfsdm_chcfg0r1) module"]
pub type DFSDM_CHCFG0R1 = crate::Reg<u32, _DFSDM_CHCFG0R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG0R1;
#[doc = "`read()` method returns [dfsdm_chcfg0r1::R](dfsdm_chcfg0r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG0R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg0r1::W](dfsdm_chcfg0r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG0R1 {}
#[doc = "DFSDM channel configuration 0 register 1"]
pub mod dfsdm_chcfg0r1;
#[doc = "DFSDM channel configuration 1 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg1r1](dfsdm_chcfg1r1) module"]
pub type DFSDM_CHCFG1R1 = crate::Reg<u32, _DFSDM_CHCFG1R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG1R1;
#[doc = "`read()` method returns [dfsdm_chcfg1r1::R](dfsdm_chcfg1r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG1R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg1r1::W](dfsdm_chcfg1r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG1R1 {}
#[doc = "DFSDM channel configuration 1 register 1"]
pub mod dfsdm_chcfg1r1;
#[doc = "DFSDM channel configuration 2 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg2r1](dfsdm_chcfg2r1) module"]
pub type DFSDM_CHCFG2R1 = crate::Reg<u32, _DFSDM_CHCFG2R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG2R1;
#[doc = "`read()` method returns [dfsdm_chcfg2r1::R](dfsdm_chcfg2r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG2R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg2r1::W](dfsdm_chcfg2r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG2R1 {}
#[doc = "DFSDM channel configuration 2 register 1"]
pub mod dfsdm_chcfg2r1;
#[doc = "DFSDM channel configuration 3 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg3r1](dfsdm_chcfg3r1) module"]
pub type DFSDM_CHCFG3R1 = crate::Reg<u32, _DFSDM_CHCFG3R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG3R1;
#[doc = "`read()` method returns [dfsdm_chcfg3r1::R](dfsdm_chcfg3r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG3R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg3r1::W](dfsdm_chcfg3r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG3R1 {}
#[doc = "DFSDM channel configuration 3 register 1"]
pub mod dfsdm_chcfg3r1;
#[doc = "DFSDM channel configuration 4 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg4r1](dfsdm_chcfg4r1) module"]
pub type DFSDM_CHCFG4R1 = crate::Reg<u32, _DFSDM_CHCFG4R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG4R1;
#[doc = "`read()` method returns [dfsdm_chcfg4r1::R](dfsdm_chcfg4r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG4R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg4r1::W](dfsdm_chcfg4r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG4R1 {}
#[doc = "DFSDM channel configuration 4 register 1"]
pub mod dfsdm_chcfg4r1;
#[doc = "DFSDM channel configuration 5 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg5r1](dfsdm_chcfg5r1) module"]
pub type DFSDM_CHCFG5R1 = crate::Reg<u32, _DFSDM_CHCFG5R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG5R1;
#[doc = "`read()` method returns [dfsdm_chcfg5r1::R](dfsdm_chcfg5r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG5R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg5r1::W](dfsdm_chcfg5r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG5R1 {}
#[doc = "DFSDM channel configuration 5 register 1"]
pub mod dfsdm_chcfg5r1;
#[doc = "DFSDM channel configuration 6 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg6r1](dfsdm_chcfg6r1) module"]
pub type DFSDM_CHCFG6R1 = crate::Reg<u32, _DFSDM_CHCFG6R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG6R1;
#[doc = "`read()` method returns [dfsdm_chcfg6r1::R](dfsdm_chcfg6r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG6R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg6r1::W](dfsdm_chcfg6r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG6R1 {}
#[doc = "DFSDM channel configuration 6 register 1"]
pub mod dfsdm_chcfg6r1;
#[doc = "DFSDM channel configuration 7 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg7r1](dfsdm_chcfg7r1) module"]
pub type DFSDM_CHCFG7R1 = crate::Reg<u32, _DFSDM_CHCFG7R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG7R1;
#[doc = "`read()` method returns [dfsdm_chcfg7r1::R](dfsdm_chcfg7r1::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG7R1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg7r1::W](dfsdm_chcfg7r1::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG7R1 {}
#[doc = "DFSDM channel configuration 7 register 1"]
pub mod dfsdm_chcfg7r1;
#[doc = "DFSDM channel configuration 0 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg0r2](dfsdm_chcfg0r2) module"]
pub type DFSDM_CHCFG0R2 = crate::Reg<u32, _DFSDM_CHCFG0R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG0R2;
#[doc = "`read()` method returns [dfsdm_chcfg0r2::R](dfsdm_chcfg0r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG0R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg0r2::W](dfsdm_chcfg0r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG0R2 {}
#[doc = "DFSDM channel configuration 0 register 2"]
pub mod dfsdm_chcfg0r2;
#[doc = "DFSDM channel configuration 1 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg1r2](dfsdm_chcfg1r2) module"]
pub type DFSDM_CHCFG1R2 = crate::Reg<u32, _DFSDM_CHCFG1R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG1R2;
#[doc = "`read()` method returns [dfsdm_chcfg1r2::R](dfsdm_chcfg1r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG1R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg1r2::W](dfsdm_chcfg1r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG1R2 {}
#[doc = "DFSDM channel configuration 1 register 2"]
pub mod dfsdm_chcfg1r2;
#[doc = "DFSDM channel configuration 2 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg2r2](dfsdm_chcfg2r2) module"]
pub type DFSDM_CHCFG2R2 = crate::Reg<u32, _DFSDM_CHCFG2R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG2R2;
#[doc = "`read()` method returns [dfsdm_chcfg2r2::R](dfsdm_chcfg2r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG2R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg2r2::W](dfsdm_chcfg2r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG2R2 {}
#[doc = "DFSDM channel configuration 2 register 2"]
pub mod dfsdm_chcfg2r2;
#[doc = "DFSDM channel configuration 3 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg3r2](dfsdm_chcfg3r2) module"]
pub type DFSDM_CHCFG3R2 = crate::Reg<u32, _DFSDM_CHCFG3R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG3R2;
#[doc = "`read()` method returns [dfsdm_chcfg3r2::R](dfsdm_chcfg3r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG3R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg3r2::W](dfsdm_chcfg3r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG3R2 {}
#[doc = "DFSDM channel configuration 3 register 2"]
pub mod dfsdm_chcfg3r2;
#[doc = "DFSDM channel configuration 4 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg4r2](dfsdm_chcfg4r2) module"]
pub type DFSDM_CHCFG4R2 = crate::Reg<u32, _DFSDM_CHCFG4R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG4R2;
#[doc = "`read()` method returns [dfsdm_chcfg4r2::R](dfsdm_chcfg4r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG4R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg4r2::W](dfsdm_chcfg4r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG4R2 {}
#[doc = "DFSDM channel configuration 4 register 2"]
pub mod dfsdm_chcfg4r2;
#[doc = "DFSDM channel configuration 5 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg5r2](dfsdm_chcfg5r2) module"]
pub type DFSDM_CHCFG5R2 = crate::Reg<u32, _DFSDM_CHCFG5R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG5R2;
#[doc = "`read()` method returns [dfsdm_chcfg5r2::R](dfsdm_chcfg5r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG5R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg5r2::W](dfsdm_chcfg5r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG5R2 {}
#[doc = "DFSDM channel configuration 5 register 2"]
pub mod dfsdm_chcfg5r2;
#[doc = "DFSDM channel configuration 6 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg6r2](dfsdm_chcfg6r2) module"]
pub type DFSDM_CHCFG6R2 = crate::Reg<u32, _DFSDM_CHCFG6R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG6R2;
#[doc = "`read()` method returns [dfsdm_chcfg6r2::R](dfsdm_chcfg6r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG6R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg6r2::W](dfsdm_chcfg6r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG6R2 {}
#[doc = "DFSDM channel configuration 6 register 2"]
pub mod dfsdm_chcfg6r2;
#[doc = "DFSDM channel configuration 7 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg7r2](dfsdm_chcfg7r2) module"]
pub type DFSDM_CHCFG7R2 = crate::Reg<u32, _DFSDM_CHCFG7R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHCFG7R2;
#[doc = "`read()` method returns [dfsdm_chcfg7r2::R](dfsdm_chcfg7r2::R) reader structure"]
impl crate::Readable for DFSDM_CHCFG7R2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg7r2::W](dfsdm_chcfg7r2::W) writer structure"]
impl crate::Writable for DFSDM_CHCFG7R2 {}
#[doc = "DFSDM channel configuration 7 register 2"]
pub mod dfsdm_chcfg7r2;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd0r](dfsdm_awscd0r) module"]
pub type DFSDM_AWSCD0R = crate::Reg<u32, _DFSDM_AWSCD0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD0R;
#[doc = "`read()` method returns [dfsdm_awscd0r::R](dfsdm_awscd0r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD0R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd0r::W](dfsdm_awscd0r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD0R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd0r;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd1r](dfsdm_awscd1r) module"]
pub type DFSDM_AWSCD1R = crate::Reg<u32, _DFSDM_AWSCD1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD1R;
#[doc = "`read()` method returns [dfsdm_awscd1r::R](dfsdm_awscd1r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD1R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd1r::W](dfsdm_awscd1r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD1R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd1r;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd2r](dfsdm_awscd2r) module"]
pub type DFSDM_AWSCD2R = crate::Reg<u32, _DFSDM_AWSCD2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD2R;
#[doc = "`read()` method returns [dfsdm_awscd2r::R](dfsdm_awscd2r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD2R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd2r::W](dfsdm_awscd2r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD2R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd2r;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd3r](dfsdm_awscd3r) module"]
pub type DFSDM_AWSCD3R = crate::Reg<u32, _DFSDM_AWSCD3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD3R;
#[doc = "`read()` method returns [dfsdm_awscd3r::R](dfsdm_awscd3r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD3R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd3r::W](dfsdm_awscd3r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD3R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd3r;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd4r](dfsdm_awscd4r) module"]
pub type DFSDM_AWSCD4R = crate::Reg<u32, _DFSDM_AWSCD4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD4R;
#[doc = "`read()` method returns [dfsdm_awscd4r::R](dfsdm_awscd4r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD4R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd4r::W](dfsdm_awscd4r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD4R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd4r;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd5r](dfsdm_awscd5r) module"]
pub type DFSDM_AWSCD5R = crate::Reg<u32, _DFSDM_AWSCD5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD5R;
#[doc = "`read()` method returns [dfsdm_awscd5r::R](dfsdm_awscd5r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD5R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd5r::W](dfsdm_awscd5r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD5R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd5r;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd6r](dfsdm_awscd6r) module"]
pub type DFSDM_AWSCD6R = crate::Reg<u32, _DFSDM_AWSCD6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD6R;
#[doc = "`read()` method returns [dfsdm_awscd6r::R](dfsdm_awscd6r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD6R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd6r::W](dfsdm_awscd6r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD6R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd6r;
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_awscd7r](dfsdm_awscd7r) module"]
pub type DFSDM_AWSCD7R = crate::Reg<u32, _DFSDM_AWSCD7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_AWSCD7R;
#[doc = "`read()` method returns [dfsdm_awscd7r::R](dfsdm_awscd7r::R) reader structure"]
impl crate::Readable for DFSDM_AWSCD7R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_awscd7r::W](dfsdm_awscd7r::W) writer structure"]
impl crate::Writable for DFSDM_AWSCD7R {}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd7r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat0r](dfsdm_chwdat0r) module"]
pub type DFSDM_CHWDAT0R = crate::Reg<u32, _DFSDM_CHWDAT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT0R;
#[doc = "`read()` method returns [dfsdm_chwdat0r::R](dfsdm_chwdat0r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT0R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat0r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat1r](dfsdm_chwdat1r) module"]
pub type DFSDM_CHWDAT1R = crate::Reg<u32, _DFSDM_CHWDAT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT1R;
#[doc = "`read()` method returns [dfsdm_chwdat1r::R](dfsdm_chwdat1r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT1R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat1r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat2r](dfsdm_chwdat2r) module"]
pub type DFSDM_CHWDAT2R = crate::Reg<u32, _DFSDM_CHWDAT2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT2R;
#[doc = "`read()` method returns [dfsdm_chwdat2r::R](dfsdm_chwdat2r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT2R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat2r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat3r](dfsdm_chwdat3r) module"]
pub type DFSDM_CHWDAT3R = crate::Reg<u32, _DFSDM_CHWDAT3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT3R;
#[doc = "`read()` method returns [dfsdm_chwdat3r::R](dfsdm_chwdat3r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT3R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat3r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat4r](dfsdm_chwdat4r) module"]
pub type DFSDM_CHWDAT4R = crate::Reg<u32, _DFSDM_CHWDAT4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT4R;
#[doc = "`read()` method returns [dfsdm_chwdat4r::R](dfsdm_chwdat4r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT4R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat4r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat5r](dfsdm_chwdat5r) module"]
pub type DFSDM_CHWDAT5R = crate::Reg<u32, _DFSDM_CHWDAT5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT5R;
#[doc = "`read()` method returns [dfsdm_chwdat5r::R](dfsdm_chwdat5r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT5R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat5r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat6r](dfsdm_chwdat6r) module"]
pub type DFSDM_CHWDAT6R = crate::Reg<u32, _DFSDM_CHWDAT6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT6R;
#[doc = "`read()` method returns [dfsdm_chwdat6r::R](dfsdm_chwdat6r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT6R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat6r;
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat7r](dfsdm_chwdat7r) module"]
pub type DFSDM_CHWDAT7R = crate::Reg<u32, _DFSDM_CHWDAT7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHWDAT7R;
#[doc = "`read()` method returns [dfsdm_chwdat7r::R](dfsdm_chwdat7r::R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT7R {}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat7r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin0r](dfsdm_chdatin0r) module"]
pub type DFSDM_CHDATIN0R = crate::Reg<u32, _DFSDM_CHDATIN0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN0R;
#[doc = "`read()` method returns [dfsdm_chdatin0r::R](dfsdm_chdatin0r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN0R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin0r::W](dfsdm_chdatin0r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN0R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin0r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin1r](dfsdm_chdatin1r) module"]
pub type DFSDM_CHDATIN1R = crate::Reg<u32, _DFSDM_CHDATIN1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN1R;
#[doc = "`read()` method returns [dfsdm_chdatin1r::R](dfsdm_chdatin1r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN1R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin1r::W](dfsdm_chdatin1r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN1R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin1r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin2r](dfsdm_chdatin2r) module"]
pub type DFSDM_CHDATIN2R = crate::Reg<u32, _DFSDM_CHDATIN2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN2R;
#[doc = "`read()` method returns [dfsdm_chdatin2r::R](dfsdm_chdatin2r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN2R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin2r::W](dfsdm_chdatin2r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN2R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin2r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin3r](dfsdm_chdatin3r) module"]
pub type DFSDM_CHDATIN3R = crate::Reg<u32, _DFSDM_CHDATIN3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN3R;
#[doc = "`read()` method returns [dfsdm_chdatin3r::R](dfsdm_chdatin3r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN3R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin3r::W](dfsdm_chdatin3r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN3R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin3r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin4r](dfsdm_chdatin4r) module"]
pub type DFSDM_CHDATIN4R = crate::Reg<u32, _DFSDM_CHDATIN4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN4R;
#[doc = "`read()` method returns [dfsdm_chdatin4r::R](dfsdm_chdatin4r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN4R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin4r::W](dfsdm_chdatin4r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN4R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin4r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin5r](dfsdm_chdatin5r) module"]
pub type DFSDM_CHDATIN5R = crate::Reg<u32, _DFSDM_CHDATIN5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN5R;
#[doc = "`read()` method returns [dfsdm_chdatin5r::R](dfsdm_chdatin5r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN5R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin5r::W](dfsdm_chdatin5r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN5R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin5r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin6r](dfsdm_chdatin6r) module"]
pub type DFSDM_CHDATIN6R = crate::Reg<u32, _DFSDM_CHDATIN6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN6R;
#[doc = "`read()` method returns [dfsdm_chdatin6r::R](dfsdm_chdatin6r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN6R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin6r::W](dfsdm_chdatin6r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN6R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin6r;
#[doc = "DFSDM channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chdatin7r](dfsdm_chdatin7r) module"]
pub type DFSDM_CHDATIN7R = crate::Reg<u32, _DFSDM_CHDATIN7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CHDATIN7R;
#[doc = "`read()` method returns [dfsdm_chdatin7r::R](dfsdm_chdatin7r::R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN7R {}
#[doc = "`write(|w| ..)` method takes [dfsdm_chdatin7r::W](dfsdm_chdatin7r::W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN7R {}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin7r;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_cr1](dfsdm0_cr1) module"]
pub type DFSDM0_CR1 = crate::Reg<u32, _DFSDM0_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CR1;
#[doc = "`read()` method returns [dfsdm0_cr1::R](dfsdm0_cr1::R) reader structure"]
impl crate::Readable for DFSDM0_CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_cr1::W](dfsdm0_cr1::W) writer structure"]
impl crate::Writable for DFSDM0_CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm0_cr1;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_cr1](dfsdm1_cr1) module"]
pub type DFSDM1_CR1 = crate::Reg<u32, _DFSDM1_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CR1;
#[doc = "`read()` method returns [dfsdm1_cr1::R](dfsdm1_cr1::R) reader structure"]
impl crate::Readable for DFSDM1_CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_cr1::W](dfsdm1_cr1::W) writer structure"]
impl crate::Writable for DFSDM1_CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm1_cr1;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_cr1](dfsdm2_cr1) module"]
pub type DFSDM2_CR1 = crate::Reg<u32, _DFSDM2_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CR1;
#[doc = "`read()` method returns [dfsdm2_cr1::R](dfsdm2_cr1::R) reader structure"]
impl crate::Readable for DFSDM2_CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_cr1::W](dfsdm2_cr1::W) writer structure"]
impl crate::Writable for DFSDM2_CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm2_cr1;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_cr1](dfsdm3_cr1) module"]
pub type DFSDM3_CR1 = crate::Reg<u32, _DFSDM3_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CR1;
#[doc = "`read()` method returns [dfsdm3_cr1::R](dfsdm3_cr1::R) reader structure"]
impl crate::Readable for DFSDM3_CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_cr1::W](dfsdm3_cr1::W) writer structure"]
impl crate::Writable for DFSDM3_CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm3_cr1;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_cr2](dfsdm0_cr2) module"]
pub type DFSDM0_CR2 = crate::Reg<u32, _DFSDM0_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CR2;
#[doc = "`read()` method returns [dfsdm0_cr2::R](dfsdm0_cr2::R) reader structure"]
impl crate::Readable for DFSDM0_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_cr2::W](dfsdm0_cr2::W) writer structure"]
impl crate::Writable for DFSDM0_CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm0_cr2;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_cr2](dfsdm1_cr2) module"]
pub type DFSDM1_CR2 = crate::Reg<u32, _DFSDM1_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CR2;
#[doc = "`read()` method returns [dfsdm1_cr2::R](dfsdm1_cr2::R) reader structure"]
impl crate::Readable for DFSDM1_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_cr2::W](dfsdm1_cr2::W) writer structure"]
impl crate::Writable for DFSDM1_CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm1_cr2;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_cr2](dfsdm2_cr2) module"]
pub type DFSDM2_CR2 = crate::Reg<u32, _DFSDM2_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CR2;
#[doc = "`read()` method returns [dfsdm2_cr2::R](dfsdm2_cr2::R) reader structure"]
impl crate::Readable for DFSDM2_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_cr2::W](dfsdm2_cr2::W) writer structure"]
impl crate::Writable for DFSDM2_CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm2_cr2;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_cr2](dfsdm3_cr2) module"]
pub type DFSDM3_CR2 = crate::Reg<u32, _DFSDM3_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CR2;
#[doc = "`read()` method returns [dfsdm3_cr2::R](dfsdm3_cr2::R) reader structure"]
impl crate::Readable for DFSDM3_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_cr2::W](dfsdm3_cr2::W) writer structure"]
impl crate::Writable for DFSDM3_CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm3_cr2;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_isr](dfsdm0_isr) module"]
pub type DFSDM0_ISR = crate::Reg<u32, _DFSDM0_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_ISR;
#[doc = "`read()` method returns [dfsdm0_isr::R](dfsdm0_isr::R) reader structure"]
impl crate::Readable for DFSDM0_ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm0_isr;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_isr](dfsdm1_isr) module"]
pub type DFSDM1_ISR = crate::Reg<u32, _DFSDM1_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_ISR;
#[doc = "`read()` method returns [dfsdm1_isr::R](dfsdm1_isr::R) reader structure"]
impl crate::Readable for DFSDM1_ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm1_isr;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_isr](dfsdm2_isr) module"]
pub type DFSDM2_ISR = crate::Reg<u32, _DFSDM2_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_ISR;
#[doc = "`read()` method returns [dfsdm2_isr::R](dfsdm2_isr::R) reader structure"]
impl crate::Readable for DFSDM2_ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm2_isr;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_isr](dfsdm3_isr) module"]
pub type DFSDM3_ISR = crate::Reg<u32, _DFSDM3_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_ISR;
#[doc = "`read()` method returns [dfsdm3_isr::R](dfsdm3_isr::R) reader structure"]
impl crate::Readable for DFSDM3_ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm3_isr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_icr](dfsdm0_icr) module"]
pub type DFSDM0_ICR = crate::Reg<u32, _DFSDM0_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_ICR;
#[doc = "`read()` method returns [dfsdm0_icr::R](dfsdm0_icr::R) reader structure"]
impl crate::Readable for DFSDM0_ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_icr::W](dfsdm0_icr::W) writer structure"]
impl crate::Writable for DFSDM0_ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm0_icr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_icr](dfsdm1_icr) module"]
pub type DFSDM1_ICR = crate::Reg<u32, _DFSDM1_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_ICR;
#[doc = "`read()` method returns [dfsdm1_icr::R](dfsdm1_icr::R) reader structure"]
impl crate::Readable for DFSDM1_ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_icr::W](dfsdm1_icr::W) writer structure"]
impl crate::Writable for DFSDM1_ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm1_icr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_icr](dfsdm2_icr) module"]
pub type DFSDM2_ICR = crate::Reg<u32, _DFSDM2_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_ICR;
#[doc = "`read()` method returns [dfsdm2_icr::R](dfsdm2_icr::R) reader structure"]
impl crate::Readable for DFSDM2_ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_icr::W](dfsdm2_icr::W) writer structure"]
impl crate::Writable for DFSDM2_ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm2_icr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_icr](dfsdm3_icr) module"]
pub type DFSDM3_ICR = crate::Reg<u32, _DFSDM3_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_ICR;
#[doc = "`read()` method returns [dfsdm3_icr::R](dfsdm3_icr::R) reader structure"]
impl crate::Readable for DFSDM3_ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_icr::W](dfsdm3_icr::W) writer structure"]
impl crate::Writable for DFSDM3_ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm3_icr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_jchgr](dfsdm0_jchgr) module"]
pub type DFSDM0_JCHGR = crate::Reg<u32, _DFSDM0_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_JCHGR;
#[doc = "`read()` method returns [dfsdm0_jchgr::R](dfsdm0_jchgr::R) reader structure"]
impl crate::Readable for DFSDM0_JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_jchgr::W](dfsdm0_jchgr::W) writer structure"]
impl crate::Writable for DFSDM0_JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm0_jchgr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_jchgr](dfsdm1_jchgr) module"]
pub type DFSDM1_JCHGR = crate::Reg<u32, _DFSDM1_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_JCHGR;
#[doc = "`read()` method returns [dfsdm1_jchgr::R](dfsdm1_jchgr::R) reader structure"]
impl crate::Readable for DFSDM1_JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_jchgr::W](dfsdm1_jchgr::W) writer structure"]
impl crate::Writable for DFSDM1_JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm1_jchgr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_jchgr](dfsdm2_jchgr) module"]
pub type DFSDM2_JCHGR = crate::Reg<u32, _DFSDM2_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_JCHGR;
#[doc = "`read()` method returns [dfsdm2_jchgr::R](dfsdm2_jchgr::R) reader structure"]
impl crate::Readable for DFSDM2_JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_jchgr::W](dfsdm2_jchgr::W) writer structure"]
impl crate::Writable for DFSDM2_JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm2_jchgr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_jchgr](dfsdm3_jchgr) module"]
pub type DFSDM3_JCHGR = crate::Reg<u32, _DFSDM3_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_JCHGR;
#[doc = "`read()` method returns [dfsdm3_jchgr::R](dfsdm3_jchgr::R) reader structure"]
impl crate::Readable for DFSDM3_JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_jchgr::W](dfsdm3_jchgr::W) writer structure"]
impl crate::Writable for DFSDM3_JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm3_jchgr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_fcr](dfsdm0_fcr) module"]
pub type DFSDM0_FCR = crate::Reg<u32, _DFSDM0_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_FCR;
#[doc = "`read()` method returns [dfsdm0_fcr::R](dfsdm0_fcr::R) reader structure"]
impl crate::Readable for DFSDM0_FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_fcr::W](dfsdm0_fcr::W) writer structure"]
impl crate::Writable for DFSDM0_FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm0_fcr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_fcr](dfsdm1_fcr) module"]
pub type DFSDM1_FCR = crate::Reg<u32, _DFSDM1_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_FCR;
#[doc = "`read()` method returns [dfsdm1_fcr::R](dfsdm1_fcr::R) reader structure"]
impl crate::Readable for DFSDM1_FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_fcr::W](dfsdm1_fcr::W) writer structure"]
impl crate::Writable for DFSDM1_FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm1_fcr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_fcr](dfsdm2_fcr) module"]
pub type DFSDM2_FCR = crate::Reg<u32, _DFSDM2_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_FCR;
#[doc = "`read()` method returns [dfsdm2_fcr::R](dfsdm2_fcr::R) reader structure"]
impl crate::Readable for DFSDM2_FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_fcr::W](dfsdm2_fcr::W) writer structure"]
impl crate::Writable for DFSDM2_FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm2_fcr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_fcr](dfsdm3_fcr) module"]
pub type DFSDM3_FCR = crate::Reg<u32, _DFSDM3_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_FCR;
#[doc = "`read()` method returns [dfsdm3_fcr::R](dfsdm3_fcr::R) reader structure"]
impl crate::Readable for DFSDM3_FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_fcr::W](dfsdm3_fcr::W) writer structure"]
impl crate::Writable for DFSDM3_FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm3_fcr;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_jdatar](dfsdm0_jdatar) module"]
pub type DFSDM0_JDATAR = crate::Reg<u32, _DFSDM0_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_JDATAR;
#[doc = "`read()` method returns [dfsdm0_jdatar::R](dfsdm0_jdatar::R) reader structure"]
impl crate::Readable for DFSDM0_JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm0_jdatar;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_jdatar](dfsdm1_jdatar) module"]
pub type DFSDM1_JDATAR = crate::Reg<u32, _DFSDM1_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_JDATAR;
#[doc = "`read()` method returns [dfsdm1_jdatar::R](dfsdm1_jdatar::R) reader structure"]
impl crate::Readable for DFSDM1_JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm1_jdatar;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_jdatar](dfsdm2_jdatar) module"]
pub type DFSDM2_JDATAR = crate::Reg<u32, _DFSDM2_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_JDATAR;
#[doc = "`read()` method returns [dfsdm2_jdatar::R](dfsdm2_jdatar::R) reader structure"]
impl crate::Readable for DFSDM2_JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm2_jdatar;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_jdatar](dfsdm3_jdatar) module"]
pub type DFSDM3_JDATAR = crate::Reg<u32, _DFSDM3_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_JDATAR;
#[doc = "`read()` method returns [dfsdm3_jdatar::R](dfsdm3_jdatar::R) reader structure"]
impl crate::Readable for DFSDM3_JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm3_jdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_rdatar](dfsdm0_rdatar) module"]
pub type DFSDM0_RDATAR = crate::Reg<u32, _DFSDM0_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_RDATAR;
#[doc = "`read()` method returns [dfsdm0_rdatar::R](dfsdm0_rdatar::R) reader structure"]
impl crate::Readable for DFSDM0_RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm0_rdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_rdatar](dfsdm1_rdatar) module"]
pub type DFSDM1_RDATAR = crate::Reg<u32, _DFSDM1_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_RDATAR;
#[doc = "`read()` method returns [dfsdm1_rdatar::R](dfsdm1_rdatar::R) reader structure"]
impl crate::Readable for DFSDM1_RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm1_rdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_rdatar](dfsdm2_rdatar) module"]
pub type DFSDM2_RDATAR = crate::Reg<u32, _DFSDM2_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_RDATAR;
#[doc = "`read()` method returns [dfsdm2_rdatar::R](dfsdm2_rdatar::R) reader structure"]
impl crate::Readable for DFSDM2_RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm2_rdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_rdatar](dfsdm3_rdatar) module"]
pub type DFSDM3_RDATAR = crate::Reg<u32, _DFSDM3_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_RDATAR;
#[doc = "`read()` method returns [dfsdm3_rdatar::R](dfsdm3_rdatar::R) reader structure"]
impl crate::Readable for DFSDM3_RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm3_rdatar;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_awhtr](dfsdm0_awhtr) module"]
pub type DFSDM0_AWHTR = crate::Reg<u32, _DFSDM0_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWHTR;
#[doc = "`read()` method returns [dfsdm0_awhtr::R](dfsdm0_awhtr::R) reader structure"]
impl crate::Readable for DFSDM0_AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_awhtr::W](dfsdm0_awhtr::W) writer structure"]
impl crate::Writable for DFSDM0_AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm0_awhtr;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_awhtr](dfsdm1_awhtr) module"]
pub type DFSDM1_AWHTR = crate::Reg<u32, _DFSDM1_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWHTR;
#[doc = "`read()` method returns [dfsdm1_awhtr::R](dfsdm1_awhtr::R) reader structure"]
impl crate::Readable for DFSDM1_AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_awhtr::W](dfsdm1_awhtr::W) writer structure"]
impl crate::Writable for DFSDM1_AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm1_awhtr;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_awhtr](dfsdm2_awhtr) module"]
pub type DFSDM2_AWHTR = crate::Reg<u32, _DFSDM2_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWHTR;
#[doc = "`read()` method returns [dfsdm2_awhtr::R](dfsdm2_awhtr::R) reader structure"]
impl crate::Readable for DFSDM2_AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_awhtr::W](dfsdm2_awhtr::W) writer structure"]
impl crate::Writable for DFSDM2_AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm2_awhtr;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_awhtr](dfsdm3_awhtr) module"]
pub type DFSDM3_AWHTR = crate::Reg<u32, _DFSDM3_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWHTR;
#[doc = "`read()` method returns [dfsdm3_awhtr::R](dfsdm3_awhtr::R) reader structure"]
impl crate::Readable for DFSDM3_AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_awhtr::W](dfsdm3_awhtr::W) writer structure"]
impl crate::Writable for DFSDM3_AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm3_awhtr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_awltr](dfsdm0_awltr) module"]
pub type DFSDM0_AWLTR = crate::Reg<u32, _DFSDM0_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWLTR;
#[doc = "`read()` method returns [dfsdm0_awltr::R](dfsdm0_awltr::R) reader structure"]
impl crate::Readable for DFSDM0_AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_awltr::W](dfsdm0_awltr::W) writer structure"]
impl crate::Writable for DFSDM0_AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm0_awltr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_awltr](dfsdm1_awltr) module"]
pub type DFSDM1_AWLTR = crate::Reg<u32, _DFSDM1_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWLTR;
#[doc = "`read()` method returns [dfsdm1_awltr::R](dfsdm1_awltr::R) reader structure"]
impl crate::Readable for DFSDM1_AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_awltr::W](dfsdm1_awltr::W) writer structure"]
impl crate::Writable for DFSDM1_AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm1_awltr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_awltr](dfsdm2_awltr) module"]
pub type DFSDM2_AWLTR = crate::Reg<u32, _DFSDM2_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWLTR;
#[doc = "`read()` method returns [dfsdm2_awltr::R](dfsdm2_awltr::R) reader structure"]
impl crate::Readable for DFSDM2_AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_awltr::W](dfsdm2_awltr::W) writer structure"]
impl crate::Writable for DFSDM2_AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm2_awltr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_awltr](dfsdm3_awltr) module"]
pub type DFSDM3_AWLTR = crate::Reg<u32, _DFSDM3_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWLTR;
#[doc = "`read()` method returns [dfsdm3_awltr::R](dfsdm3_awltr::R) reader structure"]
impl crate::Readable for DFSDM3_AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_awltr::W](dfsdm3_awltr::W) writer structure"]
impl crate::Writable for DFSDM3_AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm3_awltr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_awsr](dfsdm0_awsr) module"]
pub type DFSDM0_AWSR = crate::Reg<u32, _DFSDM0_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWSR;
#[doc = "`read()` method returns [dfsdm0_awsr::R](dfsdm0_awsr::R) reader structure"]
impl crate::Readable for DFSDM0_AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm0_awsr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_awsr](dfsdm1_awsr) module"]
pub type DFSDM1_AWSR = crate::Reg<u32, _DFSDM1_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWSR;
#[doc = "`read()` method returns [dfsdm1_awsr::R](dfsdm1_awsr::R) reader structure"]
impl crate::Readable for DFSDM1_AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm1_awsr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_awsr](dfsdm2_awsr) module"]
pub type DFSDM2_AWSR = crate::Reg<u32, _DFSDM2_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWSR;
#[doc = "`read()` method returns [dfsdm2_awsr::R](dfsdm2_awsr::R) reader structure"]
impl crate::Readable for DFSDM2_AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm2_awsr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_awsr](dfsdm3_awsr) module"]
pub type DFSDM3_AWSR = crate::Reg<u32, _DFSDM3_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWSR;
#[doc = "`read()` method returns [dfsdm3_awsr::R](dfsdm3_awsr::R) reader structure"]
impl crate::Readable for DFSDM3_AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm3_awsr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_awcfr](dfsdm0_awcfr) module"]
pub type DFSDM0_AWCFR = crate::Reg<u32, _DFSDM0_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWCFR;
#[doc = "`read()` method returns [dfsdm0_awcfr::R](dfsdm0_awcfr::R) reader structure"]
impl crate::Readable for DFSDM0_AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_awcfr::W](dfsdm0_awcfr::W) writer structure"]
impl crate::Writable for DFSDM0_AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm0_awcfr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_awcfr](dfsdm1_awcfr) module"]
pub type DFSDM1_AWCFR = crate::Reg<u32, _DFSDM1_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWCFR;
#[doc = "`read()` method returns [dfsdm1_awcfr::R](dfsdm1_awcfr::R) reader structure"]
impl crate::Readable for DFSDM1_AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_awcfr::W](dfsdm1_awcfr::W) writer structure"]
impl crate::Writable for DFSDM1_AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm1_awcfr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_awcfr](dfsdm2_awcfr) module"]
pub type DFSDM2_AWCFR = crate::Reg<u32, _DFSDM2_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWCFR;
#[doc = "`read()` method returns [dfsdm2_awcfr::R](dfsdm2_awcfr::R) reader structure"]
impl crate::Readable for DFSDM2_AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_awcfr::W](dfsdm2_awcfr::W) writer structure"]
impl crate::Writable for DFSDM2_AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm2_awcfr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_awcfr](dfsdm3_awcfr) module"]
pub type DFSDM3_AWCFR = crate::Reg<u32, _DFSDM3_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWCFR;
#[doc = "`read()` method returns [dfsdm3_awcfr::R](dfsdm3_awcfr::R) reader structure"]
impl crate::Readable for DFSDM3_AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_awcfr::W](dfsdm3_awcfr::W) writer structure"]
impl crate::Writable for DFSDM3_AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm3_awcfr;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_exmax](dfsdm0_exmax) module"]
pub type DFSDM0_EXMAX = crate::Reg<u32, _DFSDM0_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_EXMAX;
#[doc = "`read()` method returns [dfsdm0_exmax::R](dfsdm0_exmax::R) reader structure"]
impl crate::Readable for DFSDM0_EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm0_exmax;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_exmax](dfsdm1_exmax) module"]
pub type DFSDM1_EXMAX = crate::Reg<u32, _DFSDM1_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_EXMAX;
#[doc = "`read()` method returns [dfsdm1_exmax::R](dfsdm1_exmax::R) reader structure"]
impl crate::Readable for DFSDM1_EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm1_exmax;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_exmax](dfsdm2_exmax) module"]
pub type DFSDM2_EXMAX = crate::Reg<u32, _DFSDM2_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_EXMAX;
#[doc = "`read()` method returns [dfsdm2_exmax::R](dfsdm2_exmax::R) reader structure"]
impl crate::Readable for DFSDM2_EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm2_exmax;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_exmax](dfsdm3_exmax) module"]
pub type DFSDM3_EXMAX = crate::Reg<u32, _DFSDM3_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_EXMAX;
#[doc = "`read()` method returns [dfsdm3_exmax::R](dfsdm3_exmax::R) reader structure"]
impl crate::Readable for DFSDM3_EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm3_exmax;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_exmin](dfsdm0_exmin) module"]
pub type DFSDM0_EXMIN = crate::Reg<u32, _DFSDM0_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_EXMIN;
#[doc = "`read()` method returns [dfsdm0_exmin::R](dfsdm0_exmin::R) reader structure"]
impl crate::Readable for DFSDM0_EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm0_exmin;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_exmin](dfsdm1_exmin) module"]
pub type DFSDM1_EXMIN = crate::Reg<u32, _DFSDM1_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_EXMIN;
#[doc = "`read()` method returns [dfsdm1_exmin::R](dfsdm1_exmin::R) reader structure"]
impl crate::Readable for DFSDM1_EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm1_exmin;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_exmin](dfsdm2_exmin) module"]
pub type DFSDM2_EXMIN = crate::Reg<u32, _DFSDM2_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_EXMIN;
#[doc = "`read()` method returns [dfsdm2_exmin::R](dfsdm2_exmin::R) reader structure"]
impl crate::Readable for DFSDM2_EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm2_exmin;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_exmin](dfsdm3_exmin) module"]
pub type DFSDM3_EXMIN = crate::Reg<u32, _DFSDM3_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_EXMIN;
#[doc = "`read()` method returns [dfsdm3_exmin::R](dfsdm3_exmin::R) reader structure"]
impl crate::Readable for DFSDM3_EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm3_exmin;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_cnvtimr](dfsdm0_cnvtimr) module"]
pub type DFSDM0_CNVTIMR = crate::Reg<u32, _DFSDM0_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CNVTIMR;
#[doc = "`read()` method returns [dfsdm0_cnvtimr::R](dfsdm0_cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM0_CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm0_cnvtimr;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_cnvtimr](dfsdm1_cnvtimr) module"]
pub type DFSDM1_CNVTIMR = crate::Reg<u32, _DFSDM1_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CNVTIMR;
#[doc = "`read()` method returns [dfsdm1_cnvtimr::R](dfsdm1_cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM1_CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm1_cnvtimr;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_cnvtimr](dfsdm2_cnvtimr) module"]
pub type DFSDM2_CNVTIMR = crate::Reg<u32, _DFSDM2_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CNVTIMR;
#[doc = "`read()` method returns [dfsdm2_cnvtimr::R](dfsdm2_cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM2_CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm2_cnvtimr;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_cnvtimr](dfsdm3_cnvtimr) module"]
pub type DFSDM3_CNVTIMR = crate::Reg<u32, _DFSDM3_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CNVTIMR;
#[doc = "`read()` method returns [dfsdm3_cnvtimr::R](dfsdm3_cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM3_CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm3_cnvtimr;
