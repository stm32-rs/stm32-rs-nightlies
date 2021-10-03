#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode."]
    pub rcc_tzcr: crate::Reg<rcc_tzcr::RCC_TZCR_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_ocensetr: crate::Reg<rcc_ocensetr::RCC_OCENSETR_SPEC>,
    #[doc = "0x10 - This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_ocenclrr: crate::Reg<rcc_ocenclrr::RCC_OCENCLRR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_hsicfgr: crate::Reg<rcc_hsicfgr::RCC_HSICFGR_SPEC>,
    #[doc = "0x1c - This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details."]
    pub rcc_csicfgr: crate::Reg<rcc_csicfgr::RCC_CSICFGR_SPEC>,
    #[doc = "0x20 - This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_mpckselr: crate::Reg<rcc_mpckselr::RCC_MPCKSELR_SPEC>,
    #[doc = "0x24 - This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_assckselr: crate::Reg<rcc_assckselr::RCC_ASSCKSELR_SPEC>,
    #[doc = "0x28 - This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_rck12selr: crate::Reg<rcc_rck12selr::RCC_RCK12SELR_SPEC>,
    #[doc = "0x2c - This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mpckdivr: crate::Reg<rcc_mpckdivr::RCC_MPCKDIVR_SPEC>,
    #[doc = "0x30 - This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_axidivr: crate::Reg<rcc_axidivr::RCC_AXIDIVR_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x3c - This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb4divr: crate::Reg<rcc_apb4divr::RCC_APB4DIVR_SPEC>,
    #[doc = "0x40 - This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb5divr: crate::Reg<rcc_apb5divr::RCC_APB5DIVR_SPEC>,
    #[doc = "0x44 - This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_rtcdivr: crate::Reg<rcc_rtcdivr::RCC_RTCDIVR_SPEC>,
    #[doc = "0x48 - This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_mssckselr: crate::Reg<rcc_mssckselr::RCC_MSSCKSELR_SPEC>,
    _reserved14: [u8; 0x34],
    #[doc = "0x80 - This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1cr: crate::Reg<rcc_pll1cr::RCC_PLL1CR_SPEC>,
    #[doc = "0x84 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1cfgr1: crate::Reg<rcc_pll1cfgr1::RCC_PLL1CFGR1_SPEC>,
    #[doc = "0x88 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1cfgr2: crate::Reg<rcc_pll1cfgr2::RCC_PLL1CFGR2_SPEC>,
    #[doc = "0x8c - This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1fracr: crate::Reg<rcc_pll1fracr::RCC_PLL1FRACR_SPEC>,
    #[doc = "0x90 - This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1csgr: crate::Reg<rcc_pll1csgr::RCC_PLL1CSGR_SPEC>,
    #[doc = "0x94 - This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2cr: crate::Reg<rcc_pll2cr::RCC_PLL2CR_SPEC>,
    #[doc = "0x98 - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2cfgr1: crate::Reg<rcc_pll2cfgr1::RCC_PLL2CFGR1_SPEC>,
    #[doc = "0x9c - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2cfgr2: crate::Reg<rcc_pll2cfgr2::RCC_PLL2CFGR2_SPEC>,
    #[doc = "0xa0 - This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2fracr: crate::Reg<rcc_pll2fracr::RCC_PLL2FRACR_SPEC>,
    #[doc = "0xa4 - This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2csgr: crate::Reg<rcc_pll2csgr::RCC_PLL2CSGR_SPEC>,
    _reserved24: [u8; 0x18],
    #[doc = "0xc0 - This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_i2c46ckselr: crate::Reg<rcc_i2c46ckselr::RCC_I2C46CKSELR_SPEC>,
    #[doc = "0xc4 - This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_spi6ckselr: crate::Reg<rcc_spi6ckselr::RCC_SPI6CKSELR_SPEC>,
    #[doc = "0xc8 - This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_uart1ckselr: crate::Reg<rcc_uart1ckselr::RCC_UART1CKSELR_SPEC>,
    #[doc = "0xcc - This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_rng1ckselr: crate::Reg<rcc_rng1ckselr::RCC_RNG1CKSELR_SPEC>,
    #[doc = "0xd0 - This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays."]
    pub rcc_cperckselr: crate::Reg<rcc_cperckselr::RCC_CPERCKSELR_SPEC>,
    #[doc = "0xd4 - This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_stgenckselr: crate::Reg<rcc_stgenckselr::RCC_STGENCKSELR_SPEC>,
    #[doc = "0xd8 - This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_ddritfcr: crate::Reg<rcc_ddritfcr::RCC_DDRITFCR_SPEC>,
    _reserved31: [u8; 0x24],
    #[doc = "0x100 - This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU."]
    pub rcc_mp_bootcr: crate::Reg<rcc_mp_bootcr::RCC_MP_BOOTCR_SPEC>,
    #[doc = "0x104 - Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_sreqsetr: crate::Reg<rcc_mp_sreqsetr::RCC_MP_SREQSETR_SPEC>,
    #[doc = "0x108 - Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_sreqclrr: crate::Reg<rcc_mp_sreqclrr::RCC_MP_SREQCLRR_SPEC>,
    #[doc = "0x10c - The register contains global control bits. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_gcr: crate::Reg<rcc_mp_gcr::RCC_MP_GCR_SPEC>,
    #[doc = "0x110 - This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_aprstcr: crate::Reg<rcc_mp_aprstcr::RCC_MP_APRSTCR_SPEC>,
    #[doc = "0x114 - This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_aprstsr: crate::Reg<rcc_mp_aprstsr::RCC_MP_APRSTSR_SPEC>,
    _reserved37: [u8; 0x28],
    #[doc = "0x140 - This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_bdcr: crate::Reg<rcc_bdcr::RCC_BDCR_SPEC>,
    #[doc = "0x144 - This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_rdlsicr: crate::Reg<rcc_rdlsicr::RCC_RDLSICR_SPEC>,
    _reserved39: [u8; 0x38],
    #[doc = "0x180 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
    pub rcc_apb4rstsetr: crate::Reg<rcc_apb4rstsetr::RCC_APB4RSTSETR_SPEC>,
    #[doc = "0x184 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
    pub rcc_apb4rstclrr: crate::Reg<rcc_apb4rstclrr::RCC_APB4RSTCLRR_SPEC>,
    #[doc = "0x188 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb5rstsetr: crate::Reg<rcc_apb5rstsetr::RCC_APB5RSTSETR_SPEC>,
    #[doc = "0x18c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb5rstclrr: crate::Reg<rcc_apb5rstclrr::RCC_APB5RSTCLRR_SPEC>,
    #[doc = "0x190 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_ahb5rstsetr: crate::Reg<rcc_ahb5rstsetr::RCC_AHB5RSTSETR_SPEC>,
    #[doc = "0x194 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_ahb5rstclrr: crate::Reg<rcc_ahb5rstclrr::RCC_AHB5RSTCLRR_SPEC>,
    #[doc = "0x198 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
    pub rcc_ahb6rstsetr: crate::Reg<rcc_ahb6rstsetr::RCC_AHB6RSTSETR_SPEC>,
    #[doc = "0x19c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
    pub rcc_ahb6rstclrr: crate::Reg<rcc_ahb6rstclrr::RCC_AHB6RSTCLRR_SPEC>,
    #[doc = "0x1a0 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_tzahb6rstsetr: crate::Reg<rcc_tzahb6rstsetr::RCC_TZAHB6RSTSETR_SPEC>,
    #[doc = "0x1a4 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_tzahb6rstclrr: crate::Reg<rcc_tzahb6rstclrr::RCC_TZAHB6RSTCLRR_SPEC>,
    _reserved49: [u8; 0x58],
    #[doc = "0x200 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb4ensetr: crate::Reg<rcc_mp_apb4ensetr::RCC_MP_APB4ENSETR_SPEC>,
    #[doc = "0x204 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb4enclrr: crate::Reg<rcc_mp_apb4enclrr::RCC_MP_APB4ENCLRR_SPEC>,
    #[doc = "0x208 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb5ensetr: crate::Reg<rcc_mp_apb5ensetr::RCC_MP_APB5ENSETR_SPEC>,
    #[doc = "0x20c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb5enclrr: crate::Reg<rcc_mp_apb5enclrr::RCC_MP_APB5ENCLRR_SPEC>,
    #[doc = "0x210 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_ahb5ensetr: crate::Reg<rcc_mp_ahb5ensetr::RCC_MP_AHB5ENSETR_SPEC>,
    #[doc = "0x214 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_ahb5enclrr: crate::Reg<rcc_mp_ahb5enclrr::RCC_MP_AHB5ENCLRR_SPEC>,
    #[doc = "0x218 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_ahb6ensetr: crate::Reg<rcc_mp_ahb6ensetr::RCC_MP_AHB6ENSETR_SPEC>,
    #[doc = "0x21c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_ahb6enclrr: crate::Reg<rcc_mp_ahb6enclrr::RCC_MP_AHB6ENCLRR_SPEC>,
    #[doc = "0x220 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6ensetr: crate::Reg<rcc_mp_tzahb6ensetr::RCC_MP_TZAHB6ENSETR_SPEC>,
    #[doc = "0x224 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6enclrr: crate::Reg<rcc_mp_tzahb6enclrr::RCC_MP_TZAHB6ENCLRR_SPEC>,
    _reserved59: [u8; 0x58],
    #[doc = "0x280 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb4ensetr: crate::Reg<rcc_mc_apb4ensetr::RCC_MC_APB4ENSETR_SPEC>,
    #[doc = "0x284 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb4enclrr: crate::Reg<rcc_mc_apb4enclrr::RCC_MC_APB4ENCLRR_SPEC>,
    #[doc = "0x288 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb5ensetr: crate::Reg<rcc_mc_apb5ensetr::RCC_MC_APB5ENSETR_SPEC>,
    #[doc = "0x28c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb5enclrr: crate::Reg<rcc_mc_apb5enclrr::RCC_MC_APB5ENCLRR_SPEC>,
    #[doc = "0x290 - This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5ensetr: crate::Reg<rcc_mc_ahb5ensetr::RCC_MC_AHB5ENSETR_SPEC>,
    #[doc = "0x294 - This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5enclrr: crate::Reg<rcc_mc_ahb5enclrr::RCC_MC_AHB5ENCLRR_SPEC>,
    #[doc = "0x298 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb6ensetr: crate::Reg<rcc_mc_ahb6ensetr::RCC_MC_AHB6ENSETR_SPEC>,
    #[doc = "0x29c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb6enclrr: crate::Reg<rcc_mc_ahb6enclrr::RCC_MC_AHB6ENCLRR_SPEC>,
    _reserved67: [u8; 0x60],
    #[doc = "0x300 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb4lpensetr: crate::Reg<rcc_mp_apb4lpensetr::RCC_MP_APB4LPENSETR_SPEC>,
    #[doc = "0x304 - This register is used by the MCU"]
    pub rcc_mp_apb4lpenclrr: crate::Reg<rcc_mp_apb4lpenclrr::RCC_MP_APB4LPENCLRR_SPEC>,
    #[doc = "0x308 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_apb5lpensetr: crate::Reg<rcc_mp_apb5lpensetr::RCC_MP_APB5LPENSETR_SPEC>,
    #[doc = "0x30c - This register is used by the Mpu."]
    pub rcc_mp_apb5lpenclrr: crate::Reg<rcc_mp_apb5lpenclrr::RCC_MP_APB5LPENCLRR_SPEC>,
    #[doc = "0x310 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_ahb5lpensetr: crate::Reg<rcc_mp_ahb5lpensetr::RCC_MP_AHB5LPENSETR_SPEC>,
    #[doc = "0x314 - This register is used by the MCU"]
    pub rcc_mp_ahb5lpenclrr: crate::Reg<rcc_mp_ahb5lpenclrr::RCC_MP_AHB5LPENCLRR_SPEC>,
    #[doc = "0x318 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_ahb6lpensetr: crate::Reg<rcc_mp_ahb6lpensetr::RCC_MP_AHB6LPENSETR_SPEC>,
    #[doc = "0x31c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_ahb6lpenclrr: crate::Reg<rcc_mp_ahb6lpenclrr::RCC_MP_AHB6LPENCLRR_SPEC>,
    #[doc = "0x320 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6lpensetr: crate::Reg<rcc_mp_tzahb6lpensetr::RCC_MP_TZAHB6LPENSETR_SPEC>,
    #[doc = "0x324 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6lpenclrr: crate::Reg<rcc_mp_tzahb6lpenclrr::RCC_MP_TZAHB6LPENCLRR_SPEC>,
    _reserved77: [u8; 0x58],
    #[doc = "0x380 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb4lpensetr: crate::Reg<rcc_mc_apb4lpensetr::RCC_MC_APB4LPENSETR_SPEC>,
    #[doc = "0x384 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb4lpenclrr: crate::Reg<rcc_mc_apb4lpenclrr::RCC_MC_APB4LPENCLRR_SPEC>,
    #[doc = "0x388 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb5lpensetr: crate::Reg<rcc_mc_apb5lpensetr::RCC_MC_APB5LPENSETR_SPEC>,
    #[doc = "0x38c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb5lpenclrr: crate::Reg<rcc_mc_apb5lpenclrr::RCC_MC_APB5LPENCLRR_SPEC>,
    #[doc = "0x390 - This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5lpensetr: crate::Reg<rcc_mc_ahb5lpensetr::RCC_MC_AHB5LPENSETR_SPEC>,
    #[doc = "0x394 - This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5lpenclrr: crate::Reg<rcc_mc_ahb5lpenclrr::RCC_MC_AHB5LPENCLRR_SPEC>,
    #[doc = "0x398 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb6lpensetr: crate::Reg<rcc_mc_ahb6lpensetr::RCC_MC_AHB6LPENSETR_SPEC>,
    #[doc = "0x39c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_ahb6lpenclrr: crate::Reg<rcc_mc_ahb6lpenclrr::RCC_MC_AHB6LPENCLRR_SPEC>,
    _reserved85: [u8; 0x60],
    #[doc = "0x400 - This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_br_rstsclrr: crate::Reg<rcc_br_rstsclrr::RCC_BR_RSTSCLRR_SPEC>,
    #[doc = "0x404 - This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset."]
    pub rcc_mp_grstcsetr: crate::Reg<rcc_mp_grstcsetr::RCC_MP_GRSTCSETR_SPEC>,
    #[doc = "0x408 - This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_rstsclrr: crate::Reg<rcc_mp_rstsclrr::RCC_MP_RSTSCLRR_SPEC>,
    #[doc = "0x40c - This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_iwdgfzsetr: crate::Reg<rcc_mp_iwdgfzsetr::RCC_MP_IWDGFZSETR_SPEC>,
    #[doc = "0x410 - This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_iwdgfzclrr: crate::Reg<rcc_mp_iwdgfzclrr::RCC_MP_IWDGFZCLRR_SPEC>,
    #[doc = "0x414 - This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_cier: crate::Reg<rcc_mp_cier::RCC_MP_CIER_SPEC>,
    #[doc = "0x418 - This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_cifr: crate::Reg<rcc_mp_cifr::RCC_MP_CIFR_SPEC>,
    #[doc = "0x41c - This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_pwrlpdlycr: crate::Reg<rcc_pwrlpdlycr::RCC_PWRLPDLYCR_SPEC>,
    #[doc = "0x420 - This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_rstssetr: crate::Reg<rcc_mp_rstssetr::RCC_MP_RSTSSETR_SPEC>,
    _reserved94: [u8; 0x03dc],
    #[doc = "0x800 - This register is used to select the clock generated on MCO1 output."]
    pub rcc_mco1cfgr: crate::Reg<rcc_mco1cfgr::RCC_MCO1CFGR_SPEC>,
    #[doc = "0x804 - This register is used to select the clock generated on MCO2 output."]
    pub rcc_mco2cfgr: crate::Reg<rcc_mco2cfgr::RCC_MCO2CFGR_SPEC>,
    #[doc = "0x808 - This is a read-only access register, It contains the status flags of oscillators. Writing has no effect."]
    pub rcc_ocrdyr: crate::Reg<rcc_ocrdyr::RCC_OCRDYR_SPEC>,
    #[doc = "0x80c - This is register contains the enable control of the debug and trace function, and the clock divider for the trace function."]
    pub rcc_dbgcfgr: crate::Reg<rcc_dbgcfgr::RCC_DBGCFGR_SPEC>,
    _reserved98: [u8; 0x10],
    #[doc = "0x820 - This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_rck3selr: crate::Reg<rcc_rck3selr::RCC_RCK3SELR_SPEC>,
    #[doc = "0x824 - This register is used to select the reference clock for PLL4."]
    pub rcc_rck4selr: crate::Reg<rcc_rck4selr::RCC_RCK4SELR_SPEC>,
    #[doc = "0x828 - This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_timg1prer: crate::Reg<rcc_timg1prer::RCC_TIMG1PRER_SPEC>,
    #[doc = "0x82c - This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_timg2prer: crate::Reg<rcc_timg2prer::RCC_TIMG2PRER_SPEC>,
    #[doc = "0x830 - This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mcudivr: crate::Reg<rcc_mcudivr::RCC_MCUDIVR_SPEC>,
    #[doc = "0x834 - This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information."]
    pub rcc_apb1divr: crate::Reg<rcc_apb1divr::RCC_APB1DIVR_SPEC>,
    #[doc = "0x838 - This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_apb2divr: crate::Reg<rcc_apb2divr::RCC_APB2DIVR_SPEC>,
    #[doc = "0x83c - This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_apb3divr: crate::Reg<rcc_apb3divr::RCC_APB3DIVR_SPEC>,
    _reserved106: [u8; 0x40],
    #[doc = "0x880 - This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3cr: crate::Reg<rcc_pll3cr::RCC_PLL3CR_SPEC>,
    #[doc = "0x884 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3cfgr1: crate::Reg<rcc_pll3cfgr1::RCC_PLL3CFGR1_SPEC>,
    #[doc = "0x888 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3cfgr2: crate::Reg<rcc_pll3cfgr2::RCC_PLL3CFGR2_SPEC>,
    #[doc = "0x88c - This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3fracr: crate::Reg<rcc_pll3fracr::RCC_PLL3FRACR_SPEC>,
    #[doc = "0x890 - This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3csgr: crate::Reg<rcc_pll3csgr::RCC_PLL3CSGR_SPEC>,
    #[doc = "0x894 - This register is used to control the PLL4."]
    pub rcc_pll4cr: crate::Reg<rcc_pll4cr::RCC_PLL4CR_SPEC>,
    #[doc = "0x898 - This register is used to configure the PLL4."]
    pub rcc_pll4cfgr1: crate::Reg<rcc_pll4cfgr1::RCC_PLL4CFGR1_SPEC>,
    #[doc = "0x89c - This register is used to configure the PLL4."]
    pub rcc_pll4cfgr2: crate::Reg<rcc_pll4cfgr2::RCC_PLL4CFGR2_SPEC>,
    #[doc = "0x8a0 - This register is used to fine-tune the frequency of the PLL4 VCO."]
    pub rcc_pll4fracr: crate::Reg<rcc_pll4fracr::RCC_PLL4FRACR_SPEC>,
    #[doc = "0x8a4 - This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll4csgr: crate::Reg<rcc_pll4csgr::RCC_PLL4CSGR_SPEC>,
    _reserved116: [u8; 0x18],
    #[doc = "0x8c0 - This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_i2c12ckselr: crate::Reg<rcc_i2c12ckselr::RCC_I2C12CKSELR_SPEC>,
    #[doc = "0x8c4 - This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_i2c35ckselr: crate::Reg<rcc_i2c35ckselr::RCC_I2C35CKSELR_SPEC>,
    #[doc = "0x8c8 - This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai1ckselr: crate::Reg<rcc_sai1ckselr::RCC_SAI1CKSELR_SPEC>,
    #[doc = "0x8cc - This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai2ckselr: crate::Reg<rcc_sai2ckselr::RCC_SAI2CKSELR_SPEC>,
    #[doc = "0x8d0 - This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai3ckselr: crate::Reg<rcc_sai3ckselr::RCC_SAI3CKSELR_SPEC>,
    #[doc = "0x8d4 - This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai4ckselr: crate::Reg<rcc_sai4ckselr::RCC_SAI4CKSELR_SPEC>,
    #[doc = "0x8d8 - This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spi2s1ckselr: crate::Reg<rcc_spi2s1ckselr::RCC_SPI2S1CKSELR_SPEC>,
    #[doc = "0x8dc - This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spi2s23ckselr: crate::Reg<rcc_spi2s23ckselr::RCC_SPI2S23CKSELR_SPEC>,
    #[doc = "0x8e0 - This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spi45ckselr: crate::Reg<rcc_spi45ckselr::RCC_SPI45CKSELR_SPEC>,
    #[doc = "0x8e4 - This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart6ckselr: crate::Reg<rcc_uart6ckselr::RCC_UART6CKSELR_SPEC>,
    #[doc = "0x8e8 - This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart24ckselr: crate::Reg<rcc_uart24ckselr::RCC_UART24CKSELR_SPEC>,
    #[doc = "0x8ec - This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart35ckselr: crate::Reg<rcc_uart35ckselr::RCC_UART35CKSELR_SPEC>,
    #[doc = "0x8f0 - This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart78ckselr: crate::Reg<rcc_uart78ckselr::RCC_UART78CKSELR_SPEC>,
    #[doc = "0x8f4 - This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sdmmc12ckselr: crate::Reg<rcc_sdmmc12ckselr::RCC_SDMMC12CKSELR_SPEC>,
    #[doc = "0x8f8 - This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sdmmc3ckselr: crate::Reg<rcc_sdmmc3ckselr::RCC_SDMMC3CKSELR_SPEC>,
    #[doc = "0x8fc - This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_ethckselr: crate::Reg<rcc_ethckselr::RCC_ETHCKSELR_SPEC>,
    #[doc = "0x900 - This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_qspickselr: crate::Reg<rcc_qspickselr::RCC_QSPICKSELR_SPEC>,
    #[doc = "0x904 - This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_fmcckselr: crate::Reg<rcc_fmcckselr::RCC_FMCCKSELR_SPEC>,
    _reserved134: [u8; 0x04],
    #[doc = "0x90c - This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_fdcanckselr: crate::Reg<rcc_fdcanckselr::RCC_FDCANCKSELR_SPEC>,
    _reserved135: [u8; 0x04],
    #[doc = "0x914 - This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spdifckselr: crate::Reg<rcc_spdifckselr::RCC_SPDIFCKSELR_SPEC>,
    #[doc = "0x918 - This register is used to control the selection of the kernel clock for the CEC-HDMI."]
    pub rcc_cecckselr: crate::Reg<rcc_cecckselr::RCC_CECCKSELR_SPEC>,
    #[doc = "0x91c - This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG"]
    pub rcc_usbckselr: crate::Reg<rcc_usbckselr::RCC_USBCKSELR_SPEC>,
    #[doc = "0x920 - This register is used to control the selection of the kernel clock for the RNG2."]
    pub rcc_rng2ckselr: crate::Reg<rcc_rng2ckselr::RCC_RNG2CKSELR_SPEC>,
    #[doc = "0x924 - This register is used to control the selection of the kernel clock for the DSI block."]
    pub rcc_dsickselr: crate::Reg<rcc_dsickselr::RCC_DSICKSELR_SPEC>,
    #[doc = "0x928 - This register is used to control the selection of the kernel clock for the ADC block."]
    pub rcc_adcckselr: crate::Reg<rcc_adcckselr::RCC_ADCCKSELR_SPEC>,
    #[doc = "0x92c - This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks."]
    pub rcc_lptim45ckselr: crate::Reg<rcc_lptim45ckselr::RCC_LPTIM45CKSELR_SPEC>,
    #[doc = "0x930 - This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks."]
    pub rcc_lptim23ckselr: crate::Reg<rcc_lptim23ckselr::RCC_LPTIM23CKSELR_SPEC>,
    #[doc = "0x934 - This register is used to control the selection of the kernel clock for the LPTIM1 block."]
    pub rcc_lptim1ckselr: crate::Reg<rcc_lptim1ckselr::RCC_LPTIM1CKSELR_SPEC>,
    _reserved144: [u8; 0x48],
    #[doc = "0x980 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_apb1rstsetr: crate::Reg<rcc_apb1rstsetr::RCC_APB1RSTSETR_SPEC>,
    #[doc = "0x984 - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_apb1rstclrr: crate::Reg<rcc_apb1rstclrr::RCC_APB1RSTCLRR_SPEC>,
    #[doc = "0x988 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_apb2rstsetr: crate::Reg<rcc_apb2rstsetr::RCC_APB2RSTSETR_SPEC>,
    #[doc = "0x98c - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_apb2rstclrr: crate::Reg<rcc_apb2rstclrr::RCC_APB2RSTCLRR_SPEC>,
    #[doc = "0x990 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_apb3rstsetr: crate::Reg<rcc_apb3rstsetr::RCC_APB3RSTSETR_SPEC>,
    #[doc = "0x994 - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_apb3rstclrr: crate::Reg<rcc_apb3rstclrr::RCC_APB3RSTCLRR_SPEC>,
    #[doc = "0x998 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_ahb2rstsetr: crate::Reg<rcc_ahb2rstsetr::RCC_AHB2RSTSETR_SPEC>,
    #[doc = "0x99c - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_ahb2rstclrr: crate::Reg<rcc_ahb2rstclrr::RCC_AHB2RSTCLRR_SPEC>,
    #[doc = "0x9a0 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_ahb3rstsetr: crate::Reg<rcc_ahb3rstsetr::RCC_AHB3RSTSETR_SPEC>,
    #[doc = "0x9a4 - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_ahb3rstclrr: crate::Reg<rcc_ahb3rstclrr::RCC_AHB3RSTCLRR_SPEC>,
    #[doc = "0x9a8 - This register is used to activate the reset of the corresponding peripheral"]
    pub rcc_ahb4rstsetr: crate::Reg<rcc_ahb4rstsetr::RCC_AHB4RSTSETR_SPEC>,
    #[doc = "0x9ac - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_ahb4rstclrr: crate::Reg<rcc_ahb4rstclrr::RCC_AHB4RSTCLRR_SPEC>,
    _reserved156: [u8; 0x50],
    #[doc = "0xa00 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_apb1ensetr: crate::Reg<rcc_mp_apb1ensetr::RCC_MP_APB1ENSETR_SPEC>,
    #[doc = "0xa04 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mp_apb1enclrr: crate::Reg<rcc_mp_apb1enclrr::RCC_MP_APB1ENCLRR_SPEC>,
    #[doc = "0xa08 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_apb2ensetr: crate::Reg<rcc_mp_apb2ensetr::RCC_MP_APB2ENSETR_SPEC>,
    #[doc = "0xa0c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_apb2enclrr: crate::Reg<rcc_mp_apb2enclrr::RCC_MP_APB2ENCLRR_SPEC>,
    #[doc = "0xa10 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_apb3ensetr: crate::Reg<rcc_mp_apb3ensetr::RCC_MP_APB3ENSETR_SPEC>,
    #[doc = "0xa14 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_apb3enclrr: crate::Reg<rcc_mp_apb3enclrr::RCC_MP_APB3ENCLRR_SPEC>,
    #[doc = "0xa18 - This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
    pub rcc_mp_ahb2ensetr: crate::Reg<rcc_mp_ahb2ensetr::RCC_MP_AHB2ENSETR_SPEC>,
    #[doc = "0xa1c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_ahb2enclrr: crate::Reg<rcc_mp_ahb2enclrr::RCC_MP_AHB2ENCLRR_SPEC>,
    #[doc = "0xa20 - This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
    pub rcc_mp_ahb3ensetr: crate::Reg<rcc_mp_ahb3ensetr::RCC_MP_AHB3ENSETR_SPEC>,
    #[doc = "0xa24 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_ahb3enclrr: crate::Reg<rcc_mp_ahb3enclrr::RCC_MP_AHB3ENCLRR_SPEC>,
    #[doc = "0xa28 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU."]
    pub rcc_mp_ahb4ensetr: crate::Reg<rcc_mp_ahb4ensetr::RCC_MP_AHB4ENSETR_SPEC>,
    #[doc = "0xa2c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mp_ahb4enclrr: crate::Reg<rcc_mp_ahb4enclrr::RCC_MP_AHB4ENCLRR_SPEC>,
    _reserved168: [u8; 0x08],
    #[doc = "0xa38 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_mlahbensetr: crate::Reg<rcc_mp_mlahbensetr::RCC_MP_MLAHBENSETR_SPEC>,
    #[doc = "0xa3c - This register is used to clear the peripheral clock enable bit."]
    pub rcc_mp_mlahbenclrr: crate::Reg<rcc_mp_mlahbenclrr::RCC_MP_MLAHBENCLRR_SPEC>,
    _reserved170: [u8; 0x40],
    #[doc = "0xa80 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to ."]
    pub rcc_mc_apb1ensetr: crate::Reg<rcc_mc_apb1ensetr::RCC_MC_APB1ENSETR_SPEC>,
    #[doc = "0xa84 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mc_apb1enclrr: crate::Reg<rcc_mc_apb1enclrr::RCC_MC_APB1ENCLRR_SPEC>,
    #[doc = "0xa88 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb2ensetr: crate::Reg<rcc_mc_apb2ensetr::RCC_MC_APB2ENSETR_SPEC>,
    #[doc = "0xa8c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb2enclrr: crate::Reg<rcc_mc_apb2enclrr::RCC_MC_APB2ENCLRR_SPEC>,
    #[doc = "0xa90 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb3ensetr: crate::Reg<rcc_mc_apb3ensetr::RCC_MC_APB3ENSETR_SPEC>,
    #[doc = "0xa94 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb3enclrr: crate::Reg<rcc_mc_apb3enclrr::RCC_MC_APB3ENCLRR_SPEC>,
    #[doc = "0xa98 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb2ensetr: crate::Reg<rcc_mc_ahb2ensetr::RCC_MC_AHB2ENSETR_SPEC>,
    #[doc = "0xa9c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb2enclrr: crate::Reg<rcc_mc_ahb2enclrr::RCC_MC_AHB2ENCLRR_SPEC>,
    #[doc = "0xaa0 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb3ensetr: crate::Reg<rcc_mc_ahb3ensetr::RCC_MC_AHB3ENSETR_SPEC>,
    #[doc = "0xaa4 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb3enclrr: crate::Reg<rcc_mc_ahb3enclrr::RCC_MC_AHB3ENCLRR_SPEC>,
    #[doc = "0xaa8 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb4ensetr: crate::Reg<rcc_mc_ahb4ensetr::RCC_MC_AHB4ENSETR_SPEC>,
    #[doc = "0xaac - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb4enclrr: crate::Reg<rcc_mc_ahb4enclrr::RCC_MC_AHB4ENCLRR_SPEC>,
    #[doc = "0xab0 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_aximensetr: crate::Reg<rcc_mc_aximensetr::RCC_MC_AXIMENSETR_SPEC>,
    #[doc = "0xab4 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_aximenclrr: crate::Reg<rcc_mc_aximenclrr::RCC_MC_AXIMENCLRR_SPEC>,
    #[doc = "0xab8 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_mlahbensetr: crate::Reg<rcc_mc_mlahbensetr::RCC_MC_MLAHBENSETR_SPEC>,
    #[doc = "0xabc - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_mlahbenclrr: crate::Reg<rcc_mc_mlahbenclrr::RCC_MC_MLAHBENCLRR_SPEC>,
    _reserved186: [u8; 0x40],
    #[doc = "0xb00 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb1lpensetr: crate::Reg<rcc_mp_apb1lpensetr::RCC_MP_APB1LPENSETR_SPEC>,
    #[doc = "0xb04 - This register is used by the MPU in order to clear the PERxLPEN bits ."]
    pub rcc_mp_apb1lpenclrr: crate::Reg<rcc_mp_apb1lpenclrr::RCC_MP_APB1LPENCLRR_SPEC>,
    #[doc = "0xb08 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb2lpensetr: crate::Reg<rcc_mp_apb2lpensetr::RCC_MP_APB2LPENSETR_SPEC>,
    #[doc = "0xb0c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb2lpenclrr: crate::Reg<rcc_mp_apb2lpenclrr::RCC_MP_APB2LPENCLRR_SPEC>,
    #[doc = "0xb10 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb3lpensetr: crate::Reg<rcc_mp_apb3lpensetr::RCC_MP_APB3LPENSETR_SPEC>,
    #[doc = "0xb14 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb3lpenclrr: crate::Reg<rcc_mp_apb3lpenclrr::RCC_MP_APB3LPENCLRR_SPEC>,
    #[doc = "0xb18 - This register is used by the MPU in order to set the PERxLPEN bit."]
    pub rcc_mp_ahb2lpensetr: crate::Reg<rcc_mp_ahb2lpensetr::RCC_MP_AHB2LPENSETR_SPEC>,
    #[doc = "0xb1c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_ahb2lpenclrr: crate::Reg<rcc_mp_ahb2lpenclrr::RCC_MP_AHB2LPENCLRR_SPEC>,
    #[doc = "0xb20 - This register is used by the MPU"]
    pub rcc_mp_ahb3lpensetr: crate::Reg<rcc_mp_ahb3lpensetr::RCC_MP_AHB3LPENSETR_SPEC>,
    #[doc = "0xb24 - This register is used by the MPU in order to clear the PERxLPEN bit"]
    pub rcc_mp_ahb3lpenclrr: crate::Reg<rcc_mp_ahb3lpenclrr::RCC_MP_AHB3LPENCLRR_SPEC>,
    #[doc = "0xb28 - This register is used by the MPU"]
    pub rcc_mp_ahb4lpensetr: crate::Reg<rcc_mp_ahb4lpensetr::RCC_MP_AHB4LPENSETR_SPEC>,
    #[doc = "0xb2c - This register is used by the MPU"]
    pub rcc_mp_ahb4lpenclrr: crate::Reg<rcc_mp_ahb4lpenclrr::RCC_MP_AHB4LPENCLRR_SPEC>,
    #[doc = "0xb30 - This register is used by the MPU"]
    pub rcc_mp_aximlpensetr: crate::Reg<rcc_mp_aximlpensetr::RCC_MP_AXIMLPENSETR_SPEC>,
    #[doc = "0xb34 - This register is used by the MPU"]
    pub rcc_mp_aximlpenclrr: crate::Reg<rcc_mp_aximlpenclrr::RCC_MP_AXIMLPENCLRR_SPEC>,
    #[doc = "0xb38 - This register is used by the MPU in order to set the PERxLPEN bit"]
    pub rcc_mp_mlahblpensetr: crate::Reg<rcc_mp_mlahblpensetr::RCC_MP_MLAHBLPENSETR_SPEC>,
    #[doc = "0xb3c - This register is used by the MPU in order to clear the PERxLPEN bit"]
    pub rcc_mp_mlahblpenclrr: crate::Reg<rcc_mp_mlahblpenclrr::RCC_MP_MLAHBLPENCLRR_SPEC>,
    _reserved202: [u8; 0x40],
    #[doc = "0xb80 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb1lpensetr: crate::Reg<rcc_mc_apb1lpensetr::RCC_MC_APB1LPENSETR_SPEC>,
    #[doc = "0xb84 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mc_apb1lpenclrr: crate::Reg<rcc_mc_apb1lpenclrr::RCC_MC_APB1LPENCLRR_SPEC>,
    #[doc = "0xb88 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb2lpensetr: crate::Reg<rcc_mc_apb2lpensetr::RCC_MC_APB2LPENSETR_SPEC>,
    #[doc = "0xb8c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb2lpenclrr: crate::Reg<rcc_mc_apb2lpenclrr::RCC_MC_APB2LPENCLRR_SPEC>,
    #[doc = "0xb90 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb3lpensetr: crate::Reg<rcc_mc_apb3lpensetr::RCC_MC_APB3LPENSETR_SPEC>,
    #[doc = "0xb94 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb3lpenclrr: crate::Reg<rcc_mc_apb3lpenclrr::RCC_MC_APB3LPENCLRR_SPEC>,
    #[doc = "0xb98 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb2lpensetr: crate::Reg<rcc_mc_ahb2lpensetr::RCC_MC_AHB2LPENSETR_SPEC>,
    #[doc = "0xb9c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_ahb2lpenclrr: crate::Reg<rcc_mc_ahb2lpenclrr::RCC_MC_AHB2LPENCLRR_SPEC>,
    #[doc = "0xba0 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb3lpensetr: crate::Reg<rcc_mc_ahb3lpensetr::RCC_MC_AHB3LPENSETR_SPEC>,
    #[doc = "0xba4 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_ahb3lpenclrr: crate::Reg<rcc_mc_ahb3lpenclrr::RCC_MC_AHB3LPENCLRR_SPEC>,
    #[doc = "0xba8 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb4lpensetr: crate::Reg<rcc_mc_ahb4lpensetr::RCC_MC_AHB4LPENSETR_SPEC>,
    #[doc = "0xbac - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_ahb4lpenclrr: crate::Reg<rcc_mc_ahb4lpenclrr::RCC_MC_AHB4LPENCLRR_SPEC>,
    #[doc = "0xbb0 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_aximlpensetr: crate::Reg<rcc_mc_aximlpensetr::RCC_MC_AXIMLPENSETR_SPEC>,
    #[doc = "0xbb4 - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_aximlpenclrr: crate::Reg<rcc_mc_aximlpenclrr::RCC_MC_AXIMLPENCLRR_SPEC>,
    #[doc = "0xbb8 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_mlahblpensetr: crate::Reg<rcc_mc_mlahblpensetr::RCC_MC_MLAHBLPENSETR_SPEC>,
    #[doc = "0xbbc - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_mlahblpenclrr: crate::Reg<rcc_mc_mlahblpenclrr::RCC_MC_MLAHBLPENCLRR_SPEC>,
    _reserved218: [u8; 0x40],
    #[doc = "0xc00 - This register is used by the MCU to check the reset source."]
    pub rcc_mc_rstsclrr: crate::Reg<rcc_mc_rstsclrr::RCC_MC_RSTSCLRR_SPEC>,
    _reserved219: [u8; 0x10],
    #[doc = "0xc14 - This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details."]
    pub rcc_mc_cier: crate::Reg<rcc_mc_cier::RCC_MC_CIER_SPEC>,
    #[doc = "0xc18 - This register shall be used by the MCU in order to read and clear the interrupt flags."]
    pub rcc_mc_cifr: crate::Reg<rcc_mc_cifr::RCC_MC_CIFR_SPEC>,
    _reserved221: [u8; 0x03d8],
    #[doc = "0xff4 - This register gives the IP version"]
    pub rcc_verr: crate::Reg<rcc_verr::RCC_VERR_SPEC>,
    #[doc = "0xff8 - This register gives the unique identifier of the RCC"]
    pub rcc_idr: crate::Reg<rcc_idr::RCC_IDR_SPEC>,
    #[doc = "0xffc - This register gives the decoding space, which is for the RCC of 4 kB."]
    pub rcc_sidr: crate::Reg<rcc_sidr::RCC_SIDR_SPEC>,
}
#[doc = "RCC_TZCR register accessor: an alias for `Reg<RCC_TZCR_SPEC>`"]
pub type RCC_TZCR = crate::Reg<rcc_tzcr::RCC_TZCR_SPEC>;
#[doc = "This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode."]
pub mod rcc_tzcr;
#[doc = "RCC_OCENSETR register accessor: an alias for `Reg<RCC_OCENSETR_SPEC>`"]
pub type RCC_OCENSETR = crate::Reg<rcc_ocensetr::RCC_OCENSETR_SPEC>;
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_ocensetr;
#[doc = "RCC_OCENCLRR register accessor: an alias for `Reg<RCC_OCENCLRR_SPEC>`"]
pub type RCC_OCENCLRR = crate::Reg<rcc_ocenclrr::RCC_OCENCLRR_SPEC>;
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_ocenclrr;
#[doc = "RCC_HSICFGR register accessor: an alias for `Reg<RCC_HSICFGR_SPEC>`"]
pub type RCC_HSICFGR = crate::Reg<rcc_hsicfgr::RCC_HSICFGR_SPEC>;
#[doc = "This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_hsicfgr;
#[doc = "RCC_CSICFGR register accessor: an alias for `Reg<RCC_CSICFGR_SPEC>`"]
pub type RCC_CSICFGR = crate::Reg<rcc_csicfgr::RCC_CSICFGR_SPEC>;
#[doc = "This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details."]
pub mod rcc_csicfgr;
#[doc = "RCC_MPCKSELR register accessor: an alias for `Reg<RCC_MPCKSELR_SPEC>`"]
pub type RCC_MPCKSELR = crate::Reg<rcc_mpckselr::RCC_MPCKSELR_SPEC>;
#[doc = "This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_mpckselr;
#[doc = "RCC_ASSCKSELR register accessor: an alias for `Reg<RCC_ASSCKSELR_SPEC>`"]
pub type RCC_ASSCKSELR = crate::Reg<rcc_assckselr::RCC_ASSCKSELR_SPEC>;
#[doc = "This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_assckselr;
#[doc = "RCC_RCK12SELR register accessor: an alias for `Reg<RCC_RCK12SELR_SPEC>`"]
pub type RCC_RCK12SELR = crate::Reg<rcc_rck12selr::RCC_RCK12SELR_SPEC>;
#[doc = "This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_rck12selr;
#[doc = "RCC_MPCKDIVR register accessor: an alias for `Reg<RCC_MPCKDIVR_SPEC>`"]
pub type RCC_MPCKDIVR = crate::Reg<rcc_mpckdivr::RCC_MPCKDIVR_SPEC>;
#[doc = "This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mpckdivr;
#[doc = "RCC_AXIDIVR register accessor: an alias for `Reg<RCC_AXIDIVR_SPEC>`"]
pub type RCC_AXIDIVR = crate::Reg<rcc_axidivr::RCC_AXIDIVR_SPEC>;
#[doc = "This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_axidivr;
#[doc = "RCC_APB4DIVR register accessor: an alias for `Reg<RCC_APB4DIVR_SPEC>`"]
pub type RCC_APB4DIVR = crate::Reg<rcc_apb4divr::RCC_APB4DIVR_SPEC>;
#[doc = "This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb4divr;
#[doc = "RCC_APB5DIVR register accessor: an alias for `Reg<RCC_APB5DIVR_SPEC>`"]
pub type RCC_APB5DIVR = crate::Reg<rcc_apb5divr::RCC_APB5DIVR_SPEC>;
#[doc = "This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5divr;
#[doc = "RCC_RTCDIVR register accessor: an alias for `Reg<RCC_RTCDIVR_SPEC>`"]
pub type RCC_RTCDIVR = crate::Reg<rcc_rtcdivr::RCC_RTCDIVR_SPEC>;
#[doc = "This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rtcdivr;
#[doc = "RCC_MSSCKSELR register accessor: an alias for `Reg<RCC_MSSCKSELR_SPEC>`"]
pub type RCC_MSSCKSELR = crate::Reg<rcc_mssckselr::RCC_MSSCKSELR_SPEC>;
#[doc = "This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_mssckselr;
#[doc = "RCC_PLL1CR register accessor: an alias for `Reg<RCC_PLL1CR_SPEC>`"]
pub type RCC_PLL1CR = crate::Reg<rcc_pll1cr::RCC_PLL1CR_SPEC>;
#[doc = "This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cr;
#[doc = "RCC_PLL1CFGR1 register accessor: an alias for `Reg<RCC_PLL1CFGR1_SPEC>`"]
pub type RCC_PLL1CFGR1 = crate::Reg<rcc_pll1cfgr1::RCC_PLL1CFGR1_SPEC>;
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cfgr1;
#[doc = "RCC_PLL1CFGR2 register accessor: an alias for `Reg<RCC_PLL1CFGR2_SPEC>`"]
pub type RCC_PLL1CFGR2 = crate::Reg<rcc_pll1cfgr2::RCC_PLL1CFGR2_SPEC>;
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cfgr2;
#[doc = "RCC_PLL1FRACR register accessor: an alias for `Reg<RCC_PLL1FRACR_SPEC>`"]
pub type RCC_PLL1FRACR = crate::Reg<rcc_pll1fracr::RCC_PLL1FRACR_SPEC>;
#[doc = "This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1fracr;
#[doc = "RCC_PLL1CSGR register accessor: an alias for `Reg<RCC_PLL1CSGR_SPEC>`"]
pub type RCC_PLL1CSGR = crate::Reg<rcc_pll1csgr::RCC_PLL1CSGR_SPEC>;
#[doc = "This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1csgr;
#[doc = "RCC_PLL2CR register accessor: an alias for `Reg<RCC_PLL2CR_SPEC>`"]
pub type RCC_PLL2CR = crate::Reg<rcc_pll2cr::RCC_PLL2CR_SPEC>;
#[doc = "This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cr;
#[doc = "RCC_PLL2CFGR1 register accessor: an alias for `Reg<RCC_PLL2CFGR1_SPEC>`"]
pub type RCC_PLL2CFGR1 = crate::Reg<rcc_pll2cfgr1::RCC_PLL2CFGR1_SPEC>;
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cfgr1;
#[doc = "RCC_PLL2CFGR2 register accessor: an alias for `Reg<RCC_PLL2CFGR2_SPEC>`"]
pub type RCC_PLL2CFGR2 = crate::Reg<rcc_pll2cfgr2::RCC_PLL2CFGR2_SPEC>;
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cfgr2;
#[doc = "RCC_PLL2FRACR register accessor: an alias for `Reg<RCC_PLL2FRACR_SPEC>`"]
pub type RCC_PLL2FRACR = crate::Reg<rcc_pll2fracr::RCC_PLL2FRACR_SPEC>;
#[doc = "This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2fracr;
#[doc = "RCC_PLL2CSGR register accessor: an alias for `Reg<RCC_PLL2CSGR_SPEC>`"]
pub type RCC_PLL2CSGR = crate::Reg<rcc_pll2csgr::RCC_PLL2CSGR_SPEC>;
#[doc = "This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2csgr;
#[doc = "RCC_I2C46CKSELR register accessor: an alias for `Reg<RCC_I2C46CKSELR_SPEC>`"]
pub type RCC_I2C46CKSELR = crate::Reg<rcc_i2c46ckselr::RCC_I2C46CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_i2c46ckselr;
#[doc = "RCC_SPI6CKSELR register accessor: an alias for `Reg<RCC_SPI6CKSELR_SPEC>`"]
pub type RCC_SPI6CKSELR = crate::Reg<rcc_spi6ckselr::RCC_SPI6CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_spi6ckselr;
#[doc = "RCC_UART1CKSELR register accessor: an alias for `Reg<RCC_UART1CKSELR_SPEC>`"]
pub type RCC_UART1CKSELR = crate::Reg<rcc_uart1ckselr::RCC_UART1CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_uart1ckselr;
#[doc = "RCC_RNG1CKSELR register accessor: an alias for `Reg<RCC_RNG1CKSELR_SPEC>`"]
pub type RCC_RNG1CKSELR = crate::Reg<rcc_rng1ckselr::RCC_RNG1CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rng1ckselr;
#[doc = "RCC_CPERCKSELR register accessor: an alias for `Reg<RCC_CPERCKSELR_SPEC>`"]
pub type RCC_CPERCKSELR = crate::Reg<rcc_cperckselr::RCC_CPERCKSELR_SPEC>;
#[doc = "This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays."]
pub mod rcc_cperckselr;
#[doc = "RCC_STGENCKSELR register accessor: an alias for `Reg<RCC_STGENCKSELR_SPEC>`"]
pub type RCC_STGENCKSELR = crate::Reg<rcc_stgenckselr::RCC_STGENCKSELR_SPEC>;
#[doc = "This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_stgenckselr;
#[doc = "RCC_DDRITFCR register accessor: an alias for `Reg<RCC_DDRITFCR_SPEC>`"]
pub type RCC_DDRITFCR = crate::Reg<rcc_ddritfcr::RCC_DDRITFCR_SPEC>;
#[doc = "This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ddritfcr;
#[doc = "RCC_MP_BOOTCR register accessor: an alias for `Reg<RCC_MP_BOOTCR_SPEC>`"]
pub type RCC_MP_BOOTCR = crate::Reg<rcc_mp_bootcr::RCC_MP_BOOTCR_SPEC>;
#[doc = "This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU."]
pub mod rcc_mp_bootcr;
#[doc = "RCC_MP_SREQSETR register accessor: an alias for `Reg<RCC_MP_SREQSETR_SPEC>`"]
pub type RCC_MP_SREQSETR = crate::Reg<rcc_mp_sreqsetr::RCC_MP_SREQSETR_SPEC>;
#[doc = "Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_sreqsetr;
#[doc = "RCC_MP_SREQCLRR register accessor: an alias for `Reg<RCC_MP_SREQCLRR_SPEC>`"]
pub type RCC_MP_SREQCLRR = crate::Reg<rcc_mp_sreqclrr::RCC_MP_SREQCLRR_SPEC>;
#[doc = "Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_sreqclrr;
#[doc = "RCC_MP_GCR register accessor: an alias for `Reg<RCC_MP_GCR_SPEC>`"]
pub type RCC_MP_GCR = crate::Reg<rcc_mp_gcr::RCC_MP_GCR_SPEC>;
#[doc = "The register contains global control bits. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_gcr;
#[doc = "RCC_MP_APRSTCR register accessor: an alias for `Reg<RCC_MP_APRSTCR_SPEC>`"]
pub type RCC_MP_APRSTCR = crate::Reg<rcc_mp_aprstcr::RCC_MP_APRSTCR_SPEC>;
#[doc = "This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_aprstcr;
#[doc = "RCC_MP_APRSTSR register accessor: an alias for `Reg<RCC_MP_APRSTSR_SPEC>`"]
pub type RCC_MP_APRSTSR = crate::Reg<rcc_mp_aprstsr::RCC_MP_APRSTSR_SPEC>;
#[doc = "This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_aprstsr;
#[doc = "RCC_BDCR register accessor: an alias for `Reg<RCC_BDCR_SPEC>`"]
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCR_SPEC>;
#[doc = "This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_bdcr;
#[doc = "RCC_RDLSICR register accessor: an alias for `Reg<RCC_RDLSICR_SPEC>`"]
pub type RCC_RDLSICR = crate::Reg<rcc_rdlsicr::RCC_RDLSICR_SPEC>;
#[doc = "This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rdlsicr;
#[doc = "RCC_APB4RSTSETR register accessor: an alias for `Reg<RCC_APB4RSTSETR_SPEC>`"]
pub type RCC_APB4RSTSETR = crate::Reg<rcc_apb4rstsetr::RCC_APB4RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
pub mod rcc_apb4rstsetr;
#[doc = "RCC_APB4RSTCLRR register accessor: an alias for `Reg<RCC_APB4RSTCLRR_SPEC>`"]
pub type RCC_APB4RSTCLRR = crate::Reg<rcc_apb4rstclrr::RCC_APB4RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
pub mod rcc_apb4rstclrr;
#[doc = "RCC_APB5RSTSETR register accessor: an alias for `Reg<RCC_APB5RSTSETR_SPEC>`"]
pub type RCC_APB5RSTSETR = crate::Reg<rcc_apb5rstsetr::RCC_APB5RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5rstsetr;
#[doc = "RCC_APB5RSTCLRR register accessor: an alias for `Reg<RCC_APB5RSTCLRR_SPEC>`"]
pub type RCC_APB5RSTCLRR = crate::Reg<rcc_apb5rstclrr::RCC_APB5RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5rstclrr;
#[doc = "RCC_AHB5RSTSETR register accessor: an alias for `Reg<RCC_AHB5RSTSETR_SPEC>`"]
pub type RCC_AHB5RSTSETR = crate::Reg<rcc_ahb5rstsetr::RCC_AHB5RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ahb5rstsetr;
#[doc = "RCC_AHB5RSTCLRR register accessor: an alias for `Reg<RCC_AHB5RSTCLRR_SPEC>`"]
pub type RCC_AHB5RSTCLRR = crate::Reg<rcc_ahb5rstclrr::RCC_AHB5RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ahb5rstclrr;
#[doc = "RCC_AHB6RSTSETR register accessor: an alias for `Reg<RCC_AHB6RSTSETR_SPEC>`"]
pub type RCC_AHB6RSTSETR = crate::Reg<rcc_ahb6rstsetr::RCC_AHB6RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
pub mod rcc_ahb6rstsetr;
#[doc = "RCC_AHB6RSTCLRR register accessor: an alias for `Reg<RCC_AHB6RSTCLRR_SPEC>`"]
pub type RCC_AHB6RSTCLRR = crate::Reg<rcc_ahb6rstclrr::RCC_AHB6RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
pub mod rcc_ahb6rstclrr;
#[doc = "RCC_TZAHB6RSTSETR register accessor: an alias for `Reg<RCC_TZAHB6RSTSETR_SPEC>`"]
pub type RCC_TZAHB6RSTSETR = crate::Reg<rcc_tzahb6rstsetr::RCC_TZAHB6RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_tzahb6rstsetr;
#[doc = "RCC_TZAHB6RSTCLRR register accessor: an alias for `Reg<RCC_TZAHB6RSTCLRR_SPEC>`"]
pub type RCC_TZAHB6RSTCLRR = crate::Reg<rcc_tzahb6rstclrr::RCC_TZAHB6RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_tzahb6rstclrr;
#[doc = "RCC_MP_APB4ENSETR register accessor: an alias for `Reg<RCC_MP_APB4ENSETR_SPEC>`"]
pub type RCC_MP_APB4ENSETR = crate::Reg<rcc_mp_apb4ensetr::RCC_MP_APB4ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb4ensetr;
#[doc = "RCC_MP_APB4ENCLRR register accessor: an alias for `Reg<RCC_MP_APB4ENCLRR_SPEC>`"]
pub type RCC_MP_APB4ENCLRR = crate::Reg<rcc_mp_apb4enclrr::RCC_MP_APB4ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb4enclrr;
#[doc = "RCC_MP_APB5ENSETR register accessor: an alias for `Reg<RCC_MP_APB5ENSETR_SPEC>`"]
pub type RCC_MP_APB5ENSETR = crate::Reg<rcc_mp_apb5ensetr::RCC_MP_APB5ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb5ensetr;
#[doc = "RCC_MP_APB5ENCLRR register accessor: an alias for `Reg<RCC_MP_APB5ENCLRR_SPEC>`"]
pub type RCC_MP_APB5ENCLRR = crate::Reg<rcc_mp_apb5enclrr::RCC_MP_APB5ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb5enclrr;
#[doc = "RCC_MP_AHB5ENSETR register accessor: an alias for `Reg<RCC_MP_AHB5ENSETR_SPEC>`"]
pub type RCC_MP_AHB5ENSETR = crate::Reg<rcc_mp_ahb5ensetr::RCC_MP_AHB5ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5ensetr;
#[doc = "RCC_MP_AHB5ENCLRR register accessor: an alias for `Reg<RCC_MP_AHB5ENCLRR_SPEC>`"]
pub type RCC_MP_AHB5ENCLRR = crate::Reg<rcc_mp_ahb5enclrr::RCC_MP_AHB5ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5enclrr;
#[doc = "RCC_MP_AHB6ENSETR register accessor: an alias for `Reg<RCC_MP_AHB6ENSETR_SPEC>`"]
pub type RCC_MP_AHB6ENSETR = crate::Reg<rcc_mp_ahb6ensetr::RCC_MP_AHB6ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_ahb6ensetr;
#[doc = "RCC_MP_AHB6ENCLRR register accessor: an alias for `Reg<RCC_MP_AHB6ENCLRR_SPEC>`"]
pub type RCC_MP_AHB6ENCLRR = crate::Reg<rcc_mp_ahb6enclrr::RCC_MP_AHB6ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_ahb6enclrr;
#[doc = "RCC_MP_TZAHB6ENSETR register accessor: an alias for `Reg<RCC_MP_TZAHB6ENSETR_SPEC>`"]
pub type RCC_MP_TZAHB6ENSETR = crate::Reg<rcc_mp_tzahb6ensetr::RCC_MP_TZAHB6ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6ensetr;
#[doc = "RCC_MP_TZAHB6ENCLRR register accessor: an alias for `Reg<RCC_MP_TZAHB6ENCLRR_SPEC>`"]
pub type RCC_MP_TZAHB6ENCLRR = crate::Reg<rcc_mp_tzahb6enclrr::RCC_MP_TZAHB6ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6enclrr;
#[doc = "RCC_MC_APB4ENSETR register accessor: an alias for `Reg<RCC_MC_APB4ENSETR_SPEC>`"]
pub type RCC_MC_APB4ENSETR = crate::Reg<rcc_mc_apb4ensetr::RCC_MC_APB4ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb4ensetr;
#[doc = "RCC_MC_APB4ENCLRR register accessor: an alias for `Reg<RCC_MC_APB4ENCLRR_SPEC>`"]
pub type RCC_MC_APB4ENCLRR = crate::Reg<rcc_mc_apb4enclrr::RCC_MC_APB4ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb4enclrr;
#[doc = "RCC_MC_APB5ENSETR register accessor: an alias for `Reg<RCC_MC_APB5ENSETR_SPEC>`"]
pub type RCC_MC_APB5ENSETR = crate::Reg<rcc_mc_apb5ensetr::RCC_MC_APB5ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb5ensetr;
#[doc = "RCC_MC_APB5ENCLRR register accessor: an alias for `Reg<RCC_MC_APB5ENCLRR_SPEC>`"]
pub type RCC_MC_APB5ENCLRR = crate::Reg<rcc_mc_apb5enclrr::RCC_MC_APB5ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb5enclrr;
#[doc = "RCC_MC_AHB5ENSETR register accessor: an alias for `Reg<RCC_MC_AHB5ENSETR_SPEC>`"]
pub type RCC_MC_AHB5ENSETR = crate::Reg<rcc_mc_ahb5ensetr::RCC_MC_AHB5ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5ensetr;
#[doc = "RCC_MC_AHB5ENCLRR register accessor: an alias for `Reg<RCC_MC_AHB5ENCLRR_SPEC>`"]
pub type RCC_MC_AHB5ENCLRR = crate::Reg<rcc_mc_ahb5enclrr::RCC_MC_AHB5ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5enclrr;
#[doc = "RCC_MC_AHB6ENSETR register accessor: an alias for `Reg<RCC_MC_AHB6ENSETR_SPEC>`"]
pub type RCC_MC_AHB6ENSETR = crate::Reg<rcc_mc_ahb6ensetr::RCC_MC_AHB6ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb6ensetr;
#[doc = "RCC_MC_AHB6ENCLRR register accessor: an alias for `Reg<RCC_MC_AHB6ENCLRR_SPEC>`"]
pub type RCC_MC_AHB6ENCLRR = crate::Reg<rcc_mc_ahb6enclrr::RCC_MC_AHB6ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb6enclrr;
#[doc = "RCC_MP_APB4LPENSETR register accessor: an alias for `Reg<RCC_MP_APB4LPENSETR_SPEC>`"]
pub type RCC_MP_APB4LPENSETR = crate::Reg<rcc_mp_apb4lpensetr::RCC_MP_APB4LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb4lpensetr;
#[doc = "RCC_MP_APB4LPENCLRR register accessor: an alias for `Reg<RCC_MP_APB4LPENCLRR_SPEC>`"]
pub type RCC_MP_APB4LPENCLRR = crate::Reg<rcc_mp_apb4lpenclrr::RCC_MP_APB4LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU"]
pub mod rcc_mp_apb4lpenclrr;
#[doc = "RCC_MP_APB5LPENSETR register accessor: an alias for `Reg<RCC_MP_APB5LPENSETR_SPEC>`"]
pub type RCC_MP_APB5LPENSETR = crate::Reg<rcc_mp_apb5lpensetr::RCC_MP_APB5LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_apb5lpensetr;
#[doc = "RCC_MP_APB5LPENCLRR register accessor: an alias for `Reg<RCC_MP_APB5LPENCLRR_SPEC>`"]
pub type RCC_MP_APB5LPENCLRR = crate::Reg<rcc_mp_apb5lpenclrr::RCC_MP_APB5LPENCLRR_SPEC>;
#[doc = "This register is used by the Mpu."]
pub mod rcc_mp_apb5lpenclrr;
#[doc = "RCC_MP_AHB5LPENSETR register accessor: an alias for `Reg<RCC_MP_AHB5LPENSETR_SPEC>`"]
pub type RCC_MP_AHB5LPENSETR = crate::Reg<rcc_mp_ahb5lpensetr::RCC_MP_AHB5LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5lpensetr;
#[doc = "RCC_MP_AHB5LPENCLRR register accessor: an alias for `Reg<RCC_MP_AHB5LPENCLRR_SPEC>`"]
pub type RCC_MP_AHB5LPENCLRR = crate::Reg<rcc_mp_ahb5lpenclrr::RCC_MP_AHB5LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU"]
pub mod rcc_mp_ahb5lpenclrr;
#[doc = "RCC_MP_AHB6LPENSETR register accessor: an alias for `Reg<RCC_MP_AHB6LPENSETR_SPEC>`"]
pub type RCC_MP_AHB6LPENSETR = crate::Reg<rcc_mp_ahb6lpensetr::RCC_MP_AHB6LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb6lpensetr;
#[doc = "RCC_MP_AHB6LPENCLRR register accessor: an alias for `Reg<RCC_MP_AHB6LPENCLRR_SPEC>`"]
pub type RCC_MP_AHB6LPENCLRR = crate::Reg<rcc_mp_ahb6lpenclrr::RCC_MP_AHB6LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb6lpenclrr;
#[doc = "RCC_MP_TZAHB6LPENSETR register accessor: an alias for `Reg<RCC_MP_TZAHB6LPENSETR_SPEC>`"]
pub type RCC_MP_TZAHB6LPENSETR = crate::Reg<rcc_mp_tzahb6lpensetr::RCC_MP_TZAHB6LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6lpensetr;
#[doc = "RCC_MP_TZAHB6LPENCLRR register accessor: an alias for `Reg<RCC_MP_TZAHB6LPENCLRR_SPEC>`"]
pub type RCC_MP_TZAHB6LPENCLRR = crate::Reg<rcc_mp_tzahb6lpenclrr::RCC_MP_TZAHB6LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6lpenclrr;
#[doc = "RCC_MC_APB4LPENSETR register accessor: an alias for `Reg<RCC_MC_APB4LPENSETR_SPEC>`"]
pub type RCC_MC_APB4LPENSETR = crate::Reg<rcc_mc_apb4lpensetr::RCC_MC_APB4LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb4lpensetr;
#[doc = "RCC_MC_APB4LPENCLRR register accessor: an alias for `Reg<RCC_MC_APB4LPENCLRR_SPEC>`"]
pub type RCC_MC_APB4LPENCLRR = crate::Reg<rcc_mc_apb4lpenclrr::RCC_MC_APB4LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb4lpenclrr;
#[doc = "RCC_MC_APB5LPENSETR register accessor: an alias for `Reg<RCC_MC_APB5LPENSETR_SPEC>`"]
pub type RCC_MC_APB5LPENSETR = crate::Reg<rcc_mc_apb5lpensetr::RCC_MC_APB5LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb5lpensetr;
#[doc = "RCC_MC_APB5LPENCLRR register accessor: an alias for `Reg<RCC_MC_APB5LPENCLRR_SPEC>`"]
pub type RCC_MC_APB5LPENCLRR = crate::Reg<rcc_mc_apb5lpenclrr::RCC_MC_APB5LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb5lpenclrr;
#[doc = "RCC_MC_AHB5LPENSETR register accessor: an alias for `Reg<RCC_MC_AHB5LPENSETR_SPEC>`"]
pub type RCC_MC_AHB5LPENSETR = crate::Reg<rcc_mc_ahb5lpensetr::RCC_MC_AHB5LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5lpensetr;
#[doc = "RCC_MC_AHB5LPENCLRR register accessor: an alias for `Reg<RCC_MC_AHB5LPENCLRR_SPEC>`"]
pub type RCC_MC_AHB5LPENCLRR = crate::Reg<rcc_mc_ahb5lpenclrr::RCC_MC_AHB5LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5lpenclrr;
#[doc = "RCC_MC_AHB6LPENSETR register accessor: an alias for `Reg<RCC_MC_AHB6LPENSETR_SPEC>`"]
pub type RCC_MC_AHB6LPENSETR = crate::Reg<rcc_mc_ahb6lpensetr::RCC_MC_AHB6LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb6lpensetr;
#[doc = "RCC_MC_AHB6LPENCLRR register accessor: an alias for `Reg<RCC_MC_AHB6LPENCLRR_SPEC>`"]
pub type RCC_MC_AHB6LPENCLRR = crate::Reg<rcc_mc_ahb6lpenclrr::RCC_MC_AHB6LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb6lpenclrr;
#[doc = "RCC_BR_RSTSCLRR register accessor: an alias for `Reg<RCC_BR_RSTSCLRR_SPEC>`"]
pub type RCC_BR_RSTSCLRR = crate::Reg<rcc_br_rstsclrr::RCC_BR_RSTSCLRR_SPEC>;
#[doc = "This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_br_rstsclrr;
#[doc = "RCC_MP_GRSTCSETR register accessor: an alias for `Reg<RCC_MP_GRSTCSETR_SPEC>`"]
pub type RCC_MP_GRSTCSETR = crate::Reg<rcc_mp_grstcsetr::RCC_MP_GRSTCSETR_SPEC>;
#[doc = "This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset."]
pub mod rcc_mp_grstcsetr;
#[doc = "RCC_MP_RSTSCLRR register accessor: an alias for `Reg<RCC_MP_RSTSCLRR_SPEC>`"]
pub type RCC_MP_RSTSCLRR = crate::Reg<rcc_mp_rstsclrr::RCC_MP_RSTSCLRR_SPEC>;
#[doc = "This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_rstsclrr;
#[doc = "RCC_MP_IWDGFZSETR register accessor: an alias for `Reg<RCC_MP_IWDGFZSETR_SPEC>`"]
pub type RCC_MP_IWDGFZSETR = crate::Reg<rcc_mp_iwdgfzsetr::RCC_MP_IWDGFZSETR_SPEC>;
#[doc = "This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_iwdgfzsetr;
#[doc = "RCC_MP_IWDGFZCLRR register accessor: an alias for `Reg<RCC_MP_IWDGFZCLRR_SPEC>`"]
pub type RCC_MP_IWDGFZCLRR = crate::Reg<rcc_mp_iwdgfzclrr::RCC_MP_IWDGFZCLRR_SPEC>;
#[doc = "This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_iwdgfzclrr;
#[doc = "RCC_MP_CIER register accessor: an alias for `Reg<RCC_MP_CIER_SPEC>`"]
pub type RCC_MP_CIER = crate::Reg<rcc_mp_cier::RCC_MP_CIER_SPEC>;
#[doc = "This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_cier;
#[doc = "RCC_MP_CIFR register accessor: an alias for `Reg<RCC_MP_CIFR_SPEC>`"]
pub type RCC_MP_CIFR = crate::Reg<rcc_mp_cifr::RCC_MP_CIFR_SPEC>;
#[doc = "This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_cifr;
#[doc = "RCC_PWRLPDLYCR register accessor: an alias for `Reg<RCC_PWRLPDLYCR_SPEC>`"]
pub type RCC_PWRLPDLYCR = crate::Reg<rcc_pwrlpdlycr::RCC_PWRLPDLYCR_SPEC>;
#[doc = "This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_pwrlpdlycr;
#[doc = "RCC_MP_RSTSSETR register accessor: an alias for `Reg<RCC_MP_RSTSSETR_SPEC>`"]
pub type RCC_MP_RSTSSETR = crate::Reg<rcc_mp_rstssetr::RCC_MP_RSTSSETR_SPEC>;
#[doc = "This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_rstssetr;
#[doc = "RCC_MCO1CFGR register accessor: an alias for `Reg<RCC_MCO1CFGR_SPEC>`"]
pub type RCC_MCO1CFGR = crate::Reg<rcc_mco1cfgr::RCC_MCO1CFGR_SPEC>;
#[doc = "This register is used to select the clock generated on MCO1 output."]
pub mod rcc_mco1cfgr;
#[doc = "RCC_MCO2CFGR register accessor: an alias for `Reg<RCC_MCO2CFGR_SPEC>`"]
pub type RCC_MCO2CFGR = crate::Reg<rcc_mco2cfgr::RCC_MCO2CFGR_SPEC>;
#[doc = "This register is used to select the clock generated on MCO2 output."]
pub mod rcc_mco2cfgr;
#[doc = "RCC_OCRDYR register accessor: an alias for `Reg<RCC_OCRDYR_SPEC>`"]
pub type RCC_OCRDYR = crate::Reg<rcc_ocrdyr::RCC_OCRDYR_SPEC>;
#[doc = "This is a read-only access register, It contains the status flags of oscillators. Writing has no effect."]
pub mod rcc_ocrdyr;
#[doc = "RCC_DBGCFGR register accessor: an alias for `Reg<RCC_DBGCFGR_SPEC>`"]
pub type RCC_DBGCFGR = crate::Reg<rcc_dbgcfgr::RCC_DBGCFGR_SPEC>;
#[doc = "This is register contains the enable control of the debug and trace function, and the clock divider for the trace function."]
pub mod rcc_dbgcfgr;
#[doc = "RCC_RCK3SELR register accessor: an alias for `Reg<RCC_RCK3SELR_SPEC>`"]
pub type RCC_RCK3SELR = crate::Reg<rcc_rck3selr::RCC_RCK3SELR_SPEC>;
#[doc = "This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_rck3selr;
#[doc = "RCC_RCK4SELR register accessor: an alias for `Reg<RCC_RCK4SELR_SPEC>`"]
pub type RCC_RCK4SELR = crate::Reg<rcc_rck4selr::RCC_RCK4SELR_SPEC>;
#[doc = "This register is used to select the reference clock for PLL4."]
pub mod rcc_rck4selr;
#[doc = "RCC_TIMG1PRER register accessor: an alias for `Reg<RCC_TIMG1PRER_SPEC>`"]
pub type RCC_TIMG1PRER = crate::Reg<rcc_timg1prer::RCC_TIMG1PRER_SPEC>;
#[doc = "This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_timg1prer;
#[doc = "RCC_TIMG2PRER register accessor: an alias for `Reg<RCC_TIMG2PRER_SPEC>`"]
pub type RCC_TIMG2PRER = crate::Reg<rcc_timg2prer::RCC_TIMG2PRER_SPEC>;
#[doc = "This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_timg2prer;
#[doc = "RCC_MCUDIVR register accessor: an alias for `Reg<RCC_MCUDIVR_SPEC>`"]
pub type RCC_MCUDIVR = crate::Reg<rcc_mcudivr::RCC_MCUDIVR_SPEC>;
#[doc = "This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mcudivr;
#[doc = "RCC_APB1DIVR register accessor: an alias for `Reg<RCC_APB1DIVR_SPEC>`"]
pub type RCC_APB1DIVR = crate::Reg<rcc_apb1divr::RCC_APB1DIVR_SPEC>;
#[doc = "This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information."]
pub mod rcc_apb1divr;
#[doc = "RCC_APB2DIVR register accessor: an alias for `Reg<RCC_APB2DIVR_SPEC>`"]
pub type RCC_APB2DIVR = crate::Reg<rcc_apb2divr::RCC_APB2DIVR_SPEC>;
#[doc = "This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_apb2divr;
#[doc = "RCC_APB3DIVR register accessor: an alias for `Reg<RCC_APB3DIVR_SPEC>`"]
pub type RCC_APB3DIVR = crate::Reg<rcc_apb3divr::RCC_APB3DIVR_SPEC>;
#[doc = "This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_apb3divr;
#[doc = "RCC_PLL3CR register accessor: an alias for `Reg<RCC_PLL3CR_SPEC>`"]
pub type RCC_PLL3CR = crate::Reg<rcc_pll3cr::RCC_PLL3CR_SPEC>;
#[doc = "This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cr;
#[doc = "RCC_PLL3CFGR1 register accessor: an alias for `Reg<RCC_PLL3CFGR1_SPEC>`"]
pub type RCC_PLL3CFGR1 = crate::Reg<rcc_pll3cfgr1::RCC_PLL3CFGR1_SPEC>;
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cfgr1;
#[doc = "RCC_PLL3CFGR2 register accessor: an alias for `Reg<RCC_PLL3CFGR2_SPEC>`"]
pub type RCC_PLL3CFGR2 = crate::Reg<rcc_pll3cfgr2::RCC_PLL3CFGR2_SPEC>;
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cfgr2;
#[doc = "RCC_PLL3FRACR register accessor: an alias for `Reg<RCC_PLL3FRACR_SPEC>`"]
pub type RCC_PLL3FRACR = crate::Reg<rcc_pll3fracr::RCC_PLL3FRACR_SPEC>;
#[doc = "This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3fracr;
#[doc = "RCC_PLL3CSGR register accessor: an alias for `Reg<RCC_PLL3CSGR_SPEC>`"]
pub type RCC_PLL3CSGR = crate::Reg<rcc_pll3csgr::RCC_PLL3CSGR_SPEC>;
#[doc = "This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3csgr;
#[doc = "RCC_PLL4CR register accessor: an alias for `Reg<RCC_PLL4CR_SPEC>`"]
pub type RCC_PLL4CR = crate::Reg<rcc_pll4cr::RCC_PLL4CR_SPEC>;
#[doc = "This register is used to control the PLL4."]
pub mod rcc_pll4cr;
#[doc = "RCC_PLL4CFGR1 register accessor: an alias for `Reg<RCC_PLL4CFGR1_SPEC>`"]
pub type RCC_PLL4CFGR1 = crate::Reg<rcc_pll4cfgr1::RCC_PLL4CFGR1_SPEC>;
#[doc = "This register is used to configure the PLL4."]
pub mod rcc_pll4cfgr1;
#[doc = "RCC_PLL4CFGR2 register accessor: an alias for `Reg<RCC_PLL4CFGR2_SPEC>`"]
pub type RCC_PLL4CFGR2 = crate::Reg<rcc_pll4cfgr2::RCC_PLL4CFGR2_SPEC>;
#[doc = "This register is used to configure the PLL4."]
pub mod rcc_pll4cfgr2;
#[doc = "RCC_PLL4FRACR register accessor: an alias for `Reg<RCC_PLL4FRACR_SPEC>`"]
pub type RCC_PLL4FRACR = crate::Reg<rcc_pll4fracr::RCC_PLL4FRACR_SPEC>;
#[doc = "This register is used to fine-tune the frequency of the PLL4 VCO."]
pub mod rcc_pll4fracr;
#[doc = "RCC_PLL4CSGR register accessor: an alias for `Reg<RCC_PLL4CSGR_SPEC>`"]
pub type RCC_PLL4CSGR = crate::Reg<rcc_pll4csgr::RCC_PLL4CSGR_SPEC>;
#[doc = "This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll4csgr;
#[doc = "RCC_I2C12CKSELR register accessor: an alias for `Reg<RCC_I2C12CKSELR_SPEC>`"]
pub type RCC_I2C12CKSELR = crate::Reg<rcc_i2c12ckselr::RCC_I2C12CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_i2c12ckselr;
#[doc = "RCC_I2C35CKSELR register accessor: an alias for `Reg<RCC_I2C35CKSELR_SPEC>`"]
pub type RCC_I2C35CKSELR = crate::Reg<rcc_i2c35ckselr::RCC_I2C35CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_i2c35ckselr;
#[doc = "RCC_SAI1CKSELR register accessor: an alias for `Reg<RCC_SAI1CKSELR_SPEC>`"]
pub type RCC_SAI1CKSELR = crate::Reg<rcc_sai1ckselr::RCC_SAI1CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai1ckselr;
#[doc = "RCC_SAI2CKSELR register accessor: an alias for `Reg<RCC_SAI2CKSELR_SPEC>`"]
pub type RCC_SAI2CKSELR = crate::Reg<rcc_sai2ckselr::RCC_SAI2CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai2ckselr;
#[doc = "RCC_SAI3CKSELR register accessor: an alias for `Reg<RCC_SAI3CKSELR_SPEC>`"]
pub type RCC_SAI3CKSELR = crate::Reg<rcc_sai3ckselr::RCC_SAI3CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai3ckselr;
#[doc = "RCC_SAI4CKSELR register accessor: an alias for `Reg<RCC_SAI4CKSELR_SPEC>`"]
pub type RCC_SAI4CKSELR = crate::Reg<rcc_sai4ckselr::RCC_SAI4CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai4ckselr;
#[doc = "RCC_SPI2S1CKSELR register accessor: an alias for `Reg<RCC_SPI2S1CKSELR_SPEC>`"]
pub type RCC_SPI2S1CKSELR = crate::Reg<rcc_spi2s1ckselr::RCC_SPI2S1CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi2s1ckselr;
#[doc = "RCC_SPI2S23CKSELR register accessor: an alias for `Reg<RCC_SPI2S23CKSELR_SPEC>`"]
pub type RCC_SPI2S23CKSELR = crate::Reg<rcc_spi2s23ckselr::RCC_SPI2S23CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi2s23ckselr;
#[doc = "RCC_SPI45CKSELR register accessor: an alias for `Reg<RCC_SPI45CKSELR_SPEC>`"]
pub type RCC_SPI45CKSELR = crate::Reg<rcc_spi45ckselr::RCC_SPI45CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi45ckselr;
#[doc = "RCC_UART6CKSELR register accessor: an alias for `Reg<RCC_UART6CKSELR_SPEC>`"]
pub type RCC_UART6CKSELR = crate::Reg<rcc_uart6ckselr::RCC_UART6CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart6ckselr;
#[doc = "RCC_UART24CKSELR register accessor: an alias for `Reg<RCC_UART24CKSELR_SPEC>`"]
pub type RCC_UART24CKSELR = crate::Reg<rcc_uart24ckselr::RCC_UART24CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart24ckselr;
#[doc = "RCC_UART35CKSELR register accessor: an alias for `Reg<RCC_UART35CKSELR_SPEC>`"]
pub type RCC_UART35CKSELR = crate::Reg<rcc_uart35ckselr::RCC_UART35CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart35ckselr;
#[doc = "RCC_UART78CKSELR register accessor: an alias for `Reg<RCC_UART78CKSELR_SPEC>`"]
pub type RCC_UART78CKSELR = crate::Reg<rcc_uart78ckselr::RCC_UART78CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart78ckselr;
#[doc = "RCC_SDMMC12CKSELR register accessor: an alias for `Reg<RCC_SDMMC12CKSELR_SPEC>`"]
pub type RCC_SDMMC12CKSELR = crate::Reg<rcc_sdmmc12ckselr::RCC_SDMMC12CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sdmmc12ckselr;
#[doc = "RCC_SDMMC3CKSELR register accessor: an alias for `Reg<RCC_SDMMC3CKSELR_SPEC>`"]
pub type RCC_SDMMC3CKSELR = crate::Reg<rcc_sdmmc3ckselr::RCC_SDMMC3CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sdmmc3ckselr;
#[doc = "RCC_ETHCKSELR register accessor: an alias for `Reg<RCC_ETHCKSELR_SPEC>`"]
pub type RCC_ETHCKSELR = crate::Reg<rcc_ethckselr::RCC_ETHCKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_ethckselr;
#[doc = "RCC_QSPICKSELR register accessor: an alias for `Reg<RCC_QSPICKSELR_SPEC>`"]
pub type RCC_QSPICKSELR = crate::Reg<rcc_qspickselr::RCC_QSPICKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_qspickselr;
#[doc = "RCC_FMCCKSELR register accessor: an alias for `Reg<RCC_FMCCKSELR_SPEC>`"]
pub type RCC_FMCCKSELR = crate::Reg<rcc_fmcckselr::RCC_FMCCKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_fmcckselr;
#[doc = "RCC_FDCANCKSELR register accessor: an alias for `Reg<RCC_FDCANCKSELR_SPEC>`"]
pub type RCC_FDCANCKSELR = crate::Reg<rcc_fdcanckselr::RCC_FDCANCKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_fdcanckselr;
#[doc = "RCC_SPDIFCKSELR register accessor: an alias for `Reg<RCC_SPDIFCKSELR_SPEC>`"]
pub type RCC_SPDIFCKSELR = crate::Reg<rcc_spdifckselr::RCC_SPDIFCKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spdifckselr;
#[doc = "RCC_CECCKSELR register accessor: an alias for `Reg<RCC_CECCKSELR_SPEC>`"]
pub type RCC_CECCKSELR = crate::Reg<rcc_cecckselr::RCC_CECCKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the CEC-HDMI."]
pub mod rcc_cecckselr;
#[doc = "RCC_USBCKSELR register accessor: an alias for `Reg<RCC_USBCKSELR_SPEC>`"]
pub type RCC_USBCKSELR = crate::Reg<rcc_usbckselr::RCC_USBCKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG"]
pub mod rcc_usbckselr;
#[doc = "RCC_RNG2CKSELR register accessor: an alias for `Reg<RCC_RNG2CKSELR_SPEC>`"]
pub type RCC_RNG2CKSELR = crate::Reg<rcc_rng2ckselr::RCC_RNG2CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the RNG2."]
pub mod rcc_rng2ckselr;
#[doc = "RCC_DSICKSELR register accessor: an alias for `Reg<RCC_DSICKSELR_SPEC>`"]
pub type RCC_DSICKSELR = crate::Reg<rcc_dsickselr::RCC_DSICKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the DSI block."]
pub mod rcc_dsickselr;
#[doc = "RCC_ADCCKSELR register accessor: an alias for `Reg<RCC_ADCCKSELR_SPEC>`"]
pub type RCC_ADCCKSELR = crate::Reg<rcc_adcckselr::RCC_ADCCKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the ADC block."]
pub mod rcc_adcckselr;
#[doc = "RCC_LPTIM45CKSELR register accessor: an alias for `Reg<RCC_LPTIM45CKSELR_SPEC>`"]
pub type RCC_LPTIM45CKSELR = crate::Reg<rcc_lptim45ckselr::RCC_LPTIM45CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks."]
pub mod rcc_lptim45ckselr;
#[doc = "RCC_LPTIM23CKSELR register accessor: an alias for `Reg<RCC_LPTIM23CKSELR_SPEC>`"]
pub type RCC_LPTIM23CKSELR = crate::Reg<rcc_lptim23ckselr::RCC_LPTIM23CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks."]
pub mod rcc_lptim23ckselr;
#[doc = "RCC_LPTIM1CKSELR register accessor: an alias for `Reg<RCC_LPTIM1CKSELR_SPEC>`"]
pub type RCC_LPTIM1CKSELR = crate::Reg<rcc_lptim1ckselr::RCC_LPTIM1CKSELR_SPEC>;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM1 block."]
pub mod rcc_lptim1ckselr;
#[doc = "RCC_APB1RSTSETR register accessor: an alias for `Reg<RCC_APB1RSTSETR_SPEC>`"]
pub type RCC_APB1RSTSETR = crate::Reg<rcc_apb1rstsetr::RCC_APB1RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb1rstsetr;
#[doc = "RCC_APB1RSTCLRR register accessor: an alias for `Reg<RCC_APB1RSTCLRR_SPEC>`"]
pub type RCC_APB1RSTCLRR = crate::Reg<rcc_apb1rstclrr::RCC_APB1RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb1rstclrr;
#[doc = "RCC_APB2RSTSETR register accessor: an alias for `Reg<RCC_APB2RSTSETR_SPEC>`"]
pub type RCC_APB2RSTSETR = crate::Reg<rcc_apb2rstsetr::RCC_APB2RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb2rstsetr;
#[doc = "RCC_APB2RSTCLRR register accessor: an alias for `Reg<RCC_APB2RSTCLRR_SPEC>`"]
pub type RCC_APB2RSTCLRR = crate::Reg<rcc_apb2rstclrr::RCC_APB2RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb2rstclrr;
#[doc = "RCC_APB3RSTSETR register accessor: an alias for `Reg<RCC_APB3RSTSETR_SPEC>`"]
pub type RCC_APB3RSTSETR = crate::Reg<rcc_apb3rstsetr::RCC_APB3RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb3rstsetr;
#[doc = "RCC_APB3RSTCLRR register accessor: an alias for `Reg<RCC_APB3RSTCLRR_SPEC>`"]
pub type RCC_APB3RSTCLRR = crate::Reg<rcc_apb3rstclrr::RCC_APB3RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb3rstclrr;
#[doc = "RCC_AHB2RSTSETR register accessor: an alias for `Reg<RCC_AHB2RSTSETR_SPEC>`"]
pub type RCC_AHB2RSTSETR = crate::Reg<rcc_ahb2rstsetr::RCC_AHB2RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_ahb2rstsetr;
#[doc = "RCC_AHB2RSTCLRR register accessor: an alias for `Reg<RCC_AHB2RSTCLRR_SPEC>`"]
pub type RCC_AHB2RSTCLRR = crate::Reg<rcc_ahb2rstclrr::RCC_AHB2RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb2rstclrr;
#[doc = "RCC_AHB3RSTSETR register accessor: an alias for `Reg<RCC_AHB3RSTSETR_SPEC>`"]
pub type RCC_AHB3RSTSETR = crate::Reg<rcc_ahb3rstsetr::RCC_AHB3RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_ahb3rstsetr;
#[doc = "RCC_AHB3RSTCLRR register accessor: an alias for `Reg<RCC_AHB3RSTCLRR_SPEC>`"]
pub type RCC_AHB3RSTCLRR = crate::Reg<rcc_ahb3rstclrr::RCC_AHB3RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb3rstclrr;
#[doc = "RCC_AHB4RSTSETR register accessor: an alias for `Reg<RCC_AHB4RSTSETR_SPEC>`"]
pub type RCC_AHB4RSTSETR = crate::Reg<rcc_ahb4rstsetr::RCC_AHB4RSTSETR_SPEC>;
#[doc = "This register is used to activate the reset of the corresponding peripheral"]
pub mod rcc_ahb4rstsetr;
#[doc = "RCC_AHB4RSTCLRR register accessor: an alias for `Reg<RCC_AHB4RSTCLRR_SPEC>`"]
pub type RCC_AHB4RSTCLRR = crate::Reg<rcc_ahb4rstclrr::RCC_AHB4RSTCLRR_SPEC>;
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb4rstclrr;
#[doc = "RCC_MP_APB1ENSETR register accessor: an alias for `Reg<RCC_MP_APB1ENSETR_SPEC>`"]
pub type RCC_MP_APB1ENSETR = crate::Reg<rcc_mp_apb1ensetr::RCC_MP_APB1ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb1ensetr;
#[doc = "RCC_MP_APB1ENCLRR register accessor: an alias for `Reg<RCC_MP_APB1ENCLRR_SPEC>`"]
pub type RCC_MP_APB1ENCLRR = crate::Reg<rcc_mp_apb1enclrr::RCC_MP_APB1ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mp_apb1enclrr;
#[doc = "RCC_MP_APB2ENSETR register accessor: an alias for `Reg<RCC_MP_APB2ENSETR_SPEC>`"]
pub type RCC_MP_APB2ENSETR = crate::Reg<rcc_mp_apb2ensetr::RCC_MP_APB2ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb2ensetr;
#[doc = "RCC_MP_APB2ENCLRR register accessor: an alias for `Reg<RCC_MP_APB2ENCLRR_SPEC>`"]
pub type RCC_MP_APB2ENCLRR = crate::Reg<rcc_mp_apb2enclrr::RCC_MP_APB2ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_apb2enclrr;
#[doc = "RCC_MP_APB3ENSETR register accessor: an alias for `Reg<RCC_MP_APB3ENSETR_SPEC>`"]
pub type RCC_MP_APB3ENSETR = crate::Reg<rcc_mp_apb3ensetr::RCC_MP_APB3ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb3ensetr;
#[doc = "RCC_MP_APB3ENCLRR register accessor: an alias for `Reg<RCC_MP_APB3ENCLRR_SPEC>`"]
pub type RCC_MP_APB3ENCLRR = crate::Reg<rcc_mp_apb3enclrr::RCC_MP_APB3ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_apb3enclrr;
#[doc = "RCC_MP_AHB2ENSETR register accessor: an alias for `Reg<RCC_MP_AHB2ENSETR_SPEC>`"]
pub type RCC_MP_AHB2ENSETR = crate::Reg<rcc_mp_ahb2ensetr::RCC_MP_AHB2ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
pub mod rcc_mp_ahb2ensetr;
#[doc = "RCC_MP_AHB2ENCLRR register accessor: an alias for `Reg<RCC_MP_AHB2ENCLRR_SPEC>`"]
pub type RCC_MP_AHB2ENCLRR = crate::Reg<rcc_mp_ahb2enclrr::RCC_MP_AHB2ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_ahb2enclrr;
#[doc = "RCC_MP_AHB3ENSETR register accessor: an alias for `Reg<RCC_MP_AHB3ENSETR_SPEC>`"]
pub type RCC_MP_AHB3ENSETR = crate::Reg<rcc_mp_ahb3ensetr::RCC_MP_AHB3ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
pub mod rcc_mp_ahb3ensetr;
#[doc = "RCC_MP_AHB3ENCLRR register accessor: an alias for `Reg<RCC_MP_AHB3ENCLRR_SPEC>`"]
pub type RCC_MP_AHB3ENCLRR = crate::Reg<rcc_mp_ahb3enclrr::RCC_MP_AHB3ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_ahb3enclrr;
#[doc = "RCC_MP_AHB4ENSETR register accessor: an alias for `Reg<RCC_MP_AHB4ENSETR_SPEC>`"]
pub type RCC_MP_AHB4ENSETR = crate::Reg<rcc_mp_ahb4ensetr::RCC_MP_AHB4ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU."]
pub mod rcc_mp_ahb4ensetr;
#[doc = "RCC_MP_AHB4ENCLRR register accessor: an alias for `Reg<RCC_MP_AHB4ENCLRR_SPEC>`"]
pub type RCC_MP_AHB4ENCLRR = crate::Reg<rcc_mp_ahb4enclrr::RCC_MP_AHB4ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mp_ahb4enclrr;
#[doc = "RCC_MP_MLAHBENSETR register accessor: an alias for `Reg<RCC_MP_MLAHBENSETR_SPEC>`"]
pub type RCC_MP_MLAHBENSETR = crate::Reg<rcc_mp_mlahbensetr::RCC_MP_MLAHBENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_mlahbensetr;
#[doc = "RCC_MP_MLAHBENCLRR register accessor: an alias for `Reg<RCC_MP_MLAHBENCLRR_SPEC>`"]
pub type RCC_MP_MLAHBENCLRR = crate::Reg<rcc_mp_mlahbenclrr::RCC_MP_MLAHBENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit."]
pub mod rcc_mp_mlahbenclrr;
#[doc = "RCC_MC_APB1ENSETR register accessor: an alias for `Reg<RCC_MC_APB1ENSETR_SPEC>`"]
pub type RCC_MC_APB1ENSETR = crate::Reg<rcc_mc_apb1ensetr::RCC_MC_APB1ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to ."]
pub mod rcc_mc_apb1ensetr;
#[doc = "RCC_MC_APB1ENCLRR register accessor: an alias for `Reg<RCC_MC_APB1ENCLRR_SPEC>`"]
pub type RCC_MC_APB1ENCLRR = crate::Reg<rcc_mc_apb1enclrr::RCC_MC_APB1ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mc_apb1enclrr;
#[doc = "RCC_MC_APB2ENSETR register accessor: an alias for `Reg<RCC_MC_APB2ENSETR_SPEC>`"]
pub type RCC_MC_APB2ENSETR = crate::Reg<rcc_mc_apb2ensetr::RCC_MC_APB2ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb2ensetr;
#[doc = "RCC_MC_APB2ENCLRR register accessor: an alias for `Reg<RCC_MC_APB2ENCLRR_SPEC>`"]
pub type RCC_MC_APB2ENCLRR = crate::Reg<rcc_mc_apb2enclrr::RCC_MC_APB2ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb2enclrr;
#[doc = "RCC_MC_APB3ENSETR register accessor: an alias for `Reg<RCC_MC_APB3ENSETR_SPEC>`"]
pub type RCC_MC_APB3ENSETR = crate::Reg<rcc_mc_apb3ensetr::RCC_MC_APB3ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb3ensetr;
#[doc = "RCC_MC_APB3ENCLRR register accessor: an alias for `Reg<RCC_MC_APB3ENCLRR_SPEC>`"]
pub type RCC_MC_APB3ENCLRR = crate::Reg<rcc_mc_apb3enclrr::RCC_MC_APB3ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb3enclrr;
#[doc = "RCC_MC_AHB2ENSETR register accessor: an alias for `Reg<RCC_MC_AHB2ENSETR_SPEC>`"]
pub type RCC_MC_AHB2ENSETR = crate::Reg<rcc_mc_ahb2ensetr::RCC_MC_AHB2ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb2ensetr;
#[doc = "RCC_MC_AHB2ENCLRR register accessor: an alias for `Reg<RCC_MC_AHB2ENCLRR_SPEC>`"]
pub type RCC_MC_AHB2ENCLRR = crate::Reg<rcc_mc_ahb2enclrr::RCC_MC_AHB2ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb2enclrr;
#[doc = "RCC_MC_AHB3ENSETR register accessor: an alias for `Reg<RCC_MC_AHB3ENSETR_SPEC>`"]
pub type RCC_MC_AHB3ENSETR = crate::Reg<rcc_mc_ahb3ensetr::RCC_MC_AHB3ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb3ensetr;
#[doc = "RCC_MC_AHB3ENCLRR register accessor: an alias for `Reg<RCC_MC_AHB3ENCLRR_SPEC>`"]
pub type RCC_MC_AHB3ENCLRR = crate::Reg<rcc_mc_ahb3enclrr::RCC_MC_AHB3ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb3enclrr;
#[doc = "RCC_MC_AHB4ENSETR register accessor: an alias for `Reg<RCC_MC_AHB4ENSETR_SPEC>`"]
pub type RCC_MC_AHB4ENSETR = crate::Reg<rcc_mc_ahb4ensetr::RCC_MC_AHB4ENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb4ensetr;
#[doc = "RCC_MC_AHB4ENCLRR register accessor: an alias for `Reg<RCC_MC_AHB4ENCLRR_SPEC>`"]
pub type RCC_MC_AHB4ENCLRR = crate::Reg<rcc_mc_ahb4enclrr::RCC_MC_AHB4ENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb4enclrr;
#[doc = "RCC_MC_AXIMENSETR register accessor: an alias for `Reg<RCC_MC_AXIMENSETR_SPEC>`"]
pub type RCC_MC_AXIMENSETR = crate::Reg<rcc_mc_aximensetr::RCC_MC_AXIMENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_aximensetr;
#[doc = "RCC_MC_AXIMENCLRR register accessor: an alias for `Reg<RCC_MC_AXIMENCLRR_SPEC>`"]
pub type RCC_MC_AXIMENCLRR = crate::Reg<rcc_mc_aximenclrr::RCC_MC_AXIMENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_aximenclrr;
#[doc = "RCC_MC_MLAHBENSETR register accessor: an alias for `Reg<RCC_MC_MLAHBENSETR_SPEC>`"]
pub type RCC_MC_MLAHBENSETR = crate::Reg<rcc_mc_mlahbensetr::RCC_MC_MLAHBENSETR_SPEC>;
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_mlahbensetr;
#[doc = "RCC_MC_MLAHBENCLRR register accessor: an alias for `Reg<RCC_MC_MLAHBENCLRR_SPEC>`"]
pub type RCC_MC_MLAHBENCLRR = crate::Reg<rcc_mc_mlahbenclrr::RCC_MC_MLAHBENCLRR_SPEC>;
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_mlahbenclrr;
#[doc = "RCC_MP_APB1LPENSETR register accessor: an alias for `Reg<RCC_MP_APB1LPENSETR_SPEC>`"]
pub type RCC_MP_APB1LPENSETR = crate::Reg<rcc_mp_apb1lpensetr::RCC_MP_APB1LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb1lpensetr;
#[doc = "RCC_MP_APB1LPENCLRR register accessor: an alias for `Reg<RCC_MP_APB1LPENCLRR_SPEC>`"]
pub type RCC_MP_APB1LPENCLRR = crate::Reg<rcc_mp_apb1lpenclrr::RCC_MP_APB1LPENCLRR_SPEC>;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bits ."]
pub mod rcc_mp_apb1lpenclrr;
#[doc = "RCC_MP_APB2LPENSETR register accessor: an alias for `Reg<RCC_MP_APB2LPENSETR_SPEC>`"]
pub type RCC_MP_APB2LPENSETR = crate::Reg<rcc_mp_apb2lpensetr::RCC_MP_APB2LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb2lpensetr;
#[doc = "RCC_MP_APB2LPENCLRR register accessor: an alias for `Reg<RCC_MP_APB2LPENCLRR_SPEC>`"]
pub type RCC_MP_APB2LPENCLRR = crate::Reg<rcc_mp_apb2lpenclrr::RCC_MP_APB2LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb2lpenclrr;
#[doc = "RCC_MP_APB3LPENSETR register accessor: an alias for `Reg<RCC_MP_APB3LPENSETR_SPEC>`"]
pub type RCC_MP_APB3LPENSETR = crate::Reg<rcc_mp_apb3lpensetr::RCC_MP_APB3LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb3lpensetr;
#[doc = "RCC_MP_APB3LPENCLRR register accessor: an alias for `Reg<RCC_MP_APB3LPENCLRR_SPEC>`"]
pub type RCC_MP_APB3LPENCLRR = crate::Reg<rcc_mp_apb3lpenclrr::RCC_MP_APB3LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb3lpenclrr;
#[doc = "RCC_MP_AHB2LPENSETR register accessor: an alias for `Reg<RCC_MP_AHB2LPENSETR_SPEC>`"]
pub type RCC_MP_AHB2LPENSETR = crate::Reg<rcc_mp_ahb2lpensetr::RCC_MP_AHB2LPENSETR_SPEC>;
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit."]
pub mod rcc_mp_ahb2lpensetr;
#[doc = "RCC_MP_AHB2LPENCLRR register accessor: an alias for `Reg<RCC_MP_AHB2LPENCLRR_SPEC>`"]
pub type RCC_MP_AHB2LPENCLRR = crate::Reg<rcc_mp_ahb2lpenclrr::RCC_MP_AHB2LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb2lpenclrr;
#[doc = "RCC_MP_AHB3LPENSETR register accessor: an alias for `Reg<RCC_MP_AHB3LPENSETR_SPEC>`"]
pub type RCC_MP_AHB3LPENSETR = crate::Reg<rcc_mp_ahb3lpensetr::RCC_MP_AHB3LPENSETR_SPEC>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb3lpensetr;
#[doc = "RCC_MP_AHB3LPENCLRR register accessor: an alias for `Reg<RCC_MP_AHB3LPENCLRR_SPEC>`"]
pub type RCC_MP_AHB3LPENCLRR = crate::Reg<rcc_mp_ahb3lpenclrr::RCC_MP_AHB3LPENCLRR_SPEC>;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit"]
pub mod rcc_mp_ahb3lpenclrr;
#[doc = "RCC_MP_AHB4LPENSETR register accessor: an alias for `Reg<RCC_MP_AHB4LPENSETR_SPEC>`"]
pub type RCC_MP_AHB4LPENSETR = crate::Reg<rcc_mp_ahb4lpensetr::RCC_MP_AHB4LPENSETR_SPEC>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb4lpensetr;
#[doc = "RCC_MP_AHB4LPENCLRR register accessor: an alias for `Reg<RCC_MP_AHB4LPENCLRR_SPEC>`"]
pub type RCC_MP_AHB4LPENCLRR = crate::Reg<rcc_mp_ahb4lpenclrr::RCC_MP_AHB4LPENCLRR_SPEC>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb4lpenclrr;
#[doc = "RCC_MP_AXIMLPENSETR register accessor: an alias for `Reg<RCC_MP_AXIMLPENSETR_SPEC>`"]
pub type RCC_MP_AXIMLPENSETR = crate::Reg<rcc_mp_aximlpensetr::RCC_MP_AXIMLPENSETR_SPEC>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_aximlpensetr;
#[doc = "RCC_MP_AXIMLPENCLRR register accessor: an alias for `Reg<RCC_MP_AXIMLPENCLRR_SPEC>`"]
pub type RCC_MP_AXIMLPENCLRR = crate::Reg<rcc_mp_aximlpenclrr::RCC_MP_AXIMLPENCLRR_SPEC>;
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_aximlpenclrr;
#[doc = "RCC_MP_MLAHBLPENSETR register accessor: an alias for `Reg<RCC_MP_MLAHBLPENSETR_SPEC>`"]
pub type RCC_MP_MLAHBLPENSETR = crate::Reg<rcc_mp_mlahblpensetr::RCC_MP_MLAHBLPENSETR_SPEC>;
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit"]
pub mod rcc_mp_mlahblpensetr;
#[doc = "RCC_MP_MLAHBLPENCLRR register accessor: an alias for `Reg<RCC_MP_MLAHBLPENCLRR_SPEC>`"]
pub type RCC_MP_MLAHBLPENCLRR = crate::Reg<rcc_mp_mlahblpenclrr::RCC_MP_MLAHBLPENCLRR_SPEC>;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit"]
pub mod rcc_mp_mlahblpenclrr;
#[doc = "RCC_MC_APB1LPENSETR register accessor: an alias for `Reg<RCC_MC_APB1LPENSETR_SPEC>`"]
pub type RCC_MC_APB1LPENSETR = crate::Reg<rcc_mc_apb1lpensetr::RCC_MC_APB1LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb1lpensetr;
#[doc = "RCC_MC_APB1LPENCLRR register accessor: an alias for `Reg<RCC_MC_APB1LPENCLRR_SPEC>`"]
pub type RCC_MC_APB1LPENCLRR = crate::Reg<rcc_mc_apb1lpenclrr::RCC_MC_APB1LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mc_apb1lpenclrr;
#[doc = "RCC_MC_APB2LPENSETR register accessor: an alias for `Reg<RCC_MC_APB2LPENSETR_SPEC>`"]
pub type RCC_MC_APB2LPENSETR = crate::Reg<rcc_mc_apb2lpensetr::RCC_MC_APB2LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb2lpensetr;
#[doc = "RCC_MC_APB2LPENCLRR register accessor: an alias for `Reg<RCC_MC_APB2LPENCLRR_SPEC>`"]
pub type RCC_MC_APB2LPENCLRR = crate::Reg<rcc_mc_apb2lpenclrr::RCC_MC_APB2LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb2lpenclrr;
#[doc = "RCC_MC_APB3LPENSETR register accessor: an alias for `Reg<RCC_MC_APB3LPENSETR_SPEC>`"]
pub type RCC_MC_APB3LPENSETR = crate::Reg<rcc_mc_apb3lpensetr::RCC_MC_APB3LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb3lpensetr;
#[doc = "RCC_MC_APB3LPENCLRR register accessor: an alias for `Reg<RCC_MC_APB3LPENCLRR_SPEC>`"]
pub type RCC_MC_APB3LPENCLRR = crate::Reg<rcc_mc_apb3lpenclrr::RCC_MC_APB3LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb3lpenclrr;
#[doc = "RCC_MC_AHB2LPENSETR register accessor: an alias for `Reg<RCC_MC_AHB2LPENSETR_SPEC>`"]
pub type RCC_MC_AHB2LPENSETR = crate::Reg<rcc_mc_ahb2lpensetr::RCC_MC_AHB2LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb2lpensetr;
#[doc = "RCC_MC_AHB2LPENCLRR register accessor: an alias for `Reg<RCC_MC_AHB2LPENCLRR_SPEC>`"]
pub type RCC_MC_AHB2LPENCLRR = crate::Reg<rcc_mc_ahb2lpenclrr::RCC_MC_AHB2LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb2lpenclrr;
#[doc = "RCC_MC_AHB3LPENSETR register accessor: an alias for `Reg<RCC_MC_AHB3LPENSETR_SPEC>`"]
pub type RCC_MC_AHB3LPENSETR = crate::Reg<rcc_mc_ahb3lpensetr::RCC_MC_AHB3LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb3lpensetr;
#[doc = "RCC_MC_AHB3LPENCLRR register accessor: an alias for `Reg<RCC_MC_AHB3LPENCLRR_SPEC>`"]
pub type RCC_MC_AHB3LPENCLRR = crate::Reg<rcc_mc_ahb3lpenclrr::RCC_MC_AHB3LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb3lpenclrr;
#[doc = "RCC_MC_AHB4LPENSETR register accessor: an alias for `Reg<RCC_MC_AHB4LPENSETR_SPEC>`"]
pub type RCC_MC_AHB4LPENSETR = crate::Reg<rcc_mc_ahb4lpensetr::RCC_MC_AHB4LPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb4lpensetr;
#[doc = "RCC_MC_AHB4LPENCLRR register accessor: an alias for `Reg<RCC_MC_AHB4LPENCLRR_SPEC>`"]
pub type RCC_MC_AHB4LPENCLRR = crate::Reg<rcc_mc_ahb4lpenclrr::RCC_MC_AHB4LPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_ahb4lpenclrr;
#[doc = "RCC_MC_AXIMLPENSETR register accessor: an alias for `Reg<RCC_MC_AXIMLPENSETR_SPEC>`"]
pub type RCC_MC_AXIMLPENSETR = crate::Reg<rcc_mc_aximlpensetr::RCC_MC_AXIMLPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_aximlpensetr;
#[doc = "RCC_MC_AXIMLPENCLRR register accessor: an alias for `Reg<RCC_MC_AXIMLPENCLRR_SPEC>`"]
pub type RCC_MC_AXIMLPENCLRR = crate::Reg<rcc_mc_aximlpenclrr::RCC_MC_AXIMLPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_aximlpenclrr;
#[doc = "RCC_MC_MLAHBLPENSETR register accessor: an alias for `Reg<RCC_MC_MLAHBLPENSETR_SPEC>`"]
pub type RCC_MC_MLAHBLPENSETR = crate::Reg<rcc_mc_mlahblpensetr::RCC_MC_MLAHBLPENSETR_SPEC>;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_mlahblpensetr;
#[doc = "RCC_MC_MLAHBLPENCLRR register accessor: an alias for `Reg<RCC_MC_MLAHBLPENCLRR_SPEC>`"]
pub type RCC_MC_MLAHBLPENCLRR = crate::Reg<rcc_mc_mlahblpenclrr::RCC_MC_MLAHBLPENCLRR_SPEC>;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_mlahblpenclrr;
#[doc = "RCC_MC_RSTSCLRR register accessor: an alias for `Reg<RCC_MC_RSTSCLRR_SPEC>`"]
pub type RCC_MC_RSTSCLRR = crate::Reg<rcc_mc_rstsclrr::RCC_MC_RSTSCLRR_SPEC>;
#[doc = "This register is used by the MCU to check the reset source."]
pub mod rcc_mc_rstsclrr;
#[doc = "RCC_MC_CIER register accessor: an alias for `Reg<RCC_MC_CIER_SPEC>`"]
pub type RCC_MC_CIER = crate::Reg<rcc_mc_cier::RCC_MC_CIER_SPEC>;
#[doc = "This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details."]
pub mod rcc_mc_cier;
#[doc = "RCC_MC_CIFR register accessor: an alias for `Reg<RCC_MC_CIFR_SPEC>`"]
pub type RCC_MC_CIFR = crate::Reg<rcc_mc_cifr::RCC_MC_CIFR_SPEC>;
#[doc = "This register shall be used by the MCU in order to read and clear the interrupt flags."]
pub mod rcc_mc_cifr;
#[doc = "RCC_VERR register accessor: an alias for `Reg<RCC_VERR_SPEC>`"]
pub type RCC_VERR = crate::Reg<rcc_verr::RCC_VERR_SPEC>;
#[doc = "This register gives the IP version"]
pub mod rcc_verr;
#[doc = "RCC_IDR register accessor: an alias for `Reg<RCC_IDR_SPEC>`"]
pub type RCC_IDR = crate::Reg<rcc_idr::RCC_IDR_SPEC>;
#[doc = "This register gives the unique identifier of the RCC"]
pub mod rcc_idr;
#[doc = "RCC_SIDR register accessor: an alias for `Reg<RCC_SIDR_SPEC>`"]
pub type RCC_SIDR = crate::Reg<rcc_sidr::RCC_SIDR_SPEC>;
#[doc = "This register gives the decoding space, which is for the RCC of 4 kB."]
pub mod rcc_sidr;
