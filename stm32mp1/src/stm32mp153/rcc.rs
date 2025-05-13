#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tzcr: TZCR,
    _reserved1: [u8; 0x08],
    ocensetr: OCENSETR,
    ocenclrr: OCENCLRR,
    _reserved3: [u8; 0x04],
    hsicfgr: HSICFGR,
    csicfgr: CSICFGR,
    mpckselr: MPCKSELR,
    assckselr: ASSCKSELR,
    rck12selr: RCK12SELR,
    mpckdivr: MPCKDIVR,
    axidivr: AXIDIVR,
    _reserved10: [u8; 0x08],
    apb4divr: APB4DIVR,
    apb5divr: APB5DIVR,
    rtcdivr: RTCDIVR,
    mssckselr: MSSCKSELR,
    _reserved14: [u8; 0x34],
    pll1cr: PLL1CR,
    pll1cfgr1: PLL1CFGR1,
    pll1cfgr2: PLL1CFGR2,
    pll1fracr: PLL1FRACR,
    pll1csgr: PLL1CSGR,
    pll2cr: PLL2CR,
    pll2cfgr1: PLL2CFGR1,
    pll2cfgr2: PLL2CFGR2,
    pll2fracr: PLL2FRACR,
    pll2csgr: PLL2CSGR,
    _reserved24: [u8; 0x18],
    i2c46ckselr: I2C46CKSELR,
    spi6ckselr: SPI6CKSELR,
    uart1ckselr: UART1CKSELR,
    rng1ckselr: RNG1CKSELR,
    cperckselr: CPERCKSELR,
    stgenckselr: STGENCKSELR,
    ddritfcr: DDRITFCR,
    _reserved31: [u8; 0x24],
    mp_bootcr: MP_BOOTCR,
    mp_sreqsetr: MP_SREQSETR,
    mp_sreqclrr: MP_SREQCLRR,
    mp_gcr: MP_GCR,
    mp_aprstcr: MP_APRSTCR,
    mp_aprstsr: MP_APRSTSR,
    _reserved37: [u8; 0x28],
    bdcr: BDCR,
    rdlsicr: RDLSICR,
    _reserved39: [u8; 0x38],
    apb4rstsetr: APB4RSTSETR,
    apb4rstclrr: APB4RSTCLRR,
    apb5rstsetr: APB5RSTSETR,
    apb5rstclrr: APB5RSTCLRR,
    ahb5rstsetr: AHB5RSTSETR,
    ahb5rstclrr: AHB5RSTCLRR,
    ahb6rstsetr: AHB6RSTSETR,
    ahb6rstclrr: AHB6RSTCLRR,
    tzahb6rstsetr: TZAHB6RSTSETR,
    tzahb6rstclrr: TZAHB6RSTCLRR,
    _reserved49: [u8; 0x58],
    mp_apb4ensetr: MP_APB4ENSETR,
    mp_apb4enclrr: MP_APB4ENCLRR,
    mp_apb5ensetr: MP_APB5ENSETR,
    mp_apb5enclrr: MP_APB5ENCLRR,
    mp_ahb5ensetr: MP_AHB5ENSETR,
    mp_ahb5enclrr: MP_AHB5ENCLRR,
    mp_ahb6ensetr: MP_AHB6ENSETR,
    mp_ahb6enclrr: MP_AHB6ENCLRR,
    mp_tzahb6ensetr: MP_TZAHB6ENSETR,
    mp_tzahb6enclrr: MP_TZAHB6ENCLRR,
    _reserved59: [u8; 0x58],
    mc_apb4ensetr: MC_APB4ENSETR,
    mc_apb4enclrr: MC_APB4ENCLRR,
    mc_apb5ensetr: MC_APB5ENSETR,
    mc_apb5enclrr: MC_APB5ENCLRR,
    mc_ahb5ensetr: MC_AHB5ENSETR,
    mc_ahb5enclrr: MC_AHB5ENCLRR,
    mc_ahb6ensetr: MC_AHB6ENSETR,
    mc_ahb6enclrr: MC_AHB6ENCLRR,
    _reserved67: [u8; 0x60],
    mp_apb4lpensetr: MP_APB4LPENSETR,
    mp_apb4lpenclrr: MP_APB4LPENCLRR,
    mp_apb5lpensetr: MP_APB5LPENSETR,
    mp_apb5lpenclrr: MP_APB5LPENCLRR,
    mp_ahb5lpensetr: MP_AHB5LPENSETR,
    mp_ahb5lpenclrr: MP_AHB5LPENCLRR,
    mp_ahb6lpensetr: MP_AHB6LPENSETR,
    mp_ahb6lpenclrr: MP_AHB6LPENCLRR,
    mp_tzahb6lpensetr: MP_TZAHB6LPENSETR,
    mp_tzahb6lpenclrr: MP_TZAHB6LPENCLRR,
    _reserved77: [u8; 0x58],
    mc_apb4lpensetr: MC_APB4LPENSETR,
    mc_apb4lpenclrr: MC_APB4LPENCLRR,
    mc_apb5lpensetr: MC_APB5LPENSETR,
    mc_apb5lpenclrr: MC_APB5LPENCLRR,
    mc_ahb5lpensetr: MC_AHB5LPENSETR,
    mc_ahb5lpenclrr: MC_AHB5LPENCLRR,
    mc_ahb6lpensetr: MC_AHB6LPENSETR,
    mc_ahb6lpenclrr: MC_AHB6LPENCLRR,
    _reserved85: [u8; 0x60],
    br_rstsclrr: BR_RSTSCLRR,
    mp_grstcsetr: MP_GRSTCSETR,
    mp_rstsclrr: MP_RSTSCLRR,
    mp_iwdgfzsetr: MP_IWDGFZSETR,
    mp_iwdgfzclrr: MP_IWDGFZCLRR,
    mp_cier: MP_CIER,
    mp_cifr: MP_CIFR,
    pwrlpdlycr: PWRLPDLYCR,
    mp_rstssetr: MP_RSTSSETR,
    _reserved94: [u8; 0x03dc],
    mco1cfgr: MCO1CFGR,
    mco2cfgr: MCO2CFGR,
    ocrdyr: OCRDYR,
    dbgcfgr: DBGCFGR,
    _reserved98: [u8; 0x10],
    rck3selr: RCK3SELR,
    rck4selr: RCK4SELR,
    timg1prer: TIMG1PRER,
    timg2prer: TIMG2PRER,
    mcudivr: MCUDIVR,
    apb1divr: APB1DIVR,
    apb2divr: APB2DIVR,
    apb3divr: APB3DIVR,
    _reserved106: [u8; 0x40],
    pll3cr: PLL3CR,
    pll3cfgr1: PLL3CFGR1,
    pll3cfgr2: PLL3CFGR2,
    pll3fracr: PLL3FRACR,
    pll3csgr: PLL3CSGR,
    pll4cr: PLL4CR,
    pll4cfgr1: PLL4CFGR1,
    pll4cfgr2: PLL4CFGR2,
    pll4fracr: PLL4FRACR,
    pll4csgr: PLL4CSGR,
    _reserved116: [u8; 0x18],
    i2c12ckselr: I2C12CKSELR,
    i2c35ckselr: I2C35CKSELR,
    sai1ckselr: SAI1CKSELR,
    sai2ckselr: SAI2CKSELR,
    sai3ckselr: SAI3CKSELR,
    sai4ckselr: SAI4CKSELR,
    spi2s1ckselr: SPI2S1CKSELR,
    spi2s23ckselr: SPI2S23CKSELR,
    spi45ckselr: SPI45CKSELR,
    uart6ckselr: UART6CKSELR,
    uart24ckselr: UART24CKSELR,
    uart35ckselr: UART35CKSELR,
    uart78ckselr: UART78CKSELR,
    sdmmc12ckselr: SDMMC12CKSELR,
    sdmmc3ckselr: SDMMC3CKSELR,
    ethckselr: ETHCKSELR,
    qspickselr: QSPICKSELR,
    fmcckselr: FMCCKSELR,
    _reserved134: [u8; 0x04],
    fdcanckselr: FDCANCKSELR,
    _reserved135: [u8; 0x04],
    spdifckselr: SPDIFCKSELR,
    cecckselr: CECCKSELR,
    usbckselr: USBCKSELR,
    rng2ckselr: RNG2CKSELR,
    dsickselr: DSICKSELR,
    adcckselr: ADCCKSELR,
    lptim45ckselr: LPTIM45CKSELR,
    lptim23ckselr: LPTIM23CKSELR,
    lptim1ckselr: LPTIM1CKSELR,
    _reserved144: [u8; 0x48],
    apb1rstsetr: APB1RSTSETR,
    apb1rstclrr: APB1RSTCLRR,
    apb2rstsetr: APB2RSTSETR,
    apb2rstclrr: APB2RSTCLRR,
    apb3rstsetr: APB3RSTSETR,
    apb3rstclrr: APB3RSTCLRR,
    ahb2rstsetr: AHB2RSTSETR,
    ahb2rstclrr: AHB2RSTCLRR,
    ahb3rstsetr: AHB3RSTSETR,
    ahb3rstclrr: AHB3RSTCLRR,
    ahb4rstsetr: AHB4RSTSETR,
    ahb4rstclrr: AHB4RSTCLRR,
    _reserved156: [u8; 0x50],
    mp_apb1ensetr: MP_APB1ENSETR,
    mp_apb1enclrr: MP_APB1ENCLRR,
    mp_apb2ensetr: MP_APB2ENSETR,
    mp_apb2enclrr: MP_APB2ENCLRR,
    mp_apb3ensetr: MP_APB3ENSETR,
    mp_apb3enclrr: MP_APB3ENCLRR,
    mp_ahb2ensetr: MP_AHB2ENSETR,
    mp_ahb2enclrr: MP_AHB2ENCLRR,
    mp_ahb3ensetr: MP_AHB3ENSETR,
    mp_ahb3enclrr: MP_AHB3ENCLRR,
    mp_ahb4ensetr: MP_AHB4ENSETR,
    mp_ahb4enclrr: MP_AHB4ENCLRR,
    _reserved168: [u8; 0x08],
    mp_mlahbensetr: MP_MLAHBENSETR,
    mp_mlahbenclrr: MP_MLAHBENCLRR,
    _reserved170: [u8; 0x40],
    mc_apb1ensetr: MC_APB1ENSETR,
    mc_apb1enclrr: MC_APB1ENCLRR,
    mc_apb2ensetr: MC_APB2ENSETR,
    mc_apb2enclrr: MC_APB2ENCLRR,
    mc_apb3ensetr: MC_APB3ENSETR,
    mc_apb3enclrr: MC_APB3ENCLRR,
    mc_ahb2ensetr: MC_AHB2ENSETR,
    mc_ahb2enclrr: MC_AHB2ENCLRR,
    mc_ahb3ensetr: MC_AHB3ENSETR,
    mc_ahb3enclrr: MC_AHB3ENCLRR,
    mc_ahb4ensetr: MC_AHB4ENSETR,
    mc_ahb4enclrr: MC_AHB4ENCLRR,
    mc_aximensetr: MC_AXIMENSETR,
    mc_aximenclrr: MC_AXIMENCLRR,
    mc_mlahbensetr: MC_MLAHBENSETR,
    mc_mlahbenclrr: MC_MLAHBENCLRR,
    _reserved186: [u8; 0x40],
    mp_apb1lpensetr: MP_APB1LPENSETR,
    mp_apb1lpenclrr: MP_APB1LPENCLRR,
    mp_apb2lpensetr: MP_APB2LPENSETR,
    mp_apb2lpenclrr: MP_APB2LPENCLRR,
    mp_apb3lpensetr: MP_APB3LPENSETR,
    mp_apb3lpenclrr: MP_APB3LPENCLRR,
    mp_ahb2lpensetr: MP_AHB2LPENSETR,
    mp_ahb2lpenclrr: MP_AHB2LPENCLRR,
    mp_ahb3lpensetr: MP_AHB3LPENSETR,
    mp_ahb3lpenclrr: MP_AHB3LPENCLRR,
    mp_ahb4lpensetr: MP_AHB4LPENSETR,
    mp_ahb4lpenclrr: MP_AHB4LPENCLRR,
    mp_aximlpensetr: MP_AXIMLPENSETR,
    mp_aximlpenclrr: MP_AXIMLPENCLRR,
    mp_mlahblpensetr: MP_MLAHBLPENSETR,
    mp_mlahblpenclrr: MP_MLAHBLPENCLRR,
    _reserved202: [u8; 0x40],
    mc_apb1lpensetr: MC_APB1LPENSETR,
    mc_apb1lpenclrr: MC_APB1LPENCLRR,
    mc_apb2lpensetr: MC_APB2LPENSETR,
    mc_apb2lpenclrr: MC_APB2LPENCLRR,
    mc_apb3lpensetr: MC_APB3LPENSETR,
    mc_apb3lpenclrr: MC_APB3LPENCLRR,
    mc_ahb2lpensetr: MC_AHB2LPENSETR,
    mc_ahb2lpenclrr: MC_AHB2LPENCLRR,
    mc_ahb3lpensetr: MC_AHB3LPENSETR,
    mc_ahb3lpenclrr: MC_AHB3LPENCLRR,
    mc_ahb4lpensetr: MC_AHB4LPENSETR,
    mc_ahb4lpenclrr: MC_AHB4LPENCLRR,
    mc_aximlpensetr: MC_AXIMLPENSETR,
    mc_aximlpenclrr: MC_AXIMLPENCLRR,
    mc_mlahblpensetr: MC_MLAHBLPENSETR,
    mc_mlahblpenclrr: MC_MLAHBLPENCLRR,
    _reserved218: [u8; 0x40],
    mc_rstsclrr: MC_RSTSCLRR,
    _reserved219: [u8; 0x10],
    mc_cier: MC_CIER,
    mc_cifr: MC_CIFR,
    _reserved221: [u8; 0x03d8],
    verr: VERR,
    idr: IDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.
    #[inline(always)]
    pub const fn tzcr(&self) -> &TZCR {
        &self.tzcr
    }
    ///0x0c - This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn ocensetr(&self) -> &OCENSETR {
        &self.ocensetr
    }
    ///0x10 - This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn ocenclrr(&self) -> &OCENCLRR {
        &self.ocenclrr
    }
    ///0x18 - This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &HSICFGR {
        &self.hsicfgr
    }
    ///0x1c - This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.
    #[inline(always)]
    pub const fn csicfgr(&self) -> &CSICFGR {
        &self.csicfgr
    }
    ///0x20 - This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn mpckselr(&self) -> &MPCKSELR {
        &self.mpckselr
    }
    ///0x24 - This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn assckselr(&self) -> &ASSCKSELR {
        &self.assckselr
    }
    ///0x28 - This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn rck12selr(&self) -> &RCK12SELR {
        &self.rck12selr
    }
    ///0x2c - This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mpckdivr(&self) -> &MPCKDIVR {
        &self.mpckdivr
    }
    ///0x30 - This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn axidivr(&self) -> &AXIDIVR {
        &self.axidivr
    }
    ///0x3c - This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn apb4divr(&self) -> &APB4DIVR {
        &self.apb4divr
    }
    ///0x40 - This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn apb5divr(&self) -> &APB5DIVR {
        &self.apb5divr
    }
    ///0x44 - This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn rtcdivr(&self) -> &RTCDIVR {
        &self.rtcdivr
    }
    ///0x48 - This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn mssckselr(&self) -> &MSSCKSELR {
        &self.mssckselr
    }
    ///0x80 - This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll1cr(&self) -> &PLL1CR {
        &self.pll1cr
    }
    ///0x84 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll1cfgr1(&self) -> &PLL1CFGR1 {
        &self.pll1cfgr1
    }
    ///0x88 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll1cfgr2(&self) -> &PLL1CFGR2 {
        &self.pll1cfgr2
    }
    ///0x8c - This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &PLL1FRACR {
        &self.pll1fracr
    }
    ///0x90 - This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll1csgr(&self) -> &PLL1CSGR {
        &self.pll1csgr
    }
    ///0x94 - This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll2cr(&self) -> &PLL2CR {
        &self.pll2cr
    }
    ///0x98 - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll2cfgr1(&self) -> &PLL2CFGR1 {
        &self.pll2cfgr1
    }
    ///0x9c - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll2cfgr2(&self) -> &PLL2CFGR2 {
        &self.pll2cfgr2
    }
    ///0xa0 - This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &PLL2FRACR {
        &self.pll2fracr
    }
    ///0xa4 - This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    #[inline(always)]
    pub const fn pll2csgr(&self) -> &PLL2CSGR {
        &self.pll2csgr
    }
    ///0xc0 - This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn i2c46ckselr(&self) -> &I2C46CKSELR {
        &self.i2c46ckselr
    }
    ///0xc4 - This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn spi6ckselr(&self) -> &SPI6CKSELR {
        &self.spi6ckselr
    }
    ///0xc8 - This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn uart1ckselr(&self) -> &UART1CKSELR {
        &self.uart1ckselr
    }
    ///0xcc - This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn rng1ckselr(&self) -> &RNG1CKSELR {
        &self.rng1ckselr
    }
    ///0xd0 - This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn cperckselr(&self) -> &CPERCKSELR {
        &self.cperckselr
    }
    ///0xd4 - This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn stgenckselr(&self) -> &STGENCKSELR {
        &self.stgenckselr
    }
    ///0xd8 - This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn ddritfcr(&self) -> &DDRITFCR {
        &self.ddritfcr
    }
    ///0x100 - This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.
    #[inline(always)]
    pub const fn mp_bootcr(&self) -> &MP_BOOTCR {
        &self.mp_bootcr
    }
    ///0x104 - Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_sreqsetr(&self) -> &MP_SREQSETR {
        &self.mp_sreqsetr
    }
    ///0x108 - Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_sreqclrr(&self) -> &MP_SREQCLRR {
        &self.mp_sreqclrr
    }
    ///0x10c - The register contains global control bits. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_gcr(&self) -> &MP_GCR {
        &self.mp_gcr
    }
    ///0x110 - This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_aprstcr(&self) -> &MP_APRSTCR {
        &self.mp_aprstcr
    }
    ///0x114 - This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_aprstsr(&self) -> &MP_APRSTSR {
        &self.mp_aprstsr
    }
    ///0x140 - This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn bdcr(&self) -> &BDCR {
        &self.bdcr
    }
    ///0x144 - This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn rdlsicr(&self) -> &RDLSICR {
        &self.rdlsicr
    }
    ///0x180 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb4rstsetr(&self) -> &APB4RSTSETR {
        &self.apb4rstsetr
    }
    ///0x184 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb4rstclrr(&self) -> &APB4RSTCLRR {
        &self.apb4rstclrr
    }
    ///0x188 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn apb5rstsetr(&self) -> &APB5RSTSETR {
        &self.apb5rstsetr
    }
    ///0x18c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn apb5rstclrr(&self) -> &APB5RSTCLRR {
        &self.apb5rstclrr
    }
    ///0x190 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn ahb5rstsetr(&self) -> &AHB5RSTSETR {
        &self.ahb5rstsetr
    }
    ///0x194 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn ahb5rstclrr(&self) -> &AHB5RSTCLRR {
        &self.ahb5rstclrr
    }
    ///0x198 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn ahb6rstsetr(&self) -> &AHB6RSTSETR {
        &self.ahb6rstsetr
    }
    ///0x19c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn ahb6rstclrr(&self) -> &AHB6RSTCLRR {
        &self.ahb6rstclrr
    }
    ///0x1a0 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn tzahb6rstsetr(&self) -> &TZAHB6RSTSETR {
        &self.tzahb6rstsetr
    }
    ///0x1a4 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn tzahb6rstclrr(&self) -> &TZAHB6RSTCLRR {
        &self.tzahb6rstclrr
    }
    ///0x200 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    #[inline(always)]
    pub const fn mp_apb4ensetr(&self) -> &MP_APB4ENSETR {
        &self.mp_apb4ensetr
    }
    ///0x204 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    #[inline(always)]
    pub const fn mp_apb4enclrr(&self) -> &MP_APB4ENCLRR {
        &self.mp_apb4enclrr
    }
    ///0x208 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    #[inline(always)]
    pub const fn mp_apb5ensetr(&self) -> &MP_APB5ENSETR {
        &self.mp_apb5ensetr
    }
    ///0x20c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    #[inline(always)]
    pub const fn mp_apb5enclrr(&self) -> &MP_APB5ENCLRR {
        &self.mp_apb5enclrr
    }
    ///0x210 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_ahb5ensetr(&self) -> &MP_AHB5ENSETR {
        &self.mp_ahb5ensetr
    }
    ///0x214 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_ahb5enclrr(&self) -> &MP_AHB5ENCLRR {
        &self.mp_ahb5enclrr
    }
    ///0x218 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    #[inline(always)]
    pub const fn mp_ahb6ensetr(&self) -> &MP_AHB6ENSETR {
        &self.mp_ahb6ensetr
    }
    ///0x21c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    #[inline(always)]
    pub const fn mp_ahb6enclrr(&self) -> &MP_AHB6ENCLRR {
        &self.mp_ahb6enclrr
    }
    ///0x220 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_tzahb6ensetr(&self) -> &MP_TZAHB6ENSETR {
        &self.mp_tzahb6ensetr
    }
    ///0x224 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_tzahb6enclrr(&self) -> &MP_TZAHB6ENCLRR {
        &self.mp_tzahb6enclrr
    }
    ///0x280 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb4ensetr(&self) -> &MC_APB4ENSETR {
        &self.mc_apb4ensetr
    }
    ///0x284 - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb4enclrr(&self) -> &MC_APB4ENCLRR {
        &self.mc_apb4enclrr
    }
    ///0x288 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb5ensetr(&self) -> &MC_APB5ENSETR {
        &self.mc_apb5ensetr
    }
    ///0x28c - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb5enclrr(&self) -> &MC_APB5ENCLRR {
        &self.mc_apb5enclrr
    }
    ///0x290 - This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mc_ahb5ensetr(&self) -> &MC_AHB5ENSETR {
        &self.mc_ahb5ensetr
    }
    ///0x294 - This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mc_ahb5enclrr(&self) -> &MC_AHB5ENCLRR {
        &self.mc_ahb5enclrr
    }
    ///0x298 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb6ensetr(&self) -> &MC_AHB6ENSETR {
        &self.mc_ahb6ensetr
    }
    ///0x29c - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb6enclrr(&self) -> &MC_AHB6ENCLRR {
        &self.mc_ahb6enclrr
    }
    ///0x300 - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_apb4lpensetr(&self) -> &MP_APB4LPENSETR {
        &self.mp_apb4lpensetr
    }
    ///0x304 - This register is used by the MCU
    #[inline(always)]
    pub const fn mp_apb4lpenclrr(&self) -> &MP_APB4LPENCLRR {
        &self.mp_apb4lpenclrr
    }
    ///0x308 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_apb5lpensetr(&self) -> &MP_APB5LPENSETR {
        &self.mp_apb5lpensetr
    }
    ///0x30c - This register is used by the Mpu.
    #[inline(always)]
    pub const fn mp_apb5lpenclrr(&self) -> &MP_APB5LPENCLRR {
        &self.mp_apb5lpenclrr
    }
    ///0x310 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_ahb5lpensetr(&self) -> &MP_AHB5LPENSETR {
        &self.mp_ahb5lpensetr
    }
    ///0x314 - This register is used by the MCU
    #[inline(always)]
    pub const fn mp_ahb5lpenclrr(&self) -> &MP_AHB5LPENCLRR {
        &self.mp_ahb5lpenclrr
    }
    ///0x318 - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_ahb6lpensetr(&self) -> &MP_AHB6LPENSETR {
        &self.mp_ahb6lpensetr
    }
    ///0x31c - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_ahb6lpenclrr(&self) -> &MP_AHB6LPENCLRR {
        &self.mp_ahb6lpenclrr
    }
    ///0x320 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_tzahb6lpensetr(&self) -> &MP_TZAHB6LPENSETR {
        &self.mp_tzahb6lpensetr
    }
    ///0x324 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_tzahb6lpenclrr(&self) -> &MP_TZAHB6LPENCLRR {
        &self.mp_tzahb6lpenclrr
    }
    ///0x380 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_apb4lpensetr(&self) -> &MC_APB4LPENSETR {
        &self.mc_apb4lpensetr
    }
    ///0x384 - This register is used by the MCU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mc_apb4lpenclrr(&self) -> &MC_APB4LPENCLRR {
        &self.mc_apb4lpenclrr
    }
    ///0x388 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_apb5lpensetr(&self) -> &MC_APB5LPENSETR {
        &self.mc_apb5lpensetr
    }
    ///0x38c - This register is used by the MCU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mc_apb5lpenclrr(&self) -> &MC_APB5LPENCLRR {
        &self.mc_apb5lpenclrr
    }
    ///0x390 - This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mc_ahb5lpensetr(&self) -> &MC_AHB5LPENSETR {
        &self.mc_ahb5lpensetr
    }
    ///0x394 - This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mc_ahb5lpenclrr(&self) -> &MC_AHB5LPENCLRR {
        &self.mc_ahb5lpenclrr
    }
    ///0x398 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_ahb6lpensetr(&self) -> &MC_AHB6LPENSETR {
        &self.mc_ahb6lpensetr
    }
    ///0x39c - This register is used by the MCU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mc_ahb6lpenclrr(&self) -> &MC_AHB6LPENCLRR {
        &self.mc_ahb6lpenclrr
    }
    ///0x400 - This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn br_rstsclrr(&self) -> &BR_RSTSCLRR {
        &self.br_rstsclrr
    }
    ///0x404 - This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.
    #[inline(always)]
    pub const fn mp_grstcsetr(&self) -> &MP_GRSTCSETR {
        &self.mp_grstcsetr
    }
    ///0x408 - This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_rstsclrr(&self) -> &MP_RSTSCLRR {
        &self.mp_rstsclrr
    }
    ///0x40c - This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_iwdgfzsetr(&self) -> &MP_IWDGFZSETR {
        &self.mp_iwdgfzsetr
    }
    ///0x410 - This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_iwdgfzclrr(&self) -> &MP_IWDGFZCLRR {
        &self.mp_iwdgfzclrr
    }
    ///0x414 - This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_cier(&self) -> &MP_CIER {
        &self.mp_cier
    }
    ///0x418 - This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_cifr(&self) -> &MP_CIFR {
        &self.mp_cifr
    }
    ///0x41c - This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn pwrlpdlycr(&self) -> &PWRLPDLYCR {
        &self.pwrlpdlycr
    }
    ///0x420 - This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mp_rstssetr(&self) -> &MP_RSTSSETR {
        &self.mp_rstssetr
    }
    ///0x800 - This register is used to select the clock generated on MCO1 output.
    #[inline(always)]
    pub const fn mco1cfgr(&self) -> &MCO1CFGR {
        &self.mco1cfgr
    }
    ///0x804 - This register is used to select the clock generated on MCO2 output.
    #[inline(always)]
    pub const fn mco2cfgr(&self) -> &MCO2CFGR {
        &self.mco2cfgr
    }
    ///0x808 - This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.
    #[inline(always)]
    pub const fn ocrdyr(&self) -> &OCRDYR {
        &self.ocrdyr
    }
    ///0x80c - This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.
    #[inline(always)]
    pub const fn dbgcfgr(&self) -> &DBGCFGR {
        &self.dbgcfgr
    }
    ///0x820 - This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn rck3selr(&self) -> &RCK3SELR {
        &self.rck3selr
    }
    ///0x824 - This register is used to select the reference clock for PLL4.
    #[inline(always)]
    pub const fn rck4selr(&self) -> &RCK4SELR {
        &self.rck4selr
    }
    ///0x828 - This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.
    #[inline(always)]
    pub const fn timg1prer(&self) -> &TIMG1PRER {
        &self.timg1prer
    }
    ///0x82c - This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.
    #[inline(always)]
    pub const fn timg2prer(&self) -> &TIMG2PRER {
        &self.timg2prer
    }
    ///0x830 - This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn mcudivr(&self) -> &MCUDIVR {
        &self.mcudivr
    }
    ///0x834 - This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.
    #[inline(always)]
    pub const fn apb1divr(&self) -> &APB1DIVR {
        &self.apb1divr
    }
    ///0x838 - This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
    #[inline(always)]
    pub const fn apb2divr(&self) -> &APB2DIVR {
        &self.apb2divr
    }
    ///0x83c - This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
    #[inline(always)]
    pub const fn apb3divr(&self) -> &APB3DIVR {
        &self.apb3divr
    }
    ///0x880 - This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn pll3cr(&self) -> &PLL3CR {
        &self.pll3cr
    }
    ///0x884 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn pll3cfgr1(&self) -> &PLL3CFGR1 {
        &self.pll3cfgr1
    }
    ///0x888 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn pll3cfgr2(&self) -> &PLL3CFGR2 {
        &self.pll3cfgr2
    }
    ///0x88c - This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &PLL3FRACR {
        &self.pll3fracr
    }
    ///0x890 - This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn pll3csgr(&self) -> &PLL3CSGR {
        &self.pll3csgr
    }
    ///0x894 - This register is used to control the PLL4.
    #[inline(always)]
    pub const fn pll4cr(&self) -> &PLL4CR {
        &self.pll4cr
    }
    ///0x898 - This register is used to configure the PLL4.
    #[inline(always)]
    pub const fn pll4cfgr1(&self) -> &PLL4CFGR1 {
        &self.pll4cfgr1
    }
    ///0x89c - This register is used to configure the PLL4.
    #[inline(always)]
    pub const fn pll4cfgr2(&self) -> &PLL4CFGR2 {
        &self.pll4cfgr2
    }
    ///0x8a0 - This register is used to fine-tune the frequency of the PLL4 VCO.
    #[inline(always)]
    pub const fn pll4fracr(&self) -> &PLL4FRACR {
        &self.pll4fracr
    }
    ///0x8a4 - This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    #[inline(always)]
    pub const fn pll4csgr(&self) -> &PLL4CSGR {
        &self.pll4csgr
    }
    ///0x8c0 - This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn i2c12ckselr(&self) -> &I2C12CKSELR {
        &self.i2c12ckselr
    }
    ///0x8c4 - This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn i2c35ckselr(&self) -> &I2C35CKSELR {
        &self.i2c35ckselr
    }
    ///0x8c8 - This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn sai1ckselr(&self) -> &SAI1CKSELR {
        &self.sai1ckselr
    }
    ///0x8cc - This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn sai2ckselr(&self) -> &SAI2CKSELR {
        &self.sai2ckselr
    }
    ///0x8d0 - This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn sai3ckselr(&self) -> &SAI3CKSELR {
        &self.sai3ckselr
    }
    ///0x8d4 - This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn sai4ckselr(&self) -> &SAI4CKSELR {
        &self.sai4ckselr
    }
    ///0x8d8 - This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn spi2s1ckselr(&self) -> &SPI2S1CKSELR {
        &self.spi2s1ckselr
    }
    ///0x8dc - This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn spi2s23ckselr(&self) -> &SPI2S23CKSELR {
        &self.spi2s23ckselr
    }
    ///0x8e0 - This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn spi45ckselr(&self) -> &SPI45CKSELR {
        &self.spi45ckselr
    }
    ///0x8e4 - This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn uart6ckselr(&self) -> &UART6CKSELR {
        &self.uart6ckselr
    }
    ///0x8e8 - This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn uart24ckselr(&self) -> &UART24CKSELR {
        &self.uart24ckselr
    }
    ///0x8ec - This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn uart35ckselr(&self) -> &UART35CKSELR {
        &self.uart35ckselr
    }
    ///0x8f0 - This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn uart78ckselr(&self) -> &UART78CKSELR {
        &self.uart78ckselr
    }
    ///0x8f4 - This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn sdmmc12ckselr(&self) -> &SDMMC12CKSELR {
        &self.sdmmc12ckselr
    }
    ///0x8f8 - This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn sdmmc3ckselr(&self) -> &SDMMC3CKSELR {
        &self.sdmmc3ckselr
    }
    ///0x8fc - This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn ethckselr(&self) -> &ETHCKSELR {
        &self.ethckselr
    }
    ///0x900 - This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn qspickselr(&self) -> &QSPICKSELR {
        &self.qspickselr
    }
    ///0x904 - This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn fmcckselr(&self) -> &FMCCKSELR {
        &self.fmcckselr
    }
    ///0x90c - This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn fdcanckselr(&self) -> &FDCANCKSELR {
        &self.fdcanckselr
    }
    ///0x914 - This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    #[inline(always)]
    pub const fn spdifckselr(&self) -> &SPDIFCKSELR {
        &self.spdifckselr
    }
    ///0x918 - This register is used to control the selection of the kernel clock for the CEC-HDMI.
    #[inline(always)]
    pub const fn cecckselr(&self) -> &CECCKSELR {
        &self.cecckselr
    }
    ///0x91c - This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG
    #[inline(always)]
    pub const fn usbckselr(&self) -> &USBCKSELR {
        &self.usbckselr
    }
    ///0x920 - This register is used to control the selection of the kernel clock for the RNG2.
    #[inline(always)]
    pub const fn rng2ckselr(&self) -> &RNG2CKSELR {
        &self.rng2ckselr
    }
    ///0x924 - This register is used to control the selection of the kernel clock for the DSI block.
    #[inline(always)]
    pub const fn dsickselr(&self) -> &DSICKSELR {
        &self.dsickselr
    }
    ///0x928 - This register is used to control the selection of the kernel clock for the ADC block.
    #[inline(always)]
    pub const fn adcckselr(&self) -> &ADCCKSELR {
        &self.adcckselr
    }
    ///0x92c - This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.
    #[inline(always)]
    pub const fn lptim45ckselr(&self) -> &LPTIM45CKSELR {
        &self.lptim45ckselr
    }
    ///0x930 - This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.
    #[inline(always)]
    pub const fn lptim23ckselr(&self) -> &LPTIM23CKSELR {
        &self.lptim23ckselr
    }
    ///0x934 - This register is used to control the selection of the kernel clock for the LPTIM1 block.
    #[inline(always)]
    pub const fn lptim1ckselr(&self) -> &LPTIM1CKSELR {
        &self.lptim1ckselr
    }
    ///0x980 - This register is used to activate the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb1rstsetr(&self) -> &APB1RSTSETR {
        &self.apb1rstsetr
    }
    ///0x984 - This register is used to release the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb1rstclrr(&self) -> &APB1RSTCLRR {
        &self.apb1rstclrr
    }
    ///0x988 - This register is used to activate the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb2rstsetr(&self) -> &APB2RSTSETR {
        &self.apb2rstsetr
    }
    ///0x98c - This register is used to release the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb2rstclrr(&self) -> &APB2RSTCLRR {
        &self.apb2rstclrr
    }
    ///0x990 - This register is used to activate the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb3rstsetr(&self) -> &APB3RSTSETR {
        &self.apb3rstsetr
    }
    ///0x994 - This register is used to release the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn apb3rstclrr(&self) -> &APB3RSTCLRR {
        &self.apb3rstclrr
    }
    ///0x998 - This register is used to activate the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn ahb2rstsetr(&self) -> &AHB2RSTSETR {
        &self.ahb2rstsetr
    }
    ///0x99c - This register is used to release the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn ahb2rstclrr(&self) -> &AHB2RSTCLRR {
        &self.ahb2rstclrr
    }
    ///0x9a0 - This register is used to activate the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn ahb3rstsetr(&self) -> &AHB3RSTSETR {
        &self.ahb3rstsetr
    }
    ///0x9a4 - This register is used to release the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn ahb3rstclrr(&self) -> &AHB3RSTCLRR {
        &self.ahb3rstclrr
    }
    ///0x9a8 - This register is used to activate the reset of the corresponding peripheral
    #[inline(always)]
    pub const fn ahb4rstsetr(&self) -> &AHB4RSTSETR {
        &self.ahb4rstsetr
    }
    ///0x9ac - This register is used to release the reset of the corresponding peripheral.
    #[inline(always)]
    pub const fn ahb4rstclrr(&self) -> &AHB4RSTCLRR {
        &self.ahb4rstclrr
    }
    ///0xa00 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mp_apb1ensetr(&self) -> &MP_APB1ENSETR {
        &self.mp_apb1ensetr
    }
    ///0xa04 - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mp_apb1enclrr(&self) -> &MP_APB1ENCLRR {
        &self.mp_apb1enclrr
    }
    ///0xa08 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mp_apb2ensetr(&self) -> &MP_APB2ENSETR {
        &self.mp_apb2ensetr
    }
    ///0xa0c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mp_apb2enclrr(&self) -> &MP_APB2ENCLRR {
        &self.mp_apb2enclrr
    }
    ///0xa10 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mp_apb3ensetr(&self) -> &MP_APB3ENSETR {
        &self.mp_apb3ensetr
    }
    ///0xa14 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mp_apb3enclrr(&self) -> &MP_APB3ENCLRR {
        &self.mp_apb3enclrr
    }
    ///0xa18 - This register is used to set the peripheral clock enable bit of the corresponding peripheral
    #[inline(always)]
    pub const fn mp_ahb2ensetr(&self) -> &MP_AHB2ENSETR {
        &self.mp_ahb2ensetr
    }
    ///0xa1c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mp_ahb2enclrr(&self) -> &MP_AHB2ENCLRR {
        &self.mp_ahb2enclrr
    }
    ///0xa20 - This register is used to set the peripheral clock enable bit of the corresponding peripheral
    #[inline(always)]
    pub const fn mp_ahb3ensetr(&self) -> &MP_AHB3ENSETR {
        &self.mp_ahb3ensetr
    }
    ///0xa24 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mp_ahb3enclrr(&self) -> &MP_AHB3ENCLRR {
        &self.mp_ahb3enclrr
    }
    ///0xa28 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.
    #[inline(always)]
    pub const fn mp_ahb4ensetr(&self) -> &MP_AHB4ENSETR {
        &self.mp_ahb4ensetr
    }
    ///0xa2c - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mp_ahb4enclrr(&self) -> &MP_AHB4ENCLRR {
        &self.mp_ahb4enclrr
    }
    ///0xa38 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mp_mlahbensetr(&self) -> &MP_MLAHBENSETR {
        &self.mp_mlahbensetr
    }
    ///0xa3c - This register is used to clear the peripheral clock enable bit.
    #[inline(always)]
    pub const fn mp_mlahbenclrr(&self) -> &MP_MLAHBENCLRR {
        &self.mp_mlahbenclrr
    }
    ///0xa80 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .
    #[inline(always)]
    pub const fn mc_apb1ensetr(&self) -> &MC_APB1ENSETR {
        &self.mc_apb1ensetr
    }
    ///0xa84 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mc_apb1enclrr(&self) -> &MC_APB1ENCLRR {
        &self.mc_apb1enclrr
    }
    ///0xa88 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb2ensetr(&self) -> &MC_APB2ENSETR {
        &self.mc_apb2ensetr
    }
    ///0xa8c - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb2enclrr(&self) -> &MC_APB2ENCLRR {
        &self.mc_apb2enclrr
    }
    ///0xa90 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb3ensetr(&self) -> &MC_APB3ENSETR {
        &self.mc_apb3ensetr
    }
    ///0xa94 - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_apb3enclrr(&self) -> &MC_APB3ENCLRR {
        &self.mc_apb3enclrr
    }
    ///0xa98 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb2ensetr(&self) -> &MC_AHB2ENSETR {
        &self.mc_ahb2ensetr
    }
    ///0xa9c - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb2enclrr(&self) -> &MC_AHB2ENCLRR {
        &self.mc_ahb2enclrr
    }
    ///0xaa0 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb3ensetr(&self) -> &MC_AHB3ENSETR {
        &self.mc_ahb3ensetr
    }
    ///0xaa4 - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb3enclrr(&self) -> &MC_AHB3ENCLRR {
        &self.mc_ahb3enclrr
    }
    ///0xaa8 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb4ensetr(&self) -> &MC_AHB4ENSETR {
        &self.mc_ahb4ensetr
    }
    ///0xaac - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_ahb4enclrr(&self) -> &MC_AHB4ENCLRR {
        &self.mc_ahb4enclrr
    }
    ///0xab0 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_aximensetr(&self) -> &MC_AXIMENSETR {
        &self.mc_aximensetr
    }
    ///0xab4 - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_aximenclrr(&self) -> &MC_AXIMENCLRR {
        &self.mc_aximenclrr
    }
    ///0xab8 - This register is used to set the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_mlahbensetr(&self) -> &MC_MLAHBENSETR {
        &self.mc_mlahbensetr
    }
    ///0xabc - This register is used to clear the peripheral clock enable bit
    #[inline(always)]
    pub const fn mc_mlahbenclrr(&self) -> &MC_MLAHBENCLRR {
        &self.mc_mlahbenclrr
    }
    ///0xb00 - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_apb1lpensetr(&self) -> &MP_APB1LPENSETR {
        &self.mp_apb1lpensetr
    }
    ///0xb04 - This register is used by the MPU in order to clear the PERxLPEN bits .
    #[inline(always)]
    pub const fn mp_apb1lpenclrr(&self) -> &MP_APB1LPENCLRR {
        &self.mp_apb1lpenclrr
    }
    ///0xb08 - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_apb2lpensetr(&self) -> &MP_APB2LPENSETR {
        &self.mp_apb2lpensetr
    }
    ///0xb0c - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_apb2lpenclrr(&self) -> &MP_APB2LPENCLRR {
        &self.mp_apb2lpenclrr
    }
    ///0xb10 - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_apb3lpensetr(&self) -> &MP_APB3LPENSETR {
        &self.mp_apb3lpensetr
    }
    ///0xb14 - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_apb3lpenclrr(&self) -> &MP_APB3LPENCLRR {
        &self.mp_apb3lpenclrr
    }
    ///0xb18 - This register is used by the MPU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mp_ahb2lpensetr(&self) -> &MP_AHB2LPENSETR {
        &self.mp_ahb2lpensetr
    }
    ///0xb1c - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mp_ahb2lpenclrr(&self) -> &MP_AHB2LPENCLRR {
        &self.mp_ahb2lpenclrr
    }
    ///0xb20 - This register is used by the MPU
    #[inline(always)]
    pub const fn mp_ahb3lpensetr(&self) -> &MP_AHB3LPENSETR {
        &self.mp_ahb3lpensetr
    }
    ///0xb24 - This register is used by the MPU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mp_ahb3lpenclrr(&self) -> &MP_AHB3LPENCLRR {
        &self.mp_ahb3lpenclrr
    }
    ///0xb28 - This register is used by the MPU
    #[inline(always)]
    pub const fn mp_ahb4lpensetr(&self) -> &MP_AHB4LPENSETR {
        &self.mp_ahb4lpensetr
    }
    ///0xb2c - This register is used by the MPU
    #[inline(always)]
    pub const fn mp_ahb4lpenclrr(&self) -> &MP_AHB4LPENCLRR {
        &self.mp_ahb4lpenclrr
    }
    ///0xb30 - This register is used by the MPU
    #[inline(always)]
    pub const fn mp_aximlpensetr(&self) -> &MP_AXIMLPENSETR {
        &self.mp_aximlpensetr
    }
    ///0xb34 - This register is used by the MPU
    #[inline(always)]
    pub const fn mp_aximlpenclrr(&self) -> &MP_AXIMLPENCLRR {
        &self.mp_aximlpenclrr
    }
    ///0xb38 - This register is used by the MPU in order to set the PERxLPEN bit
    #[inline(always)]
    pub const fn mp_mlahblpensetr(&self) -> &MP_MLAHBLPENSETR {
        &self.mp_mlahblpensetr
    }
    ///0xb3c - This register is used by the MPU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mp_mlahblpenclrr(&self) -> &MP_MLAHBLPENCLRR {
        &self.mp_mlahblpenclrr
    }
    ///0xb80 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_apb1lpensetr(&self) -> &MC_APB1LPENSETR {
        &self.mc_apb1lpensetr
    }
    ///0xb84 - This register is used by the MCU in order to clear the PERxLPEN bits
    #[inline(always)]
    pub const fn mc_apb1lpenclrr(&self) -> &MC_APB1LPENCLRR {
        &self.mc_apb1lpenclrr
    }
    ///0xb88 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_apb2lpensetr(&self) -> &MC_APB2LPENSETR {
        &self.mc_apb2lpensetr
    }
    ///0xb8c - This register is used by the MCU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mc_apb2lpenclrr(&self) -> &MC_APB2LPENCLRR {
        &self.mc_apb2lpenclrr
    }
    ///0xb90 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_apb3lpensetr(&self) -> &MC_APB3LPENSETR {
        &self.mc_apb3lpensetr
    }
    ///0xb94 - This register is used by the MCU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mc_apb3lpenclrr(&self) -> &MC_APB3LPENCLRR {
        &self.mc_apb3lpenclrr
    }
    ///0xb98 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_ahb2lpensetr(&self) -> &MC_AHB2LPENSETR {
        &self.mc_ahb2lpensetr
    }
    ///0xb9c - This register is used by the MCU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mc_ahb2lpenclrr(&self) -> &MC_AHB2LPENCLRR {
        &self.mc_ahb2lpenclrr
    }
    ///0xba0 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_ahb3lpensetr(&self) -> &MC_AHB3LPENSETR {
        &self.mc_ahb3lpensetr
    }
    ///0xba4 - This register is used by the MCU in order to clear the PERxLPEN bit
    #[inline(always)]
    pub const fn mc_ahb3lpenclrr(&self) -> &MC_AHB3LPENCLRR {
        &self.mc_ahb3lpenclrr
    }
    ///0xba8 - This register is used by the MCU in order to set the PERxLPEN bit.
    #[inline(always)]
    pub const fn mc_ahb4lpensetr(&self) -> &MC_AHB4LPENSETR {
        &self.mc_ahb4lpensetr
    }
    ///0xbac - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mc_ahb4lpenclrr(&self) -> &MC_AHB4LPENCLRR {
        &self.mc_ahb4lpenclrr
    }
    ///0xbb0 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mc_aximlpensetr(&self) -> &MC_AXIMLPENSETR {
        &self.mc_aximlpensetr
    }
    ///0xbb4 - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mc_aximlpenclrr(&self) -> &MC_AXIMLPENCLRR {
        &self.mc_aximlpenclrr
    }
    ///0xbb8 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mc_mlahblpensetr(&self) -> &MC_MLAHBLPENSETR {
        &self.mc_mlahblpensetr
    }
    ///0xbbc - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    #[inline(always)]
    pub const fn mc_mlahblpenclrr(&self) -> &MC_MLAHBLPENCLRR {
        &self.mc_mlahblpenclrr
    }
    ///0xc00 - This register is used by the MCU to check the reset source.
    #[inline(always)]
    pub const fn mc_rstsclrr(&self) -> &MC_RSTSCLRR {
        &self.mc_rstsclrr
    }
    ///0xc14 - This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.
    #[inline(always)]
    pub const fn mc_cier(&self) -> &MC_CIER {
        &self.mc_cier
    }
    ///0xc18 - This register shall be used by the MCU in order to read and clear the interrupt flags.
    #[inline(always)]
    pub const fn mc_cifr(&self) -> &MC_CIFR {
        &self.mc_cifr
    }
    ///0xff4 - This register gives the IP version
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0xff8 - This register gives the unique identifier of the RCC
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0xffc - This register gives the decoding space, which is for the RCC of 4 kB.
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**TZCR (rw) register accessor: This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.

You can [`read`](crate::Reg::read) this register and get [`tzcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:TZCR)

For information about available fields see [`mod@tzcr`] module*/
pub type TZCR = crate::Reg<tzcr::TZCRrs>;
///This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.
pub mod tzcr;
/**OCENSETR (rw) register accessor: This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`ocensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:OCENSETR)

For information about available fields see [`mod@ocensetr`] module*/
pub type OCENSETR = crate::Reg<ocensetr::OCENSETRrs>;
///This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod ocensetr;
/**OCENCLRR (rw) register accessor: This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`ocenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:OCENCLRR)

For information about available fields see [`mod@ocenclrr`] module*/
pub type OCENCLRR = crate::Reg<ocenclrr::OCENCLRRrs>;
///This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod ocenclrr;
/**HSICFGR (rw) register accessor: This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:HSICFGR)

For information about available fields see [`mod@hsicfgr`] module*/
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGRrs>;
///This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod hsicfgr;
/**CSICFGR (rw) register accessor: This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`csicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:CSICFGR)

For information about available fields see [`mod@csicfgr`] module*/
pub type CSICFGR = crate::Reg<csicfgr::CSICFGRrs>;
///This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.
pub mod csicfgr;
/**MPCKSELR (rw) register accessor: This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`mpckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MPCKSELR)

For information about available fields see [`mod@mpckselr`] module*/
pub type MPCKSELR = crate::Reg<mpckselr::MPCKSELRrs>;
///This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod mpckselr;
/**ASSCKSELR (rw) register accessor: This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`assckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:ASSCKSELR)

For information about available fields see [`mod@assckselr`] module*/
pub type ASSCKSELR = crate::Reg<assckselr::ASSCKSELRrs>;
///This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod assckselr;
/**RCK12SELR (rw) register accessor: This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`rck12selr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rck12selr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCK12SELR)

For information about available fields see [`mod@rck12selr`] module*/
pub type RCK12SELR = crate::Reg<rck12selr::RCK12SELRrs>;
///This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rck12selr;
/**MPCKDIVR (rw) register accessor: This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mpckdivr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpckdivr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MPCKDIVR)

For information about available fields see [`mod@mpckdivr`] module*/
pub type MPCKDIVR = crate::Reg<mpckdivr::MPCKDIVRrs>;
///This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod mpckdivr;
/**AXIDIVR (rw) register accessor: This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`axidivr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axidivr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AXIDIVR)

For information about available fields see [`mod@axidivr`] module*/
pub type AXIDIVR = crate::Reg<axidivr::AXIDIVRrs>;
///This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod axidivr;
/**APB4DIVR (rw) register accessor: This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`apb4divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB4DIVR)

For information about available fields see [`mod@apb4divr`] module*/
pub type APB4DIVR = crate::Reg<apb4divr::APB4DIVRrs>;
///This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod apb4divr;
/**APB5DIVR (rw) register accessor: This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`apb5divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB5DIVR)

For information about available fields see [`mod@apb5divr`] module*/
pub type APB5DIVR = crate::Reg<apb5divr::APB5DIVRrs>;
///This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod apb5divr;
/**RTCDIVR (rw) register accessor: This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rtcdivr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcdivr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RTCDIVR)

For information about available fields see [`mod@rtcdivr`] module*/
pub type RTCDIVR = crate::Reg<rtcdivr::RTCDIVRrs>;
///This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.
pub mod rtcdivr;
/**MSSCKSELR (rw) register accessor: This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`mssckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mssckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MSSCKSELR)

For information about available fields see [`mod@mssckselr`] module*/
pub type MSSCKSELR = crate::Reg<mssckselr::MSSCKSELRrs>;
///This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod mssckselr;
/**PLL1CR (rw) register accessor: This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL1CR)

For information about available fields see [`mod@pll1cr`] module*/
pub type PLL1CR = crate::Reg<pll1cr::PLL1CRrs>;
///This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll1cr;
/**PLL1CFGR1 (rw) register accessor: This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL1CFGR1)

For information about available fields see [`mod@pll1cfgr1`] module*/
pub type PLL1CFGR1 = crate::Reg<pll1cfgr1::PLL1CFGR1rs>;
///This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll1cfgr1;
/**PLL1CFGR2 (rw) register accessor: This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL1CFGR2)

For information about available fields see [`mod@pll1cfgr2`] module*/
pub type PLL1CFGR2 = crate::Reg<pll1cfgr2::PLL1CFGR2rs>;
///This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll1cfgr2;
/**PLL1FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL1FRACR)

For information about available fields see [`mod@pll1fracr`] module*/
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACRrs>;
///This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll1fracr;
/**PLL1CSGR (rw) register accessor: This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1csgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1csgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL1CSGR)

For information about available fields see [`mod@pll1csgr`] module*/
pub type PLL1CSGR = crate::Reg<pll1csgr::PLL1CSGRrs>;
///This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll1csgr;
/**PLL2CR (rw) register accessor: This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL2CR)

For information about available fields see [`mod@pll2cr`] module*/
pub type PLL2CR = crate::Reg<pll2cr::PLL2CRrs>;
///This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll2cr;
/**PLL2CFGR1 (rw) register accessor: This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL2CFGR1)

For information about available fields see [`mod@pll2cfgr1`] module*/
pub type PLL2CFGR1 = crate::Reg<pll2cfgr1::PLL2CFGR1rs>;
///This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll2cfgr1;
/**PLL2CFGR2 (rw) register accessor: This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL2CFGR2)

For information about available fields see [`mod@pll2cfgr2`] module*/
pub type PLL2CFGR2 = crate::Reg<pll2cfgr2::PLL2CFGR2rs>;
///This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll2cfgr2;
/**PLL2FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL2FRACR)

For information about available fields see [`mod@pll2fracr`] module*/
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACRrs>;
///This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll2fracr;
/**PLL2CSGR (rw) register accessor: This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll2csgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2csgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL2CSGR)

For information about available fields see [`mod@pll2csgr`] module*/
pub type PLL2CSGR = crate::Reg<pll2csgr::PLL2CSGRrs>;
///This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod pll2csgr;
/**I2C46CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`i2c46ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c46ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:I2C46CKSELR)

For information about available fields see [`mod@i2c46ckselr`] module*/
pub type I2C46CKSELR = crate::Reg<i2c46ckselr::I2C46CKSELRrs>;
///This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod i2c46ckselr;
/**SPI6CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`spi6ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi6ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SPI6CKSELR)

For information about available fields see [`mod@spi6ckselr`] module*/
pub type SPI6CKSELR = crate::Reg<spi6ckselr::SPI6CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod spi6ckselr;
/**UART1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`uart1ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:UART1CKSELR)

For information about available fields see [`mod@uart1ckselr`] module*/
pub type UART1CKSELR = crate::Reg<uart1ckselr::UART1CKSELRrs>;
///This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod uart1ckselr;
/**RNG1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rng1ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng1ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RNG1CKSELR)

For information about available fields see [`mod@rng1ckselr`] module*/
pub type RNG1CKSELR = crate::Reg<rng1ckselr::RNG1CKSELRrs>;
///This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod rng1ckselr;
/**CPERCKSELR (rw) register accessor: This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`cperckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cperckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:CPERCKSELR)

For information about available fields see [`mod@cperckselr`] module*/
pub type CPERCKSELR = crate::Reg<cperckselr::CPERCKSELRrs>;
///This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.
pub mod cperckselr;
/**STGENCKSELR (rw) register accessor: This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`stgenckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:STGENCKSELR)

For information about available fields see [`mod@stgenckselr`] module*/
pub type STGENCKSELR = crate::Reg<stgenckselr::STGENCKSELRrs>;
///This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod stgenckselr;
/**DDRITFCR (rw) register accessor: This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`ddritfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddritfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:DDRITFCR)

For information about available fields see [`mod@ddritfcr`] module*/
pub type DDRITFCR = crate::Reg<ddritfcr::DDRITFCRrs>;
///This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.
pub mod ddritfcr;
/**MP_BOOTCR (rw) register accessor: This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.

You can [`read`](crate::Reg::read) this register and get [`mp_bootcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_bootcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_BOOTCR)

For information about available fields see [`mod@mp_bootcr`] module*/
pub type MP_BOOTCR = crate::Reg<mp_bootcr::MP_BOOTCRrs>;
///This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.
pub mod mp_bootcr;
/**MP_SREQSETR (rw) register accessor: Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_sreqsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_sreqsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_SREQSETR)

For information about available fields see [`mod@mp_sreqsetr`] module*/
pub type MP_SREQSETR = crate::Reg<mp_sreqsetr::MP_SREQSETRrs>;
///Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
pub mod mp_sreqsetr;
/**MP_SREQCLRR (rw) register accessor: Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_sreqclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_sreqclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_SREQCLRR)

For information about available fields see [`mod@mp_sreqclrr`] module*/
pub type MP_SREQCLRR = crate::Reg<mp_sreqclrr::MP_SREQCLRRrs>;
///Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
pub mod mp_sreqclrr;
/**MP_GCR (rw) register accessor: The register contains global control bits. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_GCR)

For information about available fields see [`mod@mp_gcr`] module*/
pub type MP_GCR = crate::Reg<mp_gcr::MP_GCRrs>;
///The register contains global control bits. If TZEN = , this register can only be modified in secure mode.
pub mod mp_gcr;
/**MP_APRSTCR (rw) register accessor: This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_aprstcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_aprstcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APRSTCR)

For information about available fields see [`mod@mp_aprstcr`] module*/
pub type MP_APRSTCR = crate::Reg<mp_aprstcr::MP_APRSTCRrs>;
///This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.
pub mod mp_aprstcr;
/**MP_APRSTSR (r) register accessor: This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_aprstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APRSTSR)

For information about available fields see [`mod@mp_aprstsr`] module*/
pub type MP_APRSTSR = crate::Reg<mp_aprstsr::MP_APRSTSRrs>;
///This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.
pub mod mp_aprstsr;
/**BDCR (rw) register accessor: This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:BDCR)

For information about available fields see [`mod@bdcr`] module*/
pub type BDCR = crate::Reg<bdcr::BDCRrs>;
///This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.
pub mod bdcr;
/**RDLSICR (rw) register accessor: This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rdlsicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdlsicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RDLSICR)

For information about available fields see [`mod@rdlsicr`] module*/
pub type RDLSICR = crate::Reg<rdlsicr::RDLSICRrs>;
///This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.
pub mod rdlsicr;
/**APB4RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb4rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB4RSTSETR)

For information about available fields see [`mod@apb4rstsetr`] module*/
pub type APB4RSTSETR = crate::Reg<apb4rstsetr::APB4RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
pub mod apb4rstsetr;
/**APB4RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb4rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB4RSTCLRR)

For information about available fields see [`mod@apb4rstclrr`] module*/
pub type APB4RSTCLRR = crate::Reg<apb4rstclrr::APB4RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
pub mod apb4rstclrr;
/**APB5RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`apb5rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB5RSTSETR)

For information about available fields see [`mod@apb5rstsetr`] module*/
pub type APB5RSTSETR = crate::Reg<apb5rstsetr::APB5RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod apb5rstsetr;
/**APB5RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`apb5rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB5RSTCLRR)

For information about available fields see [`mod@apb5rstclrr`] module*/
pub type APB5RSTCLRR = crate::Reg<apb5rstclrr::APB5RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod apb5rstclrr;
/**AHB5RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`ahb5rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB5RSTSETR)

For information about available fields see [`mod@ahb5rstsetr`] module*/
pub type AHB5RSTSETR = crate::Reg<ahb5rstsetr::AHB5RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod ahb5rstsetr;
/**AHB5RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`ahb5rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb5rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB5RSTCLRR)

For information about available fields see [`mod@ahb5rstclrr`] module*/
pub type AHB5RSTCLRR = crate::Reg<ahb5rstclrr::AHB5RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod ahb5rstclrr;
/**AHB6RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb6rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb6rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB6RSTSETR)

For information about available fields see [`mod@ahb6rstsetr`] module*/
pub type AHB6RSTSETR = crate::Reg<ahb6rstsetr::AHB6RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
pub mod ahb6rstsetr;
/**AHB6RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb6rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb6rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB6RSTCLRR)

For information about available fields see [`mod@ahb6rstclrr`] module*/
pub type AHB6RSTCLRR = crate::Reg<ahb6rstclrr::AHB6RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
pub mod ahb6rstclrr;
/**TZAHB6RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`tzahb6rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzahb6rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:TZAHB6RSTSETR)

For information about available fields see [`mod@tzahb6rstsetr`] module*/
pub type TZAHB6RSTSETR = crate::Reg<tzahb6rstsetr::TZAHB6RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod tzahb6rstsetr;
/**TZAHB6RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`tzahb6rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzahb6rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:TZAHB6RSTCLRR)

For information about available fields see [`mod@tzahb6rstclrr`] module*/
pub type TZAHB6RSTCLRR = crate::Reg<tzahb6rstclrr::TZAHB6RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod tzahb6rstclrr;
/**MP_APB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_apb4ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb4ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB4ENSETR)

For information about available fields see [`mod@mp_apb4ensetr`] module*/
pub type MP_APB4ENSETR = crate::Reg<mp_apb4ensetr::MP_APB4ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod mp_apb4ensetr;
/**MP_APB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_apb4enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb4enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB4ENCLRR)

For information about available fields see [`mod@mp_apb4enclrr`] module*/
pub type MP_APB4ENCLRR = crate::Reg<mp_apb4enclrr::MP_APB4ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod mp_apb4enclrr;
/**MP_APB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_apb5ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb5ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB5ENSETR)

For information about available fields see [`mod@mp_apb5ensetr`] module*/
pub type MP_APB5ENSETR = crate::Reg<mp_apb5ensetr::MP_APB5ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod mp_apb5ensetr;
/**MP_APB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_apb5enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb5enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB5ENCLRR)

For information about available fields see [`mod@mp_apb5enclrr`] module*/
pub type MP_APB5ENCLRR = crate::Reg<mp_apb5enclrr::MP_APB5ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod mp_apb5enclrr;
/**MP_AHB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb5ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb5ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB5ENSETR)

For information about available fields see [`mod@mp_ahb5ensetr`] module*/
pub type MP_AHB5ENSETR = crate::Reg<mp_ahb5ensetr::MP_AHB5ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod mp_ahb5ensetr;
/**MP_AHB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb5enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb5enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB5ENCLRR)

For information about available fields see [`mod@mp_ahb5enclrr`] module*/
pub type MP_AHB5ENCLRR = crate::Reg<mp_ahb5enclrr::MP_AHB5ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod mp_ahb5enclrr;
/**MP_AHB6ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_ahb6ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb6ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB6ENSETR)

For information about available fields see [`mod@mp_ahb6ensetr`] module*/
pub type MP_AHB6ENSETR = crate::Reg<mp_ahb6ensetr::MP_AHB6ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod mp_ahb6ensetr;
/**MP_AHB6ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_ahb6enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb6enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB6ENCLRR)

For information about available fields see [`mod@mp_ahb6enclrr`] module*/
pub type MP_AHB6ENCLRR = crate::Reg<mp_ahb6enclrr::MP_AHB6ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod mp_ahb6enclrr;
/**MP_TZAHB6ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_tzahb6ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_tzahb6ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_TZAHB6ENSETR)

For information about available fields see [`mod@mp_tzahb6ensetr`] module*/
pub type MP_TZAHB6ENSETR = crate::Reg<mp_tzahb6ensetr::MP_TZAHB6ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod mp_tzahb6ensetr;
/**MP_TZAHB6ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_tzahb6enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_tzahb6enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_TZAHB6ENCLRR)

For information about available fields see [`mod@mp_tzahb6enclrr`] module*/
pub type MP_TZAHB6ENCLRR = crate::Reg<mp_tzahb6enclrr::MP_TZAHB6ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod mp_tzahb6enclrr;
/**MC_APB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb4ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb4ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB4ENSETR)

For information about available fields see [`mod@mc_apb4ensetr`] module*/
pub type MC_APB4ENSETR = crate::Reg<mc_apb4ensetr::MC_APB4ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_apb4ensetr;
/**MC_APB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb4enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb4enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB4ENCLRR)

For information about available fields see [`mod@mc_apb4enclrr`] module*/
pub type MC_APB4ENCLRR = crate::Reg<mc_apb4enclrr::MC_APB4ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_apb4enclrr;
/**MC_APB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb5ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb5ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB5ENSETR)

For information about available fields see [`mod@mc_apb5ensetr`] module*/
pub type MC_APB5ENSETR = crate::Reg<mc_apb5ensetr::MC_APB5ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_apb5ensetr;
/**MC_APB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb5enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb5enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB5ENCLRR)

For information about available fields see [`mod@mc_apb5enclrr`] module*/
pub type MC_APB5ENCLRR = crate::Reg<mc_apb5enclrr::MC_APB5ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_apb5enclrr;
/**MC_AHB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb5ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb5ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB5ENSETR)

For information about available fields see [`mod@mc_ahb5ensetr`] module*/
pub type MC_AHB5ENSETR = crate::Reg<mc_ahb5ensetr::MC_AHB5ENSETRrs>;
///This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
pub mod mc_ahb5ensetr;
/**MC_AHB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb5enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb5enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB5ENCLRR)

For information about available fields see [`mod@mc_ahb5enclrr`] module*/
pub type MC_AHB5ENCLRR = crate::Reg<mc_ahb5enclrr::MC_AHB5ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
pub mod mc_ahb5enclrr;
/**MC_AHB6ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb6ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb6ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB6ENSETR)

For information about available fields see [`mod@mc_ahb6ensetr`] module*/
pub type MC_AHB6ENSETR = crate::Reg<mc_ahb6ensetr::MC_AHB6ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_ahb6ensetr;
/**MC_AHB6ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb6enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb6enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB6ENCLRR)

For information about available fields see [`mod@mc_ahb6enclrr`] module*/
pub type MC_AHB6ENCLRR = crate::Reg<mc_ahb6enclrr::MC_AHB6ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_ahb6enclrr;
/**MP_APB4LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb4lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb4lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB4LPENSETR)

For information about available fields see [`mod@mp_apb4lpensetr`] module*/
pub type MP_APB4LPENSETR = crate::Reg<mp_apb4lpensetr::MP_APB4LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_apb4lpensetr;
/**MP_APB4LPENCLRR (rw) register accessor: This register is used by the MCU

You can [`read`](crate::Reg::read) this register and get [`mp_apb4lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb4lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB4LPENCLRR)

For information about available fields see [`mod@mp_apb4lpenclrr`] module*/
pub type MP_APB4LPENCLRR = crate::Reg<mp_apb4lpenclrr::MP_APB4LPENCLRRrs>;
///This register is used by the MCU
pub mod mp_apb4lpenclrr;
/**MP_APB5LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_apb5lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb5lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB5LPENSETR)

For information about available fields see [`mod@mp_apb5lpensetr`] module*/
pub type MP_APB5LPENSETR = crate::Reg<mp_apb5lpensetr::MP_APB5LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod mp_apb5lpensetr;
/**MP_APB5LPENCLRR (rw) register accessor: This register is used by the Mpu.

You can [`read`](crate::Reg::read) this register and get [`mp_apb5lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb5lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB5LPENCLRR)

For information about available fields see [`mod@mp_apb5lpenclrr`] module*/
pub type MP_APB5LPENCLRR = crate::Reg<mp_apb5lpenclrr::MP_APB5LPENCLRRrs>;
///This register is used by the Mpu.
pub mod mp_apb5lpenclrr;
/**MP_AHB5LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb5lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb5lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB5LPENSETR)

For information about available fields see [`mod@mp_ahb5lpensetr`] module*/
pub type MP_AHB5LPENSETR = crate::Reg<mp_ahb5lpensetr::MP_AHB5LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod mp_ahb5lpensetr;
/**MP_AHB5LPENCLRR (rw) register accessor: This register is used by the MCU

You can [`read`](crate::Reg::read) this register and get [`mp_ahb5lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb5lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB5LPENCLRR)

For information about available fields see [`mod@mp_ahb5lpenclrr`] module*/
pub type MP_AHB5LPENCLRR = crate::Reg<mp_ahb5lpenclrr::MP_AHB5LPENCLRRrs>;
///This register is used by the MCU
pub mod mp_ahb5lpenclrr;
/**MP_AHB6LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_ahb6lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb6lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB6LPENSETR)

For information about available fields see [`mod@mp_ahb6lpensetr`] module*/
pub type MP_AHB6LPENSETR = crate::Reg<mp_ahb6lpensetr::MP_AHB6LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_ahb6lpensetr;
/**MP_AHB6LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_ahb6lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb6lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB6LPENCLRR)

For information about available fields see [`mod@mp_ahb6lpenclrr`] module*/
pub type MP_AHB6LPENCLRR = crate::Reg<mp_ahb6lpenclrr::MP_AHB6LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_ahb6lpenclrr;
/**MP_TZAHB6LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_tzahb6lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_tzahb6lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_TZAHB6LPENSETR)

For information about available fields see [`mod@mp_tzahb6lpensetr`] module*/
pub type MP_TZAHB6LPENSETR = crate::Reg<mp_tzahb6lpensetr::MP_TZAHB6LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod mp_tzahb6lpensetr;
/**MP_TZAHB6LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_tzahb6lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_tzahb6lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_TZAHB6LPENCLRR)

For information about available fields see [`mod@mp_tzahb6lpenclrr`] module*/
pub type MP_TZAHB6LPENCLRR = crate::Reg<mp_tzahb6lpenclrr::MP_TZAHB6LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod mp_tzahb6lpenclrr;
/**MC_APB4LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_apb4lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb4lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB4LPENSETR)

For information about available fields see [`mod@mc_apb4lpensetr`] module*/
pub type MC_APB4LPENSETR = crate::Reg<mc_apb4lpensetr::MC_APB4LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_apb4lpensetr;
/**MC_APB4LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb4lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb4lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB4LPENCLRR)

For information about available fields see [`mod@mc_apb4lpenclrr`] module*/
pub type MC_APB4LPENCLRR = crate::Reg<mc_apb4lpenclrr::MC_APB4LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod mc_apb4lpenclrr;
/**MC_APB5LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_apb5lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb5lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB5LPENSETR)

For information about available fields see [`mod@mc_apb5lpensetr`] module*/
pub type MC_APB5LPENSETR = crate::Reg<mc_apb5lpensetr::MC_APB5LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_apb5lpensetr;
/**MC_APB5LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb5lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb5lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB5LPENCLRR)

For information about available fields see [`mod@mc_apb5lpenclrr`] module*/
pub type MC_APB5LPENCLRR = crate::Reg<mc_apb5lpenclrr::MC_APB5LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod mc_apb5lpenclrr;
/**MC_AHB5LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb5lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb5lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB5LPENSETR)

For information about available fields see [`mod@mc_ahb5lpensetr`] module*/
pub type MC_AHB5LPENSETR = crate::Reg<mc_ahb5lpensetr::MC_AHB5LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.
pub mod mc_ahb5lpensetr;
/**MC_AHB5LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb5lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb5lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB5LPENCLRR)

For information about available fields see [`mod@mc_ahb5lpenclrr`] module*/
pub type MC_AHB5LPENCLRR = crate::Reg<mc_ahb5lpenclrr::MC_AHB5LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.
pub mod mc_ahb5lpenclrr;
/**MC_AHB6LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb6lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb6lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB6LPENSETR)

For information about available fields see [`mod@mc_ahb6lpensetr`] module*/
pub type MC_AHB6LPENSETR = crate::Reg<mc_ahb6lpensetr::MC_AHB6LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_ahb6lpensetr;
/**MC_AHB6LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb6lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb6lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB6LPENCLRR)

For information about available fields see [`mod@mc_ahb6lpenclrr`] module*/
pub type MC_AHB6LPENCLRR = crate::Reg<mc_ahb6lpenclrr::MC_AHB6LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod mc_ahb6lpenclrr;
/**BR_RSTSCLRR (rw) register accessor: This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`br_rstsclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br_rstsclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:BR_RSTSCLRR)

For information about available fields see [`mod@br_rstsclrr`] module*/
pub type BR_RSTSCLRR = crate::Reg<br_rstsclrr::BR_RSTSCLRRrs>;
///This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.
pub mod br_rstsclrr;
/**MP_GRSTCSETR (rw) register accessor: This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.

You can [`read`](crate::Reg::read) this register and get [`mp_grstcsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_grstcsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_GRSTCSETR)

For information about available fields see [`mod@mp_grstcsetr`] module*/
pub type MP_GRSTCSETR = crate::Reg<mp_grstcsetr::MP_GRSTCSETRrs>;
///This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.
pub mod mp_grstcsetr;
/**MP_RSTSCLRR (rw) register accessor: This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_rstsclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_rstsclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_RSTSCLRR)

For information about available fields see [`mod@mp_rstsclrr`] module*/
pub type MP_RSTSCLRR = crate::Reg<mp_rstsclrr::MP_RSTSCLRRrs>;
///This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
pub mod mp_rstsclrr;
/**MP_IWDGFZSETR (rw) register accessor: This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_iwdgfzsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_iwdgfzsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_IWDGFZSETR)

For information about available fields see [`mod@mp_iwdgfzsetr`] module*/
pub type MP_IWDGFZSETR = crate::Reg<mp_iwdgfzsetr::MP_IWDGFZSETRrs>;
///This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod mp_iwdgfzsetr;
/**MP_IWDGFZCLRR (rw) register accessor: This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_iwdgfzclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_iwdgfzclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_IWDGFZCLRR)

For information about available fields see [`mod@mp_iwdgfzclrr`] module*/
pub type MP_IWDGFZCLRR = crate::Reg<mp_iwdgfzclrr::MP_IWDGFZCLRRrs>;
///This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod mp_iwdgfzclrr;
/**MP_CIER (rw) register accessor: This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_CIER)

For information about available fields see [`mod@mp_cier`] module*/
pub type MP_CIER = crate::Reg<mp_cier::MP_CIERrs>;
///This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
pub mod mp_cier;
/**MP_CIFR (rw) register accessor: This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_cifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_cifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_CIFR)

For information about available fields see [`mod@mp_cifr`] module*/
pub type MP_CIFR = crate::Reg<mp_cifr::MP_CIFRrs>;
///This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
pub mod mp_cifr;
/**PWRLPDLYCR (rw) register accessor: This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pwrlpdlycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrlpdlycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PWRLPDLYCR)

For information about available fields see [`mod@pwrlpdlycr`] module*/
pub type PWRLPDLYCR = crate::Reg<pwrlpdlycr::PWRLPDLYCRrs>;
///This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.
pub mod pwrlpdlycr;
/**MP_RSTSSETR (rw) register accessor: This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_rstssetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_rstssetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_RSTSSETR)

For information about available fields see [`mod@mp_rstssetr`] module*/
pub type MP_RSTSSETR = crate::Reg<mp_rstssetr::MP_RSTSSETRrs>;
///This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
pub mod mp_rstssetr;
/**MCO1CFGR (rw) register accessor: This register is used to select the clock generated on MCO1 output.

You can [`read`](crate::Reg::read) this register and get [`mco1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MCO1CFGR)

For information about available fields see [`mod@mco1cfgr`] module*/
pub type MCO1CFGR = crate::Reg<mco1cfgr::MCO1CFGRrs>;
///This register is used to select the clock generated on MCO1 output.
pub mod mco1cfgr;
/**MCO2CFGR (rw) register accessor: This register is used to select the clock generated on MCO2 output.

You can [`read`](crate::Reg::read) this register and get [`mco2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MCO2CFGR)

For information about available fields see [`mod@mco2cfgr`] module*/
pub type MCO2CFGR = crate::Reg<mco2cfgr::MCO2CFGRrs>;
///This register is used to select the clock generated on MCO2 output.
pub mod mco2cfgr;
/**OCRDYR (r) register accessor: This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.

You can [`read`](crate::Reg::read) this register and get [`ocrdyr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:OCRDYR)

For information about available fields see [`mod@ocrdyr`] module*/
pub type OCRDYR = crate::Reg<ocrdyr::OCRDYRrs>;
///This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.
pub mod ocrdyr;
/**DBGCFGR (rw) register accessor: This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.

You can [`read`](crate::Reg::read) this register and get [`dbgcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:DBGCFGR)

For information about available fields see [`mod@dbgcfgr`] module*/
pub type DBGCFGR = crate::Reg<dbgcfgr::DBGCFGRrs>;
///This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.
pub mod dbgcfgr;
/**RCK3SELR (rw) register accessor: This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rck3selr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rck3selr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCK3SELR)

For information about available fields see [`mod@rck3selr`] module*/
pub type RCK3SELR = crate::Reg<rck3selr::RCK3SELRrs>;
///This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rck3selr;
/**RCK4SELR (rw) register accessor: This register is used to select the reference clock for PLL4.

You can [`read`](crate::Reg::read) this register and get [`rck4selr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rck4selr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCK4SELR)

For information about available fields see [`mod@rck4selr`] module*/
pub type RCK4SELR = crate::Reg<rck4selr::RCK4SELRrs>;
///This register is used to select the reference clock for PLL4.
pub mod rck4selr;
/**TIMG1PRER (rw) register accessor: This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`timg1prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg1prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:TIMG1PRER)

For information about available fields see [`mod@timg1prer`] module*/
pub type TIMG1PRER = crate::Reg<timg1prer::TIMG1PRERrs>;
///This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.
pub mod timg1prer;
/**TIMG2PRER (rw) register accessor: This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`timg2prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg2prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:TIMG2PRER)

For information about available fields see [`mod@timg2prer`] module*/
pub type TIMG2PRER = crate::Reg<timg2prer::TIMG2PRERrs>;
///This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.
pub mod timg2prer;
/**MCUDIVR (rw) register accessor: This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mcudivr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcudivr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MCUDIVR)

For information about available fields see [`mod@mcudivr`] module*/
pub type MCUDIVR = crate::Reg<mcudivr::MCUDIVRrs>;
///This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod mcudivr;
/**APB1DIVR (rw) register accessor: This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`apb1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB1DIVR)

For information about available fields see [`mod@apb1divr`] module*/
pub type APB1DIVR = crate::Reg<apb1divr::APB1DIVRrs>;
///This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.
pub mod apb1divr;
/**APB2DIVR (rw) register accessor: This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`apb2divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB2DIVR)

For information about available fields see [`mod@apb2divr`] module*/
pub type APB2DIVR = crate::Reg<apb2divr::APB2DIVRrs>;
///This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
pub mod apb2divr;
/**APB3DIVR (rw) register accessor: This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`apb3divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB3DIVR)

For information about available fields see [`mod@apb3divr`] module*/
pub type APB3DIVR = crate::Reg<apb3divr::APB3DIVRrs>;
///This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
pub mod apb3divr;
/**PLL3CR (rw) register accessor: This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL3CR)

For information about available fields see [`mod@pll3cr`] module*/
pub type PLL3CR = crate::Reg<pll3cr::PLL3CRrs>;
///This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod pll3cr;
/**PLL3CFGR1 (rw) register accessor: This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL3CFGR1)

For information about available fields see [`mod@pll3cfgr1`] module*/
pub type PLL3CFGR1 = crate::Reg<pll3cfgr1::PLL3CFGR1rs>;
///This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod pll3cfgr1;
/**PLL3CFGR2 (rw) register accessor: This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL3CFGR2)

For information about available fields see [`mod@pll3cfgr2`] module*/
pub type PLL3CFGR2 = crate::Reg<pll3cfgr2::PLL3CFGR2rs>;
///This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod pll3cfgr2;
/**PLL3FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL3FRACR)

For information about available fields see [`mod@pll3fracr`] module*/
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACRrs>;
///This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod pll3fracr;
/**PLL3CSGR (rw) register accessor: This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll3csgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3csgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL3CSGR)

For information about available fields see [`mod@pll3csgr`] module*/
pub type PLL3CSGR = crate::Reg<pll3csgr::PLL3CSGRrs>;
///This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod pll3csgr;
/**PLL4CR (rw) register accessor: This register is used to control the PLL4.

You can [`read`](crate::Reg::read) this register and get [`pll4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL4CR)

For information about available fields see [`mod@pll4cr`] module*/
pub type PLL4CR = crate::Reg<pll4cr::PLL4CRrs>;
///This register is used to control the PLL4.
pub mod pll4cr;
/**PLL4CFGR1 (rw) register accessor: This register is used to configure the PLL4.

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL4CFGR1)

For information about available fields see [`mod@pll4cfgr1`] module*/
pub type PLL4CFGR1 = crate::Reg<pll4cfgr1::PLL4CFGR1rs>;
///This register is used to configure the PLL4.
pub mod pll4cfgr1;
/**PLL4CFGR2 (rw) register accessor: This register is used to configure the PLL4.

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL4CFGR2)

For information about available fields see [`mod@pll4cfgr2`] module*/
pub type PLL4CFGR2 = crate::Reg<pll4cfgr2::PLL4CFGR2rs>;
///This register is used to configure the PLL4.
pub mod pll4cfgr2;
/**PLL4FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL4 VCO.

You can [`read`](crate::Reg::read) this register and get [`pll4fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL4FRACR)

For information about available fields see [`mod@pll4fracr`] module*/
pub type PLL4FRACR = crate::Reg<pll4fracr::PLL4FRACRrs>;
///This register is used to fine-tune the frequency of the PLL4 VCO.
pub mod pll4fracr;
/**PLL4CSGR (rw) register accessor: This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll4csgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4csgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL4CSGR)

For information about available fields see [`mod@pll4csgr`] module*/
pub type PLL4CSGR = crate::Reg<pll4csgr::PLL4CSGRrs>;
///This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod pll4csgr;
/**I2C12CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`i2c12ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c12ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:I2C12CKSELR)

For information about available fields see [`mod@i2c12ckselr`] module*/
pub type I2C12CKSELR = crate::Reg<i2c12ckselr::I2C12CKSELRrs>;
///This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod i2c12ckselr;
/**I2C35CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`i2c35ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c35ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:I2C35CKSELR)

For information about available fields see [`mod@i2c35ckselr`] module*/
pub type I2C35CKSELR = crate::Reg<i2c35ckselr::I2C35CKSELRrs>;
///This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod i2c35ckselr;
/**SAI1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai1ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai1ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SAI1CKSELR)

For information about available fields see [`mod@sai1ckselr`] module*/
pub type SAI1CKSELR = crate::Reg<sai1ckselr::SAI1CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod sai1ckselr;
/**SAI2CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai2ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai2ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SAI2CKSELR)

For information about available fields see [`mod@sai2ckselr`] module*/
pub type SAI2CKSELR = crate::Reg<sai2ckselr::SAI2CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod sai2ckselr;
/**SAI3CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai3ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai3ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SAI3CKSELR)

For information about available fields see [`mod@sai3ckselr`] module*/
pub type SAI3CKSELR = crate::Reg<sai3ckselr::SAI3CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod sai3ckselr;
/**SAI4CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sai4ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai4ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SAI4CKSELR)

For information about available fields see [`mod@sai4ckselr`] module*/
pub type SAI4CKSELR = crate::Reg<sai4ckselr::SAI4CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod sai4ckselr;
/**SPI2S1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`spi2s1ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s1ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SPI2S1CKSELR)

For information about available fields see [`mod@spi2s1ckselr`] module*/
pub type SPI2S1CKSELR = crate::Reg<spi2s1ckselr::SPI2S1CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod spi2s1ckselr;
/**SPI2S23CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`spi2s23ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s23ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SPI2S23CKSELR)

For information about available fields see [`mod@spi2s23ckselr`] module*/
pub type SPI2S23CKSELR = crate::Reg<spi2s23ckselr::SPI2S23CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod spi2s23ckselr;
/**SPI45CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`spi45ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi45ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SPI45CKSELR)

For information about available fields see [`mod@spi45ckselr`] module*/
pub type SPI45CKSELR = crate::Reg<spi45ckselr::SPI45CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod spi45ckselr;
/**UART6CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`uart6ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart6ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:UART6CKSELR)

For information about available fields see [`mod@uart6ckselr`] module*/
pub type UART6CKSELR = crate::Reg<uart6ckselr::UART6CKSELRrs>;
///This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod uart6ckselr;
/**UART24CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`uart24ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart24ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:UART24CKSELR)

For information about available fields see [`mod@uart24ckselr`] module*/
pub type UART24CKSELR = crate::Reg<uart24ckselr::UART24CKSELRrs>;
///This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod uart24ckselr;
/**UART35CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`uart35ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart35ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:UART35CKSELR)

For information about available fields see [`mod@uart35ckselr`] module*/
pub type UART35CKSELR = crate::Reg<uart35ckselr::UART35CKSELRrs>;
///This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod uart35ckselr;
/**UART78CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`uart78ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart78ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:UART78CKSELR)

For information about available fields see [`mod@uart78ckselr`] module*/
pub type UART78CKSELR = crate::Reg<uart78ckselr::UART78CKSELRrs>;
///This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod uart78ckselr;
/**SDMMC12CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sdmmc12ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc12ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SDMMC12CKSELR)

For information about available fields see [`mod@sdmmc12ckselr`] module*/
pub type SDMMC12CKSELR = crate::Reg<sdmmc12ckselr::SDMMC12CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod sdmmc12ckselr;
/**SDMMC3CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`sdmmc3ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc3ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SDMMC3CKSELR)

For information about available fields see [`mod@sdmmc3ckselr`] module*/
pub type SDMMC3CKSELR = crate::Reg<sdmmc3ckselr::SDMMC3CKSELRrs>;
///This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod sdmmc3ckselr;
/**ETHCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`ethckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ethckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:ETHCKSELR)

For information about available fields see [`mod@ethckselr`] module*/
pub type ETHCKSELR = crate::Reg<ethckselr::ETHCKSELRrs>;
///This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod ethckselr;
/**QSPICKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`qspickselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspickselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:QSPICKSELR)

For information about available fields see [`mod@qspickselr`] module*/
pub type QSPICKSELR = crate::Reg<qspickselr::QSPICKSELRrs>;
///This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod qspickselr;
/**FMCCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`fmcckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:FMCCKSELR)

For information about available fields see [`mod@fmcckselr`] module*/
pub type FMCCKSELR = crate::Reg<fmcckselr::FMCCKSELRrs>;
///This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod fmcckselr;
/**FDCANCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`fdcanckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcanckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:FDCANCKSELR)

For information about available fields see [`mod@fdcanckselr`] module*/
pub type FDCANCKSELR = crate::Reg<fdcanckselr::FDCANCKSELRrs>;
///This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod fdcanckselr;
/**SPDIFCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`spdifckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdifckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SPDIFCKSELR)

For information about available fields see [`mod@spdifckselr`] module*/
pub type SPDIFCKSELR = crate::Reg<spdifckselr::SPDIFCKSELRrs>;
///This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod spdifckselr;
/**CECCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the CEC-HDMI.

You can [`read`](crate::Reg::read) this register and get [`cecckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:CECCKSELR)

For information about available fields see [`mod@cecckselr`] module*/
pub type CECCKSELR = crate::Reg<cecckselr::CECCKSELRrs>;
///This register is used to control the selection of the kernel clock for the CEC-HDMI.
pub mod cecckselr;
/**USBCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG

You can [`read`](crate::Reg::read) this register and get [`usbckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:USBCKSELR)

For information about available fields see [`mod@usbckselr`] module*/
pub type USBCKSELR = crate::Reg<usbckselr::USBCKSELRrs>;
///This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG
pub mod usbckselr;
/**RNG2CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the RNG2.

You can [`read`](crate::Reg::read) this register and get [`rng2ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng2ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RNG2CKSELR)

For information about available fields see [`mod@rng2ckselr`] module*/
pub type RNG2CKSELR = crate::Reg<rng2ckselr::RNG2CKSELRrs>;
///This register is used to control the selection of the kernel clock for the RNG2.
pub mod rng2ckselr;
/**DSICKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the DSI block.

You can [`read`](crate::Reg::read) this register and get [`dsickselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsickselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:DSICKSELR)

For information about available fields see [`mod@dsickselr`] module*/
pub type DSICKSELR = crate::Reg<dsickselr::DSICKSELRrs>;
///This register is used to control the selection of the kernel clock for the DSI block.
pub mod dsickselr;
/**ADCCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the ADC block.

You can [`read`](crate::Reg::read) this register and get [`adcckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:ADCCKSELR)

For information about available fields see [`mod@adcckselr`] module*/
pub type ADCCKSELR = crate::Reg<adcckselr::ADCCKSELRrs>;
///This register is used to control the selection of the kernel clock for the ADC block.
pub mod adcckselr;
/**LPTIM45CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.

You can [`read`](crate::Reg::read) this register and get [`lptim45ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim45ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:LPTIM45CKSELR)

For information about available fields see [`mod@lptim45ckselr`] module*/
pub type LPTIM45CKSELR = crate::Reg<lptim45ckselr::LPTIM45CKSELRrs>;
///This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.
pub mod lptim45ckselr;
/**LPTIM23CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.

You can [`read`](crate::Reg::read) this register and get [`lptim23ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim23ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:LPTIM23CKSELR)

For information about available fields see [`mod@lptim23ckselr`] module*/
pub type LPTIM23CKSELR = crate::Reg<lptim23ckselr::LPTIM23CKSELRrs>;
///This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.
pub mod lptim23ckselr;
/**LPTIM1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the LPTIM1 block.

You can [`read`](crate::Reg::read) this register and get [`lptim1ckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1ckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:LPTIM1CKSELR)

For information about available fields see [`mod@lptim1ckselr`] module*/
pub type LPTIM1CKSELR = crate::Reg<lptim1ckselr::LPTIM1CKSELRrs>;
///This register is used to control the selection of the kernel clock for the LPTIM1 block.
pub mod lptim1ckselr;
/**APB1RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb1rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB1RSTSETR)

For information about available fields see [`mod@apb1rstsetr`] module*/
pub type APB1RSTSETR = crate::Reg<apb1rstsetr::APB1RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod apb1rstsetr;
/**APB1RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb1rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB1RSTCLRR)

For information about available fields see [`mod@apb1rstclrr`] module*/
pub type APB1RSTCLRR = crate::Reg<apb1rstclrr::APB1RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral.
pub mod apb1rstclrr;
/**APB2RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb2rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB2RSTSETR)

For information about available fields see [`mod@apb2rstsetr`] module*/
pub type APB2RSTSETR = crate::Reg<apb2rstsetr::APB2RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod apb2rstsetr;
/**APB2RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb2rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB2RSTCLRR)

For information about available fields see [`mod@apb2rstclrr`] module*/
pub type APB2RSTCLRR = crate::Reg<apb2rstclrr::APB2RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral.
pub mod apb2rstclrr;
/**APB3RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb3rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB3RSTSETR)

For information about available fields see [`mod@apb3rstsetr`] module*/
pub type APB3RSTSETR = crate::Reg<apb3rstsetr::APB3RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod apb3rstsetr;
/**APB3RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb3rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB3RSTCLRR)

For information about available fields see [`mod@apb3rstclrr`] module*/
pub type APB3RSTCLRR = crate::Reg<apb3rstclrr::APB3RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral.
pub mod apb3rstclrr;
/**AHB2RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb2rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB2RSTSETR)

For information about available fields see [`mod@ahb2rstsetr`] module*/
pub type AHB2RSTSETR = crate::Reg<ahb2rstsetr::AHB2RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod ahb2rstsetr;
/**AHB2RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb2rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB2RSTCLRR)

For information about available fields see [`mod@ahb2rstclrr`] module*/
pub type AHB2RSTCLRR = crate::Reg<ahb2rstclrr::AHB2RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral.
pub mod ahb2rstclrr;
/**AHB3RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb3rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB3RSTSETR)

For information about available fields see [`mod@ahb3rstsetr`] module*/
pub type AHB3RSTSETR = crate::Reg<ahb3rstsetr::AHB3RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod ahb3rstsetr;
/**AHB3RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb3rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB3RSTCLRR)

For information about available fields see [`mod@ahb3rstclrr`] module*/
pub type AHB3RSTCLRR = crate::Reg<ahb3rstclrr::AHB3RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral.
pub mod ahb3rstclrr;
/**AHB4RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral

You can [`read`](crate::Reg::read) this register and get [`ahb4rstsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB4RSTSETR)

For information about available fields see [`mod@ahb4rstsetr`] module*/
pub type AHB4RSTSETR = crate::Reg<ahb4rstsetr::AHB4RSTSETRrs>;
///This register is used to activate the reset of the corresponding peripheral
pub mod ahb4rstsetr;
/**AHB4RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb4rstclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB4RSTCLRR)

For information about available fields see [`mod@ahb4rstclrr`] module*/
pub type AHB4RSTCLRR = crate::Reg<ahb4rstclrr::AHB4RSTCLRRrs>;
///This register is used to release the reset of the corresponding peripheral.
pub mod ahb4rstclrr;
/**MP_APB1ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mp_apb1ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb1ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB1ENSETR)

For information about available fields see [`mod@mp_apb1ensetr`] module*/
pub type MP_APB1ENSETR = crate::Reg<mp_apb1ensetr::MP_APB1ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mp_apb1ensetr;
/**MP_APB1ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mp_apb1enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb1enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB1ENCLRR)

For information about available fields see [`mod@mp_apb1enclrr`] module*/
pub type MP_APB1ENCLRR = crate::Reg<mp_apb1enclrr::MP_APB1ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mp_apb1enclrr;
/**MP_APB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mp_apb2ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb2ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB2ENSETR)

For information about available fields see [`mod@mp_apb2ensetr`] module*/
pub type MP_APB2ENSETR = crate::Reg<mp_apb2ensetr::MP_APB2ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mp_apb2ensetr;
/**MP_APB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mp_apb2enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb2enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB2ENCLRR)

For information about available fields see [`mod@mp_apb2enclrr`] module*/
pub type MP_APB2ENCLRR = crate::Reg<mp_apb2enclrr::MP_APB2ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod mp_apb2enclrr;
/**MP_APB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mp_apb3ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb3ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB3ENSETR)

For information about available fields see [`mod@mp_apb3ensetr`] module*/
pub type MP_APB3ENSETR = crate::Reg<mp_apb3ensetr::MP_APB3ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mp_apb3ensetr;
/**MP_APB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mp_apb3enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb3enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB3ENCLRR)

For information about available fields see [`mod@mp_apb3enclrr`] module*/
pub type MP_APB3ENCLRR = crate::Reg<mp_apb3enclrr::MP_APB3ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod mp_apb3enclrr;
/**MP_AHB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral

You can [`read`](crate::Reg::read) this register and get [`mp_ahb2ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb2ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB2ENSETR)

For information about available fields see [`mod@mp_ahb2ensetr`] module*/
pub type MP_AHB2ENSETR = crate::Reg<mp_ahb2ensetr::MP_AHB2ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral
pub mod mp_ahb2ensetr;
/**MP_AHB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb2enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb2enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB2ENCLRR)

For information about available fields see [`mod@mp_ahb2enclrr`] module*/
pub type MP_AHB2ENCLRR = crate::Reg<mp_ahb2enclrr::MP_AHB2ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod mp_ahb2enclrr;
/**MP_AHB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral

You can [`read`](crate::Reg::read) this register and get [`mp_ahb3ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb3ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB3ENSETR)

For information about available fields see [`mod@mp_ahb3ensetr`] module*/
pub type MP_AHB3ENSETR = crate::Reg<mp_ahb3ensetr::MP_AHB3ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral
pub mod mp_ahb3ensetr;
/**MP_AHB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb3enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb3enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB3ENCLRR)

For information about available fields see [`mod@mp_ahb3enclrr`] module*/
pub type MP_AHB3ENCLRR = crate::Reg<mp_ahb3enclrr::MP_AHB3ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod mp_ahb3enclrr;
/**MP_AHB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb4ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb4ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB4ENSETR)

For information about available fields see [`mod@mp_ahb4ensetr`] module*/
pub type MP_AHB4ENSETR = crate::Reg<mp_ahb4ensetr::MP_AHB4ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.
pub mod mp_ahb4ensetr;
/**MP_AHB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mp_ahb4enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb4enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB4ENCLRR)

For information about available fields see [`mod@mp_ahb4enclrr`] module*/
pub type MP_AHB4ENCLRR = crate::Reg<mp_ahb4enclrr::MP_AHB4ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mp_ahb4enclrr;
/**MP_MLAHBENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mp_mlahbensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_mlahbensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_MLAHBENSETR)

For information about available fields see [`mod@mp_mlahbensetr`] module*/
pub type MP_MLAHBENSETR = crate::Reg<mp_mlahbensetr::MP_MLAHBENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mp_mlahbensetr;
/**MP_MLAHBENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit.

You can [`read`](crate::Reg::read) this register and get [`mp_mlahbenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_mlahbenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_MLAHBENCLRR)

For information about available fields see [`mod@mp_mlahbenclrr`] module*/
pub type MP_MLAHBENCLRR = crate::Reg<mp_mlahbenclrr::MP_MLAHBENCLRRrs>;
///This register is used to clear the peripheral clock enable bit.
pub mod mp_mlahbenclrr;
/**MC_APB1ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mc_apb1ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb1ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB1ENSETR)

For information about available fields see [`mod@mc_apb1ensetr`] module*/
pub type MC_APB1ENSETR = crate::Reg<mc_apb1ensetr::MC_APB1ENSETRrs>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .
pub mod mc_apb1ensetr;
/**MC_APB1ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_apb1enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb1enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB1ENCLRR)

For information about available fields see [`mod@mc_apb1enclrr`] module*/
pub type MC_APB1ENCLRR = crate::Reg<mc_apb1enclrr::MC_APB1ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod mc_apb1enclrr;
/**MC_APB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb2ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb2ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB2ENSETR)

For information about available fields see [`mod@mc_apb2ensetr`] module*/
pub type MC_APB2ENSETR = crate::Reg<mc_apb2ensetr::MC_APB2ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_apb2ensetr;
/**MC_APB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb2enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb2enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB2ENCLRR)

For information about available fields see [`mod@mc_apb2enclrr`] module*/
pub type MC_APB2ENCLRR = crate::Reg<mc_apb2enclrr::MC_APB2ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_apb2enclrr;
/**MC_APB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb3ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb3ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB3ENSETR)

For information about available fields see [`mod@mc_apb3ensetr`] module*/
pub type MC_APB3ENSETR = crate::Reg<mc_apb3ensetr::MC_APB3ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_apb3ensetr;
/**MC_APB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb3enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb3enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB3ENCLRR)

For information about available fields see [`mod@mc_apb3enclrr`] module*/
pub type MC_APB3ENCLRR = crate::Reg<mc_apb3enclrr::MC_APB3ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_apb3enclrr;
/**MC_AHB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb2ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb2ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB2ENSETR)

For information about available fields see [`mod@mc_ahb2ensetr`] module*/
pub type MC_AHB2ENSETR = crate::Reg<mc_ahb2ensetr::MC_AHB2ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_ahb2ensetr;
/**MC_AHB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb2enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb2enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB2ENCLRR)

For information about available fields see [`mod@mc_ahb2enclrr`] module*/
pub type MC_AHB2ENCLRR = crate::Reg<mc_ahb2enclrr::MC_AHB2ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_ahb2enclrr;
/**MC_AHB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb3ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb3ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB3ENSETR)

For information about available fields see [`mod@mc_ahb3ensetr`] module*/
pub type MC_AHB3ENSETR = crate::Reg<mc_ahb3ensetr::MC_AHB3ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_ahb3ensetr;
/**MC_AHB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb3enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb3enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB3ENCLRR)

For information about available fields see [`mod@mc_ahb3enclrr`] module*/
pub type MC_AHB3ENCLRR = crate::Reg<mc_ahb3enclrr::MC_AHB3ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_ahb3enclrr;
/**MC_AHB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb4ensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb4ensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB4ENSETR)

For information about available fields see [`mod@mc_ahb4ensetr`] module*/
pub type MC_AHB4ENSETR = crate::Reg<mc_ahb4ensetr::MC_AHB4ENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_ahb4ensetr;
/**MC_AHB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb4enclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb4enclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB4ENCLRR)

For information about available fields see [`mod@mc_ahb4enclrr`] module*/
pub type MC_AHB4ENCLRR = crate::Reg<mc_ahb4enclrr::MC_AHB4ENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_ahb4enclrr;
/**MC_AXIMENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_aximensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_aximensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AXIMENSETR)

For information about available fields see [`mod@mc_aximensetr`] module*/
pub type MC_AXIMENSETR = crate::Reg<mc_aximensetr::MC_AXIMENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_aximensetr;
/**MC_AXIMENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_aximenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_aximenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AXIMENCLRR)

For information about available fields see [`mod@mc_aximenclrr`] module*/
pub type MC_AXIMENCLRR = crate::Reg<mc_aximenclrr::MC_AXIMENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_aximenclrr;
/**MC_MLAHBENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_mlahbensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_mlahbensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_MLAHBENSETR)

For information about available fields see [`mod@mc_mlahbensetr`] module*/
pub type MC_MLAHBENSETR = crate::Reg<mc_mlahbensetr::MC_MLAHBENSETRrs>;
///This register is used to set the peripheral clock enable bit
pub mod mc_mlahbensetr;
/**MC_MLAHBENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_mlahbenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_mlahbenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_MLAHBENCLRR)

For information about available fields see [`mod@mc_mlahbenclrr`] module*/
pub type MC_MLAHBENCLRR = crate::Reg<mc_mlahbenclrr::MC_MLAHBENCLRRrs>;
///This register is used to clear the peripheral clock enable bit
pub mod mc_mlahbenclrr;
/**MP_APB1LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb1lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb1lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB1LPENSETR)

For information about available fields see [`mod@mp_apb1lpensetr`] module*/
pub type MP_APB1LPENSETR = crate::Reg<mp_apb1lpensetr::MP_APB1LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_apb1lpensetr;
/**MP_APB1LPENCLRR (rw) register accessor: This register is used by the MPU in order to clear the PERxLPEN bits .

You can [`read`](crate::Reg::read) this register and get [`mp_apb1lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb1lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB1LPENCLRR)

For information about available fields see [`mod@mp_apb1lpenclrr`] module*/
pub type MP_APB1LPENCLRR = crate::Reg<mp_apb1lpenclrr::MP_APB1LPENCLRRrs>;
///This register is used by the MPU in order to clear the PERxLPEN bits .
pub mod mp_apb1lpenclrr;
/**MP_APB2LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb2lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb2lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB2LPENSETR)

For information about available fields see [`mod@mp_apb2lpensetr`] module*/
pub type MP_APB2LPENSETR = crate::Reg<mp_apb2lpensetr::MP_APB2LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_apb2lpensetr;
/**MP_APB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb2lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb2lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB2LPENCLRR)

For information about available fields see [`mod@mp_apb2lpenclrr`] module*/
pub type MP_APB2LPENCLRR = crate::Reg<mp_apb2lpenclrr::MP_APB2LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_apb2lpenclrr;
/**MP_APB3LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb3lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb3lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB3LPENSETR)

For information about available fields see [`mod@mp_apb3lpensetr`] module*/
pub type MP_APB3LPENSETR = crate::Reg<mp_apb3lpensetr::MP_APB3LPENSETRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_apb3lpensetr;
/**MP_APB3LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb3lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb3lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_APB3LPENCLRR)

For information about available fields see [`mod@mp_apb3lpenclrr`] module*/
pub type MP_APB3LPENCLRR = crate::Reg<mp_apb3lpenclrr::MP_APB3LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_apb3lpenclrr;
/**MP_AHB2LPENSETR (rw) register accessor: This register is used by the MPU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb2lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb2lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB2LPENSETR)

For information about available fields see [`mod@mp_ahb2lpensetr`] module*/
pub type MP_AHB2LPENSETR = crate::Reg<mp_ahb2lpensetr::MP_AHB2LPENSETRrs>;
///This register is used by the MPU in order to set the PERxLPEN bit.
pub mod mp_ahb2lpensetr;
/**MP_AHB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_ahb2lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb2lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB2LPENCLRR)

For information about available fields see [`mod@mp_ahb2lpenclrr`] module*/
pub type MP_AHB2LPENCLRR = crate::Reg<mp_ahb2lpenclrr::MP_AHB2LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mp_ahb2lpenclrr;
/**MP_AHB3LPENSETR (rw) register accessor: This register is used by the MPU

You can [`read`](crate::Reg::read) this register and get [`mp_ahb3lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb3lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB3LPENSETR)

For information about available fields see [`mod@mp_ahb3lpensetr`] module*/
pub type MP_AHB3LPENSETR = crate::Reg<mp_ahb3lpensetr::MP_AHB3LPENSETRrs>;
///This register is used by the MPU
pub mod mp_ahb3lpensetr;
/**MP_AHB3LPENCLRR (rw) register accessor: This register is used by the MPU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mp_ahb3lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb3lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB3LPENCLRR)

For information about available fields see [`mod@mp_ahb3lpenclrr`] module*/
pub type MP_AHB3LPENCLRR = crate::Reg<mp_ahb3lpenclrr::MP_AHB3LPENCLRRrs>;
///This register is used by the MPU in order to clear the PERxLPEN bit
pub mod mp_ahb3lpenclrr;
/**MP_AHB4LPENSETR (rw) register accessor: This register is used by the MPU

You can [`read`](crate::Reg::read) this register and get [`mp_ahb4lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb4lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB4LPENSETR)

For information about available fields see [`mod@mp_ahb4lpensetr`] module*/
pub type MP_AHB4LPENSETR = crate::Reg<mp_ahb4lpensetr::MP_AHB4LPENSETRrs>;
///This register is used by the MPU
pub mod mp_ahb4lpensetr;
/**MP_AHB4LPENCLRR (rw) register accessor: This register is used by the MPU

You can [`read`](crate::Reg::read) this register and get [`mp_ahb4lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb4lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AHB4LPENCLRR)

For information about available fields see [`mod@mp_ahb4lpenclrr`] module*/
pub type MP_AHB4LPENCLRR = crate::Reg<mp_ahb4lpenclrr::MP_AHB4LPENCLRRrs>;
///This register is used by the MPU
pub mod mp_ahb4lpenclrr;
/**MP_AXIMLPENSETR (rw) register accessor: This register is used by the MPU

You can [`read`](crate::Reg::read) this register and get [`mp_aximlpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_aximlpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AXIMLPENSETR)

For information about available fields see [`mod@mp_aximlpensetr`] module*/
pub type MP_AXIMLPENSETR = crate::Reg<mp_aximlpensetr::MP_AXIMLPENSETRrs>;
///This register is used by the MPU
pub mod mp_aximlpensetr;
/**MP_AXIMLPENCLRR (rw) register accessor: This register is used by the MPU

You can [`read`](crate::Reg::read) this register and get [`mp_aximlpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_aximlpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_AXIMLPENCLRR)

For information about available fields see [`mod@mp_aximlpenclrr`] module*/
pub type MP_AXIMLPENCLRR = crate::Reg<mp_aximlpenclrr::MP_AXIMLPENCLRRrs>;
///This register is used by the MPU
pub mod mp_aximlpenclrr;
/**MP_MLAHBLPENSETR (rw) register accessor: This register is used by the MPU in order to set the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mp_mlahblpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_mlahblpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_MLAHBLPENSETR)

For information about available fields see [`mod@mp_mlahblpensetr`] module*/
pub type MP_MLAHBLPENSETR = crate::Reg<mp_mlahblpensetr::MP_MLAHBLPENSETRrs>;
///This register is used by the MPU in order to set the PERxLPEN bit
pub mod mp_mlahblpensetr;
/**MP_MLAHBLPENCLRR (rw) register accessor: This register is used by the MPU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mp_mlahblpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_mlahblpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MP_MLAHBLPENCLRR)

For information about available fields see [`mod@mp_mlahblpenclrr`] module*/
pub type MP_MLAHBLPENCLRR = crate::Reg<mp_mlahblpenclrr::MP_MLAHBLPENCLRRrs>;
///This register is used by the MPU in order to clear the PERxLPEN bit
pub mod mp_mlahblpenclrr;
/**MC_APB1LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_apb1lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb1lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB1LPENSETR)

For information about available fields see [`mod@mc_apb1lpensetr`] module*/
pub type MC_APB1LPENSETR = crate::Reg<mc_apb1lpensetr::MC_APB1LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_apb1lpensetr;
/**MC_APB1LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mc_apb1lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb1lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB1LPENCLRR)

For information about available fields see [`mod@mc_apb1lpenclrr`] module*/
pub type MC_APB1LPENCLRR = crate::Reg<mc_apb1lpenclrr::MC_APB1LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod mc_apb1lpenclrr;
/**MC_APB2LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_apb2lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb2lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB2LPENSETR)

For information about available fields see [`mod@mc_apb2lpensetr`] module*/
pub type MC_APB2LPENSETR = crate::Reg<mc_apb2lpensetr::MC_APB2LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_apb2lpensetr;
/**MC_APB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb2lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb2lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB2LPENCLRR)

For information about available fields see [`mod@mc_apb2lpenclrr`] module*/
pub type MC_APB2LPENCLRR = crate::Reg<mc_apb2lpenclrr::MC_APB2LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod mc_apb2lpenclrr;
/**MC_APB3LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_apb3lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb3lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB3LPENSETR)

For information about available fields see [`mod@mc_apb3lpensetr`] module*/
pub type MC_APB3LPENSETR = crate::Reg<mc_apb3lpensetr::MC_APB3LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_apb3lpensetr;
/**MC_APB3LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_apb3lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_apb3lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_APB3LPENCLRR)

For information about available fields see [`mod@mc_apb3lpenclrr`] module*/
pub type MC_APB3LPENCLRR = crate::Reg<mc_apb3lpenclrr::MC_APB3LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod mc_apb3lpenclrr;
/**MC_AHB2LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb2lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb2lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB2LPENSETR)

For information about available fields see [`mod@mc_ahb2lpensetr`] module*/
pub type MC_AHB2LPENSETR = crate::Reg<mc_ahb2lpensetr::MC_AHB2LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_ahb2lpensetr;
/**MC_AHB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb2lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb2lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB2LPENCLRR)

For information about available fields see [`mod@mc_ahb2lpenclrr`] module*/
pub type MC_AHB2LPENCLRR = crate::Reg<mc_ahb2lpenclrr::MC_AHB2LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod mc_ahb2lpenclrr;
/**MC_AHB3LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb3lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb3lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB3LPENSETR)

For information about available fields see [`mod@mc_ahb3lpensetr`] module*/
pub type MC_AHB3LPENSETR = crate::Reg<mc_ahb3lpensetr::MC_AHB3LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_ahb3lpensetr;
/**MC_AHB3LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb3lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb3lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB3LPENCLRR)

For information about available fields see [`mod@mc_ahb3lpenclrr`] module*/
pub type MC_AHB3LPENCLRR = crate::Reg<mc_ahb3lpenclrr::MC_AHB3LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod mc_ahb3lpenclrr;
/**MC_AHB4LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb4lpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb4lpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB4LPENSETR)

For information about available fields see [`mod@mc_ahb4lpensetr`] module*/
pub type MC_AHB4LPENSETR = crate::Reg<mc_ahb4lpensetr::MC_AHB4LPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod mc_ahb4lpensetr;
/**MC_AHB4LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb4lpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb4lpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB4LPENCLRR)

For information about available fields see [`mod@mc_ahb4lpenclrr`] module*/
pub type MC_AHB4LPENCLRR = crate::Reg<mc_ahb4lpenclrr::MC_AHB4LPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod mc_ahb4lpenclrr;
/**MC_AXIMLPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_aximlpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_aximlpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AXIMLPENSETR)

For information about available fields see [`mod@mc_aximlpensetr`] module*/
pub type MC_AXIMLPENSETR = crate::Reg<mc_aximlpensetr::MC_AXIMLPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
pub mod mc_aximlpensetr;
/**MC_AXIMLPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_aximlpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_aximlpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AXIMLPENCLRR)

For information about available fields see [`mod@mc_aximlpenclrr`] module*/
pub type MC_AXIMLPENCLRR = crate::Reg<mc_aximlpenclrr::MC_AXIMLPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod mc_aximlpenclrr;
/**MC_MLAHBLPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_mlahblpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_mlahblpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_MLAHBLPENSETR)

For information about available fields see [`mod@mc_mlahblpensetr`] module*/
pub type MC_MLAHBLPENSETR = crate::Reg<mc_mlahblpensetr::MC_MLAHBLPENSETRrs>;
///This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
pub mod mc_mlahblpensetr;
/**MC_MLAHBLPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`mc_mlahblpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_mlahblpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_MLAHBLPENCLRR)

For information about available fields see [`mod@mc_mlahblpenclrr`] module*/
pub type MC_MLAHBLPENCLRR = crate::Reg<mc_mlahblpenclrr::MC_MLAHBLPENCLRRrs>;
///This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod mc_mlahblpenclrr;
/**MC_RSTSCLRR (rw) register accessor: This register is used by the MCU to check the reset source.

You can [`read`](crate::Reg::read) this register and get [`mc_rstsclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_rstsclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_RSTSCLRR)

For information about available fields see [`mod@mc_rstsclrr`] module*/
pub type MC_RSTSCLRR = crate::Reg<mc_rstsclrr::MC_RSTSCLRRrs>;
///This register is used by the MCU to check the reset source.
pub mod mc_rstsclrr;
/**MC_CIER (rw) register accessor: This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.

You can [`read`](crate::Reg::read) this register and get [`mc_cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_CIER)

For information about available fields see [`mod@mc_cier`] module*/
pub type MC_CIER = crate::Reg<mc_cier::MC_CIERrs>;
///This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.
pub mod mc_cier;
/**MC_CIFR (rw) register accessor: This register shall be used by the MCU in order to read and clear the interrupt flags.

You can [`read`](crate::Reg::read) this register and get [`mc_cifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_cifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_CIFR)

For information about available fields see [`mod@mc_cifr`] module*/
pub type MC_CIFR = crate::Reg<mc_cifr::MC_CIFRrs>;
///This register shall be used by the MCU in order to read and clear the interrupt flags.
pub mod mc_cifr;
/**VERR (r) register accessor: This register gives the IP version

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///This register gives the IP version
pub mod verr;
/**IDR (r) register accessor: This register gives the unique identifier of the RCC

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///This register gives the unique identifier of the RCC
pub mod idr;
/**SIDR (r) register accessor: This register gives the decoding space, which is for the RCC of 4 kB.

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///This register gives the decoding space, which is for the RCC of 4 kB.
pub mod sidr;
