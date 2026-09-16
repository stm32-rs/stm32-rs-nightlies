#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    stopcr: STOPCR,
    _reserved3: [u8; 0x14],
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    ckprotr: CKPROTR,
    bdcr: BDCR,
    hwrsr: HWRSR,
    rsr: RSR,
    _reserved9: [u8; 0x08],
    lsecfgr: LSECFGR,
    msicfgr: MSICFGR,
    hsicfgr: HSICFGR,
    hsimcr: HSIMCR,
    hsimsr: HSIMSR,
    hsecfgr: HSECFGR,
    _reserved15: [u8; 0x28],
    pll1cfgr1: PLL1CFGR1,
    pll1cfgr2: PLL1CFGR2,
    pll1cfgr3: PLL1CFGR3,
    _reserved18: [u8; 0x04],
    pll2cfgr1: PLL2CFGR1,
    pll2cfgr2: PLL2CFGR2,
    pll2cfgr3: PLL2CFGR3,
    _reserved21: [u8; 0x04],
    pll3cfgr1: PLL3CFGR1,
    pll3cfgr2: PLL3CFGR2,
    pll3cfgr3: PLL3CFGR3,
    _reserved24: [u8; 0x04],
    pll4cfgr1: PLL4CFGR1,
    pll4cfgr2: PLL4CFGR2,
    pll4cfgr3: PLL4CFGR3,
    _reserved27: [u8; 0x08],
    ic1cfgr: IC1CFGR,
    ic2cfgr: IC2CFGR,
    ic3cfgr: IC3CFGR,
    ic4cfgr: IC4CFGR,
    ic5cfgr: IC5CFGR,
    ic6cfgr: IC6CFGR,
    ic7cfgr: IC7CFGR,
    ic8cfgr: IC8CFGR,
    ic9cfgr: IC9CFGR,
    ic10cfgr: IC10CFGR,
    ic11cfgr: IC11CFGR,
    ic12cfgr: IC12CFGR,
    ic13cfgr: IC13CFGR,
    ic14cfgr: IC14CFGR,
    ic15cfgr: IC15CFGR,
    ic16cfgr: IC16CFGR,
    ic17cfgr: IC17CFGR,
    ic18cfgr: IC18CFGR,
    ic19cfgr: IC19CFGR,
    ic20cfgr: IC20CFGR,
    _reserved47: [u8; 0x10],
    cier: CIER,
    cifr: CIFR,
    cicr: CICR,
    _reserved50: [u8; 0x14],
    ccipr1: CCIPR1,
    ccipr2: CCIPR2,
    ccipr3: CCIPR3,
    ccipr4: CCIPR4,
    ccipr5: CCIPR5,
    ccipr6: CCIPR6,
    ccipr7: CCIPR7,
    ccipr8: CCIPR8,
    ccipr9: CCIPR9,
    _reserved59: [u8; 0x08],
    ccipr12: CCIPR12,
    ccipr13: CCIPR13,
    ccipr14: CCIPR14,
    _reserved62: [u8; 0x88],
    busrstr: BUSRSTR,
    miscrstr: MISCRSTR,
    memrstr: MEMRSTR,
    ahb1rstr: AHB1RSTR,
    ahb2rstr: AHB2RSTR,
    ahb3rstr: AHB3RSTR,
    ahb4rstr: AHB4RSTR,
    ahb5rstr: AHB5RSTR,
    apb1lrstr: APB1LRSTR,
    apb1hrstr: APB1HRSTR,
    apb2rstr: APB2RSTR,
    _reserved73: [u8; 0x04],
    apb4lrstr: APB4LRSTR,
    apb4hrstr: APB4HRSTR,
    apb5rstr: APB5RSTR,
    divenr: DIVENR,
    busenr: BUSENR,
    miscenr: MISCENR,
    memenr: MEMENR,
    ahb1enr: AHB1ENR,
    ahb2enr: AHB2ENR,
    ahb3enr: AHB3ENR,
    ahb4enr: AHB4ENR,
    ahb5enr: AHB5ENR,
    apb1lenr: APB1LENR,
    apb1henr: APB1HENR,
    apb2enr: APB2ENR,
    apb3enr: APB3ENR,
    apb4lenr: APB4LENR,
    apb4henr: APB4HENR,
    apb5enr: APB5ENR,
    divlpenr: DIVLPENR,
    buslpenr: BUSLPENR,
    misclpenr: MISCLPENR,
    memlpenr: MEMLPENR,
    ahb1lpenr: AHB1LPENR,
    ahb2lpenr: AHB2LPENR,
    ahb3lpenr: AHB3LPENR,
    ahb4lpenr: AHB4LPENR,
    ahb5lpenr: AHB5LPENR,
    apb1llpenr: APB1LLPENR,
    apb1hlpenr: APB1HLPENR,
    apb2lpenr: APB2LPENR,
    apb3lpenr: APB3LPENR,
    apb4llpenr: APB4LLPENR,
    apb4hlpenr: APB4HLPENR,
    apb5lpenr: APB5LPENR,
    _reserved108: [u8; 0x018c],
    rdcr: RDCR,
    _reserved109: [u8; 0x0330],
    seccfgr0: SECCFGR0,
    privcfgr0: PRIVCFGR0,
    lockcfgr0: LOCKCFGR0,
    pubcfgr0: PUBCFGR0,
    seccfgr1: SECCFGR1,
    privcfgr1: PRIVCFGR1,
    lockcfgr1: LOCKCFGR1,
    pubcfgr1: PUBCFGR1,
    seccfgr2: SECCFGR2,
    privcfgr2: PRIVCFGR2,
    lockcfgr2: LOCKCFGR2,
    pubcfgr2: PUBCFGR2,
    seccfgr3: SECCFGR3,
    privcfgr3: PRIVCFGR3,
    lockcfgr3: LOCKCFGR3,
    pubcfgr3: PUBCFGR3,
    seccfgr4: SECCFGR4,
    privcfgr4: PRIVCFGR4,
    lockcfgr4: LOCKCFGR4,
    pubcfgr4: PUBCFGR4,
    pubcfgr5: PUBCFGR5,
    _reserved130: [u8; 0x2c],
    csr: CSR,
    _reserved131: [u8; 0x04],
    stopcsr: STOPCSR,
    _reserved132: [u8; 0x01f8],
    busrstsr: BUSRSTSR,
    miscrstsr: MISCRSTSR,
    memrstsr: MEMRSTSR,
    ahb1rstsr: AHB1RSTSR,
    ahb2rstsr: AHB2RSTSR,
    ahb3rstsr: AHB3RSTSR,
    ahb4rstsr: AHB4RSTSR,
    ahb5rstsr: AHB5RSTSR,
    apb1lrstsr: APB1LRSTSR,
    apb1hrstsr: APB1HRSTSR,
    apb2rstsr: APB2RSTSR,
    _reserved143: [u8; 0x04],
    apb4lrstsr: APB4LRSTSR,
    apb4hrstsr: APB4HRSTSR,
    apb5rstsr: APB5RSTSR,
    divensr: DIVENSR,
    busensr: BUSENSR,
    miscensr: MISCENSR,
    memensr: MEMENSR,
    ahb1ensr: AHB1ENSR,
    ahb2ensr: AHB2ENSR,
    ahb3ensr: AHB3ENSR,
    ahb4ensr: AHB4ENSR,
    ahb5ensr: AHB5ENSR,
    apb1lensr: APB1LENSR,
    apb1hensr: APB1HENSR,
    apb2ensr: APB2ENSR,
    apb3ensr: APB3ENSR,
    apb4lensr: APB4LENSR,
    apb4hensr: APB4HENSR,
    apb5ensr: APB5ENSR,
    divlpensr: DIVLPENSR,
    buslpensr: BUSLPENSR,
    misclpensr: MISCLPENSR,
    memlpensr: MEMLPENSR,
    ahb1lpensr: AHB1LPENSR,
    ahb2lpensr: AHB2LPENSR,
    ahb3lpensr: AHB3LPENSR,
    ahb4lpensr: AHB4LPENSR,
    ahb5lpensr: AHB5LPENSR,
    apb1llpensr: APB1LLPENSR,
    apb1hlpensr: APB1HLPENSR,
    apb2lpensr: APB2LPENSR,
    apb3lpensr: APB3LPENSR,
    apb4llpensr: APB4LLPENSR,
    apb4hlpensr: APB4HLPENSR,
    apb5lpensr: APB5LPENSR,
    _reserved178: [u8; 0x04c4],
    privcfgsr0: PRIVCFGSR0,
    _reserved179: [u8; 0x04],
    pubcfgsr0: PUBCFGSR0,
    _reserved180: [u8; 0x04],
    privcfgsr1: PRIVCFGSR1,
    _reserved181: [u8; 0x04],
    pubcfgsr1: PUBCFGSR1,
    _reserved182: [u8; 0x04],
    privcfgsr2: PRIVCFGSR2,
    _reserved183: [u8; 0x04],
    pubcfgsr2: PUBCFGSR2,
    seccfgsr3: SECCFGSR3,
    privcfgsr3: PRIVCFGSR3,
    lockcfgsr3: LOCKCFGSR3,
    pubcfgsr3: PUBCFGSR3,
    _reserved188: [u8; 0x04],
    privcfgsr4: PRIVCFGSR4,
    _reserved189: [u8; 0x04],
    pubcfgsr4: PUBCFGSR4,
    pubcfgsr5: PUBCFGSR5,
    _reserved191: [u8; 0x2c],
    ccr: CCR,
    _reserved192: [u8; 0x04],
    stopccr: STOPCCR,
    _reserved193: [u8; 0x01f8],
    busrstcr: BUSRSTCR,
    miscrstcr: MISCRSTCR,
    memrstcr: MEMRSTCR,
    ahb1rstcr: AHB1RSTCR,
    ahb2rstcr: AHB2RSTCR,
    ahb3rstcr: AHB3RSTCR,
    ahb4rstcr: AHB4RSTCR,
    ahb5rstcr: AHB5RSTCR,
    apb1lrstcr: APB1LRSTCR,
    apb1hrstcr: APB1HRSTCR,
    apb2rstcr: APB2RSTCR,
    _reserved204: [u8; 0x04],
    apb4lrstcr: APB4LRSTCR,
    apb4hrstcr: APB4HRSTCR,
    apb5rstcr: APB5RSTCR,
    divencr: DIVENCR,
    busencr: BUSENCR,
    miscencr: MISCENCR,
    memencr: MEMENCR,
    ahb1encr: AHB1ENCR,
    ahb2encr: AHB2ENCR,
    ahb3encr: AHB3ENCR,
    ahb4encr: AHB4ENCR,
    ahb5encr: AHB5ENCR,
    apb1lencr: APB1LENCR,
    apb1hencr: APB1HENCR,
    apb2encr: APB2ENCR,
    apb3encr: APB3ENCR,
    apb4lencr: APB4LENCR,
    apb4hencr: APB4HENCR,
    apb5encr: APB5ENCR,
    divlpencr: DIVLPENCR,
    buslpencr: BUSLPENCR,
    misclpencr: MISCLPENCR,
    memlpencr: MEMLPENCR,
    ahb1lpencr: AHB1LPENCR,
    ahb2lpencr: AHB2LPENCR,
    ahb3lpencr: AHB3LPENCR,
    ahb4lpencr: AHB4LPENCR,
    ahb5lpencr: AHB5LPENCR,
    apb1llpencr: APB1LLPENCR,
    apb1hlpencr: APB1HLPENCR,
    apb2lpencr: APB2LPENCR,
    apb3lpencr: APB3LPENCR,
    apb4llpencr: APB4LLPENCR,
    apb4hlpencr: APB4HLPENCR,
    apb5lpencr: APB5LPENCR,
    _reserved239: [u8; 0x04c4],
    privcfgcr0: PRIVCFGCR0,
    _reserved240: [u8; 0x04],
    pubcfgcr0: PUBCFGCR0,
    _reserved241: [u8; 0x04],
    privcfgcr1: PRIVCFGCR1,
    _reserved242: [u8; 0x04],
    pubcfgcr1: PUBCFGCR1,
    _reserved243: [u8; 0x04],
    privcfgcr2: PRIVCFGCR2,
    _reserved244: [u8; 0x04],
    pubcfgcr2: PUBCFGCR2,
    _reserved245: [u8; 0x04],
    privcfgcr3: PRIVCFGCR3,
    _reserved246: [u8; 0x04],
    pubcfgcr3: PUBCFGCR3,
    _reserved247: [u8; 0x04],
    privcfgcr4: PRIVCFGCR4,
    _reserved248: [u8; 0x04],
    pubcfgcr4: PUBCFGCR4,
    pubcfgcr5: PUBCFGCR5,
}
impl RegisterBlock {
    ///0x00 - RCC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - RCC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - RCC Stop mode control register
    #[inline(always)]
    pub const fn stopcr(&self) -> &STOPCR {
        &self.stopcr
    }
    ///0x20 - RCC configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x24 - RCC configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x28 - RCC clock protection register
    #[inline(always)]
    pub const fn ckprotr(&self) -> &CKPROTR {
        &self.ckprotr
    }
    ///0x2c - RCC backup domain protection register
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x30 - RCC reset status register for hardware
    #[inline(always)]
    pub const fn hwrsr(&self) -> &HWRSR {
        &self.hwrsr
    }
    ///0x34 - RCC reset register
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        &self.rsr
    }
    ///0x40 - RCC LSE configuration register
    #[inline(always)]
    pub const fn lsecfgr(&self) -> &LSECFGR {
        &self.lsecfgr
    }
    ///0x44 - RCC MSI configuration register
    #[inline(always)]
    pub const fn msicfgr(&self) -> &MSICFGR {
        &self.msicfgr
    }
    ///0x48 - RCC HSI configuration register
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &HSICFGR {
        &self.hsicfgr
    }
    ///0x4c - RCC HSI monitor control register
    #[inline(always)]
    pub const fn hsimcr(&self) -> &HSIMCR {
        &self.hsimcr
    }
    ///0x50 - RCC HSI monitor status register
    #[inline(always)]
    pub const fn hsimsr(&self) -> &HSIMSR {
        &self.hsimsr
    }
    ///0x54 - RCC HSE configuration register
    #[inline(always)]
    pub const fn hsecfgr(&self) -> &HSECFGR {
        &self.hsecfgr
    }
    ///0x80 - RCC PLL1 configuration register 1
    #[inline(always)]
    pub const fn pll1cfgr1(&self) -> &PLL1CFGR1 {
        &self.pll1cfgr1
    }
    ///0x84 - RCC PLL1 configuration register 2
    #[inline(always)]
    pub const fn pll1cfgr2(&self) -> &PLL1CFGR2 {
        &self.pll1cfgr2
    }
    ///0x88 - RCC PLL1 configuration register 3
    #[inline(always)]
    pub const fn pll1cfgr3(&self) -> &PLL1CFGR3 {
        &self.pll1cfgr3
    }
    ///0x90 - RCC PLL2 configuration register 1
    #[inline(always)]
    pub const fn pll2cfgr1(&self) -> &PLL2CFGR1 {
        &self.pll2cfgr1
    }
    ///0x94 - RCC PLL2 configuration register 2
    #[inline(always)]
    pub const fn pll2cfgr2(&self) -> &PLL2CFGR2 {
        &self.pll2cfgr2
    }
    ///0x98 - RCC PLL2 configuration register 3
    #[inline(always)]
    pub const fn pll2cfgr3(&self) -> &PLL2CFGR3 {
        &self.pll2cfgr3
    }
    ///0xa0 - RCC PLL3 configuration register 1
    #[inline(always)]
    pub const fn pll3cfgr1(&self) -> &PLL3CFGR1 {
        &self.pll3cfgr1
    }
    ///0xa4 - RCC PLL3 configuration register 2
    #[inline(always)]
    pub const fn pll3cfgr2(&self) -> &PLL3CFGR2 {
        &self.pll3cfgr2
    }
    ///0xa8 - RCC PLL3 configuration register 3
    #[inline(always)]
    pub const fn pll3cfgr3(&self) -> &PLL3CFGR3 {
        &self.pll3cfgr3
    }
    ///0xb0 - RCC PLL4 configuration register 1
    #[inline(always)]
    pub const fn pll4cfgr1(&self) -> &PLL4CFGR1 {
        &self.pll4cfgr1
    }
    ///0xb4 - RCC PLL4 configuration register 2
    #[inline(always)]
    pub const fn pll4cfgr2(&self) -> &PLL4CFGR2 {
        &self.pll4cfgr2
    }
    ///0xb8 - RCC PLL4 configuration register 3
    #[inline(always)]
    pub const fn pll4cfgr3(&self) -> &PLL4CFGR3 {
        &self.pll4cfgr3
    }
    ///0xc4 - RCC IC1 configuration register
    #[inline(always)]
    pub const fn ic1cfgr(&self) -> &IC1CFGR {
        &self.ic1cfgr
    }
    ///0xc8 - RCC IC2 configuration register
    #[inline(always)]
    pub const fn ic2cfgr(&self) -> &IC2CFGR {
        &self.ic2cfgr
    }
    ///0xcc - RCC IC3 configuration register
    #[inline(always)]
    pub const fn ic3cfgr(&self) -> &IC3CFGR {
        &self.ic3cfgr
    }
    ///0xd0 - RCC IC4 configuration register
    #[inline(always)]
    pub const fn ic4cfgr(&self) -> &IC4CFGR {
        &self.ic4cfgr
    }
    ///0xd4 - RCC IC5 configuration register
    #[inline(always)]
    pub const fn ic5cfgr(&self) -> &IC5CFGR {
        &self.ic5cfgr
    }
    ///0xd8 - RCC IC6 configuration register
    #[inline(always)]
    pub const fn ic6cfgr(&self) -> &IC6CFGR {
        &self.ic6cfgr
    }
    ///0xdc - RCC IC7 configuration register
    #[inline(always)]
    pub const fn ic7cfgr(&self) -> &IC7CFGR {
        &self.ic7cfgr
    }
    ///0xe0 - RCC IC8 configuration register
    #[inline(always)]
    pub const fn ic8cfgr(&self) -> &IC8CFGR {
        &self.ic8cfgr
    }
    ///0xe4 - RCC IC9 configuration register
    #[inline(always)]
    pub const fn ic9cfgr(&self) -> &IC9CFGR {
        &self.ic9cfgr
    }
    ///0xe8 - RCC IC10 configuration register
    #[inline(always)]
    pub const fn ic10cfgr(&self) -> &IC10CFGR {
        &self.ic10cfgr
    }
    ///0xec - RCC IC11 configuration register
    #[inline(always)]
    pub const fn ic11cfgr(&self) -> &IC11CFGR {
        &self.ic11cfgr
    }
    ///0xf0 - RCC IC12 configuration register
    #[inline(always)]
    pub const fn ic12cfgr(&self) -> &IC12CFGR {
        &self.ic12cfgr
    }
    ///0xf4 - RCC IC13 configuration register
    #[inline(always)]
    pub const fn ic13cfgr(&self) -> &IC13CFGR {
        &self.ic13cfgr
    }
    ///0xf8 - RCC IC14 configuration register
    #[inline(always)]
    pub const fn ic14cfgr(&self) -> &IC14CFGR {
        &self.ic14cfgr
    }
    ///0xfc - RCC IC15 configuration register
    #[inline(always)]
    pub const fn ic15cfgr(&self) -> &IC15CFGR {
        &self.ic15cfgr
    }
    ///0x100 - RCC IC16 configuration register
    #[inline(always)]
    pub const fn ic16cfgr(&self) -> &IC16CFGR {
        &self.ic16cfgr
    }
    ///0x104 - RCC IC17 configuration register
    #[inline(always)]
    pub const fn ic17cfgr(&self) -> &IC17CFGR {
        &self.ic17cfgr
    }
    ///0x108 - RCC IC18 configuration register
    #[inline(always)]
    pub const fn ic18cfgr(&self) -> &IC18CFGR {
        &self.ic18cfgr
    }
    ///0x10c - RCC IC19 configuration register
    #[inline(always)]
    pub const fn ic19cfgr(&self) -> &IC19CFGR {
        &self.ic19cfgr
    }
    ///0x110 - RCC IC20 configuration register
    #[inline(always)]
    pub const fn ic20cfgr(&self) -> &IC20CFGR {
        &self.ic20cfgr
    }
    ///0x124 - RCC clock-source interrupt enable register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x128 - RCC clock-source interrupt flag register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x12c - RCC clock-source interrupt Clear register
    #[inline(always)]
    pub const fn cicr(&self) -> &CICR {
        &self.cicr
    }
    ///0x144 - RCC clock configuration for independent peripheral register1
    #[inline(always)]
    pub const fn ccipr1(&self) -> &CCIPR1 {
        &self.ccipr1
    }
    ///0x148 - RCC clock configuration for independent peripheral register 2
    #[inline(always)]
    pub const fn ccipr2(&self) -> &CCIPR2 {
        &self.ccipr2
    }
    ///0x14c - RCC clock configuration for independent peripheral register3
    #[inline(always)]
    pub const fn ccipr3(&self) -> &CCIPR3 {
        &self.ccipr3
    }
    ///0x150 - RCC clock configuration for independent peripheral register4
    #[inline(always)]
    pub const fn ccipr4(&self) -> &CCIPR4 {
        &self.ccipr4
    }
    ///0x154 - RCC lock configuration for independent peripheral register5
    #[inline(always)]
    pub const fn ccipr5(&self) -> &CCIPR5 {
        &self.ccipr5
    }
    ///0x158 - RCC clock configuration for independent peripheral register6
    #[inline(always)]
    pub const fn ccipr6(&self) -> &CCIPR6 {
        &self.ccipr6
    }
    ///0x15c - RCC clock configuration for independent peripheral register7
    #[inline(always)]
    pub const fn ccipr7(&self) -> &CCIPR7 {
        &self.ccipr7
    }
    ///0x160 - RCC clock configuration for independent peripheral register8
    #[inline(always)]
    pub const fn ccipr8(&self) -> &CCIPR8 {
        &self.ccipr8
    }
    ///0x164 - RCC clock configuration for independent peripheral register9
    #[inline(always)]
    pub const fn ccipr9(&self) -> &CCIPR9 {
        &self.ccipr9
    }
    ///0x170 - RCC clock configuration for independent peripheral register12
    #[inline(always)]
    pub const fn ccipr12(&self) -> &CCIPR12 {
        &self.ccipr12
    }
    ///0x174 - RCC clock configuration for independent peripheral register13
    #[inline(always)]
    pub const fn ccipr13(&self) -> &CCIPR13 {
        &self.ccipr13
    }
    ///0x178 - RCC clock configuration for independent peripheral register14
    #[inline(always)]
    pub const fn ccipr14(&self) -> &CCIPR14 {
        &self.ccipr14
    }
    ///0x204 - RCC SoC buses reset register
    #[inline(always)]
    pub const fn busrstr(&self) -> &BUSRSTR {
        &self.busrstr
    }
    ///0x208 - RCC miscellaneous configurations reset register
    #[inline(always)]
    pub const fn miscrstr(&self) -> &MISCRSTR {
        &self.miscrstr
    }
    ///0x20c - RCC memories reset register
    #[inline(always)]
    pub const fn memrstr(&self) -> &MEMRSTR {
        &self.memrstr
    }
    ///0x210 - RCC AHB1 Reset register
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &AHB1RSTR {
        &self.ahb1rstr
    }
    ///0x214 - RCC AHB2 reset register
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &AHB2RSTR {
        &self.ahb2rstr
    }
    ///0x218 - RCC AHB3 reset register
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &AHB3RSTR {
        &self.ahb3rstr
    }
    ///0x21c - RCC AHB4 reset register
    #[inline(always)]
    pub const fn ahb4rstr(&self) -> &AHB4RSTR {
        &self.ahb4rstr
    }
    ///0x220 - RCC AHB5 reset register
    #[inline(always)]
    pub const fn ahb5rstr(&self) -> &AHB5RSTR {
        &self.ahb5rstr
    }
    ///0x224 - RCC APB1L reset register
    #[inline(always)]
    pub const fn apb1lrstr(&self) -> &APB1LRSTR {
        &self.apb1lrstr
    }
    ///0x228 - RCC APB1H reset register
    #[inline(always)]
    pub const fn apb1hrstr(&self) -> &APB1HRSTR {
        &self.apb1hrstr
    }
    ///0x22c - RCC APB2 reset register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x234 - RCC APB4L reset register
    #[inline(always)]
    pub const fn apb4lrstr(&self) -> &APB4LRSTR {
        &self.apb4lrstr
    }
    ///0x238 - RCC APB4H reset register
    #[inline(always)]
    pub const fn apb4hrstr(&self) -> &APB4HRSTR {
        &self.apb4hrstr
    }
    ///0x23c - RCC APB5 reset register
    #[inline(always)]
    pub const fn apb5rstr(&self) -> &APB5RSTR {
        &self.apb5rstr
    }
    ///0x240 - RCC IC dividers enable register
    #[inline(always)]
    pub const fn divenr(&self) -> &DIVENR {
        &self.divenr
    }
    ///0x244 - RCC SoC buses enable register
    #[inline(always)]
    pub const fn busenr(&self) -> &BUSENR {
        &self.busenr
    }
    ///0x248 - RCC miscellaneous configuration enable register
    #[inline(always)]
    pub const fn miscenr(&self) -> &MISCENR {
        &self.miscenr
    }
    ///0x24c - RCC memory enable register
    #[inline(always)]
    pub const fn memenr(&self) -> &MEMENR {
        &self.memenr
    }
    ///0x250 - RCC AHB1 enable register
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &AHB1ENR {
        &self.ahb1enr
    }
    ///0x254 - RCC AHB2 enable register
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &AHB2ENR {
        &self.ahb2enr
    }
    ///0x258 - RCC AHB3 enable register
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &AHB3ENR {
        &self.ahb3enr
    }
    ///0x25c - RCC AHB4 enable register
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &AHB4ENR {
        &self.ahb4enr
    }
    ///0x260 - RCC AHB5 enable register
    #[inline(always)]
    pub const fn ahb5enr(&self) -> &AHB5ENR {
        &self.ahb5enr
    }
    ///0x264 - RCC APB1L enable register
    #[inline(always)]
    pub const fn apb1lenr(&self) -> &APB1LENR {
        &self.apb1lenr
    }
    ///0x268 - RCC APB1H enable register
    #[inline(always)]
    pub const fn apb1henr(&self) -> &APB1HENR {
        &self.apb1henr
    }
    ///0x26c - RCC APB2 enable register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x270 - RCC APB3 enable register
    #[inline(always)]
    pub const fn apb3enr(&self) -> &APB3ENR {
        &self.apb3enr
    }
    ///0x274 - RCC APB4L enable register
    #[inline(always)]
    pub const fn apb4lenr(&self) -> &APB4LENR {
        &self.apb4lenr
    }
    ///0x278 - RCC APB4H enable register
    #[inline(always)]
    pub const fn apb4henr(&self) -> &APB4HENR {
        &self.apb4henr
    }
    ///0x27c - RCC APB5 enable register
    #[inline(always)]
    pub const fn apb5enr(&self) -> &APB5ENR {
        &self.apb5enr
    }
    ///0x280 - RCC dividers Sleep enable register
    #[inline(always)]
    pub const fn divlpenr(&self) -> &DIVLPENR {
        &self.divlpenr
    }
    ///0x284 - RCC SoC buses Sleep enable register
    #[inline(always)]
    pub const fn buslpenr(&self) -> &BUSLPENR {
        &self.buslpenr
    }
    ///0x288 - RCC miscellaneous configurations Sleep enable register
    #[inline(always)]
    pub const fn misclpenr(&self) -> &MISCLPENR {
        &self.misclpenr
    }
    ///0x28c - RCC memory Sleep enable register
    #[inline(always)]
    pub const fn memlpenr(&self) -> &MEMLPENR {
        &self.memlpenr
    }
    ///0x290 - RCC AHB1 Sleep enable register
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &AHB1LPENR {
        &self.ahb1lpenr
    }
    ///0x294 - RCC AHB2 Sleep enable register
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &AHB2LPENR {
        &self.ahb2lpenr
    }
    ///0x298 - RCC AHB3 Sleep enable register
    #[inline(always)]
    pub const fn ahb3lpenr(&self) -> &AHB3LPENR {
        &self.ahb3lpenr
    }
    ///0x29c - RCC AHB4 Sleep enable register
    #[inline(always)]
    pub const fn ahb4lpenr(&self) -> &AHB4LPENR {
        &self.ahb4lpenr
    }
    ///0x2a0 - RCC AHB5 Sleep enable register
    #[inline(always)]
    pub const fn ahb5lpenr(&self) -> &AHB5LPENR {
        &self.ahb5lpenr
    }
    ///0x2a4 - RCC APB1L Sleep enable register
    #[inline(always)]
    pub const fn apb1llpenr(&self) -> &APB1LLPENR {
        &self.apb1llpenr
    }
    ///0x2a8 - RCC APB1H Sleep enable register
    #[inline(always)]
    pub const fn apb1hlpenr(&self) -> &APB1HLPENR {
        &self.apb1hlpenr
    }
    ///0x2ac - RCC APB2 Sleep enable register
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &APB2LPENR {
        &self.apb2lpenr
    }
    ///0x2b0 - RCC APB3 Sleep enable register
    #[inline(always)]
    pub const fn apb3lpenr(&self) -> &APB3LPENR {
        &self.apb3lpenr
    }
    ///0x2b4 - RCC APB4L Sleep enable register
    #[inline(always)]
    pub const fn apb4llpenr(&self) -> &APB4LLPENR {
        &self.apb4llpenr
    }
    ///0x2b8 - RCC APB4H Sleep enable register
    #[inline(always)]
    pub const fn apb4hlpenr(&self) -> &APB4HLPENR {
        &self.apb4hlpenr
    }
    ///0x2bc - RCC APB5 Sleep enable register
    #[inline(always)]
    pub const fn apb5lpenr(&self) -> &APB5LPENR {
        &self.apb5lpenr
    }
    ///0x44c - RCC APB5 Sleep enable register
    #[inline(always)]
    pub const fn rdcr(&self) -> &RDCR {
        &self.rdcr
    }
    ///0x780 - RCC oscillator secure configuration register0
    #[inline(always)]
    pub const fn seccfgr0(&self) -> &SECCFGR0 {
        &self.seccfgr0
    }
    ///0x784 - RCC oscillator privilege configuration register0
    #[inline(always)]
    pub const fn privcfgr0(&self) -> &PRIVCFGR0 {
        &self.privcfgr0
    }
    ///0x788 - RCC oscillator lock configuration register0
    #[inline(always)]
    pub const fn lockcfgr0(&self) -> &LOCKCFGR0 {
        &self.lockcfgr0
    }
    ///0x78c - RCC oscillator public configuration register0
    #[inline(always)]
    pub const fn pubcfgr0(&self) -> &PUBCFGR0 {
        &self.pubcfgr0
    }
    ///0x790 - RCC PLL secure configuration register1
    #[inline(always)]
    pub const fn seccfgr1(&self) -> &SECCFGR1 {
        &self.seccfgr1
    }
    ///0x794 - RCC PLL privilege configuration register1
    #[inline(always)]
    pub const fn privcfgr1(&self) -> &PRIVCFGR1 {
        &self.privcfgr1
    }
    ///0x798 - RCC PLL lock configuration register1
    #[inline(always)]
    pub const fn lockcfgr1(&self) -> &LOCKCFGR1 {
        &self.lockcfgr1
    }
    ///0x79c - RCC PLL public configuration register1
    #[inline(always)]
    pub const fn pubcfgr1(&self) -> &PUBCFGR1 {
        &self.pubcfgr1
    }
    ///0x7a0 - RCC divider secure configuration register2
    #[inline(always)]
    pub const fn seccfgr2(&self) -> &SECCFGR2 {
        &self.seccfgr2
    }
    ///0x7a4 - RCC divider privilege configuration register2
    #[inline(always)]
    pub const fn privcfgr2(&self) -> &PRIVCFGR2 {
        &self.privcfgr2
    }
    ///0x7a8 - RCC divider lock configuration register2
    #[inline(always)]
    pub const fn lockcfgr2(&self) -> &LOCKCFGR2 {
        &self.lockcfgr2
    }
    ///0x7ac - RCC divider public configuration register2
    #[inline(always)]
    pub const fn pubcfgr2(&self) -> &PUBCFGR2 {
        &self.pubcfgr2
    }
    ///0x7b0 - RCC system secure configuration register3
    #[inline(always)]
    pub const fn seccfgr3(&self) -> &SECCFGR3 {
        &self.seccfgr3
    }
    ///0x7b4 - RCC system privilege configuration register3
    #[inline(always)]
    pub const fn privcfgr3(&self) -> &PRIVCFGR3 {
        &self.privcfgr3
    }
    ///0x7b8 - RCC system lock configuration register3
    #[inline(always)]
    pub const fn lockcfgr3(&self) -> &LOCKCFGR3 {
        &self.lockcfgr3
    }
    ///0x7bc - RCC system public configuration register3
    #[inline(always)]
    pub const fn pubcfgr3(&self) -> &PUBCFGR3 {
        &self.pubcfgr3
    }
    ///0x7c0 - RCC bus secure configuration register4
    #[inline(always)]
    pub const fn seccfgr4(&self) -> &SECCFGR4 {
        &self.seccfgr4
    }
    ///0x7c4 - RCC bus privilege configuration register4
    #[inline(always)]
    pub const fn privcfgr4(&self) -> &PRIVCFGR4 {
        &self.privcfgr4
    }
    ///0x7c8 - RCC bus lock configuration register4
    #[inline(always)]
    pub const fn lockcfgr4(&self) -> &LOCKCFGR4 {
        &self.lockcfgr4
    }
    ///0x7cc - RCC bus public configuration register4
    #[inline(always)]
    pub const fn pubcfgr4(&self) -> &PUBCFGR4 {
        &self.pubcfgr4
    }
    ///0x7d0 - RCC bus public configuration register4
    #[inline(always)]
    pub const fn pubcfgr5(&self) -> &PUBCFGR5 {
        &self.pubcfgr5
    }
    ///0x800 - RCC control set register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x808 - RCC Stop configuration register
    #[inline(always)]
    pub const fn stopcsr(&self) -> &STOPCSR {
        &self.stopcsr
    }
    ///0xa04 - RCC bus reset set register
    #[inline(always)]
    pub const fn busrstsr(&self) -> &BUSRSTSR {
        &self.busrstsr
    }
    ///0xa08 - RCC miscellaneous reset register
    #[inline(always)]
    pub const fn miscrstsr(&self) -> &MISCRSTSR {
        &self.miscrstsr
    }
    ///0xa0c - RCC memory reset register
    #[inline(always)]
    pub const fn memrstsr(&self) -> &MEMRSTSR {
        &self.memrstsr
    }
    ///0xa10 - RCC AHB1 reset register
    #[inline(always)]
    pub const fn ahb1rstsr(&self) -> &AHB1RSTSR {
        &self.ahb1rstsr
    }
    ///0xa14 - RCC AHB2 reset register
    #[inline(always)]
    pub const fn ahb2rstsr(&self) -> &AHB2RSTSR {
        &self.ahb2rstsr
    }
    ///0xa18 - RCC AHB3 reset register
    #[inline(always)]
    pub const fn ahb3rstsr(&self) -> &AHB3RSTSR {
        &self.ahb3rstsr
    }
    ///0xa1c - RCC AHB4 reset register
    #[inline(always)]
    pub const fn ahb4rstsr(&self) -> &AHB4RSTSR {
        &self.ahb4rstsr
    }
    ///0xa20 - RCC AHB5 reset register
    #[inline(always)]
    pub const fn ahb5rstsr(&self) -> &AHB5RSTSR {
        &self.ahb5rstsr
    }
    ///0xa24 - RCC APB1L reset register
    #[inline(always)]
    pub const fn apb1lrstsr(&self) -> &APB1LRSTSR {
        &self.apb1lrstsr
    }
    ///0xa28 - RCC APB1H reset register
    #[inline(always)]
    pub const fn apb1hrstsr(&self) -> &APB1HRSTSR {
        &self.apb1hrstsr
    }
    ///0xa2c - RCC APB2 reset register
    #[inline(always)]
    pub const fn apb2rstsr(&self) -> &APB2RSTSR {
        &self.apb2rstsr
    }
    ///0xa34 - RCC APB4L reset register
    #[inline(always)]
    pub const fn apb4lrstsr(&self) -> &APB4LRSTSR {
        &self.apb4lrstsr
    }
    ///0xa38 - RCC APB4H reset register
    #[inline(always)]
    pub const fn apb4hrstsr(&self) -> &APB4HRSTSR {
        &self.apb4hrstsr
    }
    ///0xa3c - RCC APB5 reset register
    #[inline(always)]
    pub const fn apb5rstsr(&self) -> &APB5RSTSR {
        &self.apb5rstsr
    }
    ///0xa40 - RCC Divider enable register
    #[inline(always)]
    pub const fn divensr(&self) -> &DIVENSR {
        &self.divensr
    }
    ///0xa44 - RCC bus enable register
    #[inline(always)]
    pub const fn busensr(&self) -> &BUSENSR {
        &self.busensr
    }
    ///0xa48 - RCC miscellaneous enable register
    #[inline(always)]
    pub const fn miscensr(&self) -> &MISCENSR {
        &self.miscensr
    }
    ///0xa4c - RCC memory enable register
    #[inline(always)]
    pub const fn memensr(&self) -> &MEMENSR {
        &self.memensr
    }
    ///0xa50 - RCC AHB1 enable register
    #[inline(always)]
    pub const fn ahb1ensr(&self) -> &AHB1ENSR {
        &self.ahb1ensr
    }
    ///0xa54 - RCC AHB2 enable register
    #[inline(always)]
    pub const fn ahb2ensr(&self) -> &AHB2ENSR {
        &self.ahb2ensr
    }
    ///0xa58 - RCC AHB3 enable register
    #[inline(always)]
    pub const fn ahb3ensr(&self) -> &AHB3ENSR {
        &self.ahb3ensr
    }
    ///0xa5c - RCC AHB4 enable register
    #[inline(always)]
    pub const fn ahb4ensr(&self) -> &AHB4ENSR {
        &self.ahb4ensr
    }
    ///0xa60 - RCC AHB5 enable register
    #[inline(always)]
    pub const fn ahb5ensr(&self) -> &AHB5ENSR {
        &self.ahb5ensr
    }
    ///0xa64 - RCC APB1L enable register
    #[inline(always)]
    pub const fn apb1lensr(&self) -> &APB1LENSR {
        &self.apb1lensr
    }
    ///0xa68 - RCC APB1H enable register
    #[inline(always)]
    pub const fn apb1hensr(&self) -> &APB1HENSR {
        &self.apb1hensr
    }
    ///0xa6c - RCC APB2 enable register
    #[inline(always)]
    pub const fn apb2ensr(&self) -> &APB2ENSR {
        &self.apb2ensr
    }
    ///0xa70 - RCC APB3 enable register
    #[inline(always)]
    pub const fn apb3ensr(&self) -> &APB3ENSR {
        &self.apb3ensr
    }
    ///0xa74 - RCC APB4L enable register
    #[inline(always)]
    pub const fn apb4lensr(&self) -> &APB4LENSR {
        &self.apb4lensr
    }
    ///0xa78 - RCC APB4H enable register
    #[inline(always)]
    pub const fn apb4hensr(&self) -> &APB4HENSR {
        &self.apb4hensr
    }
    ///0xa7c - RCC APB5 enable register
    #[inline(always)]
    pub const fn apb5ensr(&self) -> &APB5ENSR {
        &self.apb5ensr
    }
    ///0xa80 - RCC divider Sleep enable register
    #[inline(always)]
    pub const fn divlpensr(&self) -> &DIVLPENSR {
        &self.divlpensr
    }
    ///0xa84 - RCC bus Sleep enable register
    #[inline(always)]
    pub const fn buslpensr(&self) -> &BUSLPENSR {
        &self.buslpensr
    }
    ///0xa88 - RCC miscellaneous Sleep enable register
    #[inline(always)]
    pub const fn misclpensr(&self) -> &MISCLPENSR {
        &self.misclpensr
    }
    ///0xa8c - RCC memory sleep enable register
    #[inline(always)]
    pub const fn memlpensr(&self) -> &MEMLPENSR {
        &self.memlpensr
    }
    ///0xa90 - RCC AHB1 Sleep enable register
    #[inline(always)]
    pub const fn ahb1lpensr(&self) -> &AHB1LPENSR {
        &self.ahb1lpensr
    }
    ///0xa94 - RCC AHB2 Sleep enable register
    #[inline(always)]
    pub const fn ahb2lpensr(&self) -> &AHB2LPENSR {
        &self.ahb2lpensr
    }
    ///0xa98 - RCC AHB3 Sleep enable register
    #[inline(always)]
    pub const fn ahb3lpensr(&self) -> &AHB3LPENSR {
        &self.ahb3lpensr
    }
    ///0xa9c - RCC AHB4 Sleep enable register
    #[inline(always)]
    pub const fn ahb4lpensr(&self) -> &AHB4LPENSR {
        &self.ahb4lpensr
    }
    ///0xaa0 - RCC AHB5 Sleep enable register
    #[inline(always)]
    pub const fn ahb5lpensr(&self) -> &AHB5LPENSR {
        &self.ahb5lpensr
    }
    ///0xaa4 - RCC APB1L Sleep enable register
    #[inline(always)]
    pub const fn apb1llpensr(&self) -> &APB1LLPENSR {
        &self.apb1llpensr
    }
    ///0xaa8 - RCC APB1H Sleep enable register
    #[inline(always)]
    pub const fn apb1hlpensr(&self) -> &APB1HLPENSR {
        &self.apb1hlpensr
    }
    ///0xaac - RCC APB2 Sleep enable register
    #[inline(always)]
    pub const fn apb2lpensr(&self) -> &APB2LPENSR {
        &self.apb2lpensr
    }
    ///0xab0 - RCC APB3 Sleep enable register
    #[inline(always)]
    pub const fn apb3lpensr(&self) -> &APB3LPENSR {
        &self.apb3lpensr
    }
    ///0xab4 - RCC APB4L Sleep enable register
    #[inline(always)]
    pub const fn apb4llpensr(&self) -> &APB4LLPENSR {
        &self.apb4llpensr
    }
    ///0xab8 - RCC APB4H Sleep enable register
    #[inline(always)]
    pub const fn apb4hlpensr(&self) -> &APB4HLPENSR {
        &self.apb4hlpensr
    }
    ///0xabc - RCC APB5 Sleep enable register
    #[inline(always)]
    pub const fn apb5lpensr(&self) -> &APB5LPENSR {
        &self.apb5lpensr
    }
    ///0xf84 - RCC oscillator privilege configuration register0
    #[inline(always)]
    pub const fn privcfgsr0(&self) -> &PRIVCFGSR0 {
        &self.privcfgsr0
    }
    ///0xf8c - RCC oscillator public configuration register0
    #[inline(always)]
    pub const fn pubcfgsr0(&self) -> &PUBCFGSR0 {
        &self.pubcfgsr0
    }
    ///0xf94 - RCC PLL privilege configuration register1
    #[inline(always)]
    pub const fn privcfgsr1(&self) -> &PRIVCFGSR1 {
        &self.privcfgsr1
    }
    ///0xf9c - RCC PLL public configuration register1
    #[inline(always)]
    pub const fn pubcfgsr1(&self) -> &PUBCFGSR1 {
        &self.pubcfgsr1
    }
    ///0xfa4 - RCC divider privilege configuration register2
    #[inline(always)]
    pub const fn privcfgsr2(&self) -> &PRIVCFGSR2 {
        &self.privcfgsr2
    }
    ///0xfac - RCC divider public configuration register2
    #[inline(always)]
    pub const fn pubcfgsr2(&self) -> &PUBCFGSR2 {
        &self.pubcfgsr2
    }
    ///0xfb0 - RCC system secure configuration register3
    #[inline(always)]
    pub const fn seccfgsr3(&self) -> &SECCFGSR3 {
        &self.seccfgsr3
    }
    ///0xfb4 - RCC system privilege configuration register3
    #[inline(always)]
    pub const fn privcfgsr3(&self) -> &PRIVCFGSR3 {
        &self.privcfgsr3
    }
    ///0xfb8 - RCC system lock configuration register3
    #[inline(always)]
    pub const fn lockcfgsr3(&self) -> &LOCKCFGSR3 {
        &self.lockcfgsr3
    }
    ///0xfbc - RCC system public configuration register3
    #[inline(always)]
    pub const fn pubcfgsr3(&self) -> &PUBCFGSR3 {
        &self.pubcfgsr3
    }
    ///0xfc4 - RCC privilege configuration register4
    #[inline(always)]
    pub const fn privcfgsr4(&self) -> &PRIVCFGSR4 {
        &self.privcfgsr4
    }
    ///0xfcc - RCC public configuration register4
    #[inline(always)]
    pub const fn pubcfgsr4(&self) -> &PUBCFGSR4 {
        &self.pubcfgsr4
    }
    ///0xfd0 - RCC public configuration register4
    #[inline(always)]
    pub const fn pubcfgsr5(&self) -> &PUBCFGSR5 {
        &self.pubcfgsr5
    }
    ///0x1000 - RCC control Clear register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x1008 - RCC StopCCR configuration register
    #[inline(always)]
    pub const fn stopccr(&self) -> &STOPCCR {
        &self.stopccr
    }
    ///0x1204 - RCC bus reset register
    #[inline(always)]
    pub const fn busrstcr(&self) -> &BUSRSTCR {
        &self.busrstcr
    }
    ///0x1208 - RCC miscellaneous reset register
    #[inline(always)]
    pub const fn miscrstcr(&self) -> &MISCRSTCR {
        &self.miscrstcr
    }
    ///0x120c - RCC memory reset register
    #[inline(always)]
    pub const fn memrstcr(&self) -> &MEMRSTCR {
        &self.memrstcr
    }
    ///0x1210 - RCC AHB1 reset register
    #[inline(always)]
    pub const fn ahb1rstcr(&self) -> &AHB1RSTCR {
        &self.ahb1rstcr
    }
    ///0x1214 - RCC AHB2 Reset register
    #[inline(always)]
    pub const fn ahb2rstcr(&self) -> &AHB2RSTCR {
        &self.ahb2rstcr
    }
    ///0x1218 - RCC AHB3 reset register
    #[inline(always)]
    pub const fn ahb3rstcr(&self) -> &AHB3RSTCR {
        &self.ahb3rstcr
    }
    ///0x121c - RCC AHB4 reset register
    #[inline(always)]
    pub const fn ahb4rstcr(&self) -> &AHB4RSTCR {
        &self.ahb4rstcr
    }
    ///0x1220 - RCC AHB5 reset register
    #[inline(always)]
    pub const fn ahb5rstcr(&self) -> &AHB5RSTCR {
        &self.ahb5rstcr
    }
    ///0x1224 - RCC APB1L reset register
    #[inline(always)]
    pub const fn apb1lrstcr(&self) -> &APB1LRSTCR {
        &self.apb1lrstcr
    }
    ///0x1228 - RCC APB1H reset register
    #[inline(always)]
    pub const fn apb1hrstcr(&self) -> &APB1HRSTCR {
        &self.apb1hrstcr
    }
    ///0x122c - RCC APB2 reset register
    #[inline(always)]
    pub const fn apb2rstcr(&self) -> &APB2RSTCR {
        &self.apb2rstcr
    }
    ///0x1234 - RCC APB4L reset register
    #[inline(always)]
    pub const fn apb4lrstcr(&self) -> &APB4LRSTCR {
        &self.apb4lrstcr
    }
    ///0x1238 - RCC APB4H reset register
    #[inline(always)]
    pub const fn apb4hrstcr(&self) -> &APB4HRSTCR {
        &self.apb4hrstcr
    }
    ///0x123c - RCC APB5 reset register
    #[inline(always)]
    pub const fn apb5rstcr(&self) -> &APB5RSTCR {
        &self.apb5rstcr
    }
    ///0x1240 - RCC divider enable register
    #[inline(always)]
    pub const fn divencr(&self) -> &DIVENCR {
        &self.divencr
    }
    ///0x1244 - RCC bus enable register
    #[inline(always)]
    pub const fn busencr(&self) -> &BUSENCR {
        &self.busencr
    }
    ///0x1248 - RCC miscellaneous enable register
    #[inline(always)]
    pub const fn miscencr(&self) -> &MISCENCR {
        &self.miscencr
    }
    ///0x124c - RCC memory enable register
    #[inline(always)]
    pub const fn memencr(&self) -> &MEMENCR {
        &self.memencr
    }
    ///0x1250 - RCC AHB1 enable register
    #[inline(always)]
    pub const fn ahb1encr(&self) -> &AHB1ENCR {
        &self.ahb1encr
    }
    ///0x1254 - RCC AHB2 enable register
    #[inline(always)]
    pub const fn ahb2encr(&self) -> &AHB2ENCR {
        &self.ahb2encr
    }
    ///0x1258 - RCC AHB3 enable register
    #[inline(always)]
    pub const fn ahb3encr(&self) -> &AHB3ENCR {
        &self.ahb3encr
    }
    ///0x125c - RCC AHB4 enable register
    #[inline(always)]
    pub const fn ahb4encr(&self) -> &AHB4ENCR {
        &self.ahb4encr
    }
    ///0x1260 - RCC AHB5 enable register
    #[inline(always)]
    pub const fn ahb5encr(&self) -> &AHB5ENCR {
        &self.ahb5encr
    }
    ///0x1264 - RCC APB1L enable register
    #[inline(always)]
    pub const fn apb1lencr(&self) -> &APB1LENCR {
        &self.apb1lencr
    }
    ///0x1268 - RCC APB1H enable register
    #[inline(always)]
    pub const fn apb1hencr(&self) -> &APB1HENCR {
        &self.apb1hencr
    }
    ///0x126c - RCC APB2 enable register
    #[inline(always)]
    pub const fn apb2encr(&self) -> &APB2ENCR {
        &self.apb2encr
    }
    ///0x1270 - RCC APB3 enable register
    #[inline(always)]
    pub const fn apb3encr(&self) -> &APB3ENCR {
        &self.apb3encr
    }
    ///0x1274 - RCC APB4L enable register
    #[inline(always)]
    pub const fn apb4lencr(&self) -> &APB4LENCR {
        &self.apb4lencr
    }
    ///0x1278 - RCC APB4H enable register
    #[inline(always)]
    pub const fn apb4hencr(&self) -> &APB4HENCR {
        &self.apb4hencr
    }
    ///0x127c - RCC APB5 enable register
    #[inline(always)]
    pub const fn apb5encr(&self) -> &APB5ENCR {
        &self.apb5encr
    }
    ///0x1280 - RCC divider Sleep enable register
    #[inline(always)]
    pub const fn divlpencr(&self) -> &DIVLPENCR {
        &self.divlpencr
    }
    ///0x1284 - RCC bus Sleep enable register
    #[inline(always)]
    pub const fn buslpencr(&self) -> &BUSLPENCR {
        &self.buslpencr
    }
    ///0x1288 - RCC miscellaneous Sleep enable register
    #[inline(always)]
    pub const fn misclpencr(&self) -> &MISCLPENCR {
        &self.misclpencr
    }
    ///0x128c - RCC memory Sleep enable register
    #[inline(always)]
    pub const fn memlpencr(&self) -> &MEMLPENCR {
        &self.memlpencr
    }
    ///0x1290 - RCC AHB1 Sleep enable register
    #[inline(always)]
    pub const fn ahb1lpencr(&self) -> &AHB1LPENCR {
        &self.ahb1lpencr
    }
    ///0x1294 - RCC AHB2 Sleep enable register
    #[inline(always)]
    pub const fn ahb2lpencr(&self) -> &AHB2LPENCR {
        &self.ahb2lpencr
    }
    ///0x1298 - RCC AHB3 Sleep enable register
    #[inline(always)]
    pub const fn ahb3lpencr(&self) -> &AHB3LPENCR {
        &self.ahb3lpencr
    }
    ///0x129c - RCC AHB4 Sleep enable register
    #[inline(always)]
    pub const fn ahb4lpencr(&self) -> &AHB4LPENCR {
        &self.ahb4lpencr
    }
    ///0x12a0 - RCC AHB5 Sleep enable register
    #[inline(always)]
    pub const fn ahb5lpencr(&self) -> &AHB5LPENCR {
        &self.ahb5lpencr
    }
    ///0x12a4 - RCC APB1L Sleep enable register
    #[inline(always)]
    pub const fn apb1llpencr(&self) -> &APB1LLPENCR {
        &self.apb1llpencr
    }
    ///0x12a8 - RCC APB1H Sleep enable register
    #[inline(always)]
    pub const fn apb1hlpencr(&self) -> &APB1HLPENCR {
        &self.apb1hlpencr
    }
    ///0x12ac - RCC APB2 Sleep enable register
    #[inline(always)]
    pub const fn apb2lpencr(&self) -> &APB2LPENCR {
        &self.apb2lpencr
    }
    ///0x12b0 - RCC APB3 Sleep enable register
    #[inline(always)]
    pub const fn apb3lpencr(&self) -> &APB3LPENCR {
        &self.apb3lpencr
    }
    ///0x12b4 - RCC APB4L Sleep enable register
    #[inline(always)]
    pub const fn apb4llpencr(&self) -> &APB4LLPENCR {
        &self.apb4llpencr
    }
    ///0x12b8 - RCC APB4H Sleep enable register
    #[inline(always)]
    pub const fn apb4hlpencr(&self) -> &APB4HLPENCR {
        &self.apb4hlpencr
    }
    ///0x12bc - RCC APB5 Sleep enable register
    #[inline(always)]
    pub const fn apb5lpencr(&self) -> &APB5LPENCR {
        &self.apb5lpencr
    }
    ///0x1784 - RCC oscillator privilege configuration register0
    #[inline(always)]
    pub const fn privcfgcr0(&self) -> &PRIVCFGCR0 {
        &self.privcfgcr0
    }
    ///0x178c - RCC oscillator public configuration register0
    #[inline(always)]
    pub const fn pubcfgcr0(&self) -> &PUBCFGCR0 {
        &self.pubcfgcr0
    }
    ///0x1794 - RCC PLL privilege configuration register1
    #[inline(always)]
    pub const fn privcfgcr1(&self) -> &PRIVCFGCR1 {
        &self.privcfgcr1
    }
    ///0x179c - RCC PLL public configuration register1
    #[inline(always)]
    pub const fn pubcfgcr1(&self) -> &PUBCFGCR1 {
        &self.pubcfgcr1
    }
    ///0x17a4 - RCC divider privilege configuration register2
    #[inline(always)]
    pub const fn privcfgcr2(&self) -> &PRIVCFGCR2 {
        &self.privcfgcr2
    }
    ///0x17ac - RCC divider public configuration register2
    #[inline(always)]
    pub const fn pubcfgcr2(&self) -> &PUBCFGCR2 {
        &self.pubcfgcr2
    }
    ///0x17b4 - RCC system privilege configuration register3
    #[inline(always)]
    pub const fn privcfgcr3(&self) -> &PRIVCFGCR3 {
        &self.privcfgcr3
    }
    ///0x17bc - RCC system public configuration register3
    #[inline(always)]
    pub const fn pubcfgcr3(&self) -> &PUBCFGCR3 {
        &self.pubcfgcr3
    }
    ///0x17c4 - RCC privilege configuration register4
    #[inline(always)]
    pub const fn privcfgcr4(&self) -> &PRIVCFGCR4 {
        &self.privcfgcr4
    }
    ///0x17cc - RCC public configuration register4
    #[inline(always)]
    pub const fn pubcfgcr4(&self) -> &PUBCFGCR4 {
        &self.pubcfgcr4
    }
    ///0x17d0 - RCC public configuration register4
    #[inline(always)]
    pub const fn pubcfgcr5(&self) -> &PUBCFGCR5 {
        &self.pubcfgcr5
    }
}
/**CR (rw) register accessor: RCC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RCC control register
pub mod cr;
/**SR (r) register accessor: RCC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///RCC status register
pub mod sr;
/**STOPCR (rw) register accessor: RCC Stop mode control register

You can [`read`](crate::Reg::read) this register and get [`stopcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:STOPCR)

For information about available fields see [`mod@stopcr`] module*/
pub type STOPCR = crate::Reg<stopcr::STOPCRrs>;
///RCC Stop mode control register
pub mod stopcr;
/**CFGR1 (rw) register accessor: RCC configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///RCC configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: RCC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///RCC configuration register 2
pub mod cfgr2;
/**CKPROTR (r) register accessor: RCC clock protection register

You can [`read`](crate::Reg::read) this register and get [`ckprotr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CKPROTR)

For information about available fields see [`mod@ckprotr`] module*/
pub type CKPROTR = crate::Reg<ckprotr::CKPROTRrs>;
///RCC clock protection register
pub mod ckprotr;
/**BDCR (rw) register accessor: RCC backup domain protection register

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///RCC backup domain protection register
pub mod bdcr;
/**HWRSR (rw) register accessor: RCC reset status register for hardware

You can [`read`](crate::Reg::read) this register and get [`hwrsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwrsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:HWRSR)

For information about available fields see [`mod@hwrsr`] module*/
pub type HWRSR = crate::Reg<hwrsr::HWRSRrs>;
///RCC reset status register for hardware
pub mod hwrsr;
/**RSR (rw) register accessor: RCC reset register

You can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:RSR)

For information about available fields see [`mod@rsr`] module*/
pub type RSR = crate::Reg<rsr::RSRrs>;
///RCC reset register
pub mod rsr;
/**LSECFGR (rw) register accessor: RCC LSE configuration register

You can [`read`](crate::Reg::read) this register and get [`lsecfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsecfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LSECFGR)

For information about available fields see [`mod@lsecfgr`] module*/
pub type LSECFGR = crate::Reg<lsecfgr::LSECFGRrs>;
///RCC LSE configuration register
pub mod lsecfgr;
/**MSICFGR (rw) register accessor: RCC MSI configuration register

You can [`read`](crate::Reg::read) this register and get [`msicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MSICFGR)

For information about available fields see [`mod@msicfgr`] module*/
pub type MSICFGR = crate::Reg<msicfgr::MSICFGRrs>;
///RCC MSI configuration register
pub mod msicfgr;
/**HSICFGR (rw) register accessor: RCC HSI configuration register

You can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:HSICFGR)

For information about available fields see [`mod@hsicfgr`] module*/
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGRrs>;
///RCC HSI configuration register
pub mod hsicfgr;
/**HSIMCR (rw) register accessor: RCC HSI monitor control register

You can [`read`](crate::Reg::read) this register and get [`hsimcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsimcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:HSIMCR)

For information about available fields see [`mod@hsimcr`] module*/
pub type HSIMCR = crate::Reg<hsimcr::HSIMCRrs>;
///RCC HSI monitor control register
pub mod hsimcr;
/**HSIMSR (r) register accessor: RCC HSI monitor status register

You can [`read`](crate::Reg::read) this register and get [`hsimsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:HSIMSR)

For information about available fields see [`mod@hsimsr`] module*/
pub type HSIMSR = crate::Reg<hsimsr::HSIMSRrs>;
///RCC HSI monitor status register
pub mod hsimsr;
/**HSECFGR (rw) register accessor: RCC HSE configuration register

You can [`read`](crate::Reg::read) this register and get [`hsecfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsecfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:HSECFGR)

For information about available fields see [`mod@hsecfgr`] module*/
pub type HSECFGR = crate::Reg<hsecfgr::HSECFGRrs>;
///RCC HSE configuration register
pub mod hsecfgr;
/**PLL1CFGR1 (rw) register accessor: RCC PLL1 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL1CFGR1)

For information about available fields see [`mod@pll1cfgr1`] module*/
pub type PLL1CFGR1 = crate::Reg<pll1cfgr1::PLL1CFGR1rs>;
///RCC PLL1 configuration register 1
pub mod pll1cfgr1;
/**PLL1CFGR2 (rw) register accessor: RCC PLL1 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL1CFGR2)

For information about available fields see [`mod@pll1cfgr2`] module*/
pub type PLL1CFGR2 = crate::Reg<pll1cfgr2::PLL1CFGR2rs>;
///RCC PLL1 configuration register 2
pub mod pll1cfgr2;
/**PLL1CFGR3 (rw) register accessor: RCC PLL1 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL1CFGR3)

For information about available fields see [`mod@pll1cfgr3`] module*/
pub type PLL1CFGR3 = crate::Reg<pll1cfgr3::PLL1CFGR3rs>;
///RCC PLL1 configuration register 3
pub mod pll1cfgr3;
/**PLL2CFGR1 (rw) register accessor: RCC PLL2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL2CFGR1)

For information about available fields see [`mod@pll2cfgr1`] module*/
pub type PLL2CFGR1 = crate::Reg<pll2cfgr1::PLL2CFGR1rs>;
///RCC PLL2 configuration register 1
pub mod pll2cfgr1;
/**PLL2CFGR2 (rw) register accessor: RCC PLL2 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL2CFGR2)

For information about available fields see [`mod@pll2cfgr2`] module*/
pub type PLL2CFGR2 = crate::Reg<pll2cfgr2::PLL2CFGR2rs>;
///RCC PLL2 configuration register 2
pub mod pll2cfgr2;
/**PLL2CFGR3 (rw) register accessor: RCC PLL2 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL2CFGR3)

For information about available fields see [`mod@pll2cfgr3`] module*/
pub type PLL2CFGR3 = crate::Reg<pll2cfgr3::PLL2CFGR3rs>;
///RCC PLL2 configuration register 3
pub mod pll2cfgr3;
/**PLL3CFGR1 (rw) register accessor: RCC PLL3 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL3CFGR1)

For information about available fields see [`mod@pll3cfgr1`] module*/
pub type PLL3CFGR1 = crate::Reg<pll3cfgr1::PLL3CFGR1rs>;
///RCC PLL3 configuration register 1
pub mod pll3cfgr1;
/**PLL3CFGR2 (rw) register accessor: RCC PLL3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL3CFGR2)

For information about available fields see [`mod@pll3cfgr2`] module*/
pub type PLL3CFGR2 = crate::Reg<pll3cfgr2::PLL3CFGR2rs>;
///RCC PLL3 configuration register 2
pub mod pll3cfgr2;
/**PLL3CFGR3 (rw) register accessor: RCC PLL3 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL3CFGR3)

For information about available fields see [`mod@pll3cfgr3`] module*/
pub type PLL3CFGR3 = crate::Reg<pll3cfgr3::PLL3CFGR3rs>;
///RCC PLL3 configuration register 3
pub mod pll3cfgr3;
/**PLL4CFGR1 (rw) register accessor: RCC PLL4 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL4CFGR1)

For information about available fields see [`mod@pll4cfgr1`] module*/
pub type PLL4CFGR1 = crate::Reg<pll4cfgr1::PLL4CFGR1rs>;
///RCC PLL4 configuration register 1
pub mod pll4cfgr1;
/**PLL4CFGR2 (rw) register accessor: RCC PLL4 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL4CFGR2)

For information about available fields see [`mod@pll4cfgr2`] module*/
pub type PLL4CFGR2 = crate::Reg<pll4cfgr2::PLL4CFGR2rs>;
///RCC PLL4 configuration register 2
pub mod pll4cfgr2;
/**PLL4CFGR3 (rw) register accessor: RCC PLL4 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL4CFGR3)

For information about available fields see [`mod@pll4cfgr3`] module*/
pub type PLL4CFGR3 = crate::Reg<pll4cfgr3::PLL4CFGR3rs>;
///RCC PLL4 configuration register 3
pub mod pll4cfgr3;
/**IC1CFGR (rw) register accessor: RCC IC1 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC1CFGR)

For information about available fields see [`mod@ic1cfgr`] module*/
pub type IC1CFGR = crate::Reg<ic1cfgr::IC1CFGRrs>;
///RCC IC1 configuration register
pub mod ic1cfgr;
/**IC2CFGR (rw) register accessor: RCC IC2 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC2CFGR)

For information about available fields see [`mod@ic2cfgr`] module*/
pub type IC2CFGR = crate::Reg<ic2cfgr::IC2CFGRrs>;
///RCC IC2 configuration register
pub mod ic2cfgr;
/**IC3CFGR (rw) register accessor: RCC IC3 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC3CFGR)

For information about available fields see [`mod@ic3cfgr`] module*/
pub type IC3CFGR = crate::Reg<ic3cfgr::IC3CFGRrs>;
///RCC IC3 configuration register
pub mod ic3cfgr;
/**IC4CFGR (rw) register accessor: RCC IC4 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic4cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic4cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC4CFGR)

For information about available fields see [`mod@ic4cfgr`] module*/
pub type IC4CFGR = crate::Reg<ic4cfgr::IC4CFGRrs>;
///RCC IC4 configuration register
pub mod ic4cfgr;
/**IC5CFGR (rw) register accessor: RCC IC5 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic5cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic5cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC5CFGR)

For information about available fields see [`mod@ic5cfgr`] module*/
pub type IC5CFGR = crate::Reg<ic5cfgr::IC5CFGRrs>;
///RCC IC5 configuration register
pub mod ic5cfgr;
/**IC6CFGR (rw) register accessor: RCC IC6 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic6cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic6cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC6CFGR)

For information about available fields see [`mod@ic6cfgr`] module*/
pub type IC6CFGR = crate::Reg<ic6cfgr::IC6CFGRrs>;
///RCC IC6 configuration register
pub mod ic6cfgr;
/**IC7CFGR (rw) register accessor: RCC IC7 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic7cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic7cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC7CFGR)

For information about available fields see [`mod@ic7cfgr`] module*/
pub type IC7CFGR = crate::Reg<ic7cfgr::IC7CFGRrs>;
///RCC IC7 configuration register
pub mod ic7cfgr;
/**IC8CFGR (rw) register accessor: RCC IC8 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic8cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic8cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC8CFGR)

For information about available fields see [`mod@ic8cfgr`] module*/
pub type IC8CFGR = crate::Reg<ic8cfgr::IC8CFGRrs>;
///RCC IC8 configuration register
pub mod ic8cfgr;
/**IC9CFGR (rw) register accessor: RCC IC9 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic9cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic9cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC9CFGR)

For information about available fields see [`mod@ic9cfgr`] module*/
pub type IC9CFGR = crate::Reg<ic9cfgr::IC9CFGRrs>;
///RCC IC9 configuration register
pub mod ic9cfgr;
/**IC10CFGR (rw) register accessor: RCC IC10 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic10cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic10cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC10CFGR)

For information about available fields see [`mod@ic10cfgr`] module*/
pub type IC10CFGR = crate::Reg<ic10cfgr::IC10CFGRrs>;
///RCC IC10 configuration register
pub mod ic10cfgr;
/**IC11CFGR (rw) register accessor: RCC IC11 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic11cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic11cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC11CFGR)

For information about available fields see [`mod@ic11cfgr`] module*/
pub type IC11CFGR = crate::Reg<ic11cfgr::IC11CFGRrs>;
///RCC IC11 configuration register
pub mod ic11cfgr;
/**IC12CFGR (rw) register accessor: RCC IC12 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic12cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic12cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC12CFGR)

For information about available fields see [`mod@ic12cfgr`] module*/
pub type IC12CFGR = crate::Reg<ic12cfgr::IC12CFGRrs>;
///RCC IC12 configuration register
pub mod ic12cfgr;
/**IC13CFGR (rw) register accessor: RCC IC13 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic13cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic13cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC13CFGR)

For information about available fields see [`mod@ic13cfgr`] module*/
pub type IC13CFGR = crate::Reg<ic13cfgr::IC13CFGRrs>;
///RCC IC13 configuration register
pub mod ic13cfgr;
/**IC14CFGR (rw) register accessor: RCC IC14 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic14cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic14cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC14CFGR)

For information about available fields see [`mod@ic14cfgr`] module*/
pub type IC14CFGR = crate::Reg<ic14cfgr::IC14CFGRrs>;
///RCC IC14 configuration register
pub mod ic14cfgr;
/**IC15CFGR (rw) register accessor: RCC IC15 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic15cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic15cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC15CFGR)

For information about available fields see [`mod@ic15cfgr`] module*/
pub type IC15CFGR = crate::Reg<ic15cfgr::IC15CFGRrs>;
///RCC IC15 configuration register
pub mod ic15cfgr;
/**IC16CFGR (rw) register accessor: RCC IC16 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic16cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic16cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC16CFGR)

For information about available fields see [`mod@ic16cfgr`] module*/
pub type IC16CFGR = crate::Reg<ic16cfgr::IC16CFGRrs>;
///RCC IC16 configuration register
pub mod ic16cfgr;
/**IC17CFGR (rw) register accessor: RCC IC17 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic17cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic17cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC17CFGR)

For information about available fields see [`mod@ic17cfgr`] module*/
pub type IC17CFGR = crate::Reg<ic17cfgr::IC17CFGRrs>;
///RCC IC17 configuration register
pub mod ic17cfgr;
/**IC18CFGR (rw) register accessor: RCC IC18 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic18cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic18cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC18CFGR)

For information about available fields see [`mod@ic18cfgr`] module*/
pub type IC18CFGR = crate::Reg<ic18cfgr::IC18CFGRrs>;
///RCC IC18 configuration register
pub mod ic18cfgr;
/**IC19CFGR (rw) register accessor: RCC IC19 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic19cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic19cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC19CFGR)

For information about available fields see [`mod@ic19cfgr`] module*/
pub type IC19CFGR = crate::Reg<ic19cfgr::IC19CFGRrs>;
///RCC IC19 configuration register
pub mod ic19cfgr;
/**IC20CFGR (rw) register accessor: RCC IC20 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic20cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic20cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC20CFGR)

For information about available fields see [`mod@ic20cfgr`] module*/
pub type IC20CFGR = crate::Reg<ic20cfgr::IC20CFGRrs>;
///RCC IC20 configuration register
pub mod ic20cfgr;
/**CIER (rw) register accessor: RCC clock-source interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///RCC clock-source interrupt enable register
pub mod cier;
/**CIFR (r) register accessor: RCC clock-source interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///RCC clock-source interrupt flag register
pub mod cifr;
/**CICR (w) register accessor: RCC clock-source interrupt Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CICR)

For information about available fields see [`mod@cicr`] module*/
pub type CICR = crate::Reg<cicr::CICRrs>;
///RCC clock-source interrupt Clear register
pub mod cicr;
/**CCIPR1 (rw) register accessor: RCC clock configuration for independent peripheral register1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR1)

For information about available fields see [`mod@ccipr1`] module*/
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1rs>;
///RCC clock configuration for independent peripheral register1
pub mod ccipr1;
/**CCIPR2 (rw) register accessor: RCC clock configuration for independent peripheral register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR2)

For information about available fields see [`mod@ccipr2`] module*/
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2rs>;
///RCC clock configuration for independent peripheral register 2
pub mod ccipr2;
/**CCIPR3 (rw) register accessor: RCC clock configuration for independent peripheral register3

You can [`read`](crate::Reg::read) this register and get [`ccipr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR3)

For information about available fields see [`mod@ccipr3`] module*/
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3rs>;
///RCC clock configuration for independent peripheral register3
pub mod ccipr3;
/**CCIPR4 (rw) register accessor: RCC clock configuration for independent peripheral register4

You can [`read`](crate::Reg::read) this register and get [`ccipr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR4)

For information about available fields see [`mod@ccipr4`] module*/
pub type CCIPR4 = crate::Reg<ccipr4::CCIPR4rs>;
///RCC clock configuration for independent peripheral register4
pub mod ccipr4;
/**CCIPR5 (rw) register accessor: RCC lock configuration for independent peripheral register5

You can [`read`](crate::Reg::read) this register and get [`ccipr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR5)

For information about available fields see [`mod@ccipr5`] module*/
pub type CCIPR5 = crate::Reg<ccipr5::CCIPR5rs>;
///RCC lock configuration for independent peripheral register5
pub mod ccipr5;
/**CCIPR6 (rw) register accessor: RCC clock configuration for independent peripheral register6

You can [`read`](crate::Reg::read) this register and get [`ccipr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR6)

For information about available fields see [`mod@ccipr6`] module*/
pub type CCIPR6 = crate::Reg<ccipr6::CCIPR6rs>;
///RCC clock configuration for independent peripheral register6
pub mod ccipr6;
/**CCIPR7 (rw) register accessor: RCC clock configuration for independent peripheral register7

You can [`read`](crate::Reg::read) this register and get [`ccipr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR7)

For information about available fields see [`mod@ccipr7`] module*/
pub type CCIPR7 = crate::Reg<ccipr7::CCIPR7rs>;
///RCC clock configuration for independent peripheral register7
pub mod ccipr7;
/**CCIPR8 (rw) register accessor: RCC clock configuration for independent peripheral register8

You can [`read`](crate::Reg::read) this register and get [`ccipr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR8)

For information about available fields see [`mod@ccipr8`] module*/
pub type CCIPR8 = crate::Reg<ccipr8::CCIPR8rs>;
///RCC clock configuration for independent peripheral register8
pub mod ccipr8;
/**CCIPR9 (rw) register accessor: RCC clock configuration for independent peripheral register9

You can [`read`](crate::Reg::read) this register and get [`ccipr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR9)

For information about available fields see [`mod@ccipr9`] module*/
pub type CCIPR9 = crate::Reg<ccipr9::CCIPR9rs>;
///RCC clock configuration for independent peripheral register9
pub mod ccipr9;
/**CCIPR12 (rw) register accessor: RCC clock configuration for independent peripheral register12

You can [`read`](crate::Reg::read) this register and get [`ccipr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR12)

For information about available fields see [`mod@ccipr12`] module*/
pub type CCIPR12 = crate::Reg<ccipr12::CCIPR12rs>;
///RCC clock configuration for independent peripheral register12
pub mod ccipr12;
/**CCIPR13 (rw) register accessor: RCC clock configuration for independent peripheral register13

You can [`read`](crate::Reg::read) this register and get [`ccipr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR13)

For information about available fields see [`mod@ccipr13`] module*/
pub type CCIPR13 = crate::Reg<ccipr13::CCIPR13rs>;
///RCC clock configuration for independent peripheral register13
pub mod ccipr13;
/**CCIPR14 (rw) register accessor: RCC clock configuration for independent peripheral register14

You can [`read`](crate::Reg::read) this register and get [`ccipr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCIPR14)

For information about available fields see [`mod@ccipr14`] module*/
pub type CCIPR14 = crate::Reg<ccipr14::CCIPR14rs>;
///RCC clock configuration for independent peripheral register14
pub mod ccipr14;
/**BUSRSTR (rw) register accessor: RCC SoC buses reset register

You can [`read`](crate::Reg::read) this register and get [`busrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSRSTR)

For information about available fields see [`mod@busrstr`] module*/
pub type BUSRSTR = crate::Reg<busrstr::BUSRSTRrs>;
///RCC SoC buses reset register
pub mod busrstr;
/**MISCRSTR (rw) register accessor: RCC miscellaneous configurations reset register

You can [`read`](crate::Reg::read) this register and get [`miscrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCRSTR)

For information about available fields see [`mod@miscrstr`] module*/
pub type MISCRSTR = crate::Reg<miscrstr::MISCRSTRrs>;
///RCC miscellaneous configurations reset register
pub mod miscrstr;
/**MEMRSTR (rw) register accessor: RCC memories reset register

You can [`read`](crate::Reg::read) this register and get [`memrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMRSTR)

For information about available fields see [`mod@memrstr`] module*/
pub type MEMRSTR = crate::Reg<memrstr::MEMRSTRrs>;
///RCC memories reset register
pub mod memrstr;
/**AHB1RSTR (rw) register accessor: RCC AHB1 Reset register

You can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1RSTR)

For information about available fields see [`mod@ahb1rstr`] module*/
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTRrs>;
///RCC AHB1 Reset register
pub mod ahb1rstr;
/**AHB2RSTR (rw) register accessor: RCC AHB2 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2RSTR)

For information about available fields see [`mod@ahb2rstr`] module*/
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTRrs>;
///RCC AHB2 reset register
pub mod ahb2rstr;
/**AHB3RSTR (rw) register accessor: RCC AHB3 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3RSTR)

For information about available fields see [`mod@ahb3rstr`] module*/
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTRrs>;
///RCC AHB3 reset register
pub mod ahb3rstr;
/**AHB4RSTR (rw) register accessor: RCC AHB4 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4RSTR)

For information about available fields see [`mod@ahb4rstr`] module*/
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTRrs>;
///RCC AHB4 reset register
pub mod ahb4rstr;
/**AHB5RSTR (rw) register accessor: RCC AHB5 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb5rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5RSTR)

For information about available fields see [`mod@ahb5rstr`] module*/
pub type AHB5RSTR = crate::Reg<ahb5rstr::AHB5RSTRrs>;
///RCC AHB5 reset register
pub mod ahb5rstr;
/**APB1LRSTR (rw) register accessor: RCC APB1L reset register

You can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LRSTR)

For information about available fields see [`mod@apb1lrstr`] module*/
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTRrs>;
///RCC APB1L reset register
pub mod apb1lrstr;
/**APB1HRSTR (rw) register accessor: RCC APB1H reset register

You can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HRSTR)

For information about available fields see [`mod@apb1hrstr`] module*/
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTRrs>;
///RCC APB1H reset register
pub mod apb1hrstr;
/**APB2RSTR (rw) register accessor: RCC APB2 reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///RCC APB2 reset register
pub mod apb2rstr;
/**APB4LRSTR (rw) register accessor: RCC APB4L reset register

You can [`read`](crate::Reg::read) this register and get [`apb4lrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LRSTR)

For information about available fields see [`mod@apb4lrstr`] module*/
pub type APB4LRSTR = crate::Reg<apb4lrstr::APB4LRSTRrs>;
///RCC APB4L reset register
pub mod apb4lrstr;
/**APB4HRSTR (rw) register accessor: RCC APB4H reset register

You can [`read`](crate::Reg::read) this register and get [`apb4hrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HRSTR)

For information about available fields see [`mod@apb4hrstr`] module*/
pub type APB4HRSTR = crate::Reg<apb4hrstr::APB4HRSTRrs>;
///RCC APB4H reset register
pub mod apb4hrstr;
/**APB5RSTR (rw) register accessor: RCC APB5 reset register

You can [`read`](crate::Reg::read) this register and get [`apb5rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5RSTR)

For information about available fields see [`mod@apb5rstr`] module*/
pub type APB5RSTR = crate::Reg<apb5rstr::APB5RSTRrs>;
///RCC APB5 reset register
pub mod apb5rstr;
/**DIVENR (rw) register accessor: RCC IC dividers enable register

You can [`read`](crate::Reg::read) this register and get [`divenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:DIVENR)

For information about available fields see [`mod@divenr`] module*/
pub type DIVENR = crate::Reg<divenr::DIVENRrs>;
///RCC IC dividers enable register
pub mod divenr;
/**BUSENR (rw) register accessor: RCC SoC buses enable register

You can [`read`](crate::Reg::read) this register and get [`busenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSENR)

For information about available fields see [`mod@busenr`] module*/
pub type BUSENR = crate::Reg<busenr::BUSENRrs>;
///RCC SoC buses enable register
pub mod busenr;
/**MISCENR (rw) register accessor: RCC miscellaneous configuration enable register

You can [`read`](crate::Reg::read) this register and get [`miscenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCENR)

For information about available fields see [`mod@miscenr`] module*/
pub type MISCENR = crate::Reg<miscenr::MISCENRrs>;
///RCC miscellaneous configuration enable register
pub mod miscenr;
/**MEMENR (rw) register accessor: RCC memory enable register

You can [`read`](crate::Reg::read) this register and get [`memenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMENR)

For information about available fields see [`mod@memenr`] module*/
pub type MEMENR = crate::Reg<memenr::MEMENRrs>;
///RCC memory enable register
pub mod memenr;
/**AHB1ENR (rw) register accessor: RCC AHB1 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1ENR)

For information about available fields see [`mod@ahb1enr`] module*/
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENRrs>;
///RCC AHB1 enable register
pub mod ahb1enr;
/**AHB2ENR (rw) register accessor: RCC AHB2 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2ENR)

For information about available fields see [`mod@ahb2enr`] module*/
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENRrs>;
///RCC AHB2 enable register
pub mod ahb2enr;
/**AHB3ENR (rw) register accessor: RCC AHB3 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3ENR)

For information about available fields see [`mod@ahb3enr`] module*/
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENRrs>;
///RCC AHB3 enable register
pub mod ahb3enr;
/**AHB4ENR (rw) register accessor: RCC AHB4 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4ENR)

For information about available fields see [`mod@ahb4enr`] module*/
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENRrs>;
///RCC AHB4 enable register
pub mod ahb4enr;
/**AHB5ENR (rw) register accessor: RCC AHB5 enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5ENR)

For information about available fields see [`mod@ahb5enr`] module*/
pub type AHB5ENR = crate::Reg<ahb5enr::AHB5ENRrs>;
///RCC AHB5 enable register
pub mod ahb5enr;
/**APB1LENR (rw) register accessor: RCC APB1L enable register

You can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LENR)

For information about available fields see [`mod@apb1lenr`] module*/
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENRrs>;
///RCC APB1L enable register
pub mod apb1lenr;
/**APB1HENR (rw) register accessor: RCC APB1H enable register

You can [`read`](crate::Reg::read) this register and get [`apb1henr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HENR)

For information about available fields see [`mod@apb1henr`] module*/
pub type APB1HENR = crate::Reg<apb1henr::APB1HENRrs>;
///RCC APB1H enable register
pub mod apb1henr;
/**APB2ENR (rw) register accessor: RCC APB2 enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///RCC APB2 enable register
pub mod apb2enr;
/**APB3ENR (rw) register accessor: RCC APB3 enable register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB3ENR)

For information about available fields see [`mod@apb3enr`] module*/
pub type APB3ENR = crate::Reg<apb3enr::APB3ENRrs>;
///RCC APB3 enable register
pub mod apb3enr;
/**APB4LENR (rw) register accessor: RCC APB4L enable register

You can [`read`](crate::Reg::read) this register and get [`apb4lenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LENR)

For information about available fields see [`mod@apb4lenr`] module*/
pub type APB4LENR = crate::Reg<apb4lenr::APB4LENRrs>;
///RCC APB4L enable register
pub mod apb4lenr;
/**APB4HENR (rw) register accessor: RCC APB4H enable register

You can [`read`](crate::Reg::read) this register and get [`apb4henr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4henr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HENR)

For information about available fields see [`mod@apb4henr`] module*/
pub type APB4HENR = crate::Reg<apb4henr::APB4HENRrs>;
///RCC APB4H enable register
pub mod apb4henr;
/**APB5ENR (rw) register accessor: RCC APB5 enable register

You can [`read`](crate::Reg::read) this register and get [`apb5enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5ENR)

For information about available fields see [`mod@apb5enr`] module*/
pub type APB5ENR = crate::Reg<apb5enr::APB5ENRrs>;
///RCC APB5 enable register
pub mod apb5enr;
/**DIVLPENR (rw) register accessor: RCC dividers Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`divlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:DIVLPENR)

For information about available fields see [`mod@divlpenr`] module*/
pub type DIVLPENR = crate::Reg<divlpenr::DIVLPENRrs>;
///RCC dividers Sleep enable register
pub mod divlpenr;
/**BUSLPENR (rw) register accessor: RCC SoC buses Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`buslpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buslpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSLPENR)

For information about available fields see [`mod@buslpenr`] module*/
pub type BUSLPENR = crate::Reg<buslpenr::BUSLPENRrs>;
///RCC SoC buses Sleep enable register
pub mod buslpenr;
/**MISCLPENR (rw) register accessor: RCC miscellaneous configurations Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`misclpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCLPENR)

For information about available fields see [`mod@misclpenr`] module*/
pub type MISCLPENR = crate::Reg<misclpenr::MISCLPENRrs>;
///RCC miscellaneous configurations Sleep enable register
pub mod misclpenr;
/**MEMLPENR (rw) register accessor: RCC memory Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`memlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMLPENR)

For information about available fields see [`mod@memlpenr`] module*/
pub type MEMLPENR = crate::Reg<memlpenr::MEMLPENRrs>;
///RCC memory Sleep enable register
pub mod memlpenr;
/**AHB1LPENR (rw) register accessor: RCC AHB1 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1LPENR)

For information about available fields see [`mod@ahb1lpenr`] module*/
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENRrs>;
///RCC AHB1 Sleep enable register
pub mod ahb1lpenr;
/**AHB2LPENR (rw) register accessor: RCC AHB2 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2LPENR)

For information about available fields see [`mod@ahb2lpenr`] module*/
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENRrs>;
///RCC AHB2 Sleep enable register
pub mod ahb2lpenr;
/**AHB3LPENR (rw) register accessor: RCC AHB3 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3LPENR)

For information about available fields see [`mod@ahb3lpenr`] module*/
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENRrs>;
///RCC AHB3 Sleep enable register
pub mod ahb3lpenr;
/**AHB4LPENR (rw) register accessor: RCC AHB4 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4LPENR)

For information about available fields see [`mod@ahb4lpenr`] module*/
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENRrs>;
///RCC AHB4 Sleep enable register
pub mod ahb4lpenr;
/**AHB5LPENR (rw) register accessor: RCC AHB5 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb5lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5LPENR)

For information about available fields see [`mod@ahb5lpenr`] module*/
pub type AHB5LPENR = crate::Reg<ahb5lpenr::AHB5LPENRrs>;
///RCC AHB5 Sleep enable register
pub mod ahb5lpenr;
/**APB1LLPENR (rw) register accessor: RCC APB1L Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LLPENR)

For information about available fields see [`mod@apb1llpenr`] module*/
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENRrs>;
///RCC APB1L Sleep enable register
pub mod apb1llpenr;
/**APB1HLPENR (rw) register accessor: RCC APB1H Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HLPENR)

For information about available fields see [`mod@apb1hlpenr`] module*/
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENRrs>;
///RCC APB1H Sleep enable register
pub mod apb1hlpenr;
/**APB2LPENR (rw) register accessor: RCC APB2 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2LPENR)

For information about available fields see [`mod@apb2lpenr`] module*/
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENRrs>;
///RCC APB2 Sleep enable register
pub mod apb2lpenr;
/**APB3LPENR (rw) register accessor: RCC APB3 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB3LPENR)

For information about available fields see [`mod@apb3lpenr`] module*/
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENRrs>;
///RCC APB3 Sleep enable register
pub mod apb3lpenr;
/**APB4LLPENR (rw) register accessor: RCC APB4L Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb4llpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4llpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LLPENR)

For information about available fields see [`mod@apb4llpenr`] module*/
pub type APB4LLPENR = crate::Reg<apb4llpenr::APB4LLPENRrs>;
///RCC APB4L Sleep enable register
pub mod apb4llpenr;
/**APB4HLPENR (rw) register accessor: RCC APB4H Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb4hlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HLPENR)

For information about available fields see [`mod@apb4hlpenr`] module*/
pub type APB4HLPENR = crate::Reg<apb4hlpenr::APB4HLPENRrs>;
///RCC APB4H Sleep enable register
pub mod apb4hlpenr;
/**APB5LPENR (rw) register accessor: RCC APB5 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb5lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5LPENR)

For information about available fields see [`mod@apb5lpenr`] module*/
pub type APB5LPENR = crate::Reg<apb5lpenr::APB5LPENRrs>;
///RCC APB5 Sleep enable register
pub mod apb5lpenr;
/**RDCR (rw) register accessor: RCC APB5 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`rdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:RDCR)

For information about available fields see [`mod@rdcr`] module*/
pub type RDCR = crate::Reg<rdcr::RDCRrs>;
///RCC APB5 Sleep enable register
pub mod rdcr;
/**SECCFGR0 (rw) register accessor: RCC oscillator secure configuration register0

You can [`read`](crate::Reg::read) this register and get [`seccfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGR0)

For information about available fields see [`mod@seccfgr0`] module*/
pub type SECCFGR0 = crate::Reg<seccfgr0::SECCFGR0rs>;
///RCC oscillator secure configuration register0
pub mod seccfgr0;
/**PRIVCFGR0 (rw) register accessor: RCC oscillator privilege configuration register0

You can [`read`](crate::Reg::read) this register and get [`privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGR0)

For information about available fields see [`mod@privcfgr0`] module*/
pub type PRIVCFGR0 = crate::Reg<privcfgr0::PRIVCFGR0rs>;
///RCC oscillator privilege configuration register0
pub mod privcfgr0;
/**LOCKCFGR0 (w) register accessor: RCC oscillator lock configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGR0)

For information about available fields see [`mod@lockcfgr0`] module*/
pub type LOCKCFGR0 = crate::Reg<lockcfgr0::LOCKCFGR0rs>;
///RCC oscillator lock configuration register0
pub mod lockcfgr0;
/**PUBCFGR0 (rw) register accessor: RCC oscillator public configuration register0

You can [`read`](crate::Reg::read) this register and get [`pubcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGR0)

For information about available fields see [`mod@pubcfgr0`] module*/
pub type PUBCFGR0 = crate::Reg<pubcfgr0::PUBCFGR0rs>;
///RCC oscillator public configuration register0
pub mod pubcfgr0;
/**SECCFGR1 (rw) register accessor: RCC PLL secure configuration register1

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGR1)

For information about available fields see [`mod@seccfgr1`] module*/
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1rs>;
///RCC PLL secure configuration register1
pub mod seccfgr1;
/**PRIVCFGR1 (rw) register accessor: RCC PLL privilege configuration register1

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGR1)

For information about available fields see [`mod@privcfgr1`] module*/
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1rs>;
///RCC PLL privilege configuration register1
pub mod privcfgr1;
/**LOCKCFGR1 (w) register accessor: RCC PLL lock configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGR1)

For information about available fields see [`mod@lockcfgr1`] module*/
pub type LOCKCFGR1 = crate::Reg<lockcfgr1::LOCKCFGR1rs>;
///RCC PLL lock configuration register1
pub mod lockcfgr1;
/**PUBCFGR1 (rw) register accessor: RCC PLL public configuration register1

You can [`read`](crate::Reg::read) this register and get [`pubcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGR1)

For information about available fields see [`mod@pubcfgr1`] module*/
pub type PUBCFGR1 = crate::Reg<pubcfgr1::PUBCFGR1rs>;
///RCC PLL public configuration register1
pub mod pubcfgr1;
/**SECCFGR2 (rw) register accessor: RCC divider secure configuration register2

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGR2)

For information about available fields see [`mod@seccfgr2`] module*/
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2rs>;
///RCC divider secure configuration register2
pub mod seccfgr2;
/**PRIVCFGR2 (rw) register accessor: RCC divider privilege configuration register2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGR2)

For information about available fields see [`mod@privcfgr2`] module*/
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2rs>;
///RCC divider privilege configuration register2
pub mod privcfgr2;
/**LOCKCFGR2 (w) register accessor: RCC divider lock configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGR2)

For information about available fields see [`mod@lockcfgr2`] module*/
pub type LOCKCFGR2 = crate::Reg<lockcfgr2::LOCKCFGR2rs>;
///RCC divider lock configuration register2
pub mod lockcfgr2;
/**PUBCFGR2 (rw) register accessor: RCC divider public configuration register2

You can [`read`](crate::Reg::read) this register and get [`pubcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGR2)

For information about available fields see [`mod@pubcfgr2`] module*/
pub type PUBCFGR2 = crate::Reg<pubcfgr2::PUBCFGR2rs>;
///RCC divider public configuration register2
pub mod pubcfgr2;
/**SECCFGR3 (rw) register accessor: RCC system secure configuration register3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGR3)

For information about available fields see [`mod@seccfgr3`] module*/
pub type SECCFGR3 = crate::Reg<seccfgr3::SECCFGR3rs>;
///RCC system secure configuration register3
pub mod seccfgr3;
/**PRIVCFGR3 (rw) register accessor: RCC system privilege configuration register3

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGR3)

For information about available fields see [`mod@privcfgr3`] module*/
pub type PRIVCFGR3 = crate::Reg<privcfgr3::PRIVCFGR3rs>;
///RCC system privilege configuration register3
pub mod privcfgr3;
/**LOCKCFGR3 (w) register accessor: RCC system lock configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGR3)

For information about available fields see [`mod@lockcfgr3`] module*/
pub type LOCKCFGR3 = crate::Reg<lockcfgr3::LOCKCFGR3rs>;
///RCC system lock configuration register3
pub mod lockcfgr3;
/**PUBCFGR3 (rw) register accessor: RCC system public configuration register3

You can [`read`](crate::Reg::read) this register and get [`pubcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGR3)

For information about available fields see [`mod@pubcfgr3`] module*/
pub type PUBCFGR3 = crate::Reg<pubcfgr3::PUBCFGR3rs>;
///RCC system public configuration register3
pub mod pubcfgr3;
/**SECCFGR4 (rw) register accessor: RCC bus secure configuration register4

You can [`read`](crate::Reg::read) this register and get [`seccfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGR4)

For information about available fields see [`mod@seccfgr4`] module*/
pub type SECCFGR4 = crate::Reg<seccfgr4::SECCFGR4rs>;
///RCC bus secure configuration register4
pub mod seccfgr4;
/**PRIVCFGR4 (rw) register accessor: RCC bus privilege configuration register4

You can [`read`](crate::Reg::read) this register and get [`privcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGR4)

For information about available fields see [`mod@privcfgr4`] module*/
pub type PRIVCFGR4 = crate::Reg<privcfgr4::PRIVCFGR4rs>;
///RCC bus privilege configuration register4
pub mod privcfgr4;
/**LOCKCFGR4 (w) register accessor: RCC bus lock configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGR4)

For information about available fields see [`mod@lockcfgr4`] module*/
pub type LOCKCFGR4 = crate::Reg<lockcfgr4::LOCKCFGR4rs>;
///RCC bus lock configuration register4
pub mod lockcfgr4;
/**PUBCFGR4 (rw) register accessor: RCC bus public configuration register4

You can [`read`](crate::Reg::read) this register and get [`pubcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGR4)

For information about available fields see [`mod@pubcfgr4`] module*/
pub type PUBCFGR4 = crate::Reg<pubcfgr4::PUBCFGR4rs>;
///RCC bus public configuration register4
pub mod pubcfgr4;
/**PUBCFGR5 (rw) register accessor: RCC bus public configuration register4

You can [`read`](crate::Reg::read) this register and get [`pubcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGR5)

For information about available fields see [`mod@pubcfgr5`] module*/
pub type PUBCFGR5 = crate::Reg<pubcfgr5::PUBCFGR5rs>;
///RCC bus public configuration register4
pub mod pubcfgr5;
/**CSR (w) register accessor: RCC control set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///RCC control set register
pub mod csr;
/**STOPCSR (w) register accessor: RCC Stop configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopcsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:STOPCSR)

For information about available fields see [`mod@stopcsr`] module*/
pub type STOPCSR = crate::Reg<stopcsr::STOPCSRrs>;
///RCC Stop configuration register
pub mod stopcsr;
/**BUSRSTSR (w) register accessor: RCC bus reset set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSRSTSR)

For information about available fields see [`mod@busrstsr`] module*/
pub type BUSRSTSR = crate::Reg<busrstsr::BUSRSTSRrs>;
///RCC bus reset set register
pub mod busrstsr;
/**MISCRSTSR (w) register accessor: RCC miscellaneous reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscrstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCRSTSR)

For information about available fields see [`mod@miscrstsr`] module*/
pub type MISCRSTSR = crate::Reg<miscrstsr::MISCRSTSRrs>;
///RCC miscellaneous reset register
pub mod miscrstsr;
/**MEMRSTSR (w) register accessor: RCC memory reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMRSTSR)

For information about available fields see [`mod@memrstsr`] module*/
pub type MEMRSTSR = crate::Reg<memrstsr::MEMRSTSRrs>;
///RCC memory reset register
pub mod memrstsr;
/**AHB1RSTSR (w) register accessor: RCC AHB1 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1RSTSR)

For information about available fields see [`mod@ahb1rstsr`] module*/
pub type AHB1RSTSR = crate::Reg<ahb1rstsr::AHB1RSTSRrs>;
///RCC AHB1 reset register
pub mod ahb1rstsr;
/**AHB2RSTSR (w) register accessor: RCC AHB2 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2RSTSR)

For information about available fields see [`mod@ahb2rstsr`] module*/
pub type AHB2RSTSR = crate::Reg<ahb2rstsr::AHB2RSTSRrs>;
///RCC AHB2 reset register
pub mod ahb2rstsr;
/**AHB3RSTSR (w) register accessor: RCC AHB3 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3RSTSR)

For information about available fields see [`mod@ahb3rstsr`] module*/
pub type AHB3RSTSR = crate::Reg<ahb3rstsr::AHB3RSTSRrs>;
///RCC AHB3 reset register
pub mod ahb3rstsr;
/**AHB4RSTSR (w) register accessor: RCC AHB4 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4RSTSR)

For information about available fields see [`mod@ahb4rstsr`] module*/
pub type AHB4RSTSR = crate::Reg<ahb4rstsr::AHB4RSTSRrs>;
///RCC AHB4 reset register
pub mod ahb4rstsr;
/**AHB5RSTSR (w) register accessor: RCC AHB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5RSTSR)

For information about available fields see [`mod@ahb5rstsr`] module*/
pub type AHB5RSTSR = crate::Reg<ahb5rstsr::AHB5RSTSRrs>;
///RCC AHB5 reset register
pub mod ahb5rstsr;
/**APB1LRSTSR (w) register accessor: RCC APB1L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LRSTSR)

For information about available fields see [`mod@apb1lrstsr`] module*/
pub type APB1LRSTSR = crate::Reg<apb1lrstsr::APB1LRSTSRrs>;
///RCC APB1L reset register
pub mod apb1lrstsr;
/**APB1HRSTSR (w) register accessor: RCC APB1H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HRSTSR)

For information about available fields see [`mod@apb1hrstsr`] module*/
pub type APB1HRSTSR = crate::Reg<apb1hrstsr::APB1HRSTSRrs>;
///RCC APB1H reset register
pub mod apb1hrstsr;
/**APB2RSTSR (w) register accessor: RCC APB2 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2RSTSR)

For information about available fields see [`mod@apb2rstsr`] module*/
pub type APB2RSTSR = crate::Reg<apb2rstsr::APB2RSTSRrs>;
///RCC APB2 reset register
pub mod apb2rstsr;
/**APB4LRSTSR (w) register accessor: RCC APB4L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lrstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LRSTSR)

For information about available fields see [`mod@apb4lrstsr`] module*/
pub type APB4LRSTSR = crate::Reg<apb4lrstsr::APB4LRSTSRrs>;
///RCC APB4L reset register
pub mod apb4lrstsr;
/**APB4HRSTSR (w) register accessor: RCC APB4H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hrstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HRSTSR)

For information about available fields see [`mod@apb4hrstsr`] module*/
pub type APB4HRSTSR = crate::Reg<apb4hrstsr::APB4HRSTSRrs>;
///RCC APB4H reset register
pub mod apb4hrstsr;
/**APB5RSTSR (w) register accessor: RCC APB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5RSTSR)

For information about available fields see [`mod@apb5rstsr`] module*/
pub type APB5RSTSR = crate::Reg<apb5rstsr::APB5RSTSRrs>;
///RCC APB5 reset register
pub mod apb5rstsr;
/**DIVENSR (w) register accessor: RCC Divider enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:DIVENSR)

For information about available fields see [`mod@divensr`] module*/
pub type DIVENSR = crate::Reg<divensr::DIVENSRrs>;
///RCC Divider enable register
pub mod divensr;
/**BUSENSR (w) register accessor: RCC bus enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSENSR)

For information about available fields see [`mod@busensr`] module*/
pub type BUSENSR = crate::Reg<busensr::BUSENSRrs>;
///RCC bus enable register
pub mod busensr;
/**MISCENSR (w) register accessor: RCC miscellaneous enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCENSR)

For information about available fields see [`mod@miscensr`] module*/
pub type MISCENSR = crate::Reg<miscensr::MISCENSRrs>;
///RCC miscellaneous enable register
pub mod miscensr;
/**MEMENSR (w) register accessor: RCC memory enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMENSR)

For information about available fields see [`mod@memensr`] module*/
pub type MEMENSR = crate::Reg<memensr::MEMENSRrs>;
///RCC memory enable register
pub mod memensr;
/**AHB1ENSR (w) register accessor: RCC AHB1 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1ENSR)

For information about available fields see [`mod@ahb1ensr`] module*/
pub type AHB1ENSR = crate::Reg<ahb1ensr::AHB1ENSRrs>;
///RCC AHB1 enable register
pub mod ahb1ensr;
/**AHB2ENSR (w) register accessor: RCC AHB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2ENSR)

For information about available fields see [`mod@ahb2ensr`] module*/
pub type AHB2ENSR = crate::Reg<ahb2ensr::AHB2ENSRrs>;
///RCC AHB2 enable register
pub mod ahb2ensr;
/**AHB3ENSR (w) register accessor: RCC AHB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3ENSR)

For information about available fields see [`mod@ahb3ensr`] module*/
pub type AHB3ENSR = crate::Reg<ahb3ensr::AHB3ENSRrs>;
///RCC AHB3 enable register
pub mod ahb3ensr;
/**AHB4ENSR (w) register accessor: RCC AHB4 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4ENSR)

For information about available fields see [`mod@ahb4ensr`] module*/
pub type AHB4ENSR = crate::Reg<ahb4ensr::AHB4ENSRrs>;
///RCC AHB4 enable register
pub mod ahb4ensr;
/**AHB5ENSR (w) register accessor: RCC AHB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5ENSR)

For information about available fields see [`mod@ahb5ensr`] module*/
pub type AHB5ENSR = crate::Reg<ahb5ensr::AHB5ENSRrs>;
///RCC AHB5 enable register
pub mod ahb5ensr;
/**APB1LENSR (w) register accessor: RCC APB1L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LENSR)

For information about available fields see [`mod@apb1lensr`] module*/
pub type APB1LENSR = crate::Reg<apb1lensr::APB1LENSRrs>;
///RCC APB1L enable register
pub mod apb1lensr;
/**APB1HENSR (w) register accessor: RCC APB1H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HENSR)

For information about available fields see [`mod@apb1hensr`] module*/
pub type APB1HENSR = crate::Reg<apb1hensr::APB1HENSRrs>;
///RCC APB1H enable register
pub mod apb1hensr;
/**APB2ENSR (w) register accessor: RCC APB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2ENSR)

For information about available fields see [`mod@apb2ensr`] module*/
pub type APB2ENSR = crate::Reg<apb2ensr::APB2ENSRrs>;
///RCC APB2 enable register
pub mod apb2ensr;
/**APB3ENSR (w) register accessor: RCC APB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB3ENSR)

For information about available fields see [`mod@apb3ensr`] module*/
pub type APB3ENSR = crate::Reg<apb3ensr::APB3ENSRrs>;
///RCC APB3 enable register
pub mod apb3ensr;
/**APB4LENSR (w) register accessor: RCC APB4L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LENSR)

For information about available fields see [`mod@apb4lensr`] module*/
pub type APB4LENSR = crate::Reg<apb4lensr::APB4LENSRrs>;
///RCC APB4L enable register
pub mod apb4lensr;
/**APB4HENSR (w) register accessor: RCC APB4H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HENSR)

For information about available fields see [`mod@apb4hensr`] module*/
pub type APB4HENSR = crate::Reg<apb4hensr::APB4HENSRrs>;
///RCC APB4H enable register
pub mod apb4hensr;
/**APB5ENSR (w) register accessor: RCC APB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5ensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5ENSR)

For information about available fields see [`mod@apb5ensr`] module*/
pub type APB5ENSR = crate::Reg<apb5ensr::APB5ENSRrs>;
///RCC APB5 enable register
pub mod apb5ensr;
/**DIVLPENSR (w) register accessor: RCC divider Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divlpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:DIVLPENSR)

For information about available fields see [`mod@divlpensr`] module*/
pub type DIVLPENSR = crate::Reg<divlpensr::DIVLPENSRrs>;
///RCC divider Sleep enable register
pub mod divlpensr;
/**BUSLPENSR (w) register accessor: RCC bus Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buslpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSLPENSR)

For information about available fields see [`mod@buslpensr`] module*/
pub type BUSLPENSR = crate::Reg<buslpensr::BUSLPENSRrs>;
///RCC bus Sleep enable register
pub mod buslpensr;
/**MISCLPENSR (w) register accessor: RCC miscellaneous Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCLPENSR)

For information about available fields see [`mod@misclpensr`] module*/
pub type MISCLPENSR = crate::Reg<misclpensr::MISCLPENSRrs>;
///RCC miscellaneous Sleep enable register
pub mod misclpensr;
/**MEMLPENSR (w) register accessor: RCC memory sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memlpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMLPENSR)

For information about available fields see [`mod@memlpensr`] module*/
pub type MEMLPENSR = crate::Reg<memlpensr::MEMLPENSRrs>;
///RCC memory sleep enable register
pub mod memlpensr;
/**AHB1LPENSR (w) register accessor: RCC AHB1 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1LPENSR)

For information about available fields see [`mod@ahb1lpensr`] module*/
pub type AHB1LPENSR = crate::Reg<ahb1lpensr::AHB1LPENSRrs>;
///RCC AHB1 Sleep enable register
pub mod ahb1lpensr;
/**AHB2LPENSR (w) register accessor: RCC AHB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2LPENSR)

For information about available fields see [`mod@ahb2lpensr`] module*/
pub type AHB2LPENSR = crate::Reg<ahb2lpensr::AHB2LPENSRrs>;
///RCC AHB2 Sleep enable register
pub mod ahb2lpensr;
/**AHB3LPENSR (w) register accessor: RCC AHB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3LPENSR)

For information about available fields see [`mod@ahb3lpensr`] module*/
pub type AHB3LPENSR = crate::Reg<ahb3lpensr::AHB3LPENSRrs>;
///RCC AHB3 Sleep enable register
pub mod ahb3lpensr;
/**AHB4LPENSR (w) register accessor: RCC AHB4 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4LPENSR)

For information about available fields see [`mod@ahb4lpensr`] module*/
pub type AHB4LPENSR = crate::Reg<ahb4lpensr::AHB4LPENSRrs>;
///RCC AHB4 Sleep enable register
pub mod ahb4lpensr;
/**AHB5LPENSR (w) register accessor: RCC AHB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5LPENSR)

For information about available fields see [`mod@ahb5lpensr`] module*/
pub type AHB5LPENSR = crate::Reg<ahb5lpensr::AHB5LPENSRrs>;
///RCC AHB5 Sleep enable register
pub mod ahb5lpensr;
/**APB1LLPENSR (w) register accessor: RCC APB1L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LLPENSR)

For information about available fields see [`mod@apb1llpensr`] module*/
pub type APB1LLPENSR = crate::Reg<apb1llpensr::APB1LLPENSRrs>;
///RCC APB1L Sleep enable register
pub mod apb1llpensr;
/**APB1HLPENSR (w) register accessor: RCC APB1H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HLPENSR)

For information about available fields see [`mod@apb1hlpensr`] module*/
pub type APB1HLPENSR = crate::Reg<apb1hlpensr::APB1HLPENSRrs>;
///RCC APB1H Sleep enable register
pub mod apb1hlpensr;
/**APB2LPENSR (w) register accessor: RCC APB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2LPENSR)

For information about available fields see [`mod@apb2lpensr`] module*/
pub type APB2LPENSR = crate::Reg<apb2lpensr::APB2LPENSRrs>;
///RCC APB2 Sleep enable register
pub mod apb2lpensr;
/**APB3LPENSR (w) register accessor: RCC APB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB3LPENSR)

For information about available fields see [`mod@apb3lpensr`] module*/
pub type APB3LPENSR = crate::Reg<apb3lpensr::APB3LPENSRrs>;
///RCC APB3 Sleep enable register
pub mod apb3lpensr;
/**APB4LLPENSR (w) register accessor: RCC APB4L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4llpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LLPENSR)

For information about available fields see [`mod@apb4llpensr`] module*/
pub type APB4LLPENSR = crate::Reg<apb4llpensr::APB4LLPENSRrs>;
///RCC APB4L Sleep enable register
pub mod apb4llpensr;
/**APB4HLPENSR (w) register accessor: RCC APB4H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hlpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HLPENSR)

For information about available fields see [`mod@apb4hlpensr`] module*/
pub type APB4HLPENSR = crate::Reg<apb4hlpensr::APB4HLPENSRrs>;
///RCC APB4H Sleep enable register
pub mod apb4hlpensr;
/**APB5LPENSR (w) register accessor: RCC APB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpensr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5LPENSR)

For information about available fields see [`mod@apb5lpensr`] module*/
pub type APB5LPENSR = crate::Reg<apb5lpensr::APB5LPENSRrs>;
///RCC APB5 Sleep enable register
pub mod apb5lpensr;
/**PRIVCFGSR0 (w) register accessor: RCC oscillator privilege configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGSR0)

For information about available fields see [`mod@privcfgsr0`] module*/
pub type PRIVCFGSR0 = crate::Reg<privcfgsr0::PRIVCFGSR0rs>;
///RCC oscillator privilege configuration register0
pub mod privcfgsr0;
/**PUBCFGSR0 (w) register accessor: RCC oscillator public configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGSR0)

For information about available fields see [`mod@pubcfgsr0`] module*/
pub type PUBCFGSR0 = crate::Reg<pubcfgsr0::PUBCFGSR0rs>;
///RCC oscillator public configuration register0
pub mod pubcfgsr0;
/**PRIVCFGSR1 (w) register accessor: RCC PLL privilege configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGSR1)

For information about available fields see [`mod@privcfgsr1`] module*/
pub type PRIVCFGSR1 = crate::Reg<privcfgsr1::PRIVCFGSR1rs>;
///RCC PLL privilege configuration register1
pub mod privcfgsr1;
/**PUBCFGSR1 (w) register accessor: RCC PLL public configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGSR1)

For information about available fields see [`mod@pubcfgsr1`] module*/
pub type PUBCFGSR1 = crate::Reg<pubcfgsr1::PUBCFGSR1rs>;
///RCC PLL public configuration register1
pub mod pubcfgsr1;
/**PRIVCFGSR2 (w) register accessor: RCC divider privilege configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGSR2)

For information about available fields see [`mod@privcfgsr2`] module*/
pub type PRIVCFGSR2 = crate::Reg<privcfgsr2::PRIVCFGSR2rs>;
///RCC divider privilege configuration register2
pub mod privcfgsr2;
/**PUBCFGSR2 (w) register accessor: RCC divider public configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGSR2)

For information about available fields see [`mod@pubcfgsr2`] module*/
pub type PUBCFGSR2 = crate::Reg<pubcfgsr2::PUBCFGSR2rs>;
///RCC divider public configuration register2
pub mod pubcfgsr2;
/**SECCFGSR3 (w) register accessor: RCC system secure configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgsr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGSR3)

For information about available fields see [`mod@seccfgsr3`] module*/
pub type SECCFGSR3 = crate::Reg<seccfgsr3::SECCFGSR3rs>;
///RCC system secure configuration register3
pub mod seccfgsr3;
/**PRIVCFGSR3 (w) register accessor: RCC system privilege configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGSR3)

For information about available fields see [`mod@privcfgsr3`] module*/
pub type PRIVCFGSR3 = crate::Reg<privcfgsr3::PRIVCFGSR3rs>;
///RCC system privilege configuration register3
pub mod privcfgsr3;
/**LOCKCFGSR3 (w) register accessor: RCC system lock configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgsr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGSR3)

For information about available fields see [`mod@lockcfgsr3`] module*/
pub type LOCKCFGSR3 = crate::Reg<lockcfgsr3::LOCKCFGSR3rs>;
///RCC system lock configuration register3
pub mod lockcfgsr3;
/**PUBCFGSR3 (w) register accessor: RCC system public configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGSR3)

For information about available fields see [`mod@pubcfgsr3`] module*/
pub type PUBCFGSR3 = crate::Reg<pubcfgsr3::PUBCFGSR3rs>;
///RCC system public configuration register3
pub mod pubcfgsr3;
/**PRIVCFGSR4 (w) register accessor: RCC privilege configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGSR4)

For information about available fields see [`mod@privcfgsr4`] module*/
pub type PRIVCFGSR4 = crate::Reg<privcfgsr4::PRIVCFGSR4rs>;
///RCC privilege configuration register4
pub mod privcfgsr4;
/**PUBCFGSR4 (w) register accessor: RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGSR4)

For information about available fields see [`mod@pubcfgsr4`] module*/
pub type PUBCFGSR4 = crate::Reg<pubcfgsr4::PUBCFGSR4rs>;
///RCC public configuration register4
pub mod pubcfgsr4;
/**PUBCFGSR5 (w) register accessor: RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGSR5)

For information about available fields see [`mod@pubcfgsr5`] module*/
pub type PUBCFGSR5 = crate::Reg<pubcfgsr5::PUBCFGSR5rs>;
///RCC public configuration register4
pub mod pubcfgsr5;
/**CCR (w) register accessor: RCC control Clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///RCC control Clear register
pub mod ccr;
/**STOPCCR (w) register accessor: RCC StopCCR configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:STOPCCR)

For information about available fields see [`mod@stopccr`] module*/
pub type STOPCCR = crate::Reg<stopccr::STOPCCRrs>;
///RCC StopCCR configuration register
pub mod stopccr;
/**BUSRSTCR (w) register accessor: RCC bus reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSRSTCR)

For information about available fields see [`mod@busrstcr`] module*/
pub type BUSRSTCR = crate::Reg<busrstcr::BUSRSTCRrs>;
///RCC bus reset register
pub mod busrstcr;
/**MISCRSTCR (w) register accessor: RCC miscellaneous reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscrstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCRSTCR)

For information about available fields see [`mod@miscrstcr`] module*/
pub type MISCRSTCR = crate::Reg<miscrstcr::MISCRSTCRrs>;
///RCC miscellaneous reset register
pub mod miscrstcr;
/**MEMRSTCR (w) register accessor: RCC memory reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMRSTCR)

For information about available fields see [`mod@memrstcr`] module*/
pub type MEMRSTCR = crate::Reg<memrstcr::MEMRSTCRrs>;
///RCC memory reset register
pub mod memrstcr;
/**AHB1RSTCR (w) register accessor: RCC AHB1 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1RSTCR)

For information about available fields see [`mod@ahb1rstcr`] module*/
pub type AHB1RSTCR = crate::Reg<ahb1rstcr::AHB1RSTCRrs>;
///RCC AHB1 reset register
pub mod ahb1rstcr;
/**AHB2RSTCR (w) register accessor: RCC AHB2 Reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2RSTCR)

For information about available fields see [`mod@ahb2rstcr`] module*/
pub type AHB2RSTCR = crate::Reg<ahb2rstcr::AHB2RSTCRrs>;
///RCC AHB2 Reset register
pub mod ahb2rstcr;
/**AHB3RSTCR (w) register accessor: RCC AHB3 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3RSTCR)

For information about available fields see [`mod@ahb3rstcr`] module*/
pub type AHB3RSTCR = crate::Reg<ahb3rstcr::AHB3RSTCRrs>;
///RCC AHB3 reset register
pub mod ahb3rstcr;
/**AHB4RSTCR (w) register accessor: RCC AHB4 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4RSTCR)

For information about available fields see [`mod@ahb4rstcr`] module*/
pub type AHB4RSTCR = crate::Reg<ahb4rstcr::AHB4RSTCRrs>;
///RCC AHB4 reset register
pub mod ahb4rstcr;
/**AHB5RSTCR (w) register accessor: RCC AHB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5RSTCR)

For information about available fields see [`mod@ahb5rstcr`] module*/
pub type AHB5RSTCR = crate::Reg<ahb5rstcr::AHB5RSTCRrs>;
///RCC AHB5 reset register
pub mod ahb5rstcr;
/**APB1LRSTCR (w) register accessor: RCC APB1L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LRSTCR)

For information about available fields see [`mod@apb1lrstcr`] module*/
pub type APB1LRSTCR = crate::Reg<apb1lrstcr::APB1LRSTCRrs>;
///RCC APB1L reset register
pub mod apb1lrstcr;
/**APB1HRSTCR (w) register accessor: RCC APB1H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HRSTCR)

For information about available fields see [`mod@apb1hrstcr`] module*/
pub type APB1HRSTCR = crate::Reg<apb1hrstcr::APB1HRSTCRrs>;
///RCC APB1H reset register
pub mod apb1hrstcr;
/**APB2RSTCR (w) register accessor: RCC APB2 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2RSTCR)

For information about available fields see [`mod@apb2rstcr`] module*/
pub type APB2RSTCR = crate::Reg<apb2rstcr::APB2RSTCRrs>;
///RCC APB2 reset register
pub mod apb2rstcr;
/**APB4LRSTCR (w) register accessor: RCC APB4L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lrstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LRSTCR)

For information about available fields see [`mod@apb4lrstcr`] module*/
pub type APB4LRSTCR = crate::Reg<apb4lrstcr::APB4LRSTCRrs>;
///RCC APB4L reset register
pub mod apb4lrstcr;
/**APB4HRSTCR (w) register accessor: RCC APB4H reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hrstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HRSTCR)

For information about available fields see [`mod@apb4hrstcr`] module*/
pub type APB4HRSTCR = crate::Reg<apb4hrstcr::APB4HRSTCRrs>;
///RCC APB4H reset register
pub mod apb4hrstcr;
/**APB5RSTCR (w) register accessor: RCC APB5 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5RSTCR)

For information about available fields see [`mod@apb5rstcr`] module*/
pub type APB5RSTCR = crate::Reg<apb5rstcr::APB5RSTCRrs>;
///RCC APB5 reset register
pub mod apb5rstcr;
/**DIVENCR (w) register accessor: RCC divider enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:DIVENCR)

For information about available fields see [`mod@divencr`] module*/
pub type DIVENCR = crate::Reg<divencr::DIVENCRrs>;
///RCC divider enable register
pub mod divencr;
/**BUSENCR (w) register accessor: RCC bus enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSENCR)

For information about available fields see [`mod@busencr`] module*/
pub type BUSENCR = crate::Reg<busencr::BUSENCRrs>;
///RCC bus enable register
pub mod busencr;
/**MISCENCR (w) register accessor: RCC miscellaneous enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCENCR)

For information about available fields see [`mod@miscencr`] module*/
pub type MISCENCR = crate::Reg<miscencr::MISCENCRrs>;
///RCC miscellaneous enable register
pub mod miscencr;
/**MEMENCR (w) register accessor: RCC memory enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMENCR)

For information about available fields see [`mod@memencr`] module*/
pub type MEMENCR = crate::Reg<memencr::MEMENCRrs>;
///RCC memory enable register
pub mod memencr;
/**AHB1ENCR (w) register accessor: RCC AHB1 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1ENCR)

For information about available fields see [`mod@ahb1encr`] module*/
pub type AHB1ENCR = crate::Reg<ahb1encr::AHB1ENCRrs>;
///RCC AHB1 enable register
pub mod ahb1encr;
/**AHB2ENCR (w) register accessor: RCC AHB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2ENCR)

For information about available fields see [`mod@ahb2encr`] module*/
pub type AHB2ENCR = crate::Reg<ahb2encr::AHB2ENCRrs>;
///RCC AHB2 enable register
pub mod ahb2encr;
/**AHB3ENCR (w) register accessor: RCC AHB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3ENCR)

For information about available fields see [`mod@ahb3encr`] module*/
pub type AHB3ENCR = crate::Reg<ahb3encr::AHB3ENCRrs>;
///RCC AHB3 enable register
pub mod ahb3encr;
/**AHB4ENCR (w) register accessor: RCC AHB4 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4ENCR)

For information about available fields see [`mod@ahb4encr`] module*/
pub type AHB4ENCR = crate::Reg<ahb4encr::AHB4ENCRrs>;
///RCC AHB4 enable register
pub mod ahb4encr;
/**AHB5ENCR (w) register accessor: RCC AHB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5ENCR)

For information about available fields see [`mod@ahb5encr`] module*/
pub type AHB5ENCR = crate::Reg<ahb5encr::AHB5ENCRrs>;
///RCC AHB5 enable register
pub mod ahb5encr;
/**APB1LENCR (w) register accessor: RCC APB1L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LENCR)

For information about available fields see [`mod@apb1lencr`] module*/
pub type APB1LENCR = crate::Reg<apb1lencr::APB1LENCRrs>;
///RCC APB1L enable register
pub mod apb1lencr;
/**APB1HENCR (w) register accessor: RCC APB1H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HENCR)

For information about available fields see [`mod@apb1hencr`] module*/
pub type APB1HENCR = crate::Reg<apb1hencr::APB1HENCRrs>;
///RCC APB1H enable register
pub mod apb1hencr;
/**APB2ENCR (w) register accessor: RCC APB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2ENCR)

For information about available fields see [`mod@apb2encr`] module*/
pub type APB2ENCR = crate::Reg<apb2encr::APB2ENCRrs>;
///RCC APB2 enable register
pub mod apb2encr;
/**APB3ENCR (w) register accessor: RCC APB3 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB3ENCR)

For information about available fields see [`mod@apb3encr`] module*/
pub type APB3ENCR = crate::Reg<apb3encr::APB3ENCRrs>;
///RCC APB3 enable register
pub mod apb3encr;
/**APB4LENCR (w) register accessor: RCC APB4L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LENCR)

For information about available fields see [`mod@apb4lencr`] module*/
pub type APB4LENCR = crate::Reg<apb4lencr::APB4LENCRrs>;
///RCC APB4L enable register
pub mod apb4lencr;
/**APB4HENCR (w) register accessor: RCC APB4H enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HENCR)

For information about available fields see [`mod@apb4hencr`] module*/
pub type APB4HENCR = crate::Reg<apb4hencr::APB4HENCRrs>;
///RCC APB4H enable register
pub mod apb4hencr;
/**APB5ENCR (w) register accessor: RCC APB5 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5encr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5ENCR)

For information about available fields see [`mod@apb5encr`] module*/
pub type APB5ENCR = crate::Reg<apb5encr::APB5ENCRrs>;
///RCC APB5 enable register
pub mod apb5encr;
/**DIVLPENCR (w) register accessor: RCC divider Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divlpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:DIVLPENCR)

For information about available fields see [`mod@divlpencr`] module*/
pub type DIVLPENCR = crate::Reg<divlpencr::DIVLPENCRrs>;
///RCC divider Sleep enable register
pub mod divlpencr;
/**BUSLPENCR (w) register accessor: RCC bus Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buslpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSLPENCR)

For information about available fields see [`mod@buslpencr`] module*/
pub type BUSLPENCR = crate::Reg<buslpencr::BUSLPENCRrs>;
///RCC bus Sleep enable register
pub mod buslpencr;
/**MISCLPENCR (w) register accessor: RCC miscellaneous Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MISCLPENCR)

For information about available fields see [`mod@misclpencr`] module*/
pub type MISCLPENCR = crate::Reg<misclpencr::MISCLPENCRrs>;
///RCC miscellaneous Sleep enable register
pub mod misclpencr;
/**MEMLPENCR (w) register accessor: RCC memory Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memlpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:MEMLPENCR)

For information about available fields see [`mod@memlpencr`] module*/
pub type MEMLPENCR = crate::Reg<memlpencr::MEMLPENCRrs>;
///RCC memory Sleep enable register
pub mod memlpencr;
/**AHB1LPENCR (w) register accessor: RCC AHB1 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1LPENCR)

For information about available fields see [`mod@ahb1lpencr`] module*/
pub type AHB1LPENCR = crate::Reg<ahb1lpencr::AHB1LPENCRrs>;
///RCC AHB1 Sleep enable register
pub mod ahb1lpencr;
/**AHB2LPENCR (w) register accessor: RCC AHB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB2LPENCR)

For information about available fields see [`mod@ahb2lpencr`] module*/
pub type AHB2LPENCR = crate::Reg<ahb2lpencr::AHB2LPENCRrs>;
///RCC AHB2 Sleep enable register
pub mod ahb2lpencr;
/**AHB3LPENCR (w) register accessor: RCC AHB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB3LPENCR)

For information about available fields see [`mod@ahb3lpencr`] module*/
pub type AHB3LPENCR = crate::Reg<ahb3lpencr::AHB3LPENCRrs>;
///RCC AHB3 Sleep enable register
pub mod ahb3lpencr;
/**AHB4LPENCR (w) register accessor: RCC AHB4 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4LPENCR)

For information about available fields see [`mod@ahb4lpencr`] module*/
pub type AHB4LPENCR = crate::Reg<ahb4lpencr::AHB4LPENCRrs>;
///RCC AHB4 Sleep enable register
pub mod ahb4lpencr;
/**AHB5LPENCR (w) register accessor: RCC AHB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB5LPENCR)

For information about available fields see [`mod@ahb5lpencr`] module*/
pub type AHB5LPENCR = crate::Reg<ahb5lpencr::AHB5LPENCRrs>;
///RCC AHB5 Sleep enable register
pub mod ahb5lpencr;
/**APB1LLPENCR (w) register accessor: RCC APB1L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LLPENCR)

For information about available fields see [`mod@apb1llpencr`] module*/
pub type APB1LLPENCR = crate::Reg<apb1llpencr::APB1LLPENCRrs>;
///RCC APB1L Sleep enable register
pub mod apb1llpencr;
/**APB1HLPENCR (w) register accessor: RCC APB1H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1HLPENCR)

For information about available fields see [`mod@apb1hlpencr`] module*/
pub type APB1HLPENCR = crate::Reg<apb1hlpencr::APB1HLPENCRrs>;
///RCC APB1H Sleep enable register
pub mod apb1hlpencr;
/**APB2LPENCR (w) register accessor: RCC APB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2LPENCR)

For information about available fields see [`mod@apb2lpencr`] module*/
pub type APB2LPENCR = crate::Reg<apb2lpencr::APB2LPENCRrs>;
///RCC APB2 Sleep enable register
pub mod apb2lpencr;
/**APB3LPENCR (w) register accessor: RCC APB3 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB3LPENCR)

For information about available fields see [`mod@apb3lpencr`] module*/
pub type APB3LPENCR = crate::Reg<apb3lpencr::APB3LPENCRrs>;
///RCC APB3 Sleep enable register
pub mod apb3lpencr;
/**APB4LLPENCR (w) register accessor: RCC APB4L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4llpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LLPENCR)

For information about available fields see [`mod@apb4llpencr`] module*/
pub type APB4LLPENCR = crate::Reg<apb4llpencr::APB4LLPENCRrs>;
///RCC APB4L Sleep enable register
pub mod apb4llpencr;
/**APB4HLPENCR (w) register accessor: RCC APB4H Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4hlpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4HLPENCR)

For information about available fields see [`mod@apb4hlpencr`] module*/
pub type APB4HLPENCR = crate::Reg<apb4hlpencr::APB4HLPENCRrs>;
///RCC APB4H Sleep enable register
pub mod apb4hlpencr;
/**APB5LPENCR (w) register accessor: RCC APB5 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5lpencr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB5LPENCR)

For information about available fields see [`mod@apb5lpencr`] module*/
pub type APB5LPENCR = crate::Reg<apb5lpencr::APB5LPENCRrs>;
///RCC APB5 Sleep enable register
pub mod apb5lpencr;
/**PRIVCFGCR0 (w) register accessor: RCC oscillator privilege configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGCR0)

For information about available fields see [`mod@privcfgcr0`] module*/
pub type PRIVCFGCR0 = crate::Reg<privcfgcr0::PRIVCFGCR0rs>;
///RCC oscillator privilege configuration register0
pub mod privcfgcr0;
/**PUBCFGCR0 (w) register accessor: RCC oscillator public configuration register0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGCR0)

For information about available fields see [`mod@pubcfgcr0`] module*/
pub type PUBCFGCR0 = crate::Reg<pubcfgcr0::PUBCFGCR0rs>;
///RCC oscillator public configuration register0
pub mod pubcfgcr0;
/**PRIVCFGCR1 (w) register accessor: RCC PLL privilege configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGCR1)

For information about available fields see [`mod@privcfgcr1`] module*/
pub type PRIVCFGCR1 = crate::Reg<privcfgcr1::PRIVCFGCR1rs>;
///RCC PLL privilege configuration register1
pub mod privcfgcr1;
/**PUBCFGCR1 (w) register accessor: RCC PLL public configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGCR1)

For information about available fields see [`mod@pubcfgcr1`] module*/
pub type PUBCFGCR1 = crate::Reg<pubcfgcr1::PUBCFGCR1rs>;
///RCC PLL public configuration register1
pub mod pubcfgcr1;
/**PRIVCFGCR2 (w) register accessor: RCC divider privilege configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGCR2)

For information about available fields see [`mod@privcfgcr2`] module*/
pub type PRIVCFGCR2 = crate::Reg<privcfgcr2::PRIVCFGCR2rs>;
///RCC divider privilege configuration register2
pub mod privcfgcr2;
/**PUBCFGCR2 (w) register accessor: RCC divider public configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGCR2)

For information about available fields see [`mod@pubcfgcr2`] module*/
pub type PUBCFGCR2 = crate::Reg<pubcfgcr2::PUBCFGCR2rs>;
///RCC divider public configuration register2
pub mod pubcfgcr2;
/**PRIVCFGCR3 (w) register accessor: RCC system privilege configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGCR3)

For information about available fields see [`mod@privcfgcr3`] module*/
pub type PRIVCFGCR3 = crate::Reg<privcfgcr3::PRIVCFGCR3rs>;
///RCC system privilege configuration register3
pub mod privcfgcr3;
/**PUBCFGCR3 (w) register accessor: RCC system public configuration register3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGCR3)

For information about available fields see [`mod@pubcfgcr3`] module*/
pub type PUBCFGCR3 = crate::Reg<pubcfgcr3::PUBCFGCR3rs>;
///RCC system public configuration register3
pub mod pubcfgcr3;
/**PRIVCFGCR4 (w) register accessor: RCC privilege configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGCR4)

For information about available fields see [`mod@privcfgcr4`] module*/
pub type PRIVCFGCR4 = crate::Reg<privcfgcr4::PRIVCFGCR4rs>;
///RCC privilege configuration register4
pub mod privcfgcr4;
/**PUBCFGCR4 (w) register accessor: RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGCR4)

For information about available fields see [`mod@pubcfgcr4`] module*/
pub type PUBCFGCR4 = crate::Reg<pubcfgcr4::PUBCFGCR4rs>;
///RCC public configuration register4
pub mod pubcfgcr4;
/**PUBCFGCR5 (w) register accessor: RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGCR5)

For information about available fields see [`mod@pubcfgcr5`] module*/
pub type PUBCFGCR5 = crate::Reg<pubcfgcr5::PUBCFGCR5rs>;
///RCC public configuration register4
pub mod pubcfgcr5;
