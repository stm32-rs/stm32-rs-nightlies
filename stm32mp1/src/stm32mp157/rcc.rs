#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcc_tzcr: RCC_TZCR,
    _reserved1: [u8; 0x08],
    rcc_ocensetr: RCC_OCENSETR,
    rcc_ocenclrr: RCC_OCENCLRR,
    _reserved3: [u8; 0x04],
    rcc_hsicfgr: RCC_HSICFGR,
    rcc_csicfgr: RCC_CSICFGR,
    rcc_mpckselr: RCC_MPCKSELR,
    rcc_assckselr: RCC_ASSCKSELR,
    rcc_rck12selr: RCC_RCK12SELR,
    rcc_mpckdivr: RCC_MPCKDIVR,
    rcc_axidivr: RCC_AXIDIVR,
    _reserved10: [u8; 0x08],
    rcc_apb4divr: RCC_APB4DIVR,
    rcc_apb5divr: RCC_APB5DIVR,
    rcc_rtcdivr: RCC_RTCDIVR,
    rcc_mssckselr: RCC_MSSCKSELR,
    _reserved14: [u8; 0x34],
    rcc_pll1cr: RCC_PLL1CR,
    rcc_pll1cfgr1: RCC_PLL1CFGR1,
    rcc_pll1cfgr2: RCC_PLL1CFGR2,
    rcc_pll1fracr: RCC_PLL1FRACR,
    rcc_pll1csgr: RCC_PLL1CSGR,
    rcc_pll2cr: RCC_PLL2CR,
    rcc_pll2cfgr1: RCC_PLL2CFGR1,
    rcc_pll2cfgr2: RCC_PLL2CFGR2,
    rcc_pll2fracr: RCC_PLL2FRACR,
    rcc_pll2csgr: RCC_PLL2CSGR,
    _reserved24: [u8; 0x18],
    rcc_i2c46ckselr: RCC_I2C46CKSELR,
    rcc_spi6ckselr: RCC_SPI6CKSELR,
    rcc_uart1ckselr: RCC_UART1CKSELR,
    rcc_rng1ckselr: RCC_RNG1CKSELR,
    rcc_cperckselr: RCC_CPERCKSELR,
    rcc_stgenckselr: RCC_STGENCKSELR,
    rcc_ddritfcr: RCC_DDRITFCR,
    _reserved31: [u8; 0x24],
    rcc_mp_bootcr: RCC_MP_BOOTCR,
    rcc_mp_sreqsetr: RCC_MP_SREQSETR,
    rcc_mp_sreqclrr: RCC_MP_SREQCLRR,
    rcc_mp_gcr: RCC_MP_GCR,
    rcc_mp_aprstcr: RCC_MP_APRSTCR,
    rcc_mp_aprstsr: RCC_MP_APRSTSR,
    _reserved37: [u8; 0x28],
    rcc_bdcr: RCC_BDCR,
    rcc_rdlsicr: RCC_RDLSICR,
    _reserved39: [u8; 0x38],
    rcc_apb4rstsetr: RCC_APB4RSTSETR,
    rcc_apb4rstclrr: RCC_APB4RSTCLRR,
    rcc_apb5rstsetr: RCC_APB5RSTSETR,
    rcc_apb5rstclrr: RCC_APB5RSTCLRR,
    rcc_ahb5rstsetr: RCC_AHB5RSTSETR,
    rcc_ahb5rstclrr: RCC_AHB5RSTCLRR,
    rcc_ahb6rstsetr: RCC_AHB6RSTSETR,
    rcc_ahb6rstclrr: RCC_AHB6RSTCLRR,
    rcc_tzahb6rstsetr: RCC_TZAHB6RSTSETR,
    rcc_tzahb6rstclrr: RCC_TZAHB6RSTCLRR,
    _reserved49: [u8; 0x58],
    rcc_mp_apb4ensetr: RCC_MP_APB4ENSETR,
    rcc_mp_apb4enclrr: RCC_MP_APB4ENCLRR,
    rcc_mp_apb5ensetr: RCC_MP_APB5ENSETR,
    rcc_mp_apb5enclrr: RCC_MP_APB5ENCLRR,
    rcc_mp_ahb5ensetr: RCC_MP_AHB5ENSETR,
    rcc_mp_ahb5enclrr: RCC_MP_AHB5ENCLRR,
    rcc_mp_ahb6ensetr: RCC_MP_AHB6ENSETR,
    rcc_mp_ahb6enclrr: RCC_MP_AHB6ENCLRR,
    rcc_mp_tzahb6ensetr: RCC_MP_TZAHB6ENSETR,
    rcc_mp_tzahb6enclrr: RCC_MP_TZAHB6ENCLRR,
    _reserved59: [u8; 0x58],
    rcc_mc_apb4ensetr: RCC_MC_APB4ENSETR,
    rcc_mc_apb4enclrr: RCC_MC_APB4ENCLRR,
    rcc_mc_apb5ensetr: RCC_MC_APB5ENSETR,
    rcc_mc_apb5enclrr: RCC_MC_APB5ENCLRR,
    rcc_mc_ahb5ensetr: RCC_MC_AHB5ENSETR,
    rcc_mc_ahb5enclrr: RCC_MC_AHB5ENCLRR,
    rcc_mc_ahb6ensetr: RCC_MC_AHB6ENSETR,
    rcc_mc_ahb6enclrr: RCC_MC_AHB6ENCLRR,
    _reserved67: [u8; 0x60],
    rcc_mp_apb4lpensetr: RCC_MP_APB4LPENSETR,
    rcc_mp_apb4lpenclrr: RCC_MP_APB4LPENCLRR,
    rcc_mp_apb5lpensetr: RCC_MP_APB5LPENSETR,
    rcc_mp_apb5lpenclrr: RCC_MP_APB5LPENCLRR,
    rcc_mp_ahb5lpensetr: RCC_MP_AHB5LPENSETR,
    rcc_mp_ahb5lpenclrr: RCC_MP_AHB5LPENCLRR,
    rcc_mp_ahb6lpensetr: RCC_MP_AHB6LPENSETR,
    rcc_mp_ahb6lpenclrr: RCC_MP_AHB6LPENCLRR,
    rcc_mp_tzahb6lpensetr: RCC_MP_TZAHB6LPENSETR,
    rcc_mp_tzahb6lpenclrr: RCC_MP_TZAHB6LPENCLRR,
    _reserved77: [u8; 0x58],
    rcc_mc_apb4lpensetr: RCC_MC_APB4LPENSETR,
    rcc_mc_apb4lpenclrr: RCC_MC_APB4LPENCLRR,
    rcc_mc_apb5lpensetr: RCC_MC_APB5LPENSETR,
    rcc_mc_apb5lpenclrr: RCC_MC_APB5LPENCLRR,
    rcc_mc_ahb5lpensetr: RCC_MC_AHB5LPENSETR,
    rcc_mc_ahb5lpenclrr: RCC_MC_AHB5LPENCLRR,
    rcc_mc_ahb6lpensetr: RCC_MC_AHB6LPENSETR,
    rcc_mc_ahb6lpenclrr: RCC_MC_AHB6LPENCLRR,
    _reserved85: [u8; 0x60],
    rcc_br_rstsclrr: RCC_BR_RSTSCLRR,
    rcc_mp_grstcsetr: RCC_MP_GRSTCSETR,
    rcc_mp_rstsclrr: RCC_MP_RSTSCLRR,
    rcc_mp_iwdgfzsetr: RCC_MP_IWDGFZSETR,
    rcc_mp_iwdgfzclrr: RCC_MP_IWDGFZCLRR,
    rcc_mp_cier: RCC_MP_CIER,
    rcc_mp_cifr: RCC_MP_CIFR,
    rcc_pwrlpdlycr: RCC_PWRLPDLYCR,
    rcc_mp_rstssetr: RCC_MP_RSTSSETR,
    _reserved94: [u8; 0x03dc],
    rcc_mco1cfgr: RCC_MCO1CFGR,
    rcc_mco2cfgr: RCC_MCO2CFGR,
    rcc_ocrdyr: RCC_OCRDYR,
    rcc_dbgcfgr: RCC_DBGCFGR,
    _reserved98: [u8; 0x10],
    rcc_rck3selr: RCC_RCK3SELR,
    rcc_rck4selr: RCC_RCK4SELR,
    rcc_timg1prer: RCC_TIMG1PRER,
    rcc_timg2prer: RCC_TIMG2PRER,
    rcc_mcudivr: RCC_MCUDIVR,
    rcc_apb1divr: RCC_APB1DIVR,
    rcc_apb2divr: RCC_APB2DIVR,
    rcc_apb3divr: RCC_APB3DIVR,
    _reserved106: [u8; 0x40],
    rcc_pll3cr: RCC_PLL3CR,
    rcc_pll3cfgr1: RCC_PLL3CFGR1,
    rcc_pll3cfgr2: RCC_PLL3CFGR2,
    rcc_pll3fracr: RCC_PLL3FRACR,
    rcc_pll3csgr: RCC_PLL3CSGR,
    rcc_pll4cr: RCC_PLL4CR,
    rcc_pll4cfgr1: RCC_PLL4CFGR1,
    rcc_pll4cfgr2: RCC_PLL4CFGR2,
    rcc_pll4fracr: RCC_PLL4FRACR,
    rcc_pll4csgr: RCC_PLL4CSGR,
    _reserved116: [u8; 0x18],
    rcc_i2c12ckselr: RCC_I2C12CKSELR,
    rcc_i2c35ckselr: RCC_I2C35CKSELR,
    rcc_sai1ckselr: RCC_SAI1CKSELR,
    rcc_sai2ckselr: RCC_SAI2CKSELR,
    rcc_sai3ckselr: RCC_SAI3CKSELR,
    rcc_sai4ckselr: RCC_SAI4CKSELR,
    rcc_spi2s1ckselr: RCC_SPI2S1CKSELR,
    rcc_spi2s23ckselr: RCC_SPI2S23CKSELR,
    rcc_spi45ckselr: RCC_SPI45CKSELR,
    rcc_uart6ckselr: RCC_UART6CKSELR,
    rcc_uart24ckselr: RCC_UART24CKSELR,
    rcc_uart35ckselr: RCC_UART35CKSELR,
    rcc_uart78ckselr: RCC_UART78CKSELR,
    rcc_sdmmc12ckselr: RCC_SDMMC12CKSELR,
    rcc_sdmmc3ckselr: RCC_SDMMC3CKSELR,
    rcc_ethckselr: RCC_ETHCKSELR,
    rcc_qspickselr: RCC_QSPICKSELR,
    rcc_fmcckselr: RCC_FMCCKSELR,
    _reserved134: [u8; 0x04],
    rcc_fdcanckselr: RCC_FDCANCKSELR,
    _reserved135: [u8; 0x04],
    rcc_spdifckselr: RCC_SPDIFCKSELR,
    rcc_cecckselr: RCC_CECCKSELR,
    rcc_usbckselr: RCC_USBCKSELR,
    rcc_rng2ckselr: RCC_RNG2CKSELR,
    rcc_dsickselr: RCC_DSICKSELR,
    rcc_adcckselr: RCC_ADCCKSELR,
    rcc_lptim45ckselr: RCC_LPTIM45CKSELR,
    rcc_lptim23ckselr: RCC_LPTIM23CKSELR,
    rcc_lptim1ckselr: RCC_LPTIM1CKSELR,
    _reserved144: [u8; 0x48],
    rcc_apb1rstsetr: RCC_APB1RSTSETR,
    rcc_apb1rstclrr: RCC_APB1RSTCLRR,
    rcc_apb2rstsetr: RCC_APB2RSTSETR,
    rcc_apb2rstclrr: RCC_APB2RSTCLRR,
    rcc_apb3rstsetr: RCC_APB3RSTSETR,
    rcc_apb3rstclrr: RCC_APB3RSTCLRR,
    rcc_ahb2rstsetr: RCC_AHB2RSTSETR,
    rcc_ahb2rstclrr: RCC_AHB2RSTCLRR,
    rcc_ahb3rstsetr: RCC_AHB3RSTSETR,
    rcc_ahb3rstclrr: RCC_AHB3RSTCLRR,
    rcc_ahb4rstsetr: RCC_AHB4RSTSETR,
    rcc_ahb4rstclrr: RCC_AHB4RSTCLRR,
    _reserved156: [u8; 0x50],
    rcc_mp_apb1ensetr: RCC_MP_APB1ENSETR,
    rcc_mp_apb1enclrr: RCC_MP_APB1ENCLRR,
    rcc_mp_apb2ensetr: RCC_MP_APB2ENSETR,
    rcc_mp_apb2enclrr: RCC_MP_APB2ENCLRR,
    rcc_mp_apb3ensetr: RCC_MP_APB3ENSETR,
    rcc_mp_apb3enclrr: RCC_MP_APB3ENCLRR,
    rcc_mp_ahb2ensetr: RCC_MP_AHB2ENSETR,
    rcc_mp_ahb2enclrr: RCC_MP_AHB2ENCLRR,
    rcc_mp_ahb3ensetr: RCC_MP_AHB3ENSETR,
    rcc_mp_ahb3enclrr: RCC_MP_AHB3ENCLRR,
    rcc_mp_ahb4ensetr: RCC_MP_AHB4ENSETR,
    rcc_mp_ahb4enclrr: RCC_MP_AHB4ENCLRR,
    _reserved168: [u8; 0x08],
    rcc_mp_mlahbensetr: RCC_MP_MLAHBENSETR,
    rcc_mp_mlahbenclrr: RCC_MP_MLAHBENCLRR,
    _reserved170: [u8; 0x40],
    rcc_mc_apb1ensetr: RCC_MC_APB1ENSETR,
    rcc_mc_apb1enclrr: RCC_MC_APB1ENCLRR,
    rcc_mc_apb2ensetr: RCC_MC_APB2ENSETR,
    rcc_mc_apb2enclrr: RCC_MC_APB2ENCLRR,
    rcc_mc_apb3ensetr: RCC_MC_APB3ENSETR,
    rcc_mc_apb3enclrr: RCC_MC_APB3ENCLRR,
    rcc_mc_ahb2ensetr: RCC_MC_AHB2ENSETR,
    rcc_mc_ahb2enclrr: RCC_MC_AHB2ENCLRR,
    rcc_mc_ahb3ensetr: RCC_MC_AHB3ENSETR,
    rcc_mc_ahb3enclrr: RCC_MC_AHB3ENCLRR,
    rcc_mc_ahb4ensetr: RCC_MC_AHB4ENSETR,
    rcc_mc_ahb4enclrr: RCC_MC_AHB4ENCLRR,
    rcc_mc_aximensetr: RCC_MC_AXIMENSETR,
    rcc_mc_aximenclrr: RCC_MC_AXIMENCLRR,
    rcc_mc_mlahbensetr: RCC_MC_MLAHBENSETR,
    rcc_mc_mlahbenclrr: RCC_MC_MLAHBENCLRR,
    _reserved186: [u8; 0x40],
    rcc_mp_apb1lpensetr: RCC_MP_APB1LPENSETR,
    rcc_mp_apb1lpenclrr: RCC_MP_APB1LPENCLRR,
    rcc_mp_apb2lpensetr: RCC_MP_APB2LPENSETR,
    rcc_mp_apb2lpenclrr: RCC_MP_APB2LPENCLRR,
    rcc_mp_apb3lpensetr: RCC_MP_APB3LPENSETR,
    rcc_mp_apb3lpenclrr: RCC_MP_APB3LPENCLRR,
    rcc_mp_ahb2lpensetr: RCC_MP_AHB2LPENSETR,
    rcc_mp_ahb2lpenclrr: RCC_MP_AHB2LPENCLRR,
    rcc_mp_ahb3lpensetr: RCC_MP_AHB3LPENSETR,
    rcc_mp_ahb3lpenclrr: RCC_MP_AHB3LPENCLRR,
    rcc_mp_ahb4lpensetr: RCC_MP_AHB4LPENSETR,
    rcc_mp_ahb4lpenclrr: RCC_MP_AHB4LPENCLRR,
    rcc_mp_aximlpensetr: RCC_MP_AXIMLPENSETR,
    rcc_mp_aximlpenclrr: RCC_MP_AXIMLPENCLRR,
    rcc_mp_mlahblpensetr: RCC_MP_MLAHBLPENSETR,
    rcc_mp_mlahblpenclrr: RCC_MP_MLAHBLPENCLRR,
    _reserved202: [u8; 0x40],
    rcc_mc_apb1lpensetr: RCC_MC_APB1LPENSETR,
    rcc_mc_apb1lpenclrr: RCC_MC_APB1LPENCLRR,
    rcc_mc_apb2lpensetr: RCC_MC_APB2LPENSETR,
    rcc_mc_apb2lpenclrr: RCC_MC_APB2LPENCLRR,
    rcc_mc_apb3lpensetr: RCC_MC_APB3LPENSETR,
    rcc_mc_apb3lpenclrr: RCC_MC_APB3LPENCLRR,
    rcc_mc_ahb2lpensetr: RCC_MC_AHB2LPENSETR,
    rcc_mc_ahb2lpenclrr: RCC_MC_AHB2LPENCLRR,
    rcc_mc_ahb3lpensetr: RCC_MC_AHB3LPENSETR,
    rcc_mc_ahb3lpenclrr: RCC_MC_AHB3LPENCLRR,
    rcc_mc_ahb4lpensetr: RCC_MC_AHB4LPENSETR,
    rcc_mc_ahb4lpenclrr: RCC_MC_AHB4LPENCLRR,
    rcc_mc_aximlpensetr: RCC_MC_AXIMLPENSETR,
    rcc_mc_aximlpenclrr: RCC_MC_AXIMLPENCLRR,
    rcc_mc_mlahblpensetr: RCC_MC_MLAHBLPENSETR,
    rcc_mc_mlahblpenclrr: RCC_MC_MLAHBLPENCLRR,
    _reserved218: [u8; 0x40],
    rcc_mc_rstsclrr: RCC_MC_RSTSCLRR,
    _reserved219: [u8; 0x10],
    rcc_mc_cier: RCC_MC_CIER,
    rcc_mc_cifr: RCC_MC_CIFR,
    _reserved221: [u8; 0x03d8],
    rcc_verr: RCC_VERR,
    rcc_idr: RCC_IDR,
    rcc_sidr: RCC_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode."]
    #[inline(always)]
    pub const fn rcc_tzcr(&self) -> &RCC_TZCR {
        &self.rcc_tzcr
    }
    #[doc = "0x0c - This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_ocensetr(&self) -> &RCC_OCENSETR {
        &self.rcc_ocensetr
    }
    #[doc = "0x10 - This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_ocenclrr(&self) -> &RCC_OCENCLRR {
        &self.rcc_ocenclrr
    }
    #[doc = "0x18 - This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_hsicfgr(&self) -> &RCC_HSICFGR {
        &self.rcc_hsicfgr
    }
    #[doc = "0x1c - This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_csicfgr(&self) -> &RCC_CSICFGR {
        &self.rcc_csicfgr
    }
    #[doc = "0x20 - This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_mpckselr(&self) -> &RCC_MPCKSELR {
        &self.rcc_mpckselr
    }
    #[doc = "0x24 - This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_assckselr(&self) -> &RCC_ASSCKSELR {
        &self.rcc_assckselr
    }
    #[doc = "0x28 - This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_rck12selr(&self) -> &RCC_RCK12SELR {
        &self.rcc_rck12selr
    }
    #[doc = "0x2c - This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mpckdivr(&self) -> &RCC_MPCKDIVR {
        &self.rcc_mpckdivr
    }
    #[doc = "0x30 - This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_axidivr(&self) -> &RCC_AXIDIVR {
        &self.rcc_axidivr
    }
    #[doc = "0x3c - This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_apb4divr(&self) -> &RCC_APB4DIVR {
        &self.rcc_apb4divr
    }
    #[doc = "0x40 - This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_apb5divr(&self) -> &RCC_APB5DIVR {
        &self.rcc_apb5divr
    }
    #[doc = "0x44 - This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_rtcdivr(&self) -> &RCC_RTCDIVR {
        &self.rcc_rtcdivr
    }
    #[doc = "0x48 - This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_mssckselr(&self) -> &RCC_MSSCKSELR {
        &self.rcc_mssckselr
    }
    #[doc = "0x80 - This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll1cr(&self) -> &RCC_PLL1CR {
        &self.rcc_pll1cr
    }
    #[doc = "0x84 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll1cfgr1(&self) -> &RCC_PLL1CFGR1 {
        &self.rcc_pll1cfgr1
    }
    #[doc = "0x88 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll1cfgr2(&self) -> &RCC_PLL1CFGR2 {
        &self.rcc_pll1cfgr2
    }
    #[doc = "0x8c - This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll1fracr(&self) -> &RCC_PLL1FRACR {
        &self.rcc_pll1fracr
    }
    #[doc = "0x90 - This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll1csgr(&self) -> &RCC_PLL1CSGR {
        &self.rcc_pll1csgr
    }
    #[doc = "0x94 - This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll2cr(&self) -> &RCC_PLL2CR {
        &self.rcc_pll2cr
    }
    #[doc = "0x98 - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll2cfgr1(&self) -> &RCC_PLL2CFGR1 {
        &self.rcc_pll2cfgr1
    }
    #[doc = "0x9c - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll2cfgr2(&self) -> &RCC_PLL2CFGR2 {
        &self.rcc_pll2cfgr2
    }
    #[doc = "0xa0 - This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll2fracr(&self) -> &RCC_PLL2FRACR {
        &self.rcc_pll2fracr
    }
    #[doc = "0xa4 - This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    #[inline(always)]
    pub const fn rcc_pll2csgr(&self) -> &RCC_PLL2CSGR {
        &self.rcc_pll2csgr
    }
    #[doc = "0xc0 - This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_i2c46ckselr(&self) -> &RCC_I2C46CKSELR {
        &self.rcc_i2c46ckselr
    }
    #[doc = "0xc4 - This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_spi6ckselr(&self) -> &RCC_SPI6CKSELR {
        &self.rcc_spi6ckselr
    }
    #[doc = "0xc8 - This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_uart1ckselr(&self) -> &RCC_UART1CKSELR {
        &self.rcc_uart1ckselr
    }
    #[doc = "0xcc - This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_rng1ckselr(&self) -> &RCC_RNG1CKSELR {
        &self.rcc_rng1ckselr
    }
    #[doc = "0xd0 - This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_cperckselr(&self) -> &RCC_CPERCKSELR {
        &self.rcc_cperckselr
    }
    #[doc = "0xd4 - This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_stgenckselr(&self) -> &RCC_STGENCKSELR {
        &self.rcc_stgenckselr
    }
    #[doc = "0xd8 - This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_ddritfcr(&self) -> &RCC_DDRITFCR {
        &self.rcc_ddritfcr
    }
    #[doc = "0x100 - This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU."]
    #[inline(always)]
    pub const fn rcc_mp_bootcr(&self) -> &RCC_MP_BOOTCR {
        &self.rcc_mp_bootcr
    }
    #[doc = "0x104 - Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_sreqsetr(&self) -> &RCC_MP_SREQSETR {
        &self.rcc_mp_sreqsetr
    }
    #[doc = "0x108 - Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_sreqclrr(&self) -> &RCC_MP_SREQCLRR {
        &self.rcc_mp_sreqclrr
    }
    #[doc = "0x10c - The register contains global control bits. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_gcr(&self) -> &RCC_MP_GCR {
        &self.rcc_mp_gcr
    }
    #[doc = "0x110 - This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_aprstcr(&self) -> &RCC_MP_APRSTCR {
        &self.rcc_mp_aprstcr
    }
    #[doc = "0x114 - This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_aprstsr(&self) -> &RCC_MP_APRSTSR {
        &self.rcc_mp_aprstsr
    }
    #[doc = "0x140 - This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_bdcr(&self) -> &RCC_BDCR {
        &self.rcc_bdcr
    }
    #[doc = "0x144 - This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_rdlsicr(&self) -> &RCC_RDLSICR {
        &self.rcc_rdlsicr
    }
    #[doc = "0x180 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb4rstsetr(&self) -> &RCC_APB4RSTSETR {
        &self.rcc_apb4rstsetr
    }
    #[doc = "0x184 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb4rstclrr(&self) -> &RCC_APB4RSTCLRR {
        &self.rcc_apb4rstclrr
    }
    #[doc = "0x188 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_apb5rstsetr(&self) -> &RCC_APB5RSTSETR {
        &self.rcc_apb5rstsetr
    }
    #[doc = "0x18c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_apb5rstclrr(&self) -> &RCC_APB5RSTCLRR {
        &self.rcc_apb5rstclrr
    }
    #[doc = "0x190 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_ahb5rstsetr(&self) -> &RCC_AHB5RSTSETR {
        &self.rcc_ahb5rstsetr
    }
    #[doc = "0x194 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_ahb5rstclrr(&self) -> &RCC_AHB5RSTCLRR {
        &self.rcc_ahb5rstclrr
    }
    #[doc = "0x198 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_ahb6rstsetr(&self) -> &RCC_AHB6RSTSETR {
        &self.rcc_ahb6rstsetr
    }
    #[doc = "0x19c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_ahb6rstclrr(&self) -> &RCC_AHB6RSTCLRR {
        &self.rcc_ahb6rstclrr
    }
    #[doc = "0x1a0 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_tzahb6rstsetr(&self) -> &RCC_TZAHB6RSTSETR {
        &self.rcc_tzahb6rstsetr
    }
    #[doc = "0x1a4 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_tzahb6rstclrr(&self) -> &RCC_TZAHB6RSTCLRR {
        &self.rcc_tzahb6rstclrr
    }
    #[doc = "0x200 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    #[inline(always)]
    pub const fn rcc_mp_apb4ensetr(&self) -> &RCC_MP_APB4ENSETR {
        &self.rcc_mp_apb4ensetr
    }
    #[doc = "0x204 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    #[inline(always)]
    pub const fn rcc_mp_apb4enclrr(&self) -> &RCC_MP_APB4ENCLRR {
        &self.rcc_mp_apb4enclrr
    }
    #[doc = "0x208 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    #[inline(always)]
    pub const fn rcc_mp_apb5ensetr(&self) -> &RCC_MP_APB5ENSETR {
        &self.rcc_mp_apb5ensetr
    }
    #[doc = "0x20c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    #[inline(always)]
    pub const fn rcc_mp_apb5enclrr(&self) -> &RCC_MP_APB5ENCLRR {
        &self.rcc_mp_apb5enclrr
    }
    #[doc = "0x210 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_ahb5ensetr(&self) -> &RCC_MP_AHB5ENSETR {
        &self.rcc_mp_ahb5ensetr
    }
    #[doc = "0x214 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_ahb5enclrr(&self) -> &RCC_MP_AHB5ENCLRR {
        &self.rcc_mp_ahb5enclrr
    }
    #[doc = "0x218 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    #[inline(always)]
    pub const fn rcc_mp_ahb6ensetr(&self) -> &RCC_MP_AHB6ENSETR {
        &self.rcc_mp_ahb6ensetr
    }
    #[doc = "0x21c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    #[inline(always)]
    pub const fn rcc_mp_ahb6enclrr(&self) -> &RCC_MP_AHB6ENCLRR {
        &self.rcc_mp_ahb6enclrr
    }
    #[doc = "0x220 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_tzahb6ensetr(&self) -> &RCC_MP_TZAHB6ENSETR {
        &self.rcc_mp_tzahb6ensetr
    }
    #[doc = "0x224 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_tzahb6enclrr(&self) -> &RCC_MP_TZAHB6ENCLRR {
        &self.rcc_mp_tzahb6enclrr
    }
    #[doc = "0x280 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb4ensetr(&self) -> &RCC_MC_APB4ENSETR {
        &self.rcc_mc_apb4ensetr
    }
    #[doc = "0x284 - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb4enclrr(&self) -> &RCC_MC_APB4ENCLRR {
        &self.rcc_mc_apb4enclrr
    }
    #[doc = "0x288 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb5ensetr(&self) -> &RCC_MC_APB5ENSETR {
        &self.rcc_mc_apb5ensetr
    }
    #[doc = "0x28c - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb5enclrr(&self) -> &RCC_MC_APB5ENCLRR {
        &self.rcc_mc_apb5enclrr
    }
    #[doc = "0x290 - This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mc_ahb5ensetr(&self) -> &RCC_MC_AHB5ENSETR {
        &self.rcc_mc_ahb5ensetr
    }
    #[doc = "0x294 - This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mc_ahb5enclrr(&self) -> &RCC_MC_AHB5ENCLRR {
        &self.rcc_mc_ahb5enclrr
    }
    #[doc = "0x298 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb6ensetr(&self) -> &RCC_MC_AHB6ENSETR {
        &self.rcc_mc_ahb6ensetr
    }
    #[doc = "0x29c - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb6enclrr(&self) -> &RCC_MC_AHB6ENCLRR {
        &self.rcc_mc_ahb6enclrr
    }
    #[doc = "0x300 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_apb4lpensetr(&self) -> &RCC_MP_APB4LPENSETR {
        &self.rcc_mp_apb4lpensetr
    }
    #[doc = "0x304 - This register is used by the MCU"]
    #[inline(always)]
    pub const fn rcc_mp_apb4lpenclrr(&self) -> &RCC_MP_APB4LPENCLRR {
        &self.rcc_mp_apb4lpenclrr
    }
    #[doc = "0x308 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_apb5lpensetr(&self) -> &RCC_MP_APB5LPENSETR {
        &self.rcc_mp_apb5lpensetr
    }
    #[doc = "0x30c - This register is used by the Mpu."]
    #[inline(always)]
    pub const fn rcc_mp_apb5lpenclrr(&self) -> &RCC_MP_APB5LPENCLRR {
        &self.rcc_mp_apb5lpenclrr
    }
    #[doc = "0x310 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_ahb5lpensetr(&self) -> &RCC_MP_AHB5LPENSETR {
        &self.rcc_mp_ahb5lpensetr
    }
    #[doc = "0x314 - This register is used by the MCU"]
    #[inline(always)]
    pub const fn rcc_mp_ahb5lpenclrr(&self) -> &RCC_MP_AHB5LPENCLRR {
        &self.rcc_mp_ahb5lpenclrr
    }
    #[doc = "0x318 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_ahb6lpensetr(&self) -> &RCC_MP_AHB6LPENSETR {
        &self.rcc_mp_ahb6lpensetr
    }
    #[doc = "0x31c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_ahb6lpenclrr(&self) -> &RCC_MP_AHB6LPENCLRR {
        &self.rcc_mp_ahb6lpenclrr
    }
    #[doc = "0x320 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_tzahb6lpensetr(&self) -> &RCC_MP_TZAHB6LPENSETR {
        &self.rcc_mp_tzahb6lpensetr
    }
    #[doc = "0x324 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_tzahb6lpenclrr(&self) -> &RCC_MP_TZAHB6LPENCLRR {
        &self.rcc_mp_tzahb6lpenclrr
    }
    #[doc = "0x380 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_apb4lpensetr(&self) -> &RCC_MC_APB4LPENSETR {
        &self.rcc_mc_apb4lpensetr
    }
    #[doc = "0x384 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb4lpenclrr(&self) -> &RCC_MC_APB4LPENCLRR {
        &self.rcc_mc_apb4lpenclrr
    }
    #[doc = "0x388 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_apb5lpensetr(&self) -> &RCC_MC_APB5LPENSETR {
        &self.rcc_mc_apb5lpensetr
    }
    #[doc = "0x38c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb5lpenclrr(&self) -> &RCC_MC_APB5LPENCLRR {
        &self.rcc_mc_apb5lpenclrr
    }
    #[doc = "0x390 - This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mc_ahb5lpensetr(&self) -> &RCC_MC_AHB5LPENSETR {
        &self.rcc_mc_ahb5lpensetr
    }
    #[doc = "0x394 - This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mc_ahb5lpenclrr(&self) -> &RCC_MC_AHB5LPENCLRR {
        &self.rcc_mc_ahb5lpenclrr
    }
    #[doc = "0x398 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_ahb6lpensetr(&self) -> &RCC_MC_AHB6LPENSETR {
        &self.rcc_mc_ahb6lpensetr
    }
    #[doc = "0x39c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb6lpenclrr(&self) -> &RCC_MC_AHB6LPENCLRR {
        &self.rcc_mc_ahb6lpenclrr
    }
    #[doc = "0x400 - This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_br_rstsclrr(&self) -> &RCC_BR_RSTSCLRR {
        &self.rcc_br_rstsclrr
    }
    #[doc = "0x404 - This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset."]
    #[inline(always)]
    pub const fn rcc_mp_grstcsetr(&self) -> &RCC_MP_GRSTCSETR {
        &self.rcc_mp_grstcsetr
    }
    #[doc = "0x408 - This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_rstsclrr(&self) -> &RCC_MP_RSTSCLRR {
        &self.rcc_mp_rstsclrr
    }
    #[doc = "0x40c - This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_iwdgfzsetr(&self) -> &RCC_MP_IWDGFZSETR {
        &self.rcc_mp_iwdgfzsetr
    }
    #[doc = "0x410 - This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_iwdgfzclrr(&self) -> &RCC_MP_IWDGFZCLRR {
        &self.rcc_mp_iwdgfzclrr
    }
    #[doc = "0x414 - This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_cier(&self) -> &RCC_MP_CIER {
        &self.rcc_mp_cier
    }
    #[doc = "0x418 - This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_cifr(&self) -> &RCC_MP_CIFR {
        &self.rcc_mp_cifr
    }
    #[doc = "0x41c - This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_pwrlpdlycr(&self) -> &RCC_PWRLPDLYCR {
        &self.rcc_pwrlpdlycr
    }
    #[doc = "0x420 - This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mp_rstssetr(&self) -> &RCC_MP_RSTSSETR {
        &self.rcc_mp_rstssetr
    }
    #[doc = "0x800 - This register is used to select the clock generated on MCO1 output."]
    #[inline(always)]
    pub const fn rcc_mco1cfgr(&self) -> &RCC_MCO1CFGR {
        &self.rcc_mco1cfgr
    }
    #[doc = "0x804 - This register is used to select the clock generated on MCO2 output."]
    #[inline(always)]
    pub const fn rcc_mco2cfgr(&self) -> &RCC_MCO2CFGR {
        &self.rcc_mco2cfgr
    }
    #[doc = "0x808 - This is a read-only access register, It contains the status flags of oscillators. Writing has no effect."]
    #[inline(always)]
    pub const fn rcc_ocrdyr(&self) -> &RCC_OCRDYR {
        &self.rcc_ocrdyr
    }
    #[doc = "0x80c - This is register contains the enable control of the debug and trace function, and the clock divider for the trace function."]
    #[inline(always)]
    pub const fn rcc_dbgcfgr(&self) -> &RCC_DBGCFGR {
        &self.rcc_dbgcfgr
    }
    #[doc = "0x820 - This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_rck3selr(&self) -> &RCC_RCK3SELR {
        &self.rcc_rck3selr
    }
    #[doc = "0x824 - This register is used to select the reference clock for PLL4."]
    #[inline(always)]
    pub const fn rcc_rck4selr(&self) -> &RCC_RCK4SELR {
        &self.rcc_rck4selr
    }
    #[doc = "0x828 - This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information."]
    #[inline(always)]
    pub const fn rcc_timg1prer(&self) -> &RCC_TIMG1PRER {
        &self.rcc_timg1prer
    }
    #[doc = "0x82c - This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information."]
    #[inline(always)]
    pub const fn rcc_timg2prer(&self) -> &RCC_TIMG2PRER {
        &self.rcc_timg2prer
    }
    #[doc = "0x830 - This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_mcudivr(&self) -> &RCC_MCUDIVR {
        &self.rcc_mcudivr
    }
    #[doc = "0x834 - This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information."]
    #[inline(always)]
    pub const fn rcc_apb1divr(&self) -> &RCC_APB1DIVR {
        &self.rcc_apb1divr
    }
    #[doc = "0x838 - This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
    #[inline(always)]
    pub const fn rcc_apb2divr(&self) -> &RCC_APB2DIVR {
        &self.rcc_apb2divr
    }
    #[doc = "0x83c - This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
    #[inline(always)]
    pub const fn rcc_apb3divr(&self) -> &RCC_APB3DIVR {
        &self.rcc_apb3divr
    }
    #[doc = "0x880 - This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_pll3cr(&self) -> &RCC_PLL3CR {
        &self.rcc_pll3cr
    }
    #[doc = "0x884 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_pll3cfgr1(&self) -> &RCC_PLL3CFGR1 {
        &self.rcc_pll3cfgr1
    }
    #[doc = "0x888 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_pll3cfgr2(&self) -> &RCC_PLL3CFGR2 {
        &self.rcc_pll3cfgr2
    }
    #[doc = "0x88c - This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_pll3fracr(&self) -> &RCC_PLL3FRACR {
        &self.rcc_pll3fracr
    }
    #[doc = "0x890 - This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_pll3csgr(&self) -> &RCC_PLL3CSGR {
        &self.rcc_pll3csgr
    }
    #[doc = "0x894 - This register is used to control the PLL4."]
    #[inline(always)]
    pub const fn rcc_pll4cr(&self) -> &RCC_PLL4CR {
        &self.rcc_pll4cr
    }
    #[doc = "0x898 - This register is used to configure the PLL4."]
    #[inline(always)]
    pub const fn rcc_pll4cfgr1(&self) -> &RCC_PLL4CFGR1 {
        &self.rcc_pll4cfgr1
    }
    #[doc = "0x89c - This register is used to configure the PLL4."]
    #[inline(always)]
    pub const fn rcc_pll4cfgr2(&self) -> &RCC_PLL4CFGR2 {
        &self.rcc_pll4cfgr2
    }
    #[doc = "0x8a0 - This register is used to fine-tune the frequency of the PLL4 VCO."]
    #[inline(always)]
    pub const fn rcc_pll4fracr(&self) -> &RCC_PLL4FRACR {
        &self.rcc_pll4fracr
    }
    #[doc = "0x8a4 - This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    #[inline(always)]
    pub const fn rcc_pll4csgr(&self) -> &RCC_PLL4CSGR {
        &self.rcc_pll4csgr
    }
    #[doc = "0x8c0 - This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_i2c12ckselr(&self) -> &RCC_I2C12CKSELR {
        &self.rcc_i2c12ckselr
    }
    #[doc = "0x8c4 - This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_i2c35ckselr(&self) -> &RCC_I2C35CKSELR {
        &self.rcc_i2c35ckselr
    }
    #[doc = "0x8c8 - This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_sai1ckselr(&self) -> &RCC_SAI1CKSELR {
        &self.rcc_sai1ckselr
    }
    #[doc = "0x8cc - This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_sai2ckselr(&self) -> &RCC_SAI2CKSELR {
        &self.rcc_sai2ckselr
    }
    #[doc = "0x8d0 - This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_sai3ckselr(&self) -> &RCC_SAI3CKSELR {
        &self.rcc_sai3ckselr
    }
    #[doc = "0x8d4 - This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_sai4ckselr(&self) -> &RCC_SAI4CKSELR {
        &self.rcc_sai4ckselr
    }
    #[doc = "0x8d8 - This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_spi2s1ckselr(&self) -> &RCC_SPI2S1CKSELR {
        &self.rcc_spi2s1ckselr
    }
    #[doc = "0x8dc - This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_spi2s23ckselr(&self) -> &RCC_SPI2S23CKSELR {
        &self.rcc_spi2s23ckselr
    }
    #[doc = "0x8e0 - This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_spi45ckselr(&self) -> &RCC_SPI45CKSELR {
        &self.rcc_spi45ckselr
    }
    #[doc = "0x8e4 - This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_uart6ckselr(&self) -> &RCC_UART6CKSELR {
        &self.rcc_uart6ckselr
    }
    #[doc = "0x8e8 - This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_uart24ckselr(&self) -> &RCC_UART24CKSELR {
        &self.rcc_uart24ckselr
    }
    #[doc = "0x8ec - This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_uart35ckselr(&self) -> &RCC_UART35CKSELR {
        &self.rcc_uart35ckselr
    }
    #[doc = "0x8f0 - This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_uart78ckselr(&self) -> &RCC_UART78CKSELR {
        &self.rcc_uart78ckselr
    }
    #[doc = "0x8f4 - This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_sdmmc12ckselr(&self) -> &RCC_SDMMC12CKSELR {
        &self.rcc_sdmmc12ckselr
    }
    #[doc = "0x8f8 - This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_sdmmc3ckselr(&self) -> &RCC_SDMMC3CKSELR {
        &self.rcc_sdmmc3ckselr
    }
    #[doc = "0x8fc - This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_ethckselr(&self) -> &RCC_ETHCKSELR {
        &self.rcc_ethckselr
    }
    #[doc = "0x900 - This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_qspickselr(&self) -> &RCC_QSPICKSELR {
        &self.rcc_qspickselr
    }
    #[doc = "0x904 - This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_fmcckselr(&self) -> &RCC_FMCCKSELR {
        &self.rcc_fmcckselr
    }
    #[doc = "0x90c - This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_fdcanckselr(&self) -> &RCC_FDCANCKSELR {
        &self.rcc_fdcanckselr
    }
    #[doc = "0x914 - This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    #[inline(always)]
    pub const fn rcc_spdifckselr(&self) -> &RCC_SPDIFCKSELR {
        &self.rcc_spdifckselr
    }
    #[doc = "0x918 - This register is used to control the selection of the kernel clock for the CEC-HDMI."]
    #[inline(always)]
    pub const fn rcc_cecckselr(&self) -> &RCC_CECCKSELR {
        &self.rcc_cecckselr
    }
    #[doc = "0x91c - This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG"]
    #[inline(always)]
    pub const fn rcc_usbckselr(&self) -> &RCC_USBCKSELR {
        &self.rcc_usbckselr
    }
    #[doc = "0x920 - This register is used to control the selection of the kernel clock for the RNG2."]
    #[inline(always)]
    pub const fn rcc_rng2ckselr(&self) -> &RCC_RNG2CKSELR {
        &self.rcc_rng2ckselr
    }
    #[doc = "0x924 - This register is used to control the selection of the kernel clock for the DSI block."]
    #[inline(always)]
    pub const fn rcc_dsickselr(&self) -> &RCC_DSICKSELR {
        &self.rcc_dsickselr
    }
    #[doc = "0x928 - This register is used to control the selection of the kernel clock for the ADC block."]
    #[inline(always)]
    pub const fn rcc_adcckselr(&self) -> &RCC_ADCCKSELR {
        &self.rcc_adcckselr
    }
    #[doc = "0x92c - This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks."]
    #[inline(always)]
    pub const fn rcc_lptim45ckselr(&self) -> &RCC_LPTIM45CKSELR {
        &self.rcc_lptim45ckselr
    }
    #[doc = "0x930 - This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks."]
    #[inline(always)]
    pub const fn rcc_lptim23ckselr(&self) -> &RCC_LPTIM23CKSELR {
        &self.rcc_lptim23ckselr
    }
    #[doc = "0x934 - This register is used to control the selection of the kernel clock for the LPTIM1 block."]
    #[inline(always)]
    pub const fn rcc_lptim1ckselr(&self) -> &RCC_LPTIM1CKSELR {
        &self.rcc_lptim1ckselr
    }
    #[doc = "0x980 - This register is used to activate the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb1rstsetr(&self) -> &RCC_APB1RSTSETR {
        &self.rcc_apb1rstsetr
    }
    #[doc = "0x984 - This register is used to release the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb1rstclrr(&self) -> &RCC_APB1RSTCLRR {
        &self.rcc_apb1rstclrr
    }
    #[doc = "0x988 - This register is used to activate the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb2rstsetr(&self) -> &RCC_APB2RSTSETR {
        &self.rcc_apb2rstsetr
    }
    #[doc = "0x98c - This register is used to release the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb2rstclrr(&self) -> &RCC_APB2RSTCLRR {
        &self.rcc_apb2rstclrr
    }
    #[doc = "0x990 - This register is used to activate the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb3rstsetr(&self) -> &RCC_APB3RSTSETR {
        &self.rcc_apb3rstsetr
    }
    #[doc = "0x994 - This register is used to release the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_apb3rstclrr(&self) -> &RCC_APB3RSTCLRR {
        &self.rcc_apb3rstclrr
    }
    #[doc = "0x998 - This register is used to activate the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_ahb2rstsetr(&self) -> &RCC_AHB2RSTSETR {
        &self.rcc_ahb2rstsetr
    }
    #[doc = "0x99c - This register is used to release the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_ahb2rstclrr(&self) -> &RCC_AHB2RSTCLRR {
        &self.rcc_ahb2rstclrr
    }
    #[doc = "0x9a0 - This register is used to activate the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_ahb3rstsetr(&self) -> &RCC_AHB3RSTSETR {
        &self.rcc_ahb3rstsetr
    }
    #[doc = "0x9a4 - This register is used to release the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_ahb3rstclrr(&self) -> &RCC_AHB3RSTCLRR {
        &self.rcc_ahb3rstclrr
    }
    #[doc = "0x9a8 - This register is used to activate the reset of the corresponding peripheral"]
    #[inline(always)]
    pub const fn rcc_ahb4rstsetr(&self) -> &RCC_AHB4RSTSETR {
        &self.rcc_ahb4rstsetr
    }
    #[doc = "0x9ac - This register is used to release the reset of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_ahb4rstclrr(&self) -> &RCC_AHB4RSTCLRR {
        &self.rcc_ahb4rstclrr
    }
    #[doc = "0xa00 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mp_apb1ensetr(&self) -> &RCC_MP_APB1ENSETR {
        &self.rcc_mp_apb1ensetr
    }
    #[doc = "0xa04 - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mp_apb1enclrr(&self) -> &RCC_MP_APB1ENCLRR {
        &self.rcc_mp_apb1enclrr
    }
    #[doc = "0xa08 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mp_apb2ensetr(&self) -> &RCC_MP_APB2ENSETR {
        &self.rcc_mp_apb2ensetr
    }
    #[doc = "0xa0c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mp_apb2enclrr(&self) -> &RCC_MP_APB2ENCLRR {
        &self.rcc_mp_apb2enclrr
    }
    #[doc = "0xa10 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mp_apb3ensetr(&self) -> &RCC_MP_APB3ENSETR {
        &self.rcc_mp_apb3ensetr
    }
    #[doc = "0xa14 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mp_apb3enclrr(&self) -> &RCC_MP_APB3ENCLRR {
        &self.rcc_mp_apb3enclrr
    }
    #[doc = "0xa18 - This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
    #[inline(always)]
    pub const fn rcc_mp_ahb2ensetr(&self) -> &RCC_MP_AHB2ENSETR {
        &self.rcc_mp_ahb2ensetr
    }
    #[doc = "0xa1c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mp_ahb2enclrr(&self) -> &RCC_MP_AHB2ENCLRR {
        &self.rcc_mp_ahb2enclrr
    }
    #[doc = "0xa20 - This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
    #[inline(always)]
    pub const fn rcc_mp_ahb3ensetr(&self) -> &RCC_MP_AHB3ENSETR {
        &self.rcc_mp_ahb3ensetr
    }
    #[doc = "0xa24 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mp_ahb3enclrr(&self) -> &RCC_MP_AHB3ENCLRR {
        &self.rcc_mp_ahb3enclrr
    }
    #[doc = "0xa28 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU."]
    #[inline(always)]
    pub const fn rcc_mp_ahb4ensetr(&self) -> &RCC_MP_AHB4ENSETR {
        &self.rcc_mp_ahb4ensetr
    }
    #[doc = "0xa2c - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mp_ahb4enclrr(&self) -> &RCC_MP_AHB4ENCLRR {
        &self.rcc_mp_ahb4enclrr
    }
    #[doc = "0xa38 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mp_mlahbensetr(&self) -> &RCC_MP_MLAHBENSETR {
        &self.rcc_mp_mlahbensetr
    }
    #[doc = "0xa3c - This register is used to clear the peripheral clock enable bit."]
    #[inline(always)]
    pub const fn rcc_mp_mlahbenclrr(&self) -> &RCC_MP_MLAHBENCLRR {
        &self.rcc_mp_mlahbenclrr
    }
    #[doc = "0xa80 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to ."]
    #[inline(always)]
    pub const fn rcc_mc_apb1ensetr(&self) -> &RCC_MC_APB1ENSETR {
        &self.rcc_mc_apb1ensetr
    }
    #[doc = "0xa84 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mc_apb1enclrr(&self) -> &RCC_MC_APB1ENCLRR {
        &self.rcc_mc_apb1enclrr
    }
    #[doc = "0xa88 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb2ensetr(&self) -> &RCC_MC_APB2ENSETR {
        &self.rcc_mc_apb2ensetr
    }
    #[doc = "0xa8c - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb2enclrr(&self) -> &RCC_MC_APB2ENCLRR {
        &self.rcc_mc_apb2enclrr
    }
    #[doc = "0xa90 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb3ensetr(&self) -> &RCC_MC_APB3ENSETR {
        &self.rcc_mc_apb3ensetr
    }
    #[doc = "0xa94 - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb3enclrr(&self) -> &RCC_MC_APB3ENCLRR {
        &self.rcc_mc_apb3enclrr
    }
    #[doc = "0xa98 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb2ensetr(&self) -> &RCC_MC_AHB2ENSETR {
        &self.rcc_mc_ahb2ensetr
    }
    #[doc = "0xa9c - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb2enclrr(&self) -> &RCC_MC_AHB2ENCLRR {
        &self.rcc_mc_ahb2enclrr
    }
    #[doc = "0xaa0 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb3ensetr(&self) -> &RCC_MC_AHB3ENSETR {
        &self.rcc_mc_ahb3ensetr
    }
    #[doc = "0xaa4 - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb3enclrr(&self) -> &RCC_MC_AHB3ENCLRR {
        &self.rcc_mc_ahb3enclrr
    }
    #[doc = "0xaa8 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb4ensetr(&self) -> &RCC_MC_AHB4ENSETR {
        &self.rcc_mc_ahb4ensetr
    }
    #[doc = "0xaac - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb4enclrr(&self) -> &RCC_MC_AHB4ENCLRR {
        &self.rcc_mc_ahb4enclrr
    }
    #[doc = "0xab0 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_aximensetr(&self) -> &RCC_MC_AXIMENSETR {
        &self.rcc_mc_aximensetr
    }
    #[doc = "0xab4 - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_aximenclrr(&self) -> &RCC_MC_AXIMENCLRR {
        &self.rcc_mc_aximenclrr
    }
    #[doc = "0xab8 - This register is used to set the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_mlahbensetr(&self) -> &RCC_MC_MLAHBENSETR {
        &self.rcc_mc_mlahbensetr
    }
    #[doc = "0xabc - This register is used to clear the peripheral clock enable bit"]
    #[inline(always)]
    pub const fn rcc_mc_mlahbenclrr(&self) -> &RCC_MC_MLAHBENCLRR {
        &self.rcc_mc_mlahbenclrr
    }
    #[doc = "0xb00 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_apb1lpensetr(&self) -> &RCC_MP_APB1LPENSETR {
        &self.rcc_mp_apb1lpensetr
    }
    #[doc = "0xb04 - This register is used by the MPU in order to clear the PERxLPEN bits ."]
    #[inline(always)]
    pub const fn rcc_mp_apb1lpenclrr(&self) -> &RCC_MP_APB1LPENCLRR {
        &self.rcc_mp_apb1lpenclrr
    }
    #[doc = "0xb08 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_apb2lpensetr(&self) -> &RCC_MP_APB2LPENSETR {
        &self.rcc_mp_apb2lpensetr
    }
    #[doc = "0xb0c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_apb2lpenclrr(&self) -> &RCC_MP_APB2LPENCLRR {
        &self.rcc_mp_apb2lpenclrr
    }
    #[doc = "0xb10 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_apb3lpensetr(&self) -> &RCC_MP_APB3LPENSETR {
        &self.rcc_mp_apb3lpensetr
    }
    #[doc = "0xb14 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_apb3lpenclrr(&self) -> &RCC_MP_APB3LPENCLRR {
        &self.rcc_mp_apb3lpenclrr
    }
    #[doc = "0xb18 - This register is used by the MPU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mp_ahb2lpensetr(&self) -> &RCC_MP_AHB2LPENSETR {
        &self.rcc_mp_ahb2lpensetr
    }
    #[doc = "0xb1c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mp_ahb2lpenclrr(&self) -> &RCC_MP_AHB2LPENCLRR {
        &self.rcc_mp_ahb2lpenclrr
    }
    #[doc = "0xb20 - This register is used by the MPU"]
    #[inline(always)]
    pub const fn rcc_mp_ahb3lpensetr(&self) -> &RCC_MP_AHB3LPENSETR {
        &self.rcc_mp_ahb3lpensetr
    }
    #[doc = "0xb24 - This register is used by the MPU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mp_ahb3lpenclrr(&self) -> &RCC_MP_AHB3LPENCLRR {
        &self.rcc_mp_ahb3lpenclrr
    }
    #[doc = "0xb28 - This register is used by the MPU"]
    #[inline(always)]
    pub const fn rcc_mp_ahb4lpensetr(&self) -> &RCC_MP_AHB4LPENSETR {
        &self.rcc_mp_ahb4lpensetr
    }
    #[doc = "0xb2c - This register is used by the MPU"]
    #[inline(always)]
    pub const fn rcc_mp_ahb4lpenclrr(&self) -> &RCC_MP_AHB4LPENCLRR {
        &self.rcc_mp_ahb4lpenclrr
    }
    #[doc = "0xb30 - This register is used by the MPU"]
    #[inline(always)]
    pub const fn rcc_mp_aximlpensetr(&self) -> &RCC_MP_AXIMLPENSETR {
        &self.rcc_mp_aximlpensetr
    }
    #[doc = "0xb34 - This register is used by the MPU"]
    #[inline(always)]
    pub const fn rcc_mp_aximlpenclrr(&self) -> &RCC_MP_AXIMLPENCLRR {
        &self.rcc_mp_aximlpenclrr
    }
    #[doc = "0xb38 - This register is used by the MPU in order to set the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mp_mlahblpensetr(&self) -> &RCC_MP_MLAHBLPENSETR {
        &self.rcc_mp_mlahblpensetr
    }
    #[doc = "0xb3c - This register is used by the MPU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mp_mlahblpenclrr(&self) -> &RCC_MP_MLAHBLPENCLRR {
        &self.rcc_mp_mlahblpenclrr
    }
    #[doc = "0xb80 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_apb1lpensetr(&self) -> &RCC_MC_APB1LPENSETR {
        &self.rcc_mc_apb1lpensetr
    }
    #[doc = "0xb84 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    #[inline(always)]
    pub const fn rcc_mc_apb1lpenclrr(&self) -> &RCC_MC_APB1LPENCLRR {
        &self.rcc_mc_apb1lpenclrr
    }
    #[doc = "0xb88 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_apb2lpensetr(&self) -> &RCC_MC_APB2LPENSETR {
        &self.rcc_mc_apb2lpensetr
    }
    #[doc = "0xb8c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb2lpenclrr(&self) -> &RCC_MC_APB2LPENCLRR {
        &self.rcc_mc_apb2lpenclrr
    }
    #[doc = "0xb90 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_apb3lpensetr(&self) -> &RCC_MC_APB3LPENSETR {
        &self.rcc_mc_apb3lpensetr
    }
    #[doc = "0xb94 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mc_apb3lpenclrr(&self) -> &RCC_MC_APB3LPENCLRR {
        &self.rcc_mc_apb3lpenclrr
    }
    #[doc = "0xb98 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_ahb2lpensetr(&self) -> &RCC_MC_AHB2LPENSETR {
        &self.rcc_mc_ahb2lpensetr
    }
    #[doc = "0xb9c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb2lpenclrr(&self) -> &RCC_MC_AHB2LPENCLRR {
        &self.rcc_mc_ahb2lpenclrr
    }
    #[doc = "0xba0 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_ahb3lpensetr(&self) -> &RCC_MC_AHB3LPENSETR {
        &self.rcc_mc_ahb3lpensetr
    }
    #[doc = "0xba4 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    #[inline(always)]
    pub const fn rcc_mc_ahb3lpenclrr(&self) -> &RCC_MC_AHB3LPENCLRR {
        &self.rcc_mc_ahb3lpenclrr
    }
    #[doc = "0xba8 - This register is used by the MCU in order to set the PERxLPEN bit."]
    #[inline(always)]
    pub const fn rcc_mc_ahb4lpensetr(&self) -> &RCC_MC_AHB4LPENSETR {
        &self.rcc_mc_ahb4lpensetr
    }
    #[doc = "0xbac - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mc_ahb4lpenclrr(&self) -> &RCC_MC_AHB4LPENCLRR {
        &self.rcc_mc_ahb4lpenclrr
    }
    #[doc = "0xbb0 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mc_aximlpensetr(&self) -> &RCC_MC_AXIMLPENSETR {
        &self.rcc_mc_aximlpensetr
    }
    #[doc = "0xbb4 - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mc_aximlpenclrr(&self) -> &RCC_MC_AXIMLPENCLRR {
        &self.rcc_mc_aximlpenclrr
    }
    #[doc = "0xbb8 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mc_mlahblpensetr(&self) -> &RCC_MC_MLAHBLPENSETR {
        &self.rcc_mc_mlahblpensetr
    }
    #[doc = "0xbbc - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    #[inline(always)]
    pub const fn rcc_mc_mlahblpenclrr(&self) -> &RCC_MC_MLAHBLPENCLRR {
        &self.rcc_mc_mlahblpenclrr
    }
    #[doc = "0xc00 - This register is used by the MCU to check the reset source."]
    #[inline(always)]
    pub const fn rcc_mc_rstsclrr(&self) -> &RCC_MC_RSTSCLRR {
        &self.rcc_mc_rstsclrr
    }
    #[doc = "0xc14 - This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details."]
    #[inline(always)]
    pub const fn rcc_mc_cier(&self) -> &RCC_MC_CIER {
        &self.rcc_mc_cier
    }
    #[doc = "0xc18 - This register shall be used by the MCU in order to read and clear the interrupt flags."]
    #[inline(always)]
    pub const fn rcc_mc_cifr(&self) -> &RCC_MC_CIFR {
        &self.rcc_mc_cifr
    }
    #[doc = "0xff4 - This register gives the IP version"]
    #[inline(always)]
    pub const fn rcc_verr(&self) -> &RCC_VERR {
        &self.rcc_verr
    }
    #[doc = "0xff8 - This register gives the unique identifier of the RCC"]
    #[inline(always)]
    pub const fn rcc_idr(&self) -> &RCC_IDR {
        &self.rcc_idr
    }
    #[doc = "0xffc - This register gives the decoding space, which is for the RCC of 4 kB."]
    #[inline(always)]
    pub const fn rcc_sidr(&self) -> &RCC_SIDR {
        &self.rcc_sidr
    }
}
#[doc = "RCC_TZCR (rw) register accessor: This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_tzcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_tzcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_tzcr`]
module"]
pub type RCC_TZCR = crate::Reg<rcc_tzcr::RCC_TZCRrs>;
#[doc = "This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode."]
pub mod rcc_tzcr;
#[doc = "RCC_OCENSETR (rw) register accessor: This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ocensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ocensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ocensetr`]
module"]
pub type RCC_OCENSETR = crate::Reg<rcc_ocensetr::RCC_OCENSETRrs>;
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_ocensetr;
#[doc = "RCC_OCENCLRR (rw) register accessor: This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ocenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ocenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ocenclrr`]
module"]
pub type RCC_OCENCLRR = crate::Reg<rcc_ocenclrr::RCC_OCENCLRRrs>;
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_ocenclrr;
#[doc = "RCC_HSICFGR (rw) register accessor: This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_hsicfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_hsicfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_hsicfgr`]
module"]
pub type RCC_HSICFGR = crate::Reg<rcc_hsicfgr::RCC_HSICFGRrs>;
#[doc = "This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_hsicfgr;
#[doc = "RCC_CSICFGR (rw) register accessor: This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_csicfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_csicfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_csicfgr`]
module"]
pub type RCC_CSICFGR = crate::Reg<rcc_csicfgr::RCC_CSICFGRrs>;
#[doc = "This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details."]
pub mod rcc_csicfgr;
#[doc = "RCC_MPCKSELR (rw) register accessor: This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mpckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mpckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mpckselr`]
module"]
pub type RCC_MPCKSELR = crate::Reg<rcc_mpckselr::RCC_MPCKSELRrs>;
#[doc = "This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_mpckselr;
#[doc = "RCC_ASSCKSELR (rw) register accessor: This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_assckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_assckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_assckselr`]
module"]
pub type RCC_ASSCKSELR = crate::Reg<rcc_assckselr::RCC_ASSCKSELRrs>;
#[doc = "This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_assckselr;
#[doc = "RCC_RCK12SELR (rw) register accessor: This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rck12selr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rck12selr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_rck12selr`]
module"]
pub type RCC_RCK12SELR = crate::Reg<rcc_rck12selr::RCC_RCK12SELRrs>;
#[doc = "This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_rck12selr;
#[doc = "RCC_MPCKDIVR (rw) register accessor: This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mpckdivr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mpckdivr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mpckdivr`]
module"]
pub type RCC_MPCKDIVR = crate::Reg<rcc_mpckdivr::RCC_MPCKDIVRrs>;
#[doc = "This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mpckdivr;
#[doc = "RCC_AXIDIVR (rw) register accessor: This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_axidivr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_axidivr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_axidivr`]
module"]
pub type RCC_AXIDIVR = crate::Reg<rcc_axidivr::RCC_AXIDIVRrs>;
#[doc = "This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_axidivr;
#[doc = "RCC_APB4DIVR (rw) register accessor: This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb4divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb4divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb4divr`]
module"]
pub type RCC_APB4DIVR = crate::Reg<rcc_apb4divr::RCC_APB4DIVRrs>;
#[doc = "This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb4divr;
#[doc = "RCC_APB5DIVR (rw) register accessor: This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb5divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb5divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb5divr`]
module"]
pub type RCC_APB5DIVR = crate::Reg<rcc_apb5divr::RCC_APB5DIVRrs>;
#[doc = "This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5divr;
#[doc = "RCC_RTCDIVR (rw) register accessor: This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rtcdivr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rtcdivr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_rtcdivr`]
module"]
pub type RCC_RTCDIVR = crate::Reg<rcc_rtcdivr::RCC_RTCDIVRrs>;
#[doc = "This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rtcdivr;
#[doc = "RCC_MSSCKSELR (rw) register accessor: This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mssckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mssckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mssckselr`]
module"]
pub type RCC_MSSCKSELR = crate::Reg<rcc_mssckselr::RCC_MSSCKSELRrs>;
#[doc = "This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_mssckselr;
#[doc = "RCC_PLL1CR (rw) register accessor: This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1cr`]
module"]
pub type RCC_PLL1CR = crate::Reg<rcc_pll1cr::RCC_PLL1CRrs>;
#[doc = "This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cr;
#[doc = "RCC_PLL1CFGR1 (rw) register accessor: This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1cfgr1`]
module"]
pub type RCC_PLL1CFGR1 = crate::Reg<rcc_pll1cfgr1::RCC_PLL1CFGR1rs>;
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cfgr1;
#[doc = "RCC_PLL1CFGR2 (rw) register accessor: This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1cfgr2`]
module"]
pub type RCC_PLL1CFGR2 = crate::Reg<rcc_pll1cfgr2::RCC_PLL1CFGR2rs>;
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cfgr2;
#[doc = "RCC_PLL1FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1fracr`]
module"]
pub type RCC_PLL1FRACR = crate::Reg<rcc_pll1fracr::RCC_PLL1FRACRrs>;
#[doc = "This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1fracr;
#[doc = "RCC_PLL1CSGR (rw) register accessor: This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll1csgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll1csgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll1csgr`]
module"]
pub type RCC_PLL1CSGR = crate::Reg<rcc_pll1csgr::RCC_PLL1CSGRrs>;
#[doc = "This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1csgr;
#[doc = "RCC_PLL2CR (rw) register accessor: This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2cr`]
module"]
pub type RCC_PLL2CR = crate::Reg<rcc_pll2cr::RCC_PLL2CRrs>;
#[doc = "This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cr;
#[doc = "RCC_PLL2CFGR1 (rw) register accessor: This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2cfgr1`]
module"]
pub type RCC_PLL2CFGR1 = crate::Reg<rcc_pll2cfgr1::RCC_PLL2CFGR1rs>;
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cfgr1;
#[doc = "RCC_PLL2CFGR2 (rw) register accessor: This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2cfgr2`]
module"]
pub type RCC_PLL2CFGR2 = crate::Reg<rcc_pll2cfgr2::RCC_PLL2CFGR2rs>;
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cfgr2;
#[doc = "RCC_PLL2FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2fracr`]
module"]
pub type RCC_PLL2FRACR = crate::Reg<rcc_pll2fracr::RCC_PLL2FRACRrs>;
#[doc = "This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2fracr;
#[doc = "RCC_PLL2CSGR (rw) register accessor: This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll2csgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll2csgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll2csgr`]
module"]
pub type RCC_PLL2CSGR = crate::Reg<rcc_pll2csgr::RCC_PLL2CSGRrs>;
#[doc = "This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2csgr;
#[doc = "RCC_I2C46CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_i2c46ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_i2c46ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_i2c46ckselr`]
module"]
pub type RCC_I2C46CKSELR = crate::Reg<rcc_i2c46ckselr::RCC_I2C46CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_i2c46ckselr;
#[doc = "RCC_SPI6CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_spi6ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_spi6ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_spi6ckselr`]
module"]
pub type RCC_SPI6CKSELR = crate::Reg<rcc_spi6ckselr::RCC_SPI6CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_spi6ckselr;
#[doc = "RCC_UART1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_uart1ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_uart1ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_uart1ckselr`]
module"]
pub type RCC_UART1CKSELR = crate::Reg<rcc_uart1ckselr::RCC_UART1CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_uart1ckselr;
#[doc = "RCC_RNG1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rng1ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rng1ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_rng1ckselr`]
module"]
pub type RCC_RNG1CKSELR = crate::Reg<rcc_rng1ckselr::RCC_RNG1CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rng1ckselr;
#[doc = "RCC_CPERCKSELR (rw) register accessor: This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cperckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cperckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cperckselr`]
module"]
pub type RCC_CPERCKSELR = crate::Reg<rcc_cperckselr::RCC_CPERCKSELRrs>;
#[doc = "This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays."]
pub mod rcc_cperckselr;
#[doc = "RCC_STGENCKSELR (rw) register accessor: This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_stgenckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_stgenckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_stgenckselr`]
module"]
pub type RCC_STGENCKSELR = crate::Reg<rcc_stgenckselr::RCC_STGENCKSELRrs>;
#[doc = "This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_stgenckselr;
#[doc = "RCC_DDRITFCR (rw) register accessor: This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ddritfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ddritfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ddritfcr`]
module"]
pub type RCC_DDRITFCR = crate::Reg<rcc_ddritfcr::RCC_DDRITFCRrs>;
#[doc = "This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ddritfcr;
#[doc = "RCC_MP_BOOTCR (rw) register accessor: This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_bootcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_bootcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_bootcr`]
module"]
pub type RCC_MP_BOOTCR = crate::Reg<rcc_mp_bootcr::RCC_MP_BOOTCRrs>;
#[doc = "This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU."]
pub mod rcc_mp_bootcr;
#[doc = "RCC_MP_SREQSETR (rw) register accessor: Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_sreqsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_sreqsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_sreqsetr`]
module"]
pub type RCC_MP_SREQSETR = crate::Reg<rcc_mp_sreqsetr::RCC_MP_SREQSETRrs>;
#[doc = "Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_sreqsetr;
#[doc = "RCC_MP_SREQCLRR (rw) register accessor: Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_sreqclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_sreqclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_sreqclrr`]
module"]
pub type RCC_MP_SREQCLRR = crate::Reg<rcc_mp_sreqclrr::RCC_MP_SREQCLRRrs>;
#[doc = "Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_sreqclrr;
#[doc = "RCC_MP_GCR (rw) register accessor: The register contains global control bits. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_gcr`]
module"]
pub type RCC_MP_GCR = crate::Reg<rcc_mp_gcr::RCC_MP_GCRrs>;
#[doc = "The register contains global control bits. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_gcr;
#[doc = "RCC_MP_APRSTCR (rw) register accessor: This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_aprstcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_aprstcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_aprstcr`]
module"]
pub type RCC_MP_APRSTCR = crate::Reg<rcc_mp_aprstcr::RCC_MP_APRSTCRrs>;
#[doc = "This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_aprstcr;
#[doc = "RCC_MP_APRSTSR (r) register accessor: This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_aprstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_aprstsr`]
module"]
pub type RCC_MP_APRSTSR = crate::Reg<rcc_mp_aprstsr::RCC_MP_APRSTSRrs>;
#[doc = "This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_aprstsr;
#[doc = "RCC_BDCR (rw) register accessor: This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_bdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_bdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_bdcr`]
module"]
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCRrs>;
#[doc = "This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_bdcr;
#[doc = "RCC_RDLSICR (rw) register accessor: This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rdlsicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rdlsicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_rdlsicr`]
module"]
pub type RCC_RDLSICR = crate::Reg<rcc_rdlsicr::RCC_RDLSICRrs>;
#[doc = "This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rdlsicr;
#[doc = "RCC_APB4RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb4rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb4rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb4rstsetr`]
module"]
pub type RCC_APB4RSTSETR = crate::Reg<rcc_apb4rstsetr::RCC_APB4RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
pub mod rcc_apb4rstsetr;
#[doc = "RCC_APB4RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb4rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb4rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb4rstclrr`]
module"]
pub type RCC_APB4RSTCLRR = crate::Reg<rcc_apb4rstclrr::RCC_APB4RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
pub mod rcc_apb4rstclrr;
#[doc = "RCC_APB5RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb5rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb5rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb5rstsetr`]
module"]
pub type RCC_APB5RSTSETR = crate::Reg<rcc_apb5rstsetr::RCC_APB5RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5rstsetr;
#[doc = "RCC_APB5RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb5rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb5rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb5rstclrr`]
module"]
pub type RCC_APB5RSTCLRR = crate::Reg<rcc_apb5rstclrr::RCC_APB5RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5rstclrr;
#[doc = "RCC_AHB5RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb5rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb5rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb5rstsetr`]
module"]
pub type RCC_AHB5RSTSETR = crate::Reg<rcc_ahb5rstsetr::RCC_AHB5RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ahb5rstsetr;
#[doc = "RCC_AHB5RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb5rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb5rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb5rstclrr`]
module"]
pub type RCC_AHB5RSTCLRR = crate::Reg<rcc_ahb5rstclrr::RCC_AHB5RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ahb5rstclrr;
#[doc = "RCC_AHB6RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb6rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb6rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb6rstsetr`]
module"]
pub type RCC_AHB6RSTSETR = crate::Reg<rcc_ahb6rstsetr::RCC_AHB6RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
pub mod rcc_ahb6rstsetr;
#[doc = "RCC_AHB6RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb6rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb6rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb6rstclrr`]
module"]
pub type RCC_AHB6RSTCLRR = crate::Reg<rcc_ahb6rstclrr::RCC_AHB6RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
pub mod rcc_ahb6rstclrr;
#[doc = "RCC_TZAHB6RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_tzahb6rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_tzahb6rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_tzahb6rstsetr`]
module"]
pub type RCC_TZAHB6RSTSETR = crate::Reg<rcc_tzahb6rstsetr::RCC_TZAHB6RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_tzahb6rstsetr;
#[doc = "RCC_TZAHB6RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_tzahb6rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_tzahb6rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_tzahb6rstclrr`]
module"]
pub type RCC_TZAHB6RSTCLRR = crate::Reg<rcc_tzahb6rstclrr::RCC_TZAHB6RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_tzahb6rstclrr;
#[doc = "RCC_MP_APB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb4ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb4ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb4ensetr`]
module"]
pub type RCC_MP_APB4ENSETR = crate::Reg<rcc_mp_apb4ensetr::RCC_MP_APB4ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb4ensetr;
#[doc = "RCC_MP_APB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb4enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb4enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb4enclrr`]
module"]
pub type RCC_MP_APB4ENCLRR = crate::Reg<rcc_mp_apb4enclrr::RCC_MP_APB4ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb4enclrr;
#[doc = "RCC_MP_APB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb5ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb5ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb5ensetr`]
module"]
pub type RCC_MP_APB5ENSETR = crate::Reg<rcc_mp_apb5ensetr::RCC_MP_APB5ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb5ensetr;
#[doc = "RCC_MP_APB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb5enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb5enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb5enclrr`]
module"]
pub type RCC_MP_APB5ENCLRR = crate::Reg<rcc_mp_apb5enclrr::RCC_MP_APB5ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb5enclrr;
#[doc = "RCC_MP_AHB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb5ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb5ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb5ensetr`]
module"]
pub type RCC_MP_AHB5ENSETR = crate::Reg<rcc_mp_ahb5ensetr::RCC_MP_AHB5ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5ensetr;
#[doc = "RCC_MP_AHB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb5enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb5enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb5enclrr`]
module"]
pub type RCC_MP_AHB5ENCLRR = crate::Reg<rcc_mp_ahb5enclrr::RCC_MP_AHB5ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5enclrr;
#[doc = "RCC_MP_AHB6ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb6ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb6ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb6ensetr`]
module"]
pub type RCC_MP_AHB6ENSETR = crate::Reg<rcc_mp_ahb6ensetr::RCC_MP_AHB6ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_ahb6ensetr;
#[doc = "RCC_MP_AHB6ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb6enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb6enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb6enclrr`]
module"]
pub type RCC_MP_AHB6ENCLRR = crate::Reg<rcc_mp_ahb6enclrr::RCC_MP_AHB6ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_ahb6enclrr;
#[doc = "RCC_MP_TZAHB6ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_tzahb6ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_tzahb6ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_tzahb6ensetr`]
module"]
pub type RCC_MP_TZAHB6ENSETR = crate::Reg<rcc_mp_tzahb6ensetr::RCC_MP_TZAHB6ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6ensetr;
#[doc = "RCC_MP_TZAHB6ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_tzahb6enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_tzahb6enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_tzahb6enclrr`]
module"]
pub type RCC_MP_TZAHB6ENCLRR = crate::Reg<rcc_mp_tzahb6enclrr::RCC_MP_TZAHB6ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6enclrr;
#[doc = "RCC_MC_APB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb4ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb4ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb4ensetr`]
module"]
pub type RCC_MC_APB4ENSETR = crate::Reg<rcc_mc_apb4ensetr::RCC_MC_APB4ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb4ensetr;
#[doc = "RCC_MC_APB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb4enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb4enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb4enclrr`]
module"]
pub type RCC_MC_APB4ENCLRR = crate::Reg<rcc_mc_apb4enclrr::RCC_MC_APB4ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb4enclrr;
#[doc = "RCC_MC_APB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb5ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb5ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb5ensetr`]
module"]
pub type RCC_MC_APB5ENSETR = crate::Reg<rcc_mc_apb5ensetr::RCC_MC_APB5ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb5ensetr;
#[doc = "RCC_MC_APB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb5enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb5enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb5enclrr`]
module"]
pub type RCC_MC_APB5ENCLRR = crate::Reg<rcc_mc_apb5enclrr::RCC_MC_APB5ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb5enclrr;
#[doc = "RCC_MC_AHB5ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb5ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb5ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb5ensetr`]
module"]
pub type RCC_MC_AHB5ENSETR = crate::Reg<rcc_mc_ahb5ensetr::RCC_MC_AHB5ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5ensetr;
#[doc = "RCC_MC_AHB5ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb5enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb5enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb5enclrr`]
module"]
pub type RCC_MC_AHB5ENCLRR = crate::Reg<rcc_mc_ahb5enclrr::RCC_MC_AHB5ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5enclrr;
#[doc = "RCC_MC_AHB6ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb6ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb6ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb6ensetr`]
module"]
pub type RCC_MC_AHB6ENSETR = crate::Reg<rcc_mc_ahb6ensetr::RCC_MC_AHB6ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb6ensetr;
#[doc = "RCC_MC_AHB6ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb6enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb6enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb6enclrr`]
module"]
pub type RCC_MC_AHB6ENCLRR = crate::Reg<rcc_mc_ahb6enclrr::RCC_MC_AHB6ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb6enclrr;
#[doc = "RCC_MP_APB4LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb4lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb4lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb4lpensetr`]
module"]
pub type RCC_MP_APB4LPENSETR = crate::Reg<rcc_mp_apb4lpensetr::RCC_MP_APB4LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb4lpensetr;
#[doc = "RCC_MP_APB4LPENCLRR (rw) register accessor: This register is used by the MCU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb4lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb4lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb4lpenclrr`]
module"]
pub type RCC_MP_APB4LPENCLRR = crate::Reg<rcc_mp_apb4lpenclrr::RCC_MP_APB4LPENCLRRrs>;
#[doc = "This register is used by the MCU"]
pub mod rcc_mp_apb4lpenclrr;
#[doc = "RCC_MP_APB5LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb5lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb5lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb5lpensetr`]
module"]
pub type RCC_MP_APB5LPENSETR = crate::Reg<rcc_mp_apb5lpensetr::RCC_MP_APB5LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_apb5lpensetr;
#[doc = "RCC_MP_APB5LPENCLRR (rw) register accessor: This register is used by the Mpu.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb5lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb5lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb5lpenclrr`]
module"]
pub type RCC_MP_APB5LPENCLRR = crate::Reg<rcc_mp_apb5lpenclrr::RCC_MP_APB5LPENCLRRrs>;
#[doc = "This register is used by the Mpu."]
pub mod rcc_mp_apb5lpenclrr;
#[doc = "RCC_MP_AHB5LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb5lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb5lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb5lpensetr`]
module"]
pub type RCC_MP_AHB5LPENSETR = crate::Reg<rcc_mp_ahb5lpensetr::RCC_MP_AHB5LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5lpensetr;
#[doc = "RCC_MP_AHB5LPENCLRR (rw) register accessor: This register is used by the MCU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb5lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb5lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb5lpenclrr`]
module"]
pub type RCC_MP_AHB5LPENCLRR = crate::Reg<rcc_mp_ahb5lpenclrr::RCC_MP_AHB5LPENCLRRrs>;
#[doc = "This register is used by the MCU"]
pub mod rcc_mp_ahb5lpenclrr;
#[doc = "RCC_MP_AHB6LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb6lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb6lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb6lpensetr`]
module"]
pub type RCC_MP_AHB6LPENSETR = crate::Reg<rcc_mp_ahb6lpensetr::RCC_MP_AHB6LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb6lpensetr;
#[doc = "RCC_MP_AHB6LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb6lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb6lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb6lpenclrr`]
module"]
pub type RCC_MP_AHB6LPENCLRR = crate::Reg<rcc_mp_ahb6lpenclrr::RCC_MP_AHB6LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb6lpenclrr;
#[doc = "RCC_MP_TZAHB6LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_tzahb6lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_tzahb6lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_tzahb6lpensetr`]
module"]
pub type RCC_MP_TZAHB6LPENSETR = crate::Reg<rcc_mp_tzahb6lpensetr::RCC_MP_TZAHB6LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6lpensetr;
#[doc = "RCC_MP_TZAHB6LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_tzahb6lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_tzahb6lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_tzahb6lpenclrr`]
module"]
pub type RCC_MP_TZAHB6LPENCLRR = crate::Reg<rcc_mp_tzahb6lpenclrr::RCC_MP_TZAHB6LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6lpenclrr;
#[doc = "RCC_MC_APB4LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb4lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb4lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb4lpensetr`]
module"]
pub type RCC_MC_APB4LPENSETR = crate::Reg<rcc_mc_apb4lpensetr::RCC_MC_APB4LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb4lpensetr;
#[doc = "RCC_MC_APB4LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb4lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb4lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb4lpenclrr`]
module"]
pub type RCC_MC_APB4LPENCLRR = crate::Reg<rcc_mc_apb4lpenclrr::RCC_MC_APB4LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb4lpenclrr;
#[doc = "RCC_MC_APB5LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb5lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb5lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb5lpensetr`]
module"]
pub type RCC_MC_APB5LPENSETR = crate::Reg<rcc_mc_apb5lpensetr::RCC_MC_APB5LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb5lpensetr;
#[doc = "RCC_MC_APB5LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb5lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb5lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb5lpenclrr`]
module"]
pub type RCC_MC_APB5LPENCLRR = crate::Reg<rcc_mc_apb5lpenclrr::RCC_MC_APB5LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb5lpenclrr;
#[doc = "RCC_MC_AHB5LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb5lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb5lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb5lpensetr`]
module"]
pub type RCC_MC_AHB5LPENSETR = crate::Reg<rcc_mc_ahb5lpensetr::RCC_MC_AHB5LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5lpensetr;
#[doc = "RCC_MC_AHB5LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb5lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb5lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb5lpenclrr`]
module"]
pub type RCC_MC_AHB5LPENCLRR = crate::Reg<rcc_mc_ahb5lpenclrr::RCC_MC_AHB5LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5lpenclrr;
#[doc = "RCC_MC_AHB6LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb6lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb6lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb6lpensetr`]
module"]
pub type RCC_MC_AHB6LPENSETR = crate::Reg<rcc_mc_ahb6lpensetr::RCC_MC_AHB6LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb6lpensetr;
#[doc = "RCC_MC_AHB6LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb6lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb6lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb6lpenclrr`]
module"]
pub type RCC_MC_AHB6LPENCLRR = crate::Reg<rcc_mc_ahb6lpenclrr::RCC_MC_AHB6LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb6lpenclrr;
#[doc = "RCC_BR_RSTSCLRR (rw) register accessor: This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_br_rstsclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_br_rstsclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_br_rstsclrr`]
module"]
pub type RCC_BR_RSTSCLRR = crate::Reg<rcc_br_rstsclrr::RCC_BR_RSTSCLRRrs>;
#[doc = "This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_br_rstsclrr;
#[doc = "RCC_MP_GRSTCSETR (rw) register accessor: This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_grstcsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_grstcsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_grstcsetr`]
module"]
pub type RCC_MP_GRSTCSETR = crate::Reg<rcc_mp_grstcsetr::RCC_MP_GRSTCSETRrs>;
#[doc = "This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset."]
pub mod rcc_mp_grstcsetr;
#[doc = "RCC_MP_RSTSCLRR (rw) register accessor: This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_rstsclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_rstsclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_rstsclrr`]
module"]
pub type RCC_MP_RSTSCLRR = crate::Reg<rcc_mp_rstsclrr::RCC_MP_RSTSCLRRrs>;
#[doc = "This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_rstsclrr;
#[doc = "RCC_MP_IWDGFZSETR (rw) register accessor: This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_iwdgfzsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_iwdgfzsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_iwdgfzsetr`]
module"]
pub type RCC_MP_IWDGFZSETR = crate::Reg<rcc_mp_iwdgfzsetr::RCC_MP_IWDGFZSETRrs>;
#[doc = "This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_iwdgfzsetr;
#[doc = "RCC_MP_IWDGFZCLRR (rw) register accessor: This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_iwdgfzclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_iwdgfzclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_iwdgfzclrr`]
module"]
pub type RCC_MP_IWDGFZCLRR = crate::Reg<rcc_mp_iwdgfzclrr::RCC_MP_IWDGFZCLRRrs>;
#[doc = "This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_iwdgfzclrr;
#[doc = "RCC_MP_CIER (rw) register accessor: This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_cier`]
module"]
pub type RCC_MP_CIER = crate::Reg<rcc_mp_cier::RCC_MP_CIERrs>;
#[doc = "This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_cier;
#[doc = "RCC_MP_CIFR (rw) register accessor: This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_cifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_cifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_cifr`]
module"]
pub type RCC_MP_CIFR = crate::Reg<rcc_mp_cifr::RCC_MP_CIFRrs>;
#[doc = "This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_cifr;
#[doc = "RCC_PWRLPDLYCR (rw) register accessor: This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pwrlpdlycr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pwrlpdlycr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pwrlpdlycr`]
module"]
pub type RCC_PWRLPDLYCR = crate::Reg<rcc_pwrlpdlycr::RCC_PWRLPDLYCRrs>;
#[doc = "This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_pwrlpdlycr;
#[doc = "RCC_MP_RSTSSETR (rw) register accessor: This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_rstssetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_rstssetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_rstssetr`]
module"]
pub type RCC_MP_RSTSSETR = crate::Reg<rcc_mp_rstssetr::RCC_MP_RSTSSETRrs>;
#[doc = "This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_rstssetr;
#[doc = "RCC_MCO1CFGR (rw) register accessor: This register is used to select the clock generated on MCO1 output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mco1cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mco1cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mco1cfgr`]
module"]
pub type RCC_MCO1CFGR = crate::Reg<rcc_mco1cfgr::RCC_MCO1CFGRrs>;
#[doc = "This register is used to select the clock generated on MCO1 output."]
pub mod rcc_mco1cfgr;
#[doc = "RCC_MCO2CFGR (rw) register accessor: This register is used to select the clock generated on MCO2 output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mco2cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mco2cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mco2cfgr`]
module"]
pub type RCC_MCO2CFGR = crate::Reg<rcc_mco2cfgr::RCC_MCO2CFGRrs>;
#[doc = "This register is used to select the clock generated on MCO2 output."]
pub mod rcc_mco2cfgr;
#[doc = "RCC_OCRDYR (r) register accessor: This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ocrdyr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ocrdyr`]
module"]
pub type RCC_OCRDYR = crate::Reg<rcc_ocrdyr::RCC_OCRDYRrs>;
#[doc = "This is a read-only access register, It contains the status flags of oscillators. Writing has no effect."]
pub mod rcc_ocrdyr;
#[doc = "RCC_DBGCFGR (rw) register accessor: This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_dbgcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_dbgcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_dbgcfgr`]
module"]
pub type RCC_DBGCFGR = crate::Reg<rcc_dbgcfgr::RCC_DBGCFGRrs>;
#[doc = "This is register contains the enable control of the debug and trace function, and the clock divider for the trace function."]
pub mod rcc_dbgcfgr;
#[doc = "RCC_RCK3SELR (rw) register accessor: This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rck3selr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rck3selr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_rck3selr`]
module"]
pub type RCC_RCK3SELR = crate::Reg<rcc_rck3selr::RCC_RCK3SELRrs>;
#[doc = "This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_rck3selr;
#[doc = "RCC_RCK4SELR (rw) register accessor: This register is used to select the reference clock for PLL4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rck4selr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rck4selr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_rck4selr`]
module"]
pub type RCC_RCK4SELR = crate::Reg<rcc_rck4selr::RCC_RCK4SELRrs>;
#[doc = "This register is used to select the reference clock for PLL4."]
pub mod rcc_rck4selr;
#[doc = "RCC_TIMG1PRER (rw) register accessor: This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_timg1prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_timg1prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_timg1prer`]
module"]
pub type RCC_TIMG1PRER = crate::Reg<rcc_timg1prer::RCC_TIMG1PRERrs>;
#[doc = "This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_timg1prer;
#[doc = "RCC_TIMG2PRER (rw) register accessor: This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_timg2prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_timg2prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_timg2prer`]
module"]
pub type RCC_TIMG2PRER = crate::Reg<rcc_timg2prer::RCC_TIMG2PRERrs>;
#[doc = "This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_timg2prer;
#[doc = "RCC_MCUDIVR (rw) register accessor: This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mcudivr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mcudivr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mcudivr`]
module"]
pub type RCC_MCUDIVR = crate::Reg<rcc_mcudivr::RCC_MCUDIVRrs>;
#[doc = "This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mcudivr;
#[doc = "RCC_APB1DIVR (rw) register accessor: This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1divr`]
module"]
pub type RCC_APB1DIVR = crate::Reg<rcc_apb1divr::RCC_APB1DIVRrs>;
#[doc = "This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information."]
pub mod rcc_apb1divr;
#[doc = "RCC_APB2DIVR (rw) register accessor: This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2divr`]
module"]
pub type RCC_APB2DIVR = crate::Reg<rcc_apb2divr::RCC_APB2DIVRrs>;
#[doc = "This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_apb2divr;
#[doc = "RCC_APB3DIVR (rw) register accessor: This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb3divr`]
module"]
pub type RCC_APB3DIVR = crate::Reg<rcc_apb3divr::RCC_APB3DIVRrs>;
#[doc = "This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_apb3divr;
#[doc = "RCC_PLL3CR (rw) register accessor: This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3cr`]
module"]
pub type RCC_PLL3CR = crate::Reg<rcc_pll3cr::RCC_PLL3CRrs>;
#[doc = "This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cr;
#[doc = "RCC_PLL3CFGR1 (rw) register accessor: This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3cfgr1`]
module"]
pub type RCC_PLL3CFGR1 = crate::Reg<rcc_pll3cfgr1::RCC_PLL3CFGR1rs>;
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cfgr1;
#[doc = "RCC_PLL3CFGR2 (rw) register accessor: This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3cfgr2`]
module"]
pub type RCC_PLL3CFGR2 = crate::Reg<rcc_pll3cfgr2::RCC_PLL3CFGR2rs>;
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cfgr2;
#[doc = "RCC_PLL3FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3fracr`]
module"]
pub type RCC_PLL3FRACR = crate::Reg<rcc_pll3fracr::RCC_PLL3FRACRrs>;
#[doc = "This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3fracr;
#[doc = "RCC_PLL3CSGR (rw) register accessor: This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll3csgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll3csgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll3csgr`]
module"]
pub type RCC_PLL3CSGR = crate::Reg<rcc_pll3csgr::RCC_PLL3CSGRrs>;
#[doc = "This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3csgr;
#[doc = "RCC_PLL4CR (rw) register accessor: This register is used to control the PLL4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll4cr`]
module"]
pub type RCC_PLL4CR = crate::Reg<rcc_pll4cr::RCC_PLL4CRrs>;
#[doc = "This register is used to control the PLL4."]
pub mod rcc_pll4cr;
#[doc = "RCC_PLL4CFGR1 (rw) register accessor: This register is used to configure the PLL4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll4cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll4cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll4cfgr1`]
module"]
pub type RCC_PLL4CFGR1 = crate::Reg<rcc_pll4cfgr1::RCC_PLL4CFGR1rs>;
#[doc = "This register is used to configure the PLL4."]
pub mod rcc_pll4cfgr1;
#[doc = "RCC_PLL4CFGR2 (rw) register accessor: This register is used to configure the PLL4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll4cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll4cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll4cfgr2`]
module"]
pub type RCC_PLL4CFGR2 = crate::Reg<rcc_pll4cfgr2::RCC_PLL4CFGR2rs>;
#[doc = "This register is used to configure the PLL4."]
pub mod rcc_pll4cfgr2;
#[doc = "RCC_PLL4FRACR (rw) register accessor: This register is used to fine-tune the frequency of the PLL4 VCO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll4fracr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll4fracr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll4fracr`]
module"]
pub type RCC_PLL4FRACR = crate::Reg<rcc_pll4fracr::RCC_PLL4FRACRrs>;
#[doc = "This register is used to fine-tune the frequency of the PLL4 VCO."]
pub mod rcc_pll4fracr;
#[doc = "RCC_PLL4CSGR (rw) register accessor: This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_pll4csgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_pll4csgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_pll4csgr`]
module"]
pub type RCC_PLL4CSGR = crate::Reg<rcc_pll4csgr::RCC_PLL4CSGRrs>;
#[doc = "This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll4csgr;
#[doc = "RCC_I2C12CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_i2c12ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_i2c12ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_i2c12ckselr`]
module"]
pub type RCC_I2C12CKSELR = crate::Reg<rcc_i2c12ckselr::RCC_I2C12CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_i2c12ckselr;
#[doc = "RCC_I2C35CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_i2c35ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_i2c35ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_i2c35ckselr`]
module"]
pub type RCC_I2C35CKSELR = crate::Reg<rcc_i2c35ckselr::RCC_I2C35CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_i2c35ckselr;
#[doc = "RCC_SAI1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sai1ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sai1ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_sai1ckselr`]
module"]
pub type RCC_SAI1CKSELR = crate::Reg<rcc_sai1ckselr::RCC_SAI1CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai1ckselr;
#[doc = "RCC_SAI2CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sai2ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sai2ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_sai2ckselr`]
module"]
pub type RCC_SAI2CKSELR = crate::Reg<rcc_sai2ckselr::RCC_SAI2CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai2ckselr;
#[doc = "RCC_SAI3CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sai3ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sai3ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_sai3ckselr`]
module"]
pub type RCC_SAI3CKSELR = crate::Reg<rcc_sai3ckselr::RCC_SAI3CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai3ckselr;
#[doc = "RCC_SAI4CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sai4ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sai4ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_sai4ckselr`]
module"]
pub type RCC_SAI4CKSELR = crate::Reg<rcc_sai4ckselr::RCC_SAI4CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai4ckselr;
#[doc = "RCC_SPI2S1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_spi2s1ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_spi2s1ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_spi2s1ckselr`]
module"]
pub type RCC_SPI2S1CKSELR = crate::Reg<rcc_spi2s1ckselr::RCC_SPI2S1CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi2s1ckselr;
#[doc = "RCC_SPI2S23CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_spi2s23ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_spi2s23ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_spi2s23ckselr`]
module"]
pub type RCC_SPI2S23CKSELR = crate::Reg<rcc_spi2s23ckselr::RCC_SPI2S23CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi2s23ckselr;
#[doc = "RCC_SPI45CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_spi45ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_spi45ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_spi45ckselr`]
module"]
pub type RCC_SPI45CKSELR = crate::Reg<rcc_spi45ckselr::RCC_SPI45CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi45ckselr;
#[doc = "RCC_UART6CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_uart6ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_uart6ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_uart6ckselr`]
module"]
pub type RCC_UART6CKSELR = crate::Reg<rcc_uart6ckselr::RCC_UART6CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart6ckselr;
#[doc = "RCC_UART24CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_uart24ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_uart24ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_uart24ckselr`]
module"]
pub type RCC_UART24CKSELR = crate::Reg<rcc_uart24ckselr::RCC_UART24CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart24ckselr;
#[doc = "RCC_UART35CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_uart35ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_uart35ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_uart35ckselr`]
module"]
pub type RCC_UART35CKSELR = crate::Reg<rcc_uart35ckselr::RCC_UART35CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart35ckselr;
#[doc = "RCC_UART78CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_uart78ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_uart78ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_uart78ckselr`]
module"]
pub type RCC_UART78CKSELR = crate::Reg<rcc_uart78ckselr::RCC_UART78CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart78ckselr;
#[doc = "RCC_SDMMC12CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sdmmc12ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sdmmc12ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_sdmmc12ckselr`]
module"]
pub type RCC_SDMMC12CKSELR = crate::Reg<rcc_sdmmc12ckselr::RCC_SDMMC12CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sdmmc12ckselr;
#[doc = "RCC_SDMMC3CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sdmmc3ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sdmmc3ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_sdmmc3ckselr`]
module"]
pub type RCC_SDMMC3CKSELR = crate::Reg<rcc_sdmmc3ckselr::RCC_SDMMC3CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sdmmc3ckselr;
#[doc = "RCC_ETHCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ethckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ethckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ethckselr`]
module"]
pub type RCC_ETHCKSELR = crate::Reg<rcc_ethckselr::RCC_ETHCKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_ethckselr;
#[doc = "RCC_QSPICKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_qspickselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_qspickselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_qspickselr`]
module"]
pub type RCC_QSPICKSELR = crate::Reg<rcc_qspickselr::RCC_QSPICKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_qspickselr;
#[doc = "RCC_FMCCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_fmcckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_fmcckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_fmcckselr`]
module"]
pub type RCC_FMCCKSELR = crate::Reg<rcc_fmcckselr::RCC_FMCCKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_fmcckselr;
#[doc = "RCC_FDCANCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_fdcanckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_fdcanckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_fdcanckselr`]
module"]
pub type RCC_FDCANCKSELR = crate::Reg<rcc_fdcanckselr::RCC_FDCANCKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_fdcanckselr;
#[doc = "RCC_SPDIFCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_spdifckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_spdifckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_spdifckselr`]
module"]
pub type RCC_SPDIFCKSELR = crate::Reg<rcc_spdifckselr::RCC_SPDIFCKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spdifckselr;
#[doc = "RCC_CECCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the CEC-HDMI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cecckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cecckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_cecckselr`]
module"]
pub type RCC_CECCKSELR = crate::Reg<rcc_cecckselr::RCC_CECCKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the CEC-HDMI."]
pub mod rcc_cecckselr;
#[doc = "RCC_USBCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_usbckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_usbckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_usbckselr`]
module"]
pub type RCC_USBCKSELR = crate::Reg<rcc_usbckselr::RCC_USBCKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG"]
pub mod rcc_usbckselr;
#[doc = "RCC_RNG2CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the RNG2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_rng2ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_rng2ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_rng2ckselr`]
module"]
pub type RCC_RNG2CKSELR = crate::Reg<rcc_rng2ckselr::RCC_RNG2CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the RNG2."]
pub mod rcc_rng2ckselr;
#[doc = "RCC_DSICKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the DSI block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_dsickselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_dsickselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_dsickselr`]
module"]
pub type RCC_DSICKSELR = crate::Reg<rcc_dsickselr::RCC_DSICKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the DSI block."]
pub mod rcc_dsickselr;
#[doc = "RCC_ADCCKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the ADC block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_adcckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_adcckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_adcckselr`]
module"]
pub type RCC_ADCCKSELR = crate::Reg<rcc_adcckselr::RCC_ADCCKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the ADC block."]
pub mod rcc_adcckselr;
#[doc = "RCC_LPTIM45CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_lptim45ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_lptim45ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_lptim45ckselr`]
module"]
pub type RCC_LPTIM45CKSELR = crate::Reg<rcc_lptim45ckselr::RCC_LPTIM45CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks."]
pub mod rcc_lptim45ckselr;
#[doc = "RCC_LPTIM23CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_lptim23ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_lptim23ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_lptim23ckselr`]
module"]
pub type RCC_LPTIM23CKSELR = crate::Reg<rcc_lptim23ckselr::RCC_LPTIM23CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks."]
pub mod rcc_lptim23ckselr;
#[doc = "RCC_LPTIM1CKSELR (rw) register accessor: This register is used to control the selection of the kernel clock for the LPTIM1 block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_lptim1ckselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_lptim1ckselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_lptim1ckselr`]
module"]
pub type RCC_LPTIM1CKSELR = crate::Reg<rcc_lptim1ckselr::RCC_LPTIM1CKSELRrs>;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM1 block."]
pub mod rcc_lptim1ckselr;
#[doc = "RCC_APB1RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1rstsetr`]
module"]
pub type RCC_APB1RSTSETR = crate::Reg<rcc_apb1rstsetr::RCC_APB1RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb1rstsetr;
#[doc = "RCC_APB1RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb1rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb1rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb1rstclrr`]
module"]
pub type RCC_APB1RSTCLRR = crate::Reg<rcc_apb1rstclrr::RCC_APB1RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb1rstclrr;
#[doc = "RCC_APB2RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2rstsetr`]
module"]
pub type RCC_APB2RSTSETR = crate::Reg<rcc_apb2rstsetr::RCC_APB2RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb2rstsetr;
#[doc = "RCC_APB2RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb2rstclrr`]
module"]
pub type RCC_APB2RSTCLRR = crate::Reg<rcc_apb2rstclrr::RCC_APB2RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb2rstclrr;
#[doc = "RCC_APB3RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb3rstsetr`]
module"]
pub type RCC_APB3RSTSETR = crate::Reg<rcc_apb3rstsetr::RCC_APB3RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb3rstsetr;
#[doc = "RCC_APB3RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_apb3rstclrr`]
module"]
pub type RCC_APB3RSTCLRR = crate::Reg<rcc_apb3rstclrr::RCC_APB3RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb3rstclrr;
#[doc = "RCC_AHB2RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2rstsetr`]
module"]
pub type RCC_AHB2RSTSETR = crate::Reg<rcc_ahb2rstsetr::RCC_AHB2RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_ahb2rstsetr;
#[doc = "RCC_AHB2RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb2rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb2rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb2rstclrr`]
module"]
pub type RCC_AHB2RSTCLRR = crate::Reg<rcc_ahb2rstclrr::RCC_AHB2RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb2rstclrr;
#[doc = "RCC_AHB3RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3rstsetr`]
module"]
pub type RCC_AHB3RSTSETR = crate::Reg<rcc_ahb3rstsetr::RCC_AHB3RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_ahb3rstsetr;
#[doc = "RCC_AHB3RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb3rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb3rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb3rstclrr`]
module"]
pub type RCC_AHB3RSTCLRR = crate::Reg<rcc_ahb3rstclrr::RCC_AHB3RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb3rstclrr;
#[doc = "RCC_AHB4RSTSETR (rw) register accessor: This register is used to activate the reset of the corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb4rstsetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb4rstsetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb4rstsetr`]
module"]
pub type RCC_AHB4RSTSETR = crate::Reg<rcc_ahb4rstsetr::RCC_AHB4RSTSETRrs>;
#[doc = "This register is used to activate the reset of the corresponding peripheral"]
pub mod rcc_ahb4rstsetr;
#[doc = "RCC_AHB4RSTCLRR (rw) register accessor: This register is used to release the reset of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb4rstclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb4rstclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_ahb4rstclrr`]
module"]
pub type RCC_AHB4RSTCLRR = crate::Reg<rcc_ahb4rstclrr::RCC_AHB4RSTCLRRrs>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb4rstclrr;
#[doc = "RCC_MP_APB1ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb1ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb1ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb1ensetr`]
module"]
pub type RCC_MP_APB1ENSETR = crate::Reg<rcc_mp_apb1ensetr::RCC_MP_APB1ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb1ensetr;
#[doc = "RCC_MP_APB1ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb1enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb1enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb1enclrr`]
module"]
pub type RCC_MP_APB1ENCLRR = crate::Reg<rcc_mp_apb1enclrr::RCC_MP_APB1ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mp_apb1enclrr;
#[doc = "RCC_MP_APB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb2ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb2ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb2ensetr`]
module"]
pub type RCC_MP_APB2ENSETR = crate::Reg<rcc_mp_apb2ensetr::RCC_MP_APB2ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb2ensetr;
#[doc = "RCC_MP_APB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb2enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb2enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb2enclrr`]
module"]
pub type RCC_MP_APB2ENCLRR = crate::Reg<rcc_mp_apb2enclrr::RCC_MP_APB2ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_apb2enclrr;
#[doc = "RCC_MP_APB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb3ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb3ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb3ensetr`]
module"]
pub type RCC_MP_APB3ENSETR = crate::Reg<rcc_mp_apb3ensetr::RCC_MP_APB3ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb3ensetr;
#[doc = "RCC_MP_APB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb3enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb3enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb3enclrr`]
module"]
pub type RCC_MP_APB3ENCLRR = crate::Reg<rcc_mp_apb3enclrr::RCC_MP_APB3ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_apb3enclrr;
#[doc = "RCC_MP_AHB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb2ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb2ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb2ensetr`]
module"]
pub type RCC_MP_AHB2ENSETR = crate::Reg<rcc_mp_ahb2ensetr::RCC_MP_AHB2ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
pub mod rcc_mp_ahb2ensetr;
#[doc = "RCC_MP_AHB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb2enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb2enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb2enclrr`]
module"]
pub type RCC_MP_AHB2ENCLRR = crate::Reg<rcc_mp_ahb2enclrr::RCC_MP_AHB2ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_ahb2enclrr;
#[doc = "RCC_MP_AHB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb3ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb3ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb3ensetr`]
module"]
pub type RCC_MP_AHB3ENSETR = crate::Reg<rcc_mp_ahb3ensetr::RCC_MP_AHB3ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
pub mod rcc_mp_ahb3ensetr;
#[doc = "RCC_MP_AHB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb3enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb3enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb3enclrr`]
module"]
pub type RCC_MP_AHB3ENCLRR = crate::Reg<rcc_mp_ahb3enclrr::RCC_MP_AHB3ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_ahb3enclrr;
#[doc = "RCC_MP_AHB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb4ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb4ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb4ensetr`]
module"]
pub type RCC_MP_AHB4ENSETR = crate::Reg<rcc_mp_ahb4ensetr::RCC_MP_AHB4ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU."]
pub mod rcc_mp_ahb4ensetr;
#[doc = "RCC_MP_AHB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb4enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb4enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb4enclrr`]
module"]
pub type RCC_MP_AHB4ENCLRR = crate::Reg<rcc_mp_ahb4enclrr::RCC_MP_AHB4ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mp_ahb4enclrr;
#[doc = "RCC_MP_MLAHBENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_mlahbensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_mlahbensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_mlahbensetr`]
module"]
pub type RCC_MP_MLAHBENSETR = crate::Reg<rcc_mp_mlahbensetr::RCC_MP_MLAHBENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_mlahbensetr;
#[doc = "RCC_MP_MLAHBENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_mlahbenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_mlahbenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_mlahbenclrr`]
module"]
pub type RCC_MP_MLAHBENCLRR = crate::Reg<rcc_mp_mlahbenclrr::RCC_MP_MLAHBENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit."]
pub mod rcc_mp_mlahbenclrr;
#[doc = "RCC_MC_APB1ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb1ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb1ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb1ensetr`]
module"]
pub type RCC_MC_APB1ENSETR = crate::Reg<rcc_mc_apb1ensetr::RCC_MC_APB1ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to ."]
pub mod rcc_mc_apb1ensetr;
#[doc = "RCC_MC_APB1ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb1enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb1enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb1enclrr`]
module"]
pub type RCC_MC_APB1ENCLRR = crate::Reg<rcc_mc_apb1enclrr::RCC_MC_APB1ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mc_apb1enclrr;
#[doc = "RCC_MC_APB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb2ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb2ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb2ensetr`]
module"]
pub type RCC_MC_APB2ENSETR = crate::Reg<rcc_mc_apb2ensetr::RCC_MC_APB2ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb2ensetr;
#[doc = "RCC_MC_APB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb2enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb2enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb2enclrr`]
module"]
pub type RCC_MC_APB2ENCLRR = crate::Reg<rcc_mc_apb2enclrr::RCC_MC_APB2ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb2enclrr;
#[doc = "RCC_MC_APB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb3ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb3ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb3ensetr`]
module"]
pub type RCC_MC_APB3ENSETR = crate::Reg<rcc_mc_apb3ensetr::RCC_MC_APB3ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb3ensetr;
#[doc = "RCC_MC_APB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb3enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb3enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb3enclrr`]
module"]
pub type RCC_MC_APB3ENCLRR = crate::Reg<rcc_mc_apb3enclrr::RCC_MC_APB3ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb3enclrr;
#[doc = "RCC_MC_AHB2ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb2ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb2ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb2ensetr`]
module"]
pub type RCC_MC_AHB2ENSETR = crate::Reg<rcc_mc_ahb2ensetr::RCC_MC_AHB2ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb2ensetr;
#[doc = "RCC_MC_AHB2ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb2enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb2enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb2enclrr`]
module"]
pub type RCC_MC_AHB2ENCLRR = crate::Reg<rcc_mc_ahb2enclrr::RCC_MC_AHB2ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb2enclrr;
#[doc = "RCC_MC_AHB3ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb3ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb3ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb3ensetr`]
module"]
pub type RCC_MC_AHB3ENSETR = crate::Reg<rcc_mc_ahb3ensetr::RCC_MC_AHB3ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb3ensetr;
#[doc = "RCC_MC_AHB3ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb3enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb3enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb3enclrr`]
module"]
pub type RCC_MC_AHB3ENCLRR = crate::Reg<rcc_mc_ahb3enclrr::RCC_MC_AHB3ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb3enclrr;
#[doc = "RCC_MC_AHB4ENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb4ensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb4ensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb4ensetr`]
module"]
pub type RCC_MC_AHB4ENSETR = crate::Reg<rcc_mc_ahb4ensetr::RCC_MC_AHB4ENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb4ensetr;
#[doc = "RCC_MC_AHB4ENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb4enclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb4enclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb4enclrr`]
module"]
pub type RCC_MC_AHB4ENCLRR = crate::Reg<rcc_mc_ahb4enclrr::RCC_MC_AHB4ENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb4enclrr;
#[doc = "RCC_MC_AXIMENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_aximensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_aximensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_aximensetr`]
module"]
pub type RCC_MC_AXIMENSETR = crate::Reg<rcc_mc_aximensetr::RCC_MC_AXIMENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_aximensetr;
#[doc = "RCC_MC_AXIMENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_aximenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_aximenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_aximenclrr`]
module"]
pub type RCC_MC_AXIMENCLRR = crate::Reg<rcc_mc_aximenclrr::RCC_MC_AXIMENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_aximenclrr;
#[doc = "RCC_MC_MLAHBENSETR (rw) register accessor: This register is used to set the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_mlahbensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_mlahbensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_mlahbensetr`]
module"]
pub type RCC_MC_MLAHBENSETR = crate::Reg<rcc_mc_mlahbensetr::RCC_MC_MLAHBENSETRrs>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_mlahbensetr;
#[doc = "RCC_MC_MLAHBENCLRR (rw) register accessor: This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_mlahbenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_mlahbenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_mlahbenclrr`]
module"]
pub type RCC_MC_MLAHBENCLRR = crate::Reg<rcc_mc_mlahbenclrr::RCC_MC_MLAHBENCLRRrs>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_mlahbenclrr;
#[doc = "RCC_MP_APB1LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb1lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb1lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb1lpensetr`]
module"]
pub type RCC_MP_APB1LPENSETR = crate::Reg<rcc_mp_apb1lpensetr::RCC_MP_APB1LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb1lpensetr;
#[doc = "RCC_MP_APB1LPENCLRR (rw) register accessor: This register is used by the MPU in order to clear the PERxLPEN bits .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb1lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb1lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb1lpenclrr`]
module"]
pub type RCC_MP_APB1LPENCLRR = crate::Reg<rcc_mp_apb1lpenclrr::RCC_MP_APB1LPENCLRRrs>;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bits ."]
pub mod rcc_mp_apb1lpenclrr;
#[doc = "RCC_MP_APB2LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb2lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb2lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb2lpensetr`]
module"]
pub type RCC_MP_APB2LPENSETR = crate::Reg<rcc_mp_apb2lpensetr::RCC_MP_APB2LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb2lpensetr;
#[doc = "RCC_MP_APB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb2lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb2lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb2lpenclrr`]
module"]
pub type RCC_MP_APB2LPENCLRR = crate::Reg<rcc_mp_apb2lpenclrr::RCC_MP_APB2LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb2lpenclrr;
#[doc = "RCC_MP_APB3LPENSETR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb3lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb3lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb3lpensetr`]
module"]
pub type RCC_MP_APB3LPENSETR = crate::Reg<rcc_mp_apb3lpensetr::RCC_MP_APB3LPENSETRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb3lpensetr;
#[doc = "RCC_MP_APB3LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_apb3lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_apb3lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_apb3lpenclrr`]
module"]
pub type RCC_MP_APB3LPENCLRR = crate::Reg<rcc_mp_apb3lpenclrr::RCC_MP_APB3LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb3lpenclrr;
#[doc = "RCC_MP_AHB2LPENSETR (rw) register accessor: This register is used by the MPU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb2lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb2lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb2lpensetr`]
module"]
pub type RCC_MP_AHB2LPENSETR = crate::Reg<rcc_mp_ahb2lpensetr::RCC_MP_AHB2LPENSETRrs>;
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit."]
pub mod rcc_mp_ahb2lpensetr;
#[doc = "RCC_MP_AHB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb2lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb2lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb2lpenclrr`]
module"]
pub type RCC_MP_AHB2LPENCLRR = crate::Reg<rcc_mp_ahb2lpenclrr::RCC_MP_AHB2LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb2lpenclrr;
#[doc = "RCC_MP_AHB3LPENSETR (rw) register accessor: This register is used by the MPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb3lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb3lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb3lpensetr`]
module"]
pub type RCC_MP_AHB3LPENSETR = crate::Reg<rcc_mp_ahb3lpensetr::RCC_MP_AHB3LPENSETRrs>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb3lpensetr;
#[doc = "RCC_MP_AHB3LPENCLRR (rw) register accessor: This register is used by the MPU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb3lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb3lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb3lpenclrr`]
module"]
pub type RCC_MP_AHB3LPENCLRR = crate::Reg<rcc_mp_ahb3lpenclrr::RCC_MP_AHB3LPENCLRRrs>;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit"]
pub mod rcc_mp_ahb3lpenclrr;
#[doc = "RCC_MP_AHB4LPENSETR (rw) register accessor: This register is used by the MPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb4lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb4lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb4lpensetr`]
module"]
pub type RCC_MP_AHB4LPENSETR = crate::Reg<rcc_mp_ahb4lpensetr::RCC_MP_AHB4LPENSETRrs>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb4lpensetr;
#[doc = "RCC_MP_AHB4LPENCLRR (rw) register accessor: This register is used by the MPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb4lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb4lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_ahb4lpenclrr`]
module"]
pub type RCC_MP_AHB4LPENCLRR = crate::Reg<rcc_mp_ahb4lpenclrr::RCC_MP_AHB4LPENCLRRrs>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb4lpenclrr;
#[doc = "RCC_MP_AXIMLPENSETR (rw) register accessor: This register is used by the MPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_aximlpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_aximlpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_aximlpensetr`]
module"]
pub type RCC_MP_AXIMLPENSETR = crate::Reg<rcc_mp_aximlpensetr::RCC_MP_AXIMLPENSETRrs>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_aximlpensetr;
#[doc = "RCC_MP_AXIMLPENCLRR (rw) register accessor: This register is used by the MPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_aximlpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_aximlpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_aximlpenclrr`]
module"]
pub type RCC_MP_AXIMLPENCLRR = crate::Reg<rcc_mp_aximlpenclrr::RCC_MP_AXIMLPENCLRRrs>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_aximlpenclrr;
#[doc = "RCC_MP_MLAHBLPENSETR (rw) register accessor: This register is used by the MPU in order to set the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_mlahblpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_mlahblpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_mlahblpensetr`]
module"]
pub type RCC_MP_MLAHBLPENSETR = crate::Reg<rcc_mp_mlahblpensetr::RCC_MP_MLAHBLPENSETRrs>;
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit"]
pub mod rcc_mp_mlahblpensetr;
#[doc = "RCC_MP_MLAHBLPENCLRR (rw) register accessor: This register is used by the MPU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_mlahblpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_mlahblpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mp_mlahblpenclrr`]
module"]
pub type RCC_MP_MLAHBLPENCLRR = crate::Reg<rcc_mp_mlahblpenclrr::RCC_MP_MLAHBLPENCLRRrs>;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit"]
pub mod rcc_mp_mlahblpenclrr;
#[doc = "RCC_MC_APB1LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb1lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb1lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb1lpensetr`]
module"]
pub type RCC_MC_APB1LPENSETR = crate::Reg<rcc_mc_apb1lpensetr::RCC_MC_APB1LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb1lpensetr;
#[doc = "RCC_MC_APB1LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb1lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb1lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb1lpenclrr`]
module"]
pub type RCC_MC_APB1LPENCLRR = crate::Reg<rcc_mc_apb1lpenclrr::RCC_MC_APB1LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mc_apb1lpenclrr;
#[doc = "RCC_MC_APB2LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb2lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb2lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb2lpensetr`]
module"]
pub type RCC_MC_APB2LPENSETR = crate::Reg<rcc_mc_apb2lpensetr::RCC_MC_APB2LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb2lpensetr;
#[doc = "RCC_MC_APB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb2lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb2lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb2lpenclrr`]
module"]
pub type RCC_MC_APB2LPENCLRR = crate::Reg<rcc_mc_apb2lpenclrr::RCC_MC_APB2LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb2lpenclrr;
#[doc = "RCC_MC_APB3LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb3lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb3lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb3lpensetr`]
module"]
pub type RCC_MC_APB3LPENSETR = crate::Reg<rcc_mc_apb3lpensetr::RCC_MC_APB3LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb3lpensetr;
#[doc = "RCC_MC_APB3LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb3lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb3lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_apb3lpenclrr`]
module"]
pub type RCC_MC_APB3LPENCLRR = crate::Reg<rcc_mc_apb3lpenclrr::RCC_MC_APB3LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb3lpenclrr;
#[doc = "RCC_MC_AHB2LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb2lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb2lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb2lpensetr`]
module"]
pub type RCC_MC_AHB2LPENSETR = crate::Reg<rcc_mc_ahb2lpensetr::RCC_MC_AHB2LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb2lpensetr;
#[doc = "RCC_MC_AHB2LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb2lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb2lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb2lpenclrr`]
module"]
pub type RCC_MC_AHB2LPENCLRR = crate::Reg<rcc_mc_ahb2lpenclrr::RCC_MC_AHB2LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb2lpenclrr;
#[doc = "RCC_MC_AHB3LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb3lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb3lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb3lpensetr`]
module"]
pub type RCC_MC_AHB3LPENSETR = crate::Reg<rcc_mc_ahb3lpensetr::RCC_MC_AHB3LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb3lpensetr;
#[doc = "RCC_MC_AHB3LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb3lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb3lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb3lpenclrr`]
module"]
pub type RCC_MC_AHB3LPENCLRR = crate::Reg<rcc_mc_ahb3lpenclrr::RCC_MC_AHB3LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb3lpenclrr;
#[doc = "RCC_MC_AHB4LPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb4lpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb4lpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb4lpensetr`]
module"]
pub type RCC_MC_AHB4LPENSETR = crate::Reg<rcc_mc_ahb4lpensetr::RCC_MC_AHB4LPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb4lpensetr;
#[doc = "RCC_MC_AHB4LPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_ahb4lpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_ahb4lpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_ahb4lpenclrr`]
module"]
pub type RCC_MC_AHB4LPENCLRR = crate::Reg<rcc_mc_ahb4lpenclrr::RCC_MC_AHB4LPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_ahb4lpenclrr;
#[doc = "RCC_MC_AXIMLPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_aximlpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_aximlpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_aximlpensetr`]
module"]
pub type RCC_MC_AXIMLPENSETR = crate::Reg<rcc_mc_aximlpensetr::RCC_MC_AXIMLPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_aximlpensetr;
#[doc = "RCC_MC_AXIMLPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_aximlpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_aximlpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_aximlpenclrr`]
module"]
pub type RCC_MC_AXIMLPENCLRR = crate::Reg<rcc_mc_aximlpenclrr::RCC_MC_AXIMLPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_aximlpenclrr;
#[doc = "RCC_MC_MLAHBLPENSETR (rw) register accessor: This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_mlahblpensetr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_mlahblpensetr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_mlahblpensetr`]
module"]
pub type RCC_MC_MLAHBLPENSETR = crate::Reg<rcc_mc_mlahblpensetr::RCC_MC_MLAHBLPENSETRrs>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_mlahblpensetr;
#[doc = "RCC_MC_MLAHBLPENCLRR (rw) register accessor: This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_mlahblpenclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_mlahblpenclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_mlahblpenclrr`]
module"]
pub type RCC_MC_MLAHBLPENCLRR = crate::Reg<rcc_mc_mlahblpenclrr::RCC_MC_MLAHBLPENCLRRrs>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_mlahblpenclrr;
#[doc = "RCC_MC_RSTSCLRR (rw) register accessor: This register is used by the MCU to check the reset source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_rstsclrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_rstsclrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_rstsclrr`]
module"]
pub type RCC_MC_RSTSCLRR = crate::Reg<rcc_mc_rstsclrr::RCC_MC_RSTSCLRRrs>;
#[doc = "This register is used by the MCU to check the reset source."]
pub mod rcc_mc_rstsclrr;
#[doc = "RCC_MC_CIER (rw) register accessor: This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_cier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_cier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_cier`]
module"]
pub type RCC_MC_CIER = crate::Reg<rcc_mc_cier::RCC_MC_CIERrs>;
#[doc = "This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details."]
pub mod rcc_mc_cier;
#[doc = "RCC_MC_CIFR (rw) register accessor: This register shall be used by the MCU in order to read and clear the interrupt flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_cifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_cifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_mc_cifr`]
module"]
pub type RCC_MC_CIFR = crate::Reg<rcc_mc_cifr::RCC_MC_CIFRrs>;
#[doc = "This register shall be used by the MCU in order to read and clear the interrupt flags."]
pub mod rcc_mc_cifr;
#[doc = "RCC_VERR (r) register accessor: This register gives the IP version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_verr`]
module"]
pub type RCC_VERR = crate::Reg<rcc_verr::RCC_VERRrs>;
#[doc = "This register gives the IP version"]
pub mod rcc_verr;
#[doc = "RCC_IDR (r) register accessor: This register gives the unique identifier of the RCC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_idr`]
module"]
pub type RCC_IDR = crate::Reg<rcc_idr::RCC_IDRrs>;
#[doc = "This register gives the unique identifier of the RCC"]
pub mod rcc_idr;
#[doc = "RCC_SIDR (r) register accessor: This register gives the decoding space, which is for the RCC of 4 kB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcc_sidr`]
module"]
pub type RCC_SIDR = crate::Reg<rcc_sidr::RCC_SIDRrs>;
#[doc = "This register gives the decoding space, which is for the RCC of 4 kB."]
pub mod rcc_sidr;
