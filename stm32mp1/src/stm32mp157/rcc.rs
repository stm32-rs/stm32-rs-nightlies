#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode."]
    pub rcc_tzcr: RCC_TZCR,
    _reserved1: [u8; 8usize],
    #[doc = "0x0c - This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_ocensetr: RCC_OCENSETR,
    #[doc = "0x10 - This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_ocenclrr: RCC_OCENCLRR,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_hsicfgr: RCC_HSICFGR,
    #[doc = "0x1c - This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details."]
    pub rcc_csicfgr: RCC_CSICFGR,
    #[doc = "0x20 - This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_mpckselr: RCC_MPCKSELR,
    #[doc = "0x24 - This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_assckselr: RCC_ASSCKSELR,
    #[doc = "0x28 - This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_rck12selr: RCC_RCK12SELR,
    #[doc = "0x2c - This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mpckdivr: RCC_MPCKDIVR,
    #[doc = "0x30 - This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_axidivr: RCC_AXIDIVR,
    _reserved10: [u8; 8usize],
    #[doc = "0x3c - This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb4divr: RCC_APB4DIVR,
    #[doc = "0x40 - This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb5divr: RCC_APB5DIVR,
    #[doc = "0x44 - This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_rtcdivr: RCC_RTCDIVR,
    #[doc = "0x48 - This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_mssckselr: RCC_MSSCKSELR,
    _reserved14: [u8; 52usize],
    #[doc = "0x80 - This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1cr: RCC_PLL1CR,
    #[doc = "0x84 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1cfgr1: RCC_PLL1CFGR1,
    #[doc = "0x88 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1cfgr2: RCC_PLL1CFGR2,
    #[doc = "0x8c - This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1fracr: RCC_PLL1FRACR,
    #[doc = "0x90 - This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll1csgr: RCC_PLL1CSGR,
    #[doc = "0x94 - This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2cr: RCC_PLL2CR,
    #[doc = "0x98 - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2cfgr1: RCC_PLL2CFGR1,
    #[doc = "0x9c - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2cfgr2: RCC_PLL2CFGR2,
    #[doc = "0xa0 - This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2fracr: RCC_PLL2FRACR,
    #[doc = "0xa4 - This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
    pub rcc_pll2csgr: RCC_PLL2CSGR,
    _reserved24: [u8; 24usize],
    #[doc = "0xc0 - This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_i2c46ckselr: RCC_I2C46CKSELR,
    #[doc = "0xc4 - This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_spi6ckselr: RCC_SPI6CKSELR,
    #[doc = "0xc8 - This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_uart1ckselr: RCC_UART1CKSELR,
    #[doc = "0xcc - This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_rng1ckselr: RCC_RNG1CKSELR,
    #[doc = "0xd0 - This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays."]
    pub rcc_cperckselr: RCC_CPERCKSELR,
    #[doc = "0xd4 - This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_stgenckselr: RCC_STGENCKSELR,
    #[doc = "0xd8 - This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_ddritfcr: RCC_DDRITFCR,
    _reserved31: [u8; 36usize],
    #[doc = "0x100 - This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU."]
    pub rcc_mp_bootcr: RCC_MP_BOOTCR,
    #[doc = "0x104 - Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_sreqsetr: RCC_MP_SREQSETR,
    #[doc = "0x108 - Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_sreqclrr: RCC_MP_SREQCLRR,
    #[doc = "0x10c - The register contains global control bits. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_gcr: RCC_MP_GCR,
    #[doc = "0x110 - This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_aprstcr: RCC_MP_APRSTCR,
    #[doc = "0x114 - This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_aprstsr: RCC_MP_APRSTSR,
    _reserved37: [u8; 40usize],
    #[doc = "0x140 - This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_bdcr: RCC_BDCR,
    #[doc = "0x144 - This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_rdlsicr: RCC_RDLSICR,
    _reserved39: [u8; 56usize],
    #[doc = "0x180 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
    pub rcc_apb4rstsetr: RCC_APB4RSTSETR,
    #[doc = "0x184 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
    pub rcc_apb4rstclrr: RCC_APB4RSTCLRR,
    #[doc = "0x188 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb5rstsetr: RCC_APB5RSTSETR,
    #[doc = "0x18c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_apb5rstclrr: RCC_APB5RSTCLRR,
    #[doc = "0x190 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_ahb5rstsetr: RCC_AHB5RSTSETR,
    #[doc = "0x194 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_ahb5rstclrr: RCC_AHB5RSTCLRR,
    #[doc = "0x198 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
    pub rcc_ahb6rstsetr: RCC_AHB6RSTSETR,
    #[doc = "0x19c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
    pub rcc_ahb6rstclrr: RCC_AHB6RSTCLRR,
    #[doc = "0x1a0 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_tzahb6rstsetr: RCC_TZAHB6RSTSETR,
    #[doc = "0x1a4 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_tzahb6rstclrr: RCC_TZAHB6RSTCLRR,
    _reserved49: [u8; 88usize],
    #[doc = "0x200 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb4ensetr: RCC_MP_APB4ENSETR,
    #[doc = "0x204 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb4enclrr: RCC_MP_APB4ENCLRR,
    #[doc = "0x208 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb5ensetr: RCC_MP_APB5ENSETR,
    #[doc = "0x20c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_apb5enclrr: RCC_MP_APB5ENCLRR,
    #[doc = "0x210 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_ahb5ensetr: RCC_MP_AHB5ENSETR,
    #[doc = "0x214 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_ahb5enclrr: RCC_MP_AHB5ENCLRR,
    #[doc = "0x218 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_ahb6ensetr: RCC_MP_AHB6ENSETR,
    #[doc = "0x21c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
    pub rcc_mp_ahb6enclrr: RCC_MP_AHB6ENCLRR,
    #[doc = "0x220 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6ensetr: RCC_MP_TZAHB6ENSETR,
    #[doc = "0x224 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6enclrr: RCC_MP_TZAHB6ENCLRR,
    _reserved59: [u8; 88usize],
    #[doc = "0x280 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb4ensetr: RCC_MC_APB4ENSETR,
    #[doc = "0x284 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb4enclrr: RCC_MC_APB4ENCLRR,
    #[doc = "0x288 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb5ensetr: RCC_MC_APB5ENSETR,
    #[doc = "0x28c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb5enclrr: RCC_MC_APB5ENCLRR,
    #[doc = "0x290 - This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5ensetr: RCC_MC_AHB5ENSETR,
    #[doc = "0x294 - This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5enclrr: RCC_MC_AHB5ENCLRR,
    #[doc = "0x298 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb6ensetr: RCC_MC_AHB6ENSETR,
    #[doc = "0x29c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb6enclrr: RCC_MC_AHB6ENCLRR,
    _reserved67: [u8; 96usize],
    #[doc = "0x300 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb4lpensetr: RCC_MP_APB4LPENSETR,
    #[doc = "0x304 - This register is used by the MCU"]
    pub rcc_mp_apb4lpenclrr: RCC_MP_APB4LPENCLRR,
    #[doc = "0x308 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_apb5lpensetr: RCC_MP_APB5LPENSETR,
    #[doc = "0x30c - This register is used by the Mpu."]
    pub rcc_mp_apb5lpenclrr: RCC_MP_APB5LPENCLRR,
    #[doc = "0x310 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_ahb5lpensetr: RCC_MP_AHB5LPENSETR,
    #[doc = "0x314 - This register is used by the MCU"]
    pub rcc_mp_ahb5lpenclrr: RCC_MP_AHB5LPENCLRR,
    #[doc = "0x318 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_ahb6lpensetr: RCC_MP_AHB6LPENSETR,
    #[doc = "0x31c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_ahb6lpenclrr: RCC_MP_AHB6LPENCLRR,
    #[doc = "0x320 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6lpensetr: RCC_MP_TZAHB6LPENSETR,
    #[doc = "0x324 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_tzahb6lpenclrr: RCC_MP_TZAHB6LPENCLRR,
    _reserved77: [u8; 88usize],
    #[doc = "0x380 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb4lpensetr: RCC_MC_APB4LPENSETR,
    #[doc = "0x384 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb4lpenclrr: RCC_MC_APB4LPENCLRR,
    #[doc = "0x388 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb5lpensetr: RCC_MC_APB5LPENSETR,
    #[doc = "0x38c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb5lpenclrr: RCC_MC_APB5LPENCLRR,
    #[doc = "0x390 - This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5lpensetr: RCC_MC_AHB5LPENSETR,
    #[doc = "0x394 - This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mc_ahb5lpenclrr: RCC_MC_AHB5LPENCLRR,
    #[doc = "0x398 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb6lpensetr: RCC_MC_AHB6LPENSETR,
    #[doc = "0x39c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_ahb6lpenclrr: RCC_MC_AHB6LPENCLRR,
    _reserved85: [u8; 96usize],
    #[doc = "0x400 - This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_br_rstsclrr: RCC_BR_RSTSCLRR,
    #[doc = "0x404 - This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset."]
    pub rcc_mp_grstcsetr: RCC_MP_GRSTCSETR,
    #[doc = "0x408 - This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_rstsclrr: RCC_MP_RSTSCLRR,
    #[doc = "0x40c - This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_iwdgfzsetr: RCC_MP_IWDGFZSETR,
    #[doc = "0x410 - This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_iwdgfzclrr: RCC_MP_IWDGFZCLRR,
    #[doc = "0x414 - This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_cier: RCC_MP_CIER,
    #[doc = "0x418 - This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_cifr: RCC_MP_CIFR,
    #[doc = "0x41c - This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_pwrlpdlycr: RCC_PWRLPDLYCR,
    #[doc = "0x420 - This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mp_rstssetr: RCC_MP_RSTSSETR,
    _reserved94: [u8; 988usize],
    #[doc = "0x800 - This register is used to select the clock generated on MCO1 output."]
    pub rcc_mco1cfgr: RCC_MCO1CFGR,
    #[doc = "0x804 - This register is used to select the clock generated on MCO2 output."]
    pub rcc_mco2cfgr: RCC_MCO2CFGR,
    #[doc = "0x808 - This is a read-only access register, It contains the status flags of oscillators. Writing has no effect."]
    pub rcc_ocrdyr: RCC_OCRDYR,
    #[doc = "0x80c - This is register contains the enable control of the debug and trace function, and the clock divider for the trace function."]
    pub rcc_dbgcfgr: RCC_DBGCFGR,
    _reserved98: [u8; 16usize],
    #[doc = "0x820 - This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_rck3selr: RCC_RCK3SELR,
    #[doc = "0x824 - This register is used to select the reference clock for PLL4."]
    pub rcc_rck4selr: RCC_RCK4SELR,
    #[doc = "0x828 - This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_timg1prer: RCC_TIMG1PRER,
    #[doc = "0x82c - This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_timg2prer: RCC_TIMG2PRER,
    #[doc = "0x830 - This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
    pub rcc_mcudivr: RCC_MCUDIVR,
    #[doc = "0x834 - This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information."]
    pub rcc_apb1divr: RCC_APB1DIVR,
    #[doc = "0x838 - This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_apb2divr: RCC_APB2DIVR,
    #[doc = "0x83c - This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
    pub rcc_apb3divr: RCC_APB3DIVR,
    _reserved106: [u8; 64usize],
    #[doc = "0x880 - This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3cr: RCC_PLL3CR,
    #[doc = "0x884 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3cfgr1: RCC_PLL3CFGR1,
    #[doc = "0x888 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3cfgr2: RCC_PLL3CFGR2,
    #[doc = "0x88c - This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3fracr: RCC_PLL3FRACR,
    #[doc = "0x890 - This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll3csgr: RCC_PLL3CSGR,
    #[doc = "0x894 - This register is used to control the PLL4."]
    pub rcc_pll4cr: RCC_PLL4CR,
    #[doc = "0x898 - This register is used to configure the PLL4."]
    pub rcc_pll4cfgr1: RCC_PLL4CFGR1,
    #[doc = "0x89c - This register is used to configure the PLL4."]
    pub rcc_pll4cfgr2: RCC_PLL4CFGR2,
    #[doc = "0x8a0 - This register is used to fine-tune the frequency of the PLL4 VCO."]
    pub rcc_pll4fracr: RCC_PLL4FRACR,
    #[doc = "0x8a4 - This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
    pub rcc_pll4csgr: RCC_PLL4CSGR,
    _reserved116: [u8; 24usize],
    #[doc = "0x8c0 - This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_i2c12ckselr: RCC_I2C12CKSELR,
    #[doc = "0x8c4 - This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_i2c35ckselr: RCC_I2C35CKSELR,
    #[doc = "0x8c8 - This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai1ckselr: RCC_SAI1CKSELR,
    #[doc = "0x8cc - This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai2ckselr: RCC_SAI2CKSELR,
    #[doc = "0x8d0 - This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai3ckselr: RCC_SAI3CKSELR,
    #[doc = "0x8d4 - This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sai4ckselr: RCC_SAI4CKSELR,
    #[doc = "0x8d8 - This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spi2s1ckselr: RCC_SPI2S1CKSELR,
    #[doc = "0x8dc - This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spi2s23ckselr: RCC_SPI2S23CKSELR,
    #[doc = "0x8e0 - This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spi45ckselr: RCC_SPI45CKSELR,
    #[doc = "0x8e4 - This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart6ckselr: RCC_UART6CKSELR,
    #[doc = "0x8e8 - This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart24ckselr: RCC_UART24CKSELR,
    #[doc = "0x8ec - This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart35ckselr: RCC_UART35CKSELR,
    #[doc = "0x8f0 - This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_uart78ckselr: RCC_UART78CKSELR,
    #[doc = "0x8f4 - This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sdmmc12ckselr: RCC_SDMMC12CKSELR,
    #[doc = "0x8f8 - This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_sdmmc3ckselr: RCC_SDMMC3CKSELR,
    #[doc = "0x8fc - This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_ethckselr: RCC_ETHCKSELR,
    #[doc = "0x900 - This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_qspickselr: RCC_QSPICKSELR,
    #[doc = "0x904 - This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_fmcckselr: RCC_FMCCKSELR,
    _reserved134: [u8; 4usize],
    #[doc = "0x90c - This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_fdcanckselr: RCC_FDCANCKSELR,
    _reserved135: [u8; 4usize],
    #[doc = "0x914 - This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
    pub rcc_spdifckselr: RCC_SPDIFCKSELR,
    #[doc = "0x918 - This register is used to control the selection of the kernel clock for the CEC-HDMI."]
    pub rcc_cecckselr: RCC_CECCKSELR,
    #[doc = "0x91c - This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG"]
    pub rcc_usbckselr: RCC_USBCKSELR,
    #[doc = "0x920 - This register is used to control the selection of the kernel clock for the RNG2."]
    pub rcc_rng2ckselr: RCC_RNG2CKSELR,
    #[doc = "0x924 - This register is used to control the selection of the kernel clock for the DSI block."]
    pub rcc_dsickselr: RCC_DSICKSELR,
    #[doc = "0x928 - This register is used to control the selection of the kernel clock for the ADC block."]
    pub rcc_adcckselr: RCC_ADCCKSELR,
    #[doc = "0x92c - This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks."]
    pub rcc_lptim45ckselr: RCC_LPTIM45CKSELR,
    #[doc = "0x930 - This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks."]
    pub rcc_lptim23ckselr: RCC_LPTIM23CKSELR,
    #[doc = "0x934 - This register is used to control the selection of the kernel clock for the LPTIM1 block."]
    pub rcc_lptim1ckselr: RCC_LPTIM1CKSELR,
    _reserved144: [u8; 72usize],
    #[doc = "0x980 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_apb1rstsetr: RCC_APB1RSTSETR,
    #[doc = "0x984 - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_apb1rstclrr: RCC_APB1RSTCLRR,
    #[doc = "0x988 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_apb2rstsetr: RCC_APB2RSTSETR,
    #[doc = "0x98c - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_apb2rstclrr: RCC_APB2RSTCLRR,
    #[doc = "0x990 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_apb3rstsetr: RCC_APB3RSTSETR,
    #[doc = "0x994 - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_apb3rstclrr: RCC_APB3RSTCLRR,
    #[doc = "0x998 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_ahb2rstsetr: RCC_AHB2RSTSETR,
    #[doc = "0x99c - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_ahb2rstclrr: RCC_AHB2RSTCLRR,
    #[doc = "0x9a0 - This register is used to activate the reset of the corresponding peripheral."]
    pub rcc_ahb3rstsetr: RCC_AHB3RSTSETR,
    #[doc = "0x9a4 - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_ahb3rstclrr: RCC_AHB3RSTCLRR,
    #[doc = "0x9a8 - This register is used to activate the reset of the corresponding peripheral"]
    pub rcc_ahb4rstsetr: RCC_AHB4RSTSETR,
    #[doc = "0x9ac - This register is used to release the reset of the corresponding peripheral."]
    pub rcc_ahb4rstclrr: RCC_AHB4RSTCLRR,
    _reserved156: [u8; 80usize],
    #[doc = "0xa00 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_apb1ensetr: RCC_MP_APB1ENSETR,
    #[doc = "0xa04 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mp_apb1enclrr: RCC_MP_APB1ENCLRR,
    #[doc = "0xa08 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_apb2ensetr: RCC_MP_APB2ENSETR,
    #[doc = "0xa0c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_apb2enclrr: RCC_MP_APB2ENCLRR,
    #[doc = "0xa10 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_apb3ensetr: RCC_MP_APB3ENSETR,
    #[doc = "0xa14 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_apb3enclrr: RCC_MP_APB3ENCLRR,
    #[doc = "0xa18 - This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
    pub rcc_mp_ahb2ensetr: RCC_MP_AHB2ENSETR,
    #[doc = "0xa1c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_ahb2enclrr: RCC_MP_AHB2ENCLRR,
    #[doc = "0xa20 - This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
    pub rcc_mp_ahb3ensetr: RCC_MP_AHB3ENSETR,
    #[doc = "0xa24 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mp_ahb3enclrr: RCC_MP_AHB3ENCLRR,
    #[doc = "0xa28 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU."]
    pub rcc_mp_ahb4ensetr: RCC_MP_AHB4ENSETR,
    #[doc = "0xa2c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mp_ahb4enclrr: RCC_MP_AHB4ENCLRR,
    _reserved168: [u8; 8usize],
    #[doc = "0xa38 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mp_mlahbensetr: RCC_MP_MLAHBENSETR,
    #[doc = "0xa3c - This register is used to clear the peripheral clock enable bit."]
    pub rcc_mp_mlahbenclrr: RCC_MP_MLAHBENCLRR,
    _reserved170: [u8; 64usize],
    #[doc = "0xa80 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to ."]
    pub rcc_mc_apb1ensetr: RCC_MC_APB1ENSETR,
    #[doc = "0xa84 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
    pub rcc_mc_apb1enclrr: RCC_MC_APB1ENCLRR,
    #[doc = "0xa88 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb2ensetr: RCC_MC_APB2ENSETR,
    #[doc = "0xa8c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb2enclrr: RCC_MC_APB2ENCLRR,
    #[doc = "0xa90 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_apb3ensetr: RCC_MC_APB3ENSETR,
    #[doc = "0xa94 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_apb3enclrr: RCC_MC_APB3ENCLRR,
    #[doc = "0xa98 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb2ensetr: RCC_MC_AHB2ENSETR,
    #[doc = "0xa9c - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb2enclrr: RCC_MC_AHB2ENCLRR,
    #[doc = "0xaa0 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb3ensetr: RCC_MC_AHB3ENSETR,
    #[doc = "0xaa4 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb3enclrr: RCC_MC_AHB3ENCLRR,
    #[doc = "0xaa8 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_ahb4ensetr: RCC_MC_AHB4ENSETR,
    #[doc = "0xaac - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_ahb4enclrr: RCC_MC_AHB4ENCLRR,
    #[doc = "0xab0 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_aximensetr: RCC_MC_AXIMENSETR,
    #[doc = "0xab4 - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_aximenclrr: RCC_MC_AXIMENCLRR,
    #[doc = "0xab8 - This register is used to set the peripheral clock enable bit"]
    pub rcc_mc_mlahbensetr: RCC_MC_MLAHBENSETR,
    #[doc = "0xabc - This register is used to clear the peripheral clock enable bit"]
    pub rcc_mc_mlahbenclrr: RCC_MC_MLAHBENCLRR,
    _reserved186: [u8; 64usize],
    #[doc = "0xb00 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb1lpensetr: RCC_MP_APB1LPENSETR,
    #[doc = "0xb04 - This register is used by the MPU in order to clear the PERxLPEN bits ."]
    pub rcc_mp_apb1lpenclrr: RCC_MP_APB1LPENCLRR,
    #[doc = "0xb08 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb2lpensetr: RCC_MP_APB2LPENSETR,
    #[doc = "0xb0c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb2lpenclrr: RCC_MP_APB2LPENCLRR,
    #[doc = "0xb10 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb3lpensetr: RCC_MP_APB3LPENSETR,
    #[doc = "0xb14 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_apb3lpenclrr: RCC_MP_APB3LPENCLRR,
    #[doc = "0xb18 - This register is used by the MPU in order to set the PERxLPEN bit."]
    pub rcc_mp_ahb2lpensetr: RCC_MP_AHB2LPENSETR,
    #[doc = "0xb1c - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mp_ahb2lpenclrr: RCC_MP_AHB2LPENCLRR,
    #[doc = "0xb20 - This register is used by the MPU"]
    pub rcc_mp_ahb3lpensetr: RCC_MP_AHB3LPENSETR,
    #[doc = "0xb24 - This register is used by the MPU in order to clear the PERxLPEN bit"]
    pub rcc_mp_ahb3lpenclrr: RCC_MP_AHB3LPENCLRR,
    #[doc = "0xb28 - This register is used by the MPU"]
    pub rcc_mp_ahb4lpensetr: RCC_MP_AHB4LPENSETR,
    #[doc = "0xb2c - This register is used by the MPU"]
    pub rcc_mp_ahb4lpenclrr: RCC_MP_AHB4LPENCLRR,
    #[doc = "0xb30 - This register is used by the MPU"]
    pub rcc_mp_aximlpensetr: RCC_MP_AXIMLPENSETR,
    #[doc = "0xb34 - This register is used by the MPU"]
    pub rcc_mp_aximlpenclrr: RCC_MP_AXIMLPENCLRR,
    #[doc = "0xb38 - This register is used by the MPU in order to set the PERxLPEN bit"]
    pub rcc_mp_mlahblpensetr: RCC_MP_MLAHBLPENSETR,
    #[doc = "0xb3c - This register is used by the MPU in order to clear the PERxLPEN bit"]
    pub rcc_mp_mlahblpenclrr: RCC_MP_MLAHBLPENCLRR,
    _reserved202: [u8; 64usize],
    #[doc = "0xb80 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb1lpensetr: RCC_MC_APB1LPENSETR,
    #[doc = "0xb84 - This register is used by the MCU in order to clear the PERxLPEN bits"]
    pub rcc_mc_apb1lpenclrr: RCC_MC_APB1LPENCLRR,
    #[doc = "0xb88 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb2lpensetr: RCC_MC_APB2LPENSETR,
    #[doc = "0xb8c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb2lpenclrr: RCC_MC_APB2LPENCLRR,
    #[doc = "0xb90 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_apb3lpensetr: RCC_MC_APB3LPENSETR,
    #[doc = "0xb94 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_apb3lpenclrr: RCC_MC_APB3LPENCLRR,
    #[doc = "0xb98 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb2lpensetr: RCC_MC_AHB2LPENSETR,
    #[doc = "0xb9c - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_ahb2lpenclrr: RCC_MC_AHB2LPENCLRR,
    #[doc = "0xba0 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb3lpensetr: RCC_MC_AHB3LPENSETR,
    #[doc = "0xba4 - This register is used by the MCU in order to clear the PERxLPEN bit"]
    pub rcc_mc_ahb3lpenclrr: RCC_MC_AHB3LPENCLRR,
    #[doc = "0xba8 - This register is used by the MCU in order to set the PERxLPEN bit."]
    pub rcc_mc_ahb4lpensetr: RCC_MC_AHB4LPENSETR,
    #[doc = "0xbac - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_ahb4lpenclrr: RCC_MC_AHB4LPENCLRR,
    #[doc = "0xbb0 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_aximlpensetr: RCC_MC_AXIMLPENSETR,
    #[doc = "0xbb4 - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_aximlpenclrr: RCC_MC_AXIMLPENCLRR,
    #[doc = "0xbb8 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_mlahblpensetr: RCC_MC_MLAHBLPENSETR,
    #[doc = "0xbbc - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
    pub rcc_mc_mlahblpenclrr: RCC_MC_MLAHBLPENCLRR,
    _reserved218: [u8; 64usize],
    #[doc = "0xc00 - This register is used by the MCU to check the reset source."]
    pub rcc_mc_rstsclrr: RCC_MC_RSTSCLRR,
    _reserved219: [u8; 16usize],
    #[doc = "0xc14 - This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details."]
    pub rcc_mc_cier: RCC_MC_CIER,
    #[doc = "0xc18 - This register shall be used by the MCU in order to read and clear the interrupt flags."]
    pub rcc_mc_cifr: RCC_MC_CIFR,
    _reserved221: [u8; 984usize],
    #[doc = "0xff4 - This register gives the IP version"]
    pub rcc_verr: RCC_VERR,
    #[doc = "0xff8 - This register gives the unique identifier of the RCC"]
    pub rcc_idr: RCC_IDR,
    #[doc = "0xffc - This register gives the decoding space, which is for the RCC of 4 kB."]
    pub rcc_sidr: RCC_SIDR,
}
#[doc = "This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_tzcr](rcc_tzcr) module"]
pub type RCC_TZCR = crate::Reg<u32, _RCC_TZCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_TZCR;
#[doc = "`read()` method returns [rcc_tzcr::R](rcc_tzcr::R) reader structure"]
impl crate::Readable for RCC_TZCR {}
#[doc = "`write(|w| ..)` method takes [rcc_tzcr::W](rcc_tzcr::W) writer structure"]
impl crate::Writable for RCC_TZCR {}
#[doc = "This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode."]
pub mod rcc_tzcr;
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ocensetr](rcc_ocensetr) module"]
pub type RCC_OCENSETR = crate::Reg<u32, _RCC_OCENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_OCENSETR;
#[doc = "`read()` method returns [rcc_ocensetr::R](rcc_ocensetr::R) reader structure"]
impl crate::Readable for RCC_OCENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_ocensetr::W](rcc_ocensetr::W) writer structure"]
impl crate::Writable for RCC_OCENSETR {}
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_ocensetr;
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ocenclrr](rcc_ocenclrr) module"]
pub type RCC_OCENCLRR = crate::Reg<u32, _RCC_OCENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_OCENCLRR;
#[doc = "`read()` method returns [rcc_ocenclrr::R](rcc_ocenclrr::R) reader structure"]
impl crate::Readable for RCC_OCENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_ocenclrr::W](rcc_ocenclrr::W) writer structure"]
impl crate::Writable for RCC_OCENCLRR {}
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_ocenclrr;
#[doc = "This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_hsicfgr](rcc_hsicfgr) module"]
pub type RCC_HSICFGR = crate::Reg<u32, _RCC_HSICFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_HSICFGR;
#[doc = "`read()` method returns [rcc_hsicfgr::R](rcc_hsicfgr::R) reader structure"]
impl crate::Readable for RCC_HSICFGR {}
#[doc = "`write(|w| ..)` method takes [rcc_hsicfgr::W](rcc_hsicfgr::W) writer structure"]
impl crate::Writable for RCC_HSICFGR {}
#[doc = "This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_hsicfgr;
#[doc = "This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_csicfgr](rcc_csicfgr) module"]
pub type RCC_CSICFGR = crate::Reg<u32, _RCC_CSICFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_CSICFGR;
#[doc = "`read()` method returns [rcc_csicfgr::R](rcc_csicfgr::R) reader structure"]
impl crate::Readable for RCC_CSICFGR {}
#[doc = "`write(|w| ..)` method takes [rcc_csicfgr::W](rcc_csicfgr::W) writer structure"]
impl crate::Writable for RCC_CSICFGR {}
#[doc = "This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details."]
pub mod rcc_csicfgr;
#[doc = "This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mpckselr](rcc_mpckselr) module"]
pub type RCC_MPCKSELR = crate::Reg<u32, _RCC_MPCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MPCKSELR;
#[doc = "`read()` method returns [rcc_mpckselr::R](rcc_mpckselr::R) reader structure"]
impl crate::Readable for RCC_MPCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_mpckselr::W](rcc_mpckselr::W) writer structure"]
impl crate::Writable for RCC_MPCKSELR {}
#[doc = "This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_mpckselr;
#[doc = "This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_assckselr](rcc_assckselr) module"]
pub type RCC_ASSCKSELR = crate::Reg<u32, _RCC_ASSCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_ASSCKSELR;
#[doc = "`read()` method returns [rcc_assckselr::R](rcc_assckselr::R) reader structure"]
impl crate::Readable for RCC_ASSCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_assckselr::W](rcc_assckselr::W) writer structure"]
impl crate::Writable for RCC_ASSCKSELR {}
#[doc = "This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_assckselr;
#[doc = "This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rck12selr](rcc_rck12selr) module"]
pub type RCC_RCK12SELR = crate::Reg<u32, _RCC_RCK12SELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_RCK12SELR;
#[doc = "`read()` method returns [rcc_rck12selr::R](rcc_rck12selr::R) reader structure"]
impl crate::Readable for RCC_RCK12SELR {}
#[doc = "`write(|w| ..)` method takes [rcc_rck12selr::W](rcc_rck12selr::W) writer structure"]
impl crate::Writable for RCC_RCK12SELR {}
#[doc = "This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_rck12selr;
#[doc = "This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mpckdivr](rcc_mpckdivr) module"]
pub type RCC_MPCKDIVR = crate::Reg<u32, _RCC_MPCKDIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MPCKDIVR;
#[doc = "`read()` method returns [rcc_mpckdivr::R](rcc_mpckdivr::R) reader structure"]
impl crate::Readable for RCC_MPCKDIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_mpckdivr::W](rcc_mpckdivr::W) writer structure"]
impl crate::Writable for RCC_MPCKDIVR {}
#[doc = "This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mpckdivr;
#[doc = "This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_axidivr](rcc_axidivr) module"]
pub type RCC_AXIDIVR = crate::Reg<u32, _RCC_AXIDIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AXIDIVR;
#[doc = "`read()` method returns [rcc_axidivr::R](rcc_axidivr::R) reader structure"]
impl crate::Readable for RCC_AXIDIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_axidivr::W](rcc_axidivr::W) writer structure"]
impl crate::Writable for RCC_AXIDIVR {}
#[doc = "This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_axidivr;
#[doc = "This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb4divr](rcc_apb4divr) module"]
pub type RCC_APB4DIVR = crate::Reg<u32, _RCC_APB4DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB4DIVR;
#[doc = "`read()` method returns [rcc_apb4divr::R](rcc_apb4divr::R) reader structure"]
impl crate::Readable for RCC_APB4DIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb4divr::W](rcc_apb4divr::W) writer structure"]
impl crate::Writable for RCC_APB4DIVR {}
#[doc = "This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb4divr;
#[doc = "This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb5divr](rcc_apb5divr) module"]
pub type RCC_APB5DIVR = crate::Reg<u32, _RCC_APB5DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB5DIVR;
#[doc = "`read()` method returns [rcc_apb5divr::R](rcc_apb5divr::R) reader structure"]
impl crate::Readable for RCC_APB5DIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb5divr::W](rcc_apb5divr::W) writer structure"]
impl crate::Writable for RCC_APB5DIVR {}
#[doc = "This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5divr;
#[doc = "This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rtcdivr](rcc_rtcdivr) module"]
pub type RCC_RTCDIVR = crate::Reg<u32, _RCC_RTCDIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_RTCDIVR;
#[doc = "`read()` method returns [rcc_rtcdivr::R](rcc_rtcdivr::R) reader structure"]
impl crate::Readable for RCC_RTCDIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_rtcdivr::W](rcc_rtcdivr::W) writer structure"]
impl crate::Writable for RCC_RTCDIVR {}
#[doc = "This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rtcdivr;
#[doc = "This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mssckselr](rcc_mssckselr) module"]
pub type RCC_MSSCKSELR = crate::Reg<u32, _RCC_MSSCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MSSCKSELR;
#[doc = "`read()` method returns [rcc_mssckselr::R](rcc_mssckselr::R) reader structure"]
impl crate::Readable for RCC_MSSCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_mssckselr::W](rcc_mssckselr::W) writer structure"]
impl crate::Writable for RCC_MSSCKSELR {}
#[doc = "This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_mssckselr;
#[doc = "This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll1cr](rcc_pll1cr) module"]
pub type RCC_PLL1CR = crate::Reg<u32, _RCC_PLL1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL1CR;
#[doc = "`read()` method returns [rcc_pll1cr::R](rcc_pll1cr::R) reader structure"]
impl crate::Readable for RCC_PLL1CR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll1cr::W](rcc_pll1cr::W) writer structure"]
impl crate::Writable for RCC_PLL1CR {}
#[doc = "This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cr;
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll1cfgr1](rcc_pll1cfgr1) module"]
pub type RCC_PLL1CFGR1 = crate::Reg<u32, _RCC_PLL1CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL1CFGR1;
#[doc = "`read()` method returns [rcc_pll1cfgr1::R](rcc_pll1cfgr1::R) reader structure"]
impl crate::Readable for RCC_PLL1CFGR1 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll1cfgr1::W](rcc_pll1cfgr1::W) writer structure"]
impl crate::Writable for RCC_PLL1CFGR1 {}
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cfgr1;
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll1cfgr2](rcc_pll1cfgr2) module"]
pub type RCC_PLL1CFGR2 = crate::Reg<u32, _RCC_PLL1CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL1CFGR2;
#[doc = "`read()` method returns [rcc_pll1cfgr2::R](rcc_pll1cfgr2::R) reader structure"]
impl crate::Readable for RCC_PLL1CFGR2 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll1cfgr2::W](rcc_pll1cfgr2::W) writer structure"]
impl crate::Writable for RCC_PLL1CFGR2 {}
#[doc = "This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1cfgr2;
#[doc = "This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll1fracr](rcc_pll1fracr) module"]
pub type RCC_PLL1FRACR = crate::Reg<u32, _RCC_PLL1FRACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL1FRACR;
#[doc = "`read()` method returns [rcc_pll1fracr::R](rcc_pll1fracr::R) reader structure"]
impl crate::Readable for RCC_PLL1FRACR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll1fracr::W](rcc_pll1fracr::W) writer structure"]
impl crate::Writable for RCC_PLL1FRACR {}
#[doc = "This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1fracr;
#[doc = "This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll1csgr](rcc_pll1csgr) module"]
pub type RCC_PLL1CSGR = crate::Reg<u32, _RCC_PLL1CSGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL1CSGR;
#[doc = "`read()` method returns [rcc_pll1csgr::R](rcc_pll1csgr::R) reader structure"]
impl crate::Readable for RCC_PLL1CSGR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll1csgr::W](rcc_pll1csgr::W) writer structure"]
impl crate::Writable for RCC_PLL1CSGR {}
#[doc = "This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll1csgr;
#[doc = "This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2cr](rcc_pll2cr) module"]
pub type RCC_PLL2CR = crate::Reg<u32, _RCC_PLL2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL2CR;
#[doc = "`read()` method returns [rcc_pll2cr::R](rcc_pll2cr::R) reader structure"]
impl crate::Readable for RCC_PLL2CR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll2cr::W](rcc_pll2cr::W) writer structure"]
impl crate::Writable for RCC_PLL2CR {}
#[doc = "This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cr;
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2cfgr1](rcc_pll2cfgr1) module"]
pub type RCC_PLL2CFGR1 = crate::Reg<u32, _RCC_PLL2CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL2CFGR1;
#[doc = "`read()` method returns [rcc_pll2cfgr1::R](rcc_pll2cfgr1::R) reader structure"]
impl crate::Readable for RCC_PLL2CFGR1 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll2cfgr1::W](rcc_pll2cfgr1::W) writer structure"]
impl crate::Writable for RCC_PLL2CFGR1 {}
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cfgr1;
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2cfgr2](rcc_pll2cfgr2) module"]
pub type RCC_PLL2CFGR2 = crate::Reg<u32, _RCC_PLL2CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL2CFGR2;
#[doc = "`read()` method returns [rcc_pll2cfgr2::R](rcc_pll2cfgr2::R) reader structure"]
impl crate::Readable for RCC_PLL2CFGR2 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll2cfgr2::W](rcc_pll2cfgr2::W) writer structure"]
impl crate::Writable for RCC_PLL2CFGR2 {}
#[doc = "This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2cfgr2;
#[doc = "This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2fracr](rcc_pll2fracr) module"]
pub type RCC_PLL2FRACR = crate::Reg<u32, _RCC_PLL2FRACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL2FRACR;
#[doc = "`read()` method returns [rcc_pll2fracr::R](rcc_pll2fracr::R) reader structure"]
impl crate::Readable for RCC_PLL2FRACR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll2fracr::W](rcc_pll2fracr::W) writer structure"]
impl crate::Writable for RCC_PLL2FRACR {}
#[doc = "This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2fracr;
#[doc = "This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2csgr](rcc_pll2csgr) module"]
pub type RCC_PLL2CSGR = crate::Reg<u32, _RCC_PLL2CSGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL2CSGR;
#[doc = "`read()` method returns [rcc_pll2csgr::R](rcc_pll2csgr::R) reader structure"]
impl crate::Readable for RCC_PLL2CSGR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll2csgr::W](rcc_pll2csgr::W) writer structure"]
impl crate::Writable for RCC_PLL2CSGR {}
#[doc = "This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details."]
pub mod rcc_pll2csgr;
#[doc = "This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_i2c46ckselr](rcc_i2c46ckselr) module"]
pub type RCC_I2C46CKSELR = crate::Reg<u32, _RCC_I2C46CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_I2C46CKSELR;
#[doc = "`read()` method returns [rcc_i2c46ckselr::R](rcc_i2c46ckselr::R) reader structure"]
impl crate::Readable for RCC_I2C46CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_i2c46ckselr::W](rcc_i2c46ckselr::W) writer structure"]
impl crate::Writable for RCC_I2C46CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_i2c46ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_spi6ckselr](rcc_spi6ckselr) module"]
pub type RCC_SPI6CKSELR = crate::Reg<u32, _RCC_SPI6CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SPI6CKSELR;
#[doc = "`read()` method returns [rcc_spi6ckselr::R](rcc_spi6ckselr::R) reader structure"]
impl crate::Readable for RCC_SPI6CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_spi6ckselr::W](rcc_spi6ckselr::W) writer structure"]
impl crate::Writable for RCC_SPI6CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_spi6ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_uart1ckselr](rcc_uart1ckselr) module"]
pub type RCC_UART1CKSELR = crate::Reg<u32, _RCC_UART1CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_UART1CKSELR;
#[doc = "`read()` method returns [rcc_uart1ckselr::R](rcc_uart1ckselr::R) reader structure"]
impl crate::Readable for RCC_UART1CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_uart1ckselr::W](rcc_uart1ckselr::W) writer structure"]
impl crate::Writable for RCC_UART1CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_uart1ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rng1ckselr](rcc_rng1ckselr) module"]
pub type RCC_RNG1CKSELR = crate::Reg<u32, _RCC_RNG1CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_RNG1CKSELR;
#[doc = "`read()` method returns [rcc_rng1ckselr::R](rcc_rng1ckselr::R) reader structure"]
impl crate::Readable for RCC_RNG1CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_rng1ckselr::W](rcc_rng1ckselr::W) writer structure"]
impl crate::Writable for RCC_RNG1CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rng1ckselr;
#[doc = "This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cperckselr](rcc_cperckselr) module"]
pub type RCC_CPERCKSELR = crate::Reg<u32, _RCC_CPERCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_CPERCKSELR;
#[doc = "`read()` method returns [rcc_cperckselr::R](rcc_cperckselr::R) reader structure"]
impl crate::Readable for RCC_CPERCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_cperckselr::W](rcc_cperckselr::W) writer structure"]
impl crate::Writable for RCC_CPERCKSELR {}
#[doc = "This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays."]
pub mod rcc_cperckselr;
#[doc = "This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_stgenckselr](rcc_stgenckselr) module"]
pub type RCC_STGENCKSELR = crate::Reg<u32, _RCC_STGENCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_STGENCKSELR;
#[doc = "`read()` method returns [rcc_stgenckselr::R](rcc_stgenckselr::R) reader structure"]
impl crate::Readable for RCC_STGENCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_stgenckselr::W](rcc_stgenckselr::W) writer structure"]
impl crate::Writable for RCC_STGENCKSELR {}
#[doc = "This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_stgenckselr;
#[doc = "This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ddritfcr](rcc_ddritfcr) module"]
pub type RCC_DDRITFCR = crate::Reg<u32, _RCC_DDRITFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_DDRITFCR;
#[doc = "`read()` method returns [rcc_ddritfcr::R](rcc_ddritfcr::R) reader structure"]
impl crate::Readable for RCC_DDRITFCR {}
#[doc = "`write(|w| ..)` method takes [rcc_ddritfcr::W](rcc_ddritfcr::W) writer structure"]
impl crate::Writable for RCC_DDRITFCR {}
#[doc = "This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ddritfcr;
#[doc = "This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_bootcr](rcc_mp_bootcr) module"]
pub type RCC_MP_BOOTCR = crate::Reg<u32, _RCC_MP_BOOTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_BOOTCR;
#[doc = "`read()` method returns [rcc_mp_bootcr::R](rcc_mp_bootcr::R) reader structure"]
impl crate::Readable for RCC_MP_BOOTCR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_bootcr::W](rcc_mp_bootcr::W) writer structure"]
impl crate::Writable for RCC_MP_BOOTCR {}
#[doc = "This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU."]
pub mod rcc_mp_bootcr;
#[doc = "Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_sreqsetr](rcc_mp_sreqsetr) module"]
pub type RCC_MP_SREQSETR = crate::Reg<u32, _RCC_MP_SREQSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_SREQSETR;
#[doc = "`read()` method returns [rcc_mp_sreqsetr::R](rcc_mp_sreqsetr::R) reader structure"]
impl crate::Readable for RCC_MP_SREQSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_sreqsetr::W](rcc_mp_sreqsetr::W) writer structure"]
impl crate::Writable for RCC_MP_SREQSETR {}
#[doc = "Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_sreqsetr;
#[doc = "Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_sreqclrr](rcc_mp_sreqclrr) module"]
pub type RCC_MP_SREQCLRR = crate::Reg<u32, _RCC_MP_SREQCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_SREQCLRR;
#[doc = "`read()` method returns [rcc_mp_sreqclrr::R](rcc_mp_sreqclrr::R) reader structure"]
impl crate::Readable for RCC_MP_SREQCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_sreqclrr::W](rcc_mp_sreqclrr::W) writer structure"]
impl crate::Writable for RCC_MP_SREQCLRR {}
#[doc = "Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_sreqclrr;
#[doc = "The register contains global control bits. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_gcr](rcc_mp_gcr) module"]
pub type RCC_MP_GCR = crate::Reg<u32, _RCC_MP_GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_GCR;
#[doc = "`read()` method returns [rcc_mp_gcr::R](rcc_mp_gcr::R) reader structure"]
impl crate::Readable for RCC_MP_GCR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_gcr::W](rcc_mp_gcr::W) writer structure"]
impl crate::Writable for RCC_MP_GCR {}
#[doc = "The register contains global control bits. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_gcr;
#[doc = "This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_aprstcr](rcc_mp_aprstcr) module"]
pub type RCC_MP_APRSTCR = crate::Reg<u32, _RCC_MP_APRSTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APRSTCR;
#[doc = "`read()` method returns [rcc_mp_aprstcr::R](rcc_mp_aprstcr::R) reader structure"]
impl crate::Readable for RCC_MP_APRSTCR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_aprstcr::W](rcc_mp_aprstcr::W) writer structure"]
impl crate::Writable for RCC_MP_APRSTCR {}
#[doc = "This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_aprstcr;
#[doc = "This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_aprstsr](rcc_mp_aprstsr) module"]
pub type RCC_MP_APRSTSR = crate::Reg<u32, _RCC_MP_APRSTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APRSTSR;
#[doc = "`read()` method returns [rcc_mp_aprstsr::R](rcc_mp_aprstsr::R) reader structure"]
impl crate::Readable for RCC_MP_APRSTSR {}
#[doc = "This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_aprstsr;
#[doc = "This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_bdcr](rcc_bdcr) module"]
pub type RCC_BDCR = crate::Reg<u32, _RCC_BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_BDCR;
#[doc = "`read()` method returns [rcc_bdcr::R](rcc_bdcr::R) reader structure"]
impl crate::Readable for RCC_BDCR {}
#[doc = "`write(|w| ..)` method takes [rcc_bdcr::W](rcc_bdcr::W) writer structure"]
impl crate::Writable for RCC_BDCR {}
#[doc = "This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_bdcr;
#[doc = "This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rdlsicr](rcc_rdlsicr) module"]
pub type RCC_RDLSICR = crate::Reg<u32, _RCC_RDLSICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_RDLSICR;
#[doc = "`read()` method returns [rcc_rdlsicr::R](rcc_rdlsicr::R) reader structure"]
impl crate::Readable for RCC_RDLSICR {}
#[doc = "`write(|w| ..)` method takes [rcc_rdlsicr::W](rcc_rdlsicr::W) writer structure"]
impl crate::Writable for RCC_RDLSICR {}
#[doc = "This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_rdlsicr;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb4rstsetr](rcc_apb4rstsetr) module"]
pub type RCC_APB4RSTSETR = crate::Reg<u32, _RCC_APB4RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB4RSTSETR;
#[doc = "`read()` method returns [rcc_apb4rstsetr::R](rcc_apb4rstsetr::R) reader structure"]
impl crate::Readable for RCC_APB4RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb4rstsetr::W](rcc_apb4rstsetr::W) writer structure"]
impl crate::Writable for RCC_APB4RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
pub mod rcc_apb4rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb4rstclrr](rcc_apb4rstclrr) module"]
pub type RCC_APB4RSTCLRR = crate::Reg<u32, _RCC_APB4RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB4RSTCLRR;
#[doc = "`read()` method returns [rcc_apb4rstclrr::R](rcc_apb4rstclrr::R) reader structure"]
impl crate::Readable for RCC_APB4RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb4rstclrr::W](rcc_apb4rstclrr::W) writer structure"]
impl crate::Writable for RCC_APB4RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
pub mod rcc_apb4rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb5rstsetr](rcc_apb5rstsetr) module"]
pub type RCC_APB5RSTSETR = crate::Reg<u32, _RCC_APB5RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB5RSTSETR;
#[doc = "`read()` method returns [rcc_apb5rstsetr::R](rcc_apb5rstsetr::R) reader structure"]
impl crate::Readable for RCC_APB5RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb5rstsetr::W](rcc_apb5rstsetr::W) writer structure"]
impl crate::Writable for RCC_APB5RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb5rstclrr](rcc_apb5rstclrr) module"]
pub type RCC_APB5RSTCLRR = crate::Reg<u32, _RCC_APB5RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB5RSTCLRR;
#[doc = "`read()` method returns [rcc_apb5rstclrr::R](rcc_apb5rstclrr::R) reader structure"]
impl crate::Readable for RCC_APB5RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb5rstclrr::W](rcc_apb5rstclrr::W) writer structure"]
impl crate::Writable for RCC_APB5RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_apb5rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb5rstsetr](rcc_ahb5rstsetr) module"]
pub type RCC_AHB5RSTSETR = crate::Reg<u32, _RCC_AHB5RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB5RSTSETR;
#[doc = "`read()` method returns [rcc_ahb5rstsetr::R](rcc_ahb5rstsetr::R) reader structure"]
impl crate::Readable for RCC_AHB5RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb5rstsetr::W](rcc_ahb5rstsetr::W) writer structure"]
impl crate::Writable for RCC_AHB5RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ahb5rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb5rstclrr](rcc_ahb5rstclrr) module"]
pub type RCC_AHB5RSTCLRR = crate::Reg<u32, _RCC_AHB5RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB5RSTCLRR;
#[doc = "`read()` method returns [rcc_ahb5rstclrr::R](rcc_ahb5rstclrr::R) reader structure"]
impl crate::Readable for RCC_AHB5RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb5rstclrr::W](rcc_ahb5rstclrr::W) writer structure"]
impl crate::Writable for RCC_AHB5RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_ahb5rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb6rstsetr](rcc_ahb6rstsetr) module"]
pub type RCC_AHB6RSTSETR = crate::Reg<u32, _RCC_AHB6RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB6RSTSETR;
#[doc = "`read()` method returns [rcc_ahb6rstsetr::R](rcc_ahb6rstsetr::R) reader structure"]
impl crate::Readable for RCC_AHB6RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb6rstsetr::W](rcc_ahb6rstsetr::W) writer structure"]
impl crate::Writable for RCC_AHB6RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral."]
pub mod rcc_ahb6rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb6rstclrr](rcc_ahb6rstclrr) module"]
pub type RCC_AHB6RSTCLRR = crate::Reg<u32, _RCC_AHB6RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB6RSTCLRR;
#[doc = "`read()` method returns [rcc_ahb6rstclrr::R](rcc_ahb6rstclrr::R) reader structure"]
impl crate::Readable for RCC_AHB6RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb6rstclrr::W](rcc_ahb6rstclrr::W) writer structure"]
impl crate::Writable for RCC_AHB6RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral."]
pub mod rcc_ahb6rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_tzahb6rstsetr](rcc_tzahb6rstsetr) module"]
pub type RCC_TZAHB6RSTSETR = crate::Reg<u32, _RCC_TZAHB6RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_TZAHB6RSTSETR;
#[doc = "`read()` method returns [rcc_tzahb6rstsetr::R](rcc_tzahb6rstsetr::R) reader structure"]
impl crate::Readable for RCC_TZAHB6RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_tzahb6rstsetr::W](rcc_tzahb6rstsetr::W) writer structure"]
impl crate::Writable for RCC_TZAHB6RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_tzahb6rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_tzahb6rstclrr](rcc_tzahb6rstclrr) module"]
pub type RCC_TZAHB6RSTCLRR = crate::Reg<u32, _RCC_TZAHB6RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_TZAHB6RSTCLRR;
#[doc = "`read()` method returns [rcc_tzahb6rstclrr::R](rcc_tzahb6rstclrr::R) reader structure"]
impl crate::Readable for RCC_TZAHB6RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_tzahb6rstclrr::W](rcc_tzahb6rstclrr::W) writer structure"]
impl crate::Writable for RCC_TZAHB6RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_tzahb6rstclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb4ensetr](rcc_mp_apb4ensetr) module"]
pub type RCC_MP_APB4ENSETR = crate::Reg<u32, _RCC_MP_APB4ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB4ENSETR;
#[doc = "`read()` method returns [rcc_mp_apb4ensetr::R](rcc_mp_apb4ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB4ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb4ensetr::W](rcc_mp_apb4ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB4ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb4ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb4enclrr](rcc_mp_apb4enclrr) module"]
pub type RCC_MP_APB4ENCLRR = crate::Reg<u32, _RCC_MP_APB4ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB4ENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb4enclrr::R](rcc_mp_apb4enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB4ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb4enclrr::W](rcc_mp_apb4enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB4ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb4enclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb5ensetr](rcc_mp_apb5ensetr) module"]
pub type RCC_MP_APB5ENSETR = crate::Reg<u32, _RCC_MP_APB5ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB5ENSETR;
#[doc = "`read()` method returns [rcc_mp_apb5ensetr::R](rcc_mp_apb5ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB5ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb5ensetr::W](rcc_mp_apb5ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB5ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb5ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb5enclrr](rcc_mp_apb5enclrr) module"]
pub type RCC_MP_APB5ENCLRR = crate::Reg<u32, _RCC_MP_APB5ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB5ENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb5enclrr::R](rcc_mp_apb5enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB5ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb5enclrr::W](rcc_mp_apb5enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB5ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_apb5enclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb5ensetr](rcc_mp_ahb5ensetr) module"]
pub type RCC_MP_AHB5ENSETR = crate::Reg<u32, _RCC_MP_AHB5ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB5ENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb5ensetr::R](rcc_mp_ahb5ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB5ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb5ensetr::W](rcc_mp_ahb5ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB5ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb5enclrr](rcc_mp_ahb5enclrr) module"]
pub type RCC_MP_AHB5ENCLRR = crate::Reg<u32, _RCC_MP_AHB5ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB5ENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb5enclrr::R](rcc_mp_ahb5enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB5ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb5enclrr::W](rcc_mp_ahb5enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB5ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5enclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb6ensetr](rcc_mp_ahb6ensetr) module"]
pub type RCC_MP_AHB6ENSETR = crate::Reg<u32, _RCC_MP_AHB6ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB6ENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb6ensetr::R](rcc_mp_ahb6ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB6ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb6ensetr::W](rcc_mp_ahb6ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB6ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_ahb6ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb6enclrr](rcc_mp_ahb6enclrr) module"]
pub type RCC_MP_AHB6ENCLRR = crate::Reg<u32, _RCC_MP_AHB6ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB6ENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb6enclrr::R](rcc_mp_ahb6enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB6ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb6enclrr::W](rcc_mp_ahb6enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB6ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to ."]
pub mod rcc_mp_ahb6enclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_tzahb6ensetr](rcc_mp_tzahb6ensetr) module"]
pub type RCC_MP_TZAHB6ENSETR = crate::Reg<u32, _RCC_MP_TZAHB6ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_TZAHB6ENSETR;
#[doc = "`read()` method returns [rcc_mp_tzahb6ensetr::R](rcc_mp_tzahb6ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_tzahb6ensetr::W](rcc_mp_tzahb6ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_tzahb6enclrr](rcc_mp_tzahb6enclrr) module"]
pub type RCC_MP_TZAHB6ENCLRR = crate::Reg<u32, _RCC_MP_TZAHB6ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_TZAHB6ENCLRR;
#[doc = "`read()` method returns [rcc_mp_tzahb6enclrr::R](rcc_mp_tzahb6enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_tzahb6enclrr::W](rcc_mp_tzahb6enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb4ensetr](rcc_mc_apb4ensetr) module"]
pub type RCC_MC_APB4ENSETR = crate::Reg<u32, _RCC_MC_APB4ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB4ENSETR;
#[doc = "`read()` method returns [rcc_mc_apb4ensetr::R](rcc_mc_apb4ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB4ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb4ensetr::W](rcc_mc_apb4ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB4ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb4ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb4enclrr](rcc_mc_apb4enclrr) module"]
pub type RCC_MC_APB4ENCLRR = crate::Reg<u32, _RCC_MC_APB4ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB4ENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb4enclrr::R](rcc_mc_apb4enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB4ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb4enclrr::W](rcc_mc_apb4enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB4ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb4enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb5ensetr](rcc_mc_apb5ensetr) module"]
pub type RCC_MC_APB5ENSETR = crate::Reg<u32, _RCC_MC_APB5ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB5ENSETR;
#[doc = "`read()` method returns [rcc_mc_apb5ensetr::R](rcc_mc_apb5ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB5ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb5ensetr::W](rcc_mc_apb5ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB5ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb5ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb5enclrr](rcc_mc_apb5enclrr) module"]
pub type RCC_MC_APB5ENCLRR = crate::Reg<u32, _RCC_MC_APB5ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB5ENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb5enclrr::R](rcc_mc_apb5enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB5ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb5enclrr::W](rcc_mc_apb5enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB5ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb5enclrr;
#[doc = "This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb5ensetr](rcc_mc_ahb5ensetr) module"]
pub type RCC_MC_AHB5ENSETR = crate::Reg<u32, _RCC_MC_AHB5ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB5ENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb5ensetr::R](rcc_mc_ahb5ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB5ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb5ensetr::W](rcc_mc_ahb5ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB5ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb5enclrr](rcc_mc_ahb5enclrr) module"]
pub type RCC_MC_AHB5ENCLRR = crate::Reg<u32, _RCC_MC_AHB5ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB5ENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb5enclrr::R](rcc_mc_ahb5enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB5ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb5enclrr::W](rcc_mc_ahb5enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB5ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb6ensetr](rcc_mc_ahb6ensetr) module"]
pub type RCC_MC_AHB6ENSETR = crate::Reg<u32, _RCC_MC_AHB6ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB6ENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb6ensetr::R](rcc_mc_ahb6ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB6ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb6ensetr::W](rcc_mc_ahb6ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB6ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb6ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb6enclrr](rcc_mc_ahb6enclrr) module"]
pub type RCC_MC_AHB6ENCLRR = crate::Reg<u32, _RCC_MC_AHB6ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB6ENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb6enclrr::R](rcc_mc_ahb6enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB6ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb6enclrr::W](rcc_mc_ahb6enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB6ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb6enclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb4lpensetr](rcc_mp_apb4lpensetr) module"]
pub type RCC_MP_APB4LPENSETR = crate::Reg<u32, _RCC_MP_APB4LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB4LPENSETR;
#[doc = "`read()` method returns [rcc_mp_apb4lpensetr::R](rcc_mp_apb4lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB4LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb4lpensetr::W](rcc_mp_apb4lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB4LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb4lpensetr;
#[doc = "This register is used by the MCU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb4lpenclrr](rcc_mp_apb4lpenclrr) module"]
pub type RCC_MP_APB4LPENCLRR = crate::Reg<u32, _RCC_MP_APB4LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB4LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb4lpenclrr::R](rcc_mp_apb4lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB4LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb4lpenclrr::W](rcc_mp_apb4lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB4LPENCLRR {}
#[doc = "This register is used by the MCU"]
pub mod rcc_mp_apb4lpenclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb5lpensetr](rcc_mp_apb5lpensetr) module"]
pub type RCC_MP_APB5LPENSETR = crate::Reg<u32, _RCC_MP_APB5LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB5LPENSETR;
#[doc = "`read()` method returns [rcc_mp_apb5lpensetr::R](rcc_mp_apb5lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB5LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb5lpensetr::W](rcc_mp_apb5lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB5LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_apb5lpensetr;
#[doc = "This register is used by the Mpu.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb5lpenclrr](rcc_mp_apb5lpenclrr) module"]
pub type RCC_MP_APB5LPENCLRR = crate::Reg<u32, _RCC_MP_APB5LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB5LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb5lpenclrr::R](rcc_mp_apb5lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB5LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb5lpenclrr::W](rcc_mp_apb5lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB5LPENCLRR {}
#[doc = "This register is used by the Mpu."]
pub mod rcc_mp_apb5lpenclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb5lpensetr](rcc_mp_ahb5lpensetr) module"]
pub type RCC_MP_AHB5LPENSETR = crate::Reg<u32, _RCC_MP_AHB5LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB5LPENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb5lpensetr::R](rcc_mp_ahb5lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB5LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb5lpensetr::W](rcc_mp_ahb5lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB5LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_ahb5lpensetr;
#[doc = "This register is used by the MCU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb5lpenclrr](rcc_mp_ahb5lpenclrr) module"]
pub type RCC_MP_AHB5LPENCLRR = crate::Reg<u32, _RCC_MP_AHB5LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB5LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb5lpenclrr::R](rcc_mp_ahb5lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB5LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb5lpenclrr::W](rcc_mp_ahb5lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB5LPENCLRR {}
#[doc = "This register is used by the MCU"]
pub mod rcc_mp_ahb5lpenclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb6lpensetr](rcc_mp_ahb6lpensetr) module"]
pub type RCC_MP_AHB6LPENSETR = crate::Reg<u32, _RCC_MP_AHB6LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB6LPENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb6lpensetr::R](rcc_mp_ahb6lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB6LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb6lpensetr::W](rcc_mp_ahb6lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB6LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb6lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb6lpenclrr](rcc_mp_ahb6lpenclrr) module"]
pub type RCC_MP_AHB6LPENCLRR = crate::Reg<u32, _RCC_MP_AHB6LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB6LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb6lpenclrr::R](rcc_mp_ahb6lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB6LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb6lpenclrr::W](rcc_mp_ahb6lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB6LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb6lpenclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_tzahb6lpensetr](rcc_mp_tzahb6lpensetr) module"]
pub type RCC_MP_TZAHB6LPENSETR = crate::Reg<u32, _RCC_MP_TZAHB6LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_TZAHB6LPENSETR;
#[doc = "`read()` method returns [rcc_mp_tzahb6lpensetr::R](rcc_mp_tzahb6lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_tzahb6lpensetr::W](rcc_mp_tzahb6lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_tzahb6lpenclrr](rcc_mp_tzahb6lpenclrr) module"]
pub type RCC_MP_TZAHB6LPENCLRR = crate::Reg<u32, _RCC_MP_TZAHB6LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_TZAHB6LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_tzahb6lpenclrr::R](rcc_mp_tzahb6lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_tzahb6lpenclrr::W](rcc_mp_tzahb6lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_tzahb6lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb4lpensetr](rcc_mc_apb4lpensetr) module"]
pub type RCC_MC_APB4LPENSETR = crate::Reg<u32, _RCC_MC_APB4LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB4LPENSETR;
#[doc = "`read()` method returns [rcc_mc_apb4lpensetr::R](rcc_mc_apb4lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB4LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb4lpensetr::W](rcc_mc_apb4lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB4LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb4lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb4lpenclrr](rcc_mc_apb4lpenclrr) module"]
pub type RCC_MC_APB4LPENCLRR = crate::Reg<u32, _RCC_MC_APB4LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB4LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb4lpenclrr::R](rcc_mc_apb4lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB4LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb4lpenclrr::W](rcc_mc_apb4lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB4LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb4lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb5lpensetr](rcc_mc_apb5lpensetr) module"]
pub type RCC_MC_APB5LPENSETR = crate::Reg<u32, _RCC_MC_APB5LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB5LPENSETR;
#[doc = "`read()` method returns [rcc_mc_apb5lpensetr::R](rcc_mc_apb5lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB5LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb5lpensetr::W](rcc_mc_apb5lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB5LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb5lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb5lpenclrr](rcc_mc_apb5lpenclrr) module"]
pub type RCC_MC_APB5LPENCLRR = crate::Reg<u32, _RCC_MC_APB5LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB5LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb5lpenclrr::R](rcc_mc_apb5lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB5LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb5lpenclrr::W](rcc_mc_apb5lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB5LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb5lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb5lpensetr](rcc_mc_ahb5lpensetr) module"]
pub type RCC_MC_AHB5LPENSETR = crate::Reg<u32, _RCC_MC_AHB5LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB5LPENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb5lpensetr::R](rcc_mc_ahb5lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB5LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb5lpensetr::W](rcc_mc_ahb5lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB5LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb5lpenclrr](rcc_mc_ahb5lpenclrr) module"]
pub type RCC_MC_AHB5LPENCLRR = crate::Reg<u32, _RCC_MC_AHB5LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB5LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb5lpenclrr::R](rcc_mc_ahb5lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB5LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb5lpenclrr::W](rcc_mc_ahb5lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB5LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mc_ahb5lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb6lpensetr](rcc_mc_ahb6lpensetr) module"]
pub type RCC_MC_AHB6LPENSETR = crate::Reg<u32, _RCC_MC_AHB6LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB6LPENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb6lpensetr::R](rcc_mc_ahb6lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB6LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb6lpensetr::W](rcc_mc_ahb6lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB6LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb6lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb6lpenclrr](rcc_mc_ahb6lpenclrr) module"]
pub type RCC_MC_AHB6LPENCLRR = crate::Reg<u32, _RCC_MC_AHB6LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB6LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb6lpenclrr::R](rcc_mc_ahb6lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB6LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb6lpenclrr::W](rcc_mc_ahb6lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB6LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb6lpenclrr;
#[doc = "This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_br_rstsclrr](rcc_br_rstsclrr) module"]
pub type RCC_BR_RSTSCLRR = crate::Reg<u32, _RCC_BR_RSTSCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_BR_RSTSCLRR;
#[doc = "`read()` method returns [rcc_br_rstsclrr::R](rcc_br_rstsclrr::R) reader structure"]
impl crate::Readable for RCC_BR_RSTSCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_br_rstsclrr::W](rcc_br_rstsclrr::W) writer structure"]
impl crate::Writable for RCC_BR_RSTSCLRR {}
#[doc = "This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\\[1:0\\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\\[1:0\\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_br_rstsclrr;
#[doc = "This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_grstcsetr](rcc_mp_grstcsetr) module"]
pub type RCC_MP_GRSTCSETR = crate::Reg<u32, _RCC_MP_GRSTCSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_GRSTCSETR;
#[doc = "`read()` method returns [rcc_mp_grstcsetr::R](rcc_mp_grstcsetr::R) reader structure"]
impl crate::Readable for RCC_MP_GRSTCSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_grstcsetr::W](rcc_mp_grstcsetr::W) writer structure"]
impl crate::Writable for RCC_MP_GRSTCSETR {}
#[doc = "This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset."]
pub mod rcc_mp_grstcsetr;
#[doc = "This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_rstsclrr](rcc_mp_rstsclrr) module"]
pub type RCC_MP_RSTSCLRR = crate::Reg<u32, _RCC_MP_RSTSCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_RSTSCLRR;
#[doc = "`read()` method returns [rcc_mp_rstsclrr::R](rcc_mp_rstsclrr::R) reader structure"]
impl crate::Readable for RCC_MP_RSTSCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_rstsclrr::W](rcc_mp_rstsclrr::W) writer structure"]
impl crate::Writable for RCC_MP_RSTSCLRR {}
#[doc = "This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_rstsclrr;
#[doc = "This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_iwdgfzsetr](rcc_mp_iwdgfzsetr) module"]
pub type RCC_MP_IWDGFZSETR = crate::Reg<u32, _RCC_MP_IWDGFZSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_IWDGFZSETR;
#[doc = "`read()` method returns [rcc_mp_iwdgfzsetr::R](rcc_mp_iwdgfzsetr::R) reader structure"]
impl crate::Readable for RCC_MP_IWDGFZSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_iwdgfzsetr::W](rcc_mp_iwdgfzsetr::W) writer structure"]
impl crate::Writable for RCC_MP_IWDGFZSETR {}
#[doc = "This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_iwdgfzsetr;
#[doc = "This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_iwdgfzclrr](rcc_mp_iwdgfzclrr) module"]
pub type RCC_MP_IWDGFZCLRR = crate::Reg<u32, _RCC_MP_IWDGFZCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_IWDGFZCLRR;
#[doc = "`read()` method returns [rcc_mp_iwdgfzclrr::R](rcc_mp_iwdgfzclrr::R) reader structure"]
impl crate::Readable for RCC_MP_IWDGFZCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_iwdgfzclrr::W](rcc_mp_iwdgfzclrr::W) writer structure"]
impl crate::Writable for RCC_MP_IWDGFZCLRR {}
#[doc = "This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_iwdgfzclrr;
#[doc = "This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_cier](rcc_mp_cier) module"]
pub type RCC_MP_CIER = crate::Reg<u32, _RCC_MP_CIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_CIER;
#[doc = "`read()` method returns [rcc_mp_cier::R](rcc_mp_cier::R) reader structure"]
impl crate::Readable for RCC_MP_CIER {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_cier::W](rcc_mp_cier::W) writer structure"]
impl crate::Writable for RCC_MP_CIER {}
#[doc = "This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_cier;
#[doc = "This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_cifr](rcc_mp_cifr) module"]
pub type RCC_MP_CIFR = crate::Reg<u32, _RCC_MP_CIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_CIFR;
#[doc = "`read()` method returns [rcc_mp_cifr::R](rcc_mp_cifr::R) reader structure"]
impl crate::Readable for RCC_MP_CIFR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_cifr::W](rcc_mp_cifr::W) writer structure"]
impl crate::Writable for RCC_MP_CIFR {}
#[doc = "This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_cifr;
#[doc = "This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pwrlpdlycr](rcc_pwrlpdlycr) module"]
pub type RCC_PWRLPDLYCR = crate::Reg<u32, _RCC_PWRLPDLYCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PWRLPDLYCR;
#[doc = "`read()` method returns [rcc_pwrlpdlycr::R](rcc_pwrlpdlycr::R) reader structure"]
impl crate::Readable for RCC_PWRLPDLYCR {}
#[doc = "`write(|w| ..)` method takes [rcc_pwrlpdlycr::W](rcc_pwrlpdlycr::W) writer structure"]
impl crate::Writable for RCC_PWRLPDLYCR {}
#[doc = "This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_pwrlpdlycr;
#[doc = "This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_rstssetr](rcc_mp_rstssetr) module"]
pub type RCC_MP_RSTSSETR = crate::Reg<u32, _RCC_MP_RSTSSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_RSTSSETR;
#[doc = "`read()` method returns [rcc_mp_rstssetr::R](rcc_mp_rstssetr::R) reader structure"]
impl crate::Readable for RCC_MP_RSTSSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_rstssetr::W](rcc_mp_rstssetr::W) writer structure"]
impl crate::Writable for RCC_MP_RSTSSETR {}
#[doc = "This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mp_rstssetr;
#[doc = "This register is used to select the clock generated on MCO1 output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mco1cfgr](rcc_mco1cfgr) module"]
pub type RCC_MCO1CFGR = crate::Reg<u32, _RCC_MCO1CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MCO1CFGR;
#[doc = "`read()` method returns [rcc_mco1cfgr::R](rcc_mco1cfgr::R) reader structure"]
impl crate::Readable for RCC_MCO1CFGR {}
#[doc = "`write(|w| ..)` method takes [rcc_mco1cfgr::W](rcc_mco1cfgr::W) writer structure"]
impl crate::Writable for RCC_MCO1CFGR {}
#[doc = "This register is used to select the clock generated on MCO1 output."]
pub mod rcc_mco1cfgr;
#[doc = "This register is used to select the clock generated on MCO2 output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mco2cfgr](rcc_mco2cfgr) module"]
pub type RCC_MCO2CFGR = crate::Reg<u32, _RCC_MCO2CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MCO2CFGR;
#[doc = "`read()` method returns [rcc_mco2cfgr::R](rcc_mco2cfgr::R) reader structure"]
impl crate::Readable for RCC_MCO2CFGR {}
#[doc = "`write(|w| ..)` method takes [rcc_mco2cfgr::W](rcc_mco2cfgr::W) writer structure"]
impl crate::Writable for RCC_MCO2CFGR {}
#[doc = "This register is used to select the clock generated on MCO2 output."]
pub mod rcc_mco2cfgr;
#[doc = "This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ocrdyr](rcc_ocrdyr) module"]
pub type RCC_OCRDYR = crate::Reg<u32, _RCC_OCRDYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_OCRDYR;
#[doc = "`read()` method returns [rcc_ocrdyr::R](rcc_ocrdyr::R) reader structure"]
impl crate::Readable for RCC_OCRDYR {}
#[doc = "This is a read-only access register, It contains the status flags of oscillators. Writing has no effect."]
pub mod rcc_ocrdyr;
#[doc = "This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_dbgcfgr](rcc_dbgcfgr) module"]
pub type RCC_DBGCFGR = crate::Reg<u32, _RCC_DBGCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_DBGCFGR;
#[doc = "`read()` method returns [rcc_dbgcfgr::R](rcc_dbgcfgr::R) reader structure"]
impl crate::Readable for RCC_DBGCFGR {}
#[doc = "`write(|w| ..)` method takes [rcc_dbgcfgr::W](rcc_dbgcfgr::W) writer structure"]
impl crate::Writable for RCC_DBGCFGR {}
#[doc = "This is register contains the enable control of the debug and trace function, and the clock divider for the trace function."]
pub mod rcc_dbgcfgr;
#[doc = "This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rck3selr](rcc_rck3selr) module"]
pub type RCC_RCK3SELR = crate::Reg<u32, _RCC_RCK3SELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_RCK3SELR;
#[doc = "`read()` method returns [rcc_rck3selr::R](rcc_rck3selr::R) reader structure"]
impl crate::Readable for RCC_RCK3SELR {}
#[doc = "`write(|w| ..)` method takes [rcc_rck3selr::W](rcc_rck3selr::W) writer structure"]
impl crate::Writable for RCC_RCK3SELR {}
#[doc = "This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_rck3selr;
#[doc = "This register is used to select the reference clock for PLL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rck4selr](rcc_rck4selr) module"]
pub type RCC_RCK4SELR = crate::Reg<u32, _RCC_RCK4SELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_RCK4SELR;
#[doc = "`read()` method returns [rcc_rck4selr::R](rcc_rck4selr::R) reader structure"]
impl crate::Readable for RCC_RCK4SELR {}
#[doc = "`write(|w| ..)` method takes [rcc_rck4selr::W](rcc_rck4selr::W) writer structure"]
impl crate::Writable for RCC_RCK4SELR {}
#[doc = "This register is used to select the reference clock for PLL4."]
pub mod rcc_rck4selr;
#[doc = "This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_timg1prer](rcc_timg1prer) module"]
pub type RCC_TIMG1PRER = crate::Reg<u32, _RCC_TIMG1PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_TIMG1PRER;
#[doc = "`read()` method returns [rcc_timg1prer::R](rcc_timg1prer::R) reader structure"]
impl crate::Readable for RCC_TIMG1PRER {}
#[doc = "`write(|w| ..)` method takes [rcc_timg1prer::W](rcc_timg1prer::W) writer structure"]
impl crate::Writable for RCC_TIMG1PRER {}
#[doc = "This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_timg1prer;
#[doc = "This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_timg2prer](rcc_timg2prer) module"]
pub type RCC_TIMG2PRER = crate::Reg<u32, _RCC_TIMG2PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_TIMG2PRER;
#[doc = "`read()` method returns [rcc_timg2prer::R](rcc_timg2prer::R) reader structure"]
impl crate::Readable for RCC_TIMG2PRER {}
#[doc = "`write(|w| ..)` method takes [rcc_timg2prer::W](rcc_timg2prer::W) writer structure"]
impl crate::Writable for RCC_TIMG2PRER {}
#[doc = "This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_timg2prer;
#[doc = "This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mcudivr](rcc_mcudivr) module"]
pub type RCC_MCUDIVR = crate::Reg<u32, _RCC_MCUDIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MCUDIVR;
#[doc = "`read()` method returns [rcc_mcudivr::R](rcc_mcudivr::R) reader structure"]
impl crate::Readable for RCC_MCUDIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_mcudivr::W](rcc_mcudivr::W) writer structure"]
impl crate::Writable for RCC_MCUDIVR {}
#[doc = "This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode."]
pub mod rcc_mcudivr;
#[doc = "This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1divr](rcc_apb1divr) module"]
pub type RCC_APB1DIVR = crate::Reg<u32, _RCC_APB1DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB1DIVR;
#[doc = "`read()` method returns [rcc_apb1divr::R](rcc_apb1divr::R) reader structure"]
impl crate::Readable for RCC_APB1DIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb1divr::W](rcc_apb1divr::W) writer structure"]
impl crate::Writable for RCC_APB1DIVR {}
#[doc = "This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information."]
pub mod rcc_apb1divr;
#[doc = "This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2divr](rcc_apb2divr) module"]
pub type RCC_APB2DIVR = crate::Reg<u32, _RCC_APB2DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB2DIVR;
#[doc = "`read()` method returns [rcc_apb2divr::R](rcc_apb2divr::R) reader structure"]
impl crate::Readable for RCC_APB2DIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb2divr::W](rcc_apb2divr::W) writer structure"]
impl crate::Writable for RCC_APB2DIVR {}
#[doc = "This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_apb2divr;
#[doc = "This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb3divr](rcc_apb3divr) module"]
pub type RCC_APB3DIVR = crate::Reg<u32, _RCC_APB3DIVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB3DIVR;
#[doc = "`read()` method returns [rcc_apb3divr::R](rcc_apb3divr::R) reader structure"]
impl crate::Readable for RCC_APB3DIVR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb3divr::W](rcc_apb3divr::W) writer structure"]
impl crate::Writable for RCC_APB3DIVR {}
#[doc = "This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information."]
pub mod rcc_apb3divr;
#[doc = "This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll3cr](rcc_pll3cr) module"]
pub type RCC_PLL3CR = crate::Reg<u32, _RCC_PLL3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL3CR;
#[doc = "`read()` method returns [rcc_pll3cr::R](rcc_pll3cr::R) reader structure"]
impl crate::Readable for RCC_PLL3CR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll3cr::W](rcc_pll3cr::W) writer structure"]
impl crate::Writable for RCC_PLL3CR {}
#[doc = "This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cr;
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll3cfgr1](rcc_pll3cfgr1) module"]
pub type RCC_PLL3CFGR1 = crate::Reg<u32, _RCC_PLL3CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL3CFGR1;
#[doc = "`read()` method returns [rcc_pll3cfgr1::R](rcc_pll3cfgr1::R) reader structure"]
impl crate::Readable for RCC_PLL3CFGR1 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll3cfgr1::W](rcc_pll3cfgr1::W) writer structure"]
impl crate::Writable for RCC_PLL3CFGR1 {}
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cfgr1;
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll3cfgr2](rcc_pll3cfgr2) module"]
pub type RCC_PLL3CFGR2 = crate::Reg<u32, _RCC_PLL3CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL3CFGR2;
#[doc = "`read()` method returns [rcc_pll3cfgr2::R](rcc_pll3cfgr2::R) reader structure"]
impl crate::Readable for RCC_PLL3CFGR2 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll3cfgr2::W](rcc_pll3cfgr2::W) writer structure"]
impl crate::Writable for RCC_PLL3CFGR2 {}
#[doc = "This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3cfgr2;
#[doc = "This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll3fracr](rcc_pll3fracr) module"]
pub type RCC_PLL3FRACR = crate::Reg<u32, _RCC_PLL3FRACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL3FRACR;
#[doc = "`read()` method returns [rcc_pll3fracr::R](rcc_pll3fracr::R) reader structure"]
impl crate::Readable for RCC_PLL3FRACR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll3fracr::W](rcc_pll3fracr::W) writer structure"]
impl crate::Writable for RCC_PLL3FRACR {}
#[doc = "This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3fracr;
#[doc = "This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll3csgr](rcc_pll3csgr) module"]
pub type RCC_PLL3CSGR = crate::Reg<u32, _RCC_PLL3CSGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL3CSGR;
#[doc = "`read()` method returns [rcc_pll3csgr::R](rcc_pll3csgr::R) reader structure"]
impl crate::Readable for RCC_PLL3CSGR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll3csgr::W](rcc_pll3csgr::W) writer structure"]
impl crate::Writable for RCC_PLL3CSGR {}
#[doc = "This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll3csgr;
#[doc = "This register is used to control the PLL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4cr](rcc_pll4cr) module"]
pub type RCC_PLL4CR = crate::Reg<u32, _RCC_PLL4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL4CR;
#[doc = "`read()` method returns [rcc_pll4cr::R](rcc_pll4cr::R) reader structure"]
impl crate::Readable for RCC_PLL4CR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll4cr::W](rcc_pll4cr::W) writer structure"]
impl crate::Writable for RCC_PLL4CR {}
#[doc = "This register is used to control the PLL4."]
pub mod rcc_pll4cr;
#[doc = "This register is used to configure the PLL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4cfgr1](rcc_pll4cfgr1) module"]
pub type RCC_PLL4CFGR1 = crate::Reg<u32, _RCC_PLL4CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL4CFGR1;
#[doc = "`read()` method returns [rcc_pll4cfgr1::R](rcc_pll4cfgr1::R) reader structure"]
impl crate::Readable for RCC_PLL4CFGR1 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll4cfgr1::W](rcc_pll4cfgr1::W) writer structure"]
impl crate::Writable for RCC_PLL4CFGR1 {}
#[doc = "This register is used to configure the PLL4."]
pub mod rcc_pll4cfgr1;
#[doc = "This register is used to configure the PLL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4cfgr2](rcc_pll4cfgr2) module"]
pub type RCC_PLL4CFGR2 = crate::Reg<u32, _RCC_PLL4CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL4CFGR2;
#[doc = "`read()` method returns [rcc_pll4cfgr2::R](rcc_pll4cfgr2::R) reader structure"]
impl crate::Readable for RCC_PLL4CFGR2 {}
#[doc = "`write(|w| ..)` method takes [rcc_pll4cfgr2::W](rcc_pll4cfgr2::W) writer structure"]
impl crate::Writable for RCC_PLL4CFGR2 {}
#[doc = "This register is used to configure the PLL4."]
pub mod rcc_pll4cfgr2;
#[doc = "This register is used to fine-tune the frequency of the PLL4 VCO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4fracr](rcc_pll4fracr) module"]
pub type RCC_PLL4FRACR = crate::Reg<u32, _RCC_PLL4FRACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL4FRACR;
#[doc = "`read()` method returns [rcc_pll4fracr::R](rcc_pll4fracr::R) reader structure"]
impl crate::Readable for RCC_PLL4FRACR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll4fracr::W](rcc_pll4fracr::W) writer structure"]
impl crate::Writable for RCC_PLL4FRACR {}
#[doc = "This register is used to fine-tune the frequency of the PLL4 VCO."]
pub mod rcc_pll4fracr;
#[doc = "This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll4csgr](rcc_pll4csgr) module"]
pub type RCC_PLL4CSGR = crate::Reg<u32, _RCC_PLL4CSGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_PLL4CSGR;
#[doc = "`read()` method returns [rcc_pll4csgr::R](rcc_pll4csgr::R) reader structure"]
impl crate::Readable for RCC_PLL4CSGR {}
#[doc = "`write(|w| ..)` method takes [rcc_pll4csgr::W](rcc_pll4csgr::W) writer structure"]
impl crate::Writable for RCC_PLL4CSGR {}
#[doc = "This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode."]
pub mod rcc_pll4csgr;
#[doc = "This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_i2c12ckselr](rcc_i2c12ckselr) module"]
pub type RCC_I2C12CKSELR = crate::Reg<u32, _RCC_I2C12CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_I2C12CKSELR;
#[doc = "`read()` method returns [rcc_i2c12ckselr::R](rcc_i2c12ckselr::R) reader structure"]
impl crate::Readable for RCC_I2C12CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_i2c12ckselr::W](rcc_i2c12ckselr::W) writer structure"]
impl crate::Writable for RCC_I2C12CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_i2c12ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_i2c35ckselr](rcc_i2c35ckselr) module"]
pub type RCC_I2C35CKSELR = crate::Reg<u32, _RCC_I2C35CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_I2C35CKSELR;
#[doc = "`read()` method returns [rcc_i2c35ckselr::R](rcc_i2c35ckselr::R) reader structure"]
impl crate::Readable for RCC_I2C35CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_i2c35ckselr::W](rcc_i2c35ckselr::W) writer structure"]
impl crate::Writable for RCC_I2C35CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_i2c35ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sai1ckselr](rcc_sai1ckselr) module"]
pub type RCC_SAI1CKSELR = crate::Reg<u32, _RCC_SAI1CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SAI1CKSELR;
#[doc = "`read()` method returns [rcc_sai1ckselr::R](rcc_sai1ckselr::R) reader structure"]
impl crate::Readable for RCC_SAI1CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_sai1ckselr::W](rcc_sai1ckselr::W) writer structure"]
impl crate::Writable for RCC_SAI1CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai1ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sai2ckselr](rcc_sai2ckselr) module"]
pub type RCC_SAI2CKSELR = crate::Reg<u32, _RCC_SAI2CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SAI2CKSELR;
#[doc = "`read()` method returns [rcc_sai2ckselr::R](rcc_sai2ckselr::R) reader structure"]
impl crate::Readable for RCC_SAI2CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_sai2ckselr::W](rcc_sai2ckselr::W) writer structure"]
impl crate::Writable for RCC_SAI2CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai2ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sai3ckselr](rcc_sai3ckselr) module"]
pub type RCC_SAI3CKSELR = crate::Reg<u32, _RCC_SAI3CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SAI3CKSELR;
#[doc = "`read()` method returns [rcc_sai3ckselr::R](rcc_sai3ckselr::R) reader structure"]
impl crate::Readable for RCC_SAI3CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_sai3ckselr::W](rcc_sai3ckselr::W) writer structure"]
impl crate::Writable for RCC_SAI3CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai3ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sai4ckselr](rcc_sai4ckselr) module"]
pub type RCC_SAI4CKSELR = crate::Reg<u32, _RCC_SAI4CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SAI4CKSELR;
#[doc = "`read()` method returns [rcc_sai4ckselr::R](rcc_sai4ckselr::R) reader structure"]
impl crate::Readable for RCC_SAI4CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_sai4ckselr::W](rcc_sai4ckselr::W) writer structure"]
impl crate::Writable for RCC_SAI4CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sai4ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_spi2s1ckselr](rcc_spi2s1ckselr) module"]
pub type RCC_SPI2S1CKSELR = crate::Reg<u32, _RCC_SPI2S1CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SPI2S1CKSELR;
#[doc = "`read()` method returns [rcc_spi2s1ckselr::R](rcc_spi2s1ckselr::R) reader structure"]
impl crate::Readable for RCC_SPI2S1CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_spi2s1ckselr::W](rcc_spi2s1ckselr::W) writer structure"]
impl crate::Writable for RCC_SPI2S1CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi2s1ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_spi2s23ckselr](rcc_spi2s23ckselr) module"]
pub type RCC_SPI2S23CKSELR = crate::Reg<u32, _RCC_SPI2S23CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SPI2S23CKSELR;
#[doc = "`read()` method returns [rcc_spi2s23ckselr::R](rcc_spi2s23ckselr::R) reader structure"]
impl crate::Readable for RCC_SPI2S23CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_spi2s23ckselr::W](rcc_spi2s23ckselr::W) writer structure"]
impl crate::Writable for RCC_SPI2S23CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi2s23ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_spi45ckselr](rcc_spi45ckselr) module"]
pub type RCC_SPI45CKSELR = crate::Reg<u32, _RCC_SPI45CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SPI45CKSELR;
#[doc = "`read()` method returns [rcc_spi45ckselr::R](rcc_spi45ckselr::R) reader structure"]
impl crate::Readable for RCC_SPI45CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_spi45ckselr::W](rcc_spi45ckselr::W) writer structure"]
impl crate::Writable for RCC_SPI45CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spi45ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_uart6ckselr](rcc_uart6ckselr) module"]
pub type RCC_UART6CKSELR = crate::Reg<u32, _RCC_UART6CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_UART6CKSELR;
#[doc = "`read()` method returns [rcc_uart6ckselr::R](rcc_uart6ckselr::R) reader structure"]
impl crate::Readable for RCC_UART6CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_uart6ckselr::W](rcc_uart6ckselr::W) writer structure"]
impl crate::Writable for RCC_UART6CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart6ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_uart24ckselr](rcc_uart24ckselr) module"]
pub type RCC_UART24CKSELR = crate::Reg<u32, _RCC_UART24CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_UART24CKSELR;
#[doc = "`read()` method returns [rcc_uart24ckselr::R](rcc_uart24ckselr::R) reader structure"]
impl crate::Readable for RCC_UART24CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_uart24ckselr::W](rcc_uart24ckselr::W) writer structure"]
impl crate::Writable for RCC_UART24CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart24ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_uart35ckselr](rcc_uart35ckselr) module"]
pub type RCC_UART35CKSELR = crate::Reg<u32, _RCC_UART35CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_UART35CKSELR;
#[doc = "`read()` method returns [rcc_uart35ckselr::R](rcc_uart35ckselr::R) reader structure"]
impl crate::Readable for RCC_UART35CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_uart35ckselr::W](rcc_uart35ckselr::W) writer structure"]
impl crate::Writable for RCC_UART35CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart35ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_uart78ckselr](rcc_uart78ckselr) module"]
pub type RCC_UART78CKSELR = crate::Reg<u32, _RCC_UART78CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_UART78CKSELR;
#[doc = "`read()` method returns [rcc_uart78ckselr::R](rcc_uart78ckselr::R) reader structure"]
impl crate::Readable for RCC_UART78CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_uart78ckselr::W](rcc_uart78ckselr::W) writer structure"]
impl crate::Writable for RCC_UART78CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_uart78ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sdmmc12ckselr](rcc_sdmmc12ckselr) module"]
pub type RCC_SDMMC12CKSELR = crate::Reg<u32, _RCC_SDMMC12CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SDMMC12CKSELR;
#[doc = "`read()` method returns [rcc_sdmmc12ckselr::R](rcc_sdmmc12ckselr::R) reader structure"]
impl crate::Readable for RCC_SDMMC12CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_sdmmc12ckselr::W](rcc_sdmmc12ckselr::W) writer structure"]
impl crate::Writable for RCC_SDMMC12CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sdmmc12ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sdmmc3ckselr](rcc_sdmmc3ckselr) module"]
pub type RCC_SDMMC3CKSELR = crate::Reg<u32, _RCC_SDMMC3CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SDMMC3CKSELR;
#[doc = "`read()` method returns [rcc_sdmmc3ckselr::R](rcc_sdmmc3ckselr::R) reader structure"]
impl crate::Readable for RCC_SDMMC3CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_sdmmc3ckselr::W](rcc_sdmmc3ckselr::W) writer structure"]
impl crate::Writable for RCC_SDMMC3CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_sdmmc3ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ethckselr](rcc_ethckselr) module"]
pub type RCC_ETHCKSELR = crate::Reg<u32, _RCC_ETHCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_ETHCKSELR;
#[doc = "`read()` method returns [rcc_ethckselr::R](rcc_ethckselr::R) reader structure"]
impl crate::Readable for RCC_ETHCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_ethckselr::W](rcc_ethckselr::W) writer structure"]
impl crate::Writable for RCC_ETHCKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_ethckselr;
#[doc = "This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_qspickselr](rcc_qspickselr) module"]
pub type RCC_QSPICKSELR = crate::Reg<u32, _RCC_QSPICKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_QSPICKSELR;
#[doc = "`read()` method returns [rcc_qspickselr::R](rcc_qspickselr::R) reader structure"]
impl crate::Readable for RCC_QSPICKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_qspickselr::W](rcc_qspickselr::W) writer structure"]
impl crate::Writable for RCC_QSPICKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_qspickselr;
#[doc = "This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_fmcckselr](rcc_fmcckselr) module"]
pub type RCC_FMCCKSELR = crate::Reg<u32, _RCC_FMCCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_FMCCKSELR;
#[doc = "`read()` method returns [rcc_fmcckselr::R](rcc_fmcckselr::R) reader structure"]
impl crate::Readable for RCC_FMCCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_fmcckselr::W](rcc_fmcckselr::W) writer structure"]
impl crate::Writable for RCC_FMCCKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_fmcckselr;
#[doc = "This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_fdcanckselr](rcc_fdcanckselr) module"]
pub type RCC_FDCANCKSELR = crate::Reg<u32, _RCC_FDCANCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_FDCANCKSELR;
#[doc = "`read()` method returns [rcc_fdcanckselr::R](rcc_fdcanckselr::R) reader structure"]
impl crate::Readable for RCC_FDCANCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_fdcanckselr::W](rcc_fdcanckselr::W) writer structure"]
impl crate::Writable for RCC_FDCANCKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_fdcanckselr;
#[doc = "This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_spdifckselr](rcc_spdifckselr) module"]
pub type RCC_SPDIFCKSELR = crate::Reg<u32, _RCC_SPDIFCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SPDIFCKSELR;
#[doc = "`read()` method returns [rcc_spdifckselr::R](rcc_spdifckselr::R) reader structure"]
impl crate::Readable for RCC_SPDIFCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_spdifckselr::W](rcc_spdifckselr::W) writer structure"]
impl crate::Writable for RCC_SPDIFCKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays."]
pub mod rcc_spdifckselr;
#[doc = "This register is used to control the selection of the kernel clock for the CEC-HDMI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cecckselr](rcc_cecckselr) module"]
pub type RCC_CECCKSELR = crate::Reg<u32, _RCC_CECCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_CECCKSELR;
#[doc = "`read()` method returns [rcc_cecckselr::R](rcc_cecckselr::R) reader structure"]
impl crate::Readable for RCC_CECCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_cecckselr::W](rcc_cecckselr::W) writer structure"]
impl crate::Writable for RCC_CECCKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the CEC-HDMI."]
pub mod rcc_cecckselr;
#[doc = "This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_usbckselr](rcc_usbckselr) module"]
pub type RCC_USBCKSELR = crate::Reg<u32, _RCC_USBCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_USBCKSELR;
#[doc = "`read()` method returns [rcc_usbckselr::R](rcc_usbckselr::R) reader structure"]
impl crate::Readable for RCC_USBCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_usbckselr::W](rcc_usbckselr::W) writer structure"]
impl crate::Writable for RCC_USBCKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG"]
pub mod rcc_usbckselr;
#[doc = "This register is used to control the selection of the kernel clock for the RNG2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rng2ckselr](rcc_rng2ckselr) module"]
pub type RCC_RNG2CKSELR = crate::Reg<u32, _RCC_RNG2CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_RNG2CKSELR;
#[doc = "`read()` method returns [rcc_rng2ckselr::R](rcc_rng2ckselr::R) reader structure"]
impl crate::Readable for RCC_RNG2CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_rng2ckselr::W](rcc_rng2ckselr::W) writer structure"]
impl crate::Writable for RCC_RNG2CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the RNG2."]
pub mod rcc_rng2ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the DSI block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_dsickselr](rcc_dsickselr) module"]
pub type RCC_DSICKSELR = crate::Reg<u32, _RCC_DSICKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_DSICKSELR;
#[doc = "`read()` method returns [rcc_dsickselr::R](rcc_dsickselr::R) reader structure"]
impl crate::Readable for RCC_DSICKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_dsickselr::W](rcc_dsickselr::W) writer structure"]
impl crate::Writable for RCC_DSICKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the DSI block."]
pub mod rcc_dsickselr;
#[doc = "This register is used to control the selection of the kernel clock for the ADC block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_adcckselr](rcc_adcckselr) module"]
pub type RCC_ADCCKSELR = crate::Reg<u32, _RCC_ADCCKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_ADCCKSELR;
#[doc = "`read()` method returns [rcc_adcckselr::R](rcc_adcckselr::R) reader structure"]
impl crate::Readable for RCC_ADCCKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_adcckselr::W](rcc_adcckselr::W) writer structure"]
impl crate::Writable for RCC_ADCCKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the ADC block."]
pub mod rcc_adcckselr;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_lptim45ckselr](rcc_lptim45ckselr) module"]
pub type RCC_LPTIM45CKSELR = crate::Reg<u32, _RCC_LPTIM45CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_LPTIM45CKSELR;
#[doc = "`read()` method returns [rcc_lptim45ckselr::R](rcc_lptim45ckselr::R) reader structure"]
impl crate::Readable for RCC_LPTIM45CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_lptim45ckselr::W](rcc_lptim45ckselr::W) writer structure"]
impl crate::Writable for RCC_LPTIM45CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks."]
pub mod rcc_lptim45ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_lptim23ckselr](rcc_lptim23ckselr) module"]
pub type RCC_LPTIM23CKSELR = crate::Reg<u32, _RCC_LPTIM23CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_LPTIM23CKSELR;
#[doc = "`read()` method returns [rcc_lptim23ckselr::R](rcc_lptim23ckselr::R) reader structure"]
impl crate::Readable for RCC_LPTIM23CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_lptim23ckselr::W](rcc_lptim23ckselr::W) writer structure"]
impl crate::Writable for RCC_LPTIM23CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks."]
pub mod rcc_lptim23ckselr;
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM1 block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_lptim1ckselr](rcc_lptim1ckselr) module"]
pub type RCC_LPTIM1CKSELR = crate::Reg<u32, _RCC_LPTIM1CKSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_LPTIM1CKSELR;
#[doc = "`read()` method returns [rcc_lptim1ckselr::R](rcc_lptim1ckselr::R) reader structure"]
impl crate::Readable for RCC_LPTIM1CKSELR {}
#[doc = "`write(|w| ..)` method takes [rcc_lptim1ckselr::W](rcc_lptim1ckselr::W) writer structure"]
impl crate::Writable for RCC_LPTIM1CKSELR {}
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM1 block."]
pub mod rcc_lptim1ckselr;
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1rstsetr](rcc_apb1rstsetr) module"]
pub type RCC_APB1RSTSETR = crate::Reg<u32, _RCC_APB1RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB1RSTSETR;
#[doc = "`read()` method returns [rcc_apb1rstsetr::R](rcc_apb1rstsetr::R) reader structure"]
impl crate::Readable for RCC_APB1RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb1rstsetr::W](rcc_apb1rstsetr::W) writer structure"]
impl crate::Writable for RCC_APB1RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb1rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1rstclrr](rcc_apb1rstclrr) module"]
pub type RCC_APB1RSTCLRR = crate::Reg<u32, _RCC_APB1RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB1RSTCLRR;
#[doc = "`read()` method returns [rcc_apb1rstclrr::R](rcc_apb1rstclrr::R) reader structure"]
impl crate::Readable for RCC_APB1RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb1rstclrr::W](rcc_apb1rstclrr::W) writer structure"]
impl crate::Writable for RCC_APB1RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb1rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2rstsetr](rcc_apb2rstsetr) module"]
pub type RCC_APB2RSTSETR = crate::Reg<u32, _RCC_APB2RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB2RSTSETR;
#[doc = "`read()` method returns [rcc_apb2rstsetr::R](rcc_apb2rstsetr::R) reader structure"]
impl crate::Readable for RCC_APB2RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb2rstsetr::W](rcc_apb2rstsetr::W) writer structure"]
impl crate::Writable for RCC_APB2RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb2rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2rstclrr](rcc_apb2rstclrr) module"]
pub type RCC_APB2RSTCLRR = crate::Reg<u32, _RCC_APB2RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB2RSTCLRR;
#[doc = "`read()` method returns [rcc_apb2rstclrr::R](rcc_apb2rstclrr::R) reader structure"]
impl crate::Readable for RCC_APB2RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb2rstclrr::W](rcc_apb2rstclrr::W) writer structure"]
impl crate::Writable for RCC_APB2RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb2rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb3rstsetr](rcc_apb3rstsetr) module"]
pub type RCC_APB3RSTSETR = crate::Reg<u32, _RCC_APB3RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB3RSTSETR;
#[doc = "`read()` method returns [rcc_apb3rstsetr::R](rcc_apb3rstsetr::R) reader structure"]
impl crate::Readable for RCC_APB3RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb3rstsetr::W](rcc_apb3rstsetr::W) writer structure"]
impl crate::Writable for RCC_APB3RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_apb3rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb3rstclrr](rcc_apb3rstclrr) module"]
pub type RCC_APB3RSTCLRR = crate::Reg<u32, _RCC_APB3RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_APB3RSTCLRR;
#[doc = "`read()` method returns [rcc_apb3rstclrr::R](rcc_apb3rstclrr::R) reader structure"]
impl crate::Readable for RCC_APB3RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_apb3rstclrr::W](rcc_apb3rstclrr::W) writer structure"]
impl crate::Writable for RCC_APB3RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_apb3rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb2rstsetr](rcc_ahb2rstsetr) module"]
pub type RCC_AHB2RSTSETR = crate::Reg<u32, _RCC_AHB2RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB2RSTSETR;
#[doc = "`read()` method returns [rcc_ahb2rstsetr::R](rcc_ahb2rstsetr::R) reader structure"]
impl crate::Readable for RCC_AHB2RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb2rstsetr::W](rcc_ahb2rstsetr::W) writer structure"]
impl crate::Writable for RCC_AHB2RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_ahb2rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb2rstclrr](rcc_ahb2rstclrr) module"]
pub type RCC_AHB2RSTCLRR = crate::Reg<u32, _RCC_AHB2RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB2RSTCLRR;
#[doc = "`read()` method returns [rcc_ahb2rstclrr::R](rcc_ahb2rstclrr::R) reader structure"]
impl crate::Readable for RCC_AHB2RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb2rstclrr::W](rcc_ahb2rstclrr::W) writer structure"]
impl crate::Writable for RCC_AHB2RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb2rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb3rstsetr](rcc_ahb3rstsetr) module"]
pub type RCC_AHB3RSTSETR = crate::Reg<u32, _RCC_AHB3RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB3RSTSETR;
#[doc = "`read()` method returns [rcc_ahb3rstsetr::R](rcc_ahb3rstsetr::R) reader structure"]
impl crate::Readable for RCC_AHB3RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb3rstsetr::W](rcc_ahb3rstsetr::W) writer structure"]
impl crate::Writable for RCC_AHB3RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral."]
pub mod rcc_ahb3rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb3rstclrr](rcc_ahb3rstclrr) module"]
pub type RCC_AHB3RSTCLRR = crate::Reg<u32, _RCC_AHB3RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB3RSTCLRR;
#[doc = "`read()` method returns [rcc_ahb3rstclrr::R](rcc_ahb3rstclrr::R) reader structure"]
impl crate::Readable for RCC_AHB3RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb3rstclrr::W](rcc_ahb3rstclrr::W) writer structure"]
impl crate::Writable for RCC_AHB3RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb3rstclrr;
#[doc = "This register is used to activate the reset of the corresponding peripheral\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb4rstsetr](rcc_ahb4rstsetr) module"]
pub type RCC_AHB4RSTSETR = crate::Reg<u32, _RCC_AHB4RSTSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB4RSTSETR;
#[doc = "`read()` method returns [rcc_ahb4rstsetr::R](rcc_ahb4rstsetr::R) reader structure"]
impl crate::Readable for RCC_AHB4RSTSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb4rstsetr::W](rcc_ahb4rstsetr::W) writer structure"]
impl crate::Writable for RCC_AHB4RSTSETR {}
#[doc = "This register is used to activate the reset of the corresponding peripheral"]
pub mod rcc_ahb4rstsetr;
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb4rstclrr](rcc_ahb4rstclrr) module"]
pub type RCC_AHB4RSTCLRR = crate::Reg<u32, _RCC_AHB4RSTCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_AHB4RSTCLRR;
#[doc = "`read()` method returns [rcc_ahb4rstclrr::R](rcc_ahb4rstclrr::R) reader structure"]
impl crate::Readable for RCC_AHB4RSTCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_ahb4rstclrr::W](rcc_ahb4rstclrr::W) writer structure"]
impl crate::Writable for RCC_AHB4RSTCLRR {}
#[doc = "This register is used to release the reset of the corresponding peripheral."]
pub mod rcc_ahb4rstclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb1ensetr](rcc_mp_apb1ensetr) module"]
pub type RCC_MP_APB1ENSETR = crate::Reg<u32, _RCC_MP_APB1ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB1ENSETR;
#[doc = "`read()` method returns [rcc_mp_apb1ensetr::R](rcc_mp_apb1ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB1ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb1ensetr::W](rcc_mp_apb1ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB1ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb1ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb1enclrr](rcc_mp_apb1enclrr) module"]
pub type RCC_MP_APB1ENCLRR = crate::Reg<u32, _RCC_MP_APB1ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB1ENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb1enclrr::R](rcc_mp_apb1enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB1ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb1enclrr::W](rcc_mp_apb1enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB1ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mp_apb1enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2ensetr](rcc_mp_apb2ensetr) module"]
pub type RCC_MP_APB2ENSETR = crate::Reg<u32, _RCC_MP_APB2ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB2ENSETR;
#[doc = "`read()` method returns [rcc_mp_apb2ensetr::R](rcc_mp_apb2ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB2ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2ensetr::W](rcc_mp_apb2ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB2ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb2ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2enclrr](rcc_mp_apb2enclrr) module"]
pub type RCC_MP_APB2ENCLRR = crate::Reg<u32, _RCC_MP_APB2ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB2ENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb2enclrr::R](rcc_mp_apb2enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB2ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2enclrr::W](rcc_mp_apb2enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB2ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_apb2enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb3ensetr](rcc_mp_apb3ensetr) module"]
pub type RCC_MP_APB3ENSETR = crate::Reg<u32, _RCC_MP_APB3ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB3ENSETR;
#[doc = "`read()` method returns [rcc_mp_apb3ensetr::R](rcc_mp_apb3ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB3ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb3ensetr::W](rcc_mp_apb3ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB3ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_apb3ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb3enclrr](rcc_mp_apb3enclrr) module"]
pub type RCC_MP_APB3ENCLRR = crate::Reg<u32, _RCC_MP_APB3ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB3ENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb3enclrr::R](rcc_mp_apb3enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB3ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb3enclrr::W](rcc_mp_apb3enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB3ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_apb3enclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb2ensetr](rcc_mp_ahb2ensetr) module"]
pub type RCC_MP_AHB2ENSETR = crate::Reg<u32, _RCC_MP_AHB2ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB2ENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb2ensetr::R](rcc_mp_ahb2ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB2ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb2ensetr::W](rcc_mp_ahb2ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB2ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
pub mod rcc_mp_ahb2ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb2enclrr](rcc_mp_ahb2enclrr) module"]
pub type RCC_MP_AHB2ENCLRR = crate::Reg<u32, _RCC_MP_AHB2ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB2ENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb2enclrr::R](rcc_mp_ahb2enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB2ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb2enclrr::W](rcc_mp_ahb2enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB2ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_ahb2enclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb3ensetr](rcc_mp_ahb3ensetr) module"]
pub type RCC_MP_AHB3ENSETR = crate::Reg<u32, _RCC_MP_AHB3ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB3ENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb3ensetr::R](rcc_mp_ahb3ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB3ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb3ensetr::W](rcc_mp_ahb3ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB3ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral"]
pub mod rcc_mp_ahb3ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb3enclrr](rcc_mp_ahb3enclrr) module"]
pub type RCC_MP_AHB3ENCLRR = crate::Reg<u32, _RCC_MP_AHB3ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB3ENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb3enclrr::R](rcc_mp_ahb3enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB3ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb3enclrr::W](rcc_mp_ahb3enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB3ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mp_ahb3enclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb4ensetr](rcc_mp_ahb4ensetr) module"]
pub type RCC_MP_AHB4ENSETR = crate::Reg<u32, _RCC_MP_AHB4ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB4ENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb4ensetr::R](rcc_mp_ahb4ensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB4ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb4ensetr::W](rcc_mp_ahb4ensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB4ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU."]
pub mod rcc_mp_ahb4ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb4enclrr](rcc_mp_ahb4enclrr) module"]
pub type RCC_MP_AHB4ENCLRR = crate::Reg<u32, _RCC_MP_AHB4ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB4ENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb4enclrr::R](rcc_mp_ahb4enclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB4ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb4enclrr::W](rcc_mp_ahb4enclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB4ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mp_ahb4enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_mlahbensetr](rcc_mp_mlahbensetr) module"]
pub type RCC_MP_MLAHBENSETR = crate::Reg<u32, _RCC_MP_MLAHBENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_MLAHBENSETR;
#[doc = "`read()` method returns [rcc_mp_mlahbensetr::R](rcc_mp_mlahbensetr::R) reader structure"]
impl crate::Readable for RCC_MP_MLAHBENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_mlahbensetr::W](rcc_mp_mlahbensetr::W) writer structure"]
impl crate::Writable for RCC_MP_MLAHBENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mp_mlahbensetr;
#[doc = "This register is used to clear the peripheral clock enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_mlahbenclrr](rcc_mp_mlahbenclrr) module"]
pub type RCC_MP_MLAHBENCLRR = crate::Reg<u32, _RCC_MP_MLAHBENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_MLAHBENCLRR;
#[doc = "`read()` method returns [rcc_mp_mlahbenclrr::R](rcc_mp_mlahbenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_MLAHBENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_mlahbenclrr::W](rcc_mp_mlahbenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_MLAHBENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit."]
pub mod rcc_mp_mlahbenclrr;
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb1ensetr](rcc_mc_apb1ensetr) module"]
pub type RCC_MC_APB1ENSETR = crate::Reg<u32, _RCC_MC_APB1ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB1ENSETR;
#[doc = "`read()` method returns [rcc_mc_apb1ensetr::R](rcc_mc_apb1ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB1ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb1ensetr::W](rcc_mc_apb1ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB1ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to ."]
pub mod rcc_mc_apb1ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb1enclrr](rcc_mc_apb1enclrr) module"]
pub type RCC_MC_APB1ENCLRR = crate::Reg<u32, _RCC_MC_APB1ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB1ENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb1enclrr::R](rcc_mc_apb1enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB1ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb1enclrr::W](rcc_mc_apb1enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB1ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral."]
pub mod rcc_mc_apb1enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb2ensetr](rcc_mc_apb2ensetr) module"]
pub type RCC_MC_APB2ENSETR = crate::Reg<u32, _RCC_MC_APB2ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB2ENSETR;
#[doc = "`read()` method returns [rcc_mc_apb2ensetr::R](rcc_mc_apb2ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB2ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb2ensetr::W](rcc_mc_apb2ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB2ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb2ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb2enclrr](rcc_mc_apb2enclrr) module"]
pub type RCC_MC_APB2ENCLRR = crate::Reg<u32, _RCC_MC_APB2ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB2ENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb2enclrr::R](rcc_mc_apb2enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB2ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb2enclrr::W](rcc_mc_apb2enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB2ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb2enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb3ensetr](rcc_mc_apb3ensetr) module"]
pub type RCC_MC_APB3ENSETR = crate::Reg<u32, _RCC_MC_APB3ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB3ENSETR;
#[doc = "`read()` method returns [rcc_mc_apb3ensetr::R](rcc_mc_apb3ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB3ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb3ensetr::W](rcc_mc_apb3ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB3ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_apb3ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb3enclrr](rcc_mc_apb3enclrr) module"]
pub type RCC_MC_APB3ENCLRR = crate::Reg<u32, _RCC_MC_APB3ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB3ENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb3enclrr::R](rcc_mc_apb3enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB3ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb3enclrr::W](rcc_mc_apb3enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB3ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_apb3enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb2ensetr](rcc_mc_ahb2ensetr) module"]
pub type RCC_MC_AHB2ENSETR = crate::Reg<u32, _RCC_MC_AHB2ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB2ENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb2ensetr::R](rcc_mc_ahb2ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB2ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb2ensetr::W](rcc_mc_ahb2ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB2ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb2ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb2enclrr](rcc_mc_ahb2enclrr) module"]
pub type RCC_MC_AHB2ENCLRR = crate::Reg<u32, _RCC_MC_AHB2ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB2ENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb2enclrr::R](rcc_mc_ahb2enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB2ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb2enclrr::W](rcc_mc_ahb2enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB2ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb2enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb3ensetr](rcc_mc_ahb3ensetr) module"]
pub type RCC_MC_AHB3ENSETR = crate::Reg<u32, _RCC_MC_AHB3ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB3ENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb3ensetr::R](rcc_mc_ahb3ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB3ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb3ensetr::W](rcc_mc_ahb3ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB3ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb3ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb3enclrr](rcc_mc_ahb3enclrr) module"]
pub type RCC_MC_AHB3ENCLRR = crate::Reg<u32, _RCC_MC_AHB3ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB3ENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb3enclrr::R](rcc_mc_ahb3enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB3ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb3enclrr::W](rcc_mc_ahb3enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB3ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb3enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb4ensetr](rcc_mc_ahb4ensetr) module"]
pub type RCC_MC_AHB4ENSETR = crate::Reg<u32, _RCC_MC_AHB4ENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB4ENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb4ensetr::R](rcc_mc_ahb4ensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB4ENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb4ensetr::W](rcc_mc_ahb4ensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB4ENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_ahb4ensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb4enclrr](rcc_mc_ahb4enclrr) module"]
pub type RCC_MC_AHB4ENCLRR = crate::Reg<u32, _RCC_MC_AHB4ENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB4ENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb4enclrr::R](rcc_mc_ahb4enclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB4ENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb4enclrr::W](rcc_mc_ahb4enclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB4ENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_ahb4enclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_aximensetr](rcc_mc_aximensetr) module"]
pub type RCC_MC_AXIMENSETR = crate::Reg<u32, _RCC_MC_AXIMENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AXIMENSETR;
#[doc = "`read()` method returns [rcc_mc_aximensetr::R](rcc_mc_aximensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AXIMENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_aximensetr::W](rcc_mc_aximensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AXIMENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_aximensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_aximenclrr](rcc_mc_aximenclrr) module"]
pub type RCC_MC_AXIMENCLRR = crate::Reg<u32, _RCC_MC_AXIMENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AXIMENCLRR;
#[doc = "`read()` method returns [rcc_mc_aximenclrr::R](rcc_mc_aximenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AXIMENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_aximenclrr::W](rcc_mc_aximenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AXIMENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_aximenclrr;
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_mlahbensetr](rcc_mc_mlahbensetr) module"]
pub type RCC_MC_MLAHBENSETR = crate::Reg<u32, _RCC_MC_MLAHBENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_MLAHBENSETR;
#[doc = "`read()` method returns [rcc_mc_mlahbensetr::R](rcc_mc_mlahbensetr::R) reader structure"]
impl crate::Readable for RCC_MC_MLAHBENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_mlahbensetr::W](rcc_mc_mlahbensetr::W) writer structure"]
impl crate::Writable for RCC_MC_MLAHBENSETR {}
#[doc = "This register is used to set the peripheral clock enable bit"]
pub mod rcc_mc_mlahbensetr;
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_mlahbenclrr](rcc_mc_mlahbenclrr) module"]
pub type RCC_MC_MLAHBENCLRR = crate::Reg<u32, _RCC_MC_MLAHBENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_MLAHBENCLRR;
#[doc = "`read()` method returns [rcc_mc_mlahbenclrr::R](rcc_mc_mlahbenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_MLAHBENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_mlahbenclrr::W](rcc_mc_mlahbenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_MLAHBENCLRR {}
#[doc = "This register is used to clear the peripheral clock enable bit"]
pub mod rcc_mc_mlahbenclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb1lpensetr](rcc_mp_apb1lpensetr) module"]
pub type RCC_MP_APB1LPENSETR = crate::Reg<u32, _RCC_MP_APB1LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB1LPENSETR;
#[doc = "`read()` method returns [rcc_mp_apb1lpensetr::R](rcc_mp_apb1lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB1LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb1lpensetr::W](rcc_mp_apb1lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB1LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb1lpensetr;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bits .\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb1lpenclrr](rcc_mp_apb1lpenclrr) module"]
pub type RCC_MP_APB1LPENCLRR = crate::Reg<u32, _RCC_MP_APB1LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB1LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb1lpenclrr::R](rcc_mp_apb1lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB1LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb1lpenclrr::W](rcc_mp_apb1lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB1LPENCLRR {}
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bits ."]
pub mod rcc_mp_apb1lpenclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2lpensetr](rcc_mp_apb2lpensetr) module"]
pub type RCC_MP_APB2LPENSETR = crate::Reg<u32, _RCC_MP_APB2LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB2LPENSETR;
#[doc = "`read()` method returns [rcc_mp_apb2lpensetr::R](rcc_mp_apb2lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB2LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2lpensetr::W](rcc_mp_apb2lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB2LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb2lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2lpenclrr](rcc_mp_apb2lpenclrr) module"]
pub type RCC_MP_APB2LPENCLRR = crate::Reg<u32, _RCC_MP_APB2LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB2LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb2lpenclrr::R](rcc_mp_apb2lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB2LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2lpenclrr::W](rcc_mp_apb2lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB2LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb2lpenclrr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb3lpensetr](rcc_mp_apb3lpensetr) module"]
pub type RCC_MP_APB3LPENSETR = crate::Reg<u32, _RCC_MP_APB3LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB3LPENSETR;
#[doc = "`read()` method returns [rcc_mp_apb3lpensetr::R](rcc_mp_apb3lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_APB3LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb3lpensetr::W](rcc_mp_apb3lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_APB3LPENSETR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb3lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb3lpenclrr](rcc_mp_apb3lpenclrr) module"]
pub type RCC_MP_APB3LPENCLRR = crate::Reg<u32, _RCC_MP_APB3LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_APB3LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_apb3lpenclrr::R](rcc_mp_apb3lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_APB3LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb3lpenclrr::W](rcc_mp_apb3lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_APB3LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_apb3lpenclrr;
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb2lpensetr](rcc_mp_ahb2lpensetr) module"]
pub type RCC_MP_AHB2LPENSETR = crate::Reg<u32, _RCC_MP_AHB2LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB2LPENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb2lpensetr::R](rcc_mp_ahb2lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB2LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb2lpensetr::W](rcc_mp_ahb2lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB2LPENSETR {}
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit."]
pub mod rcc_mp_ahb2lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb2lpenclrr](rcc_mp_ahb2lpenclrr) module"]
pub type RCC_MP_AHB2LPENCLRR = crate::Reg<u32, _RCC_MP_AHB2LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB2LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb2lpenclrr::R](rcc_mp_ahb2lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB2LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb2lpenclrr::W](rcc_mp_ahb2lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB2LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mp_ahb2lpenclrr;
#[doc = "This register is used by the MPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb3lpensetr](rcc_mp_ahb3lpensetr) module"]
pub type RCC_MP_AHB3LPENSETR = crate::Reg<u32, _RCC_MP_AHB3LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB3LPENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb3lpensetr::R](rcc_mp_ahb3lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB3LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb3lpensetr::W](rcc_mp_ahb3lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB3LPENSETR {}
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb3lpensetr;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb3lpenclrr](rcc_mp_ahb3lpenclrr) module"]
pub type RCC_MP_AHB3LPENCLRR = crate::Reg<u32, _RCC_MP_AHB3LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB3LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb3lpenclrr::R](rcc_mp_ahb3lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB3LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb3lpenclrr::W](rcc_mp_ahb3lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB3LPENCLRR {}
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit"]
pub mod rcc_mp_ahb3lpenclrr;
#[doc = "This register is used by the MPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb4lpensetr](rcc_mp_ahb4lpensetr) module"]
pub type RCC_MP_AHB4LPENSETR = crate::Reg<u32, _RCC_MP_AHB4LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB4LPENSETR;
#[doc = "`read()` method returns [rcc_mp_ahb4lpensetr::R](rcc_mp_ahb4lpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB4LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb4lpensetr::W](rcc_mp_ahb4lpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB4LPENSETR {}
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb4lpensetr;
#[doc = "This register is used by the MPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb4lpenclrr](rcc_mp_ahb4lpenclrr) module"]
pub type RCC_MP_AHB4LPENCLRR = crate::Reg<u32, _RCC_MP_AHB4LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AHB4LPENCLRR;
#[doc = "`read()` method returns [rcc_mp_ahb4lpenclrr::R](rcc_mp_ahb4lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AHB4LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb4lpenclrr::W](rcc_mp_ahb4lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AHB4LPENCLRR {}
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_ahb4lpenclrr;
#[doc = "This register is used by the MPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_aximlpensetr](rcc_mp_aximlpensetr) module"]
pub type RCC_MP_AXIMLPENSETR = crate::Reg<u32, _RCC_MP_AXIMLPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AXIMLPENSETR;
#[doc = "`read()` method returns [rcc_mp_aximlpensetr::R](rcc_mp_aximlpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_AXIMLPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_aximlpensetr::W](rcc_mp_aximlpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_AXIMLPENSETR {}
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_aximlpensetr;
#[doc = "This register is used by the MPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_aximlpenclrr](rcc_mp_aximlpenclrr) module"]
pub type RCC_MP_AXIMLPENCLRR = crate::Reg<u32, _RCC_MP_AXIMLPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_AXIMLPENCLRR;
#[doc = "`read()` method returns [rcc_mp_aximlpenclrr::R](rcc_mp_aximlpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_AXIMLPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_aximlpenclrr::W](rcc_mp_aximlpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_AXIMLPENCLRR {}
#[doc = "This register is used by the MPU"]
pub mod rcc_mp_aximlpenclrr;
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_mlahblpensetr](rcc_mp_mlahblpensetr) module"]
pub type RCC_MP_MLAHBLPENSETR = crate::Reg<u32, _RCC_MP_MLAHBLPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_MLAHBLPENSETR;
#[doc = "`read()` method returns [rcc_mp_mlahblpensetr::R](rcc_mp_mlahblpensetr::R) reader structure"]
impl crate::Readable for RCC_MP_MLAHBLPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_mlahblpensetr::W](rcc_mp_mlahblpensetr::W) writer structure"]
impl crate::Writable for RCC_MP_MLAHBLPENSETR {}
#[doc = "This register is used by the MPU in order to set the PERxLPEN bit"]
pub mod rcc_mp_mlahblpensetr;
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_mlahblpenclrr](rcc_mp_mlahblpenclrr) module"]
pub type RCC_MP_MLAHBLPENCLRR = crate::Reg<u32, _RCC_MP_MLAHBLPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MP_MLAHBLPENCLRR;
#[doc = "`read()` method returns [rcc_mp_mlahblpenclrr::R](rcc_mp_mlahblpenclrr::R) reader structure"]
impl crate::Readable for RCC_MP_MLAHBLPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mp_mlahblpenclrr::W](rcc_mp_mlahblpenclrr::W) writer structure"]
impl crate::Writable for RCC_MP_MLAHBLPENCLRR {}
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit"]
pub mod rcc_mp_mlahblpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb1lpensetr](rcc_mc_apb1lpensetr) module"]
pub type RCC_MC_APB1LPENSETR = crate::Reg<u32, _RCC_MC_APB1LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB1LPENSETR;
#[doc = "`read()` method returns [rcc_mc_apb1lpensetr::R](rcc_mc_apb1lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB1LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb1lpensetr::W](rcc_mc_apb1lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB1LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb1lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb1lpenclrr](rcc_mc_apb1lpenclrr) module"]
pub type RCC_MC_APB1LPENCLRR = crate::Reg<u32, _RCC_MC_APB1LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB1LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb1lpenclrr::R](rcc_mc_apb1lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB1LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb1lpenclrr::W](rcc_mc_apb1lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB1LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits"]
pub mod rcc_mc_apb1lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb2lpensetr](rcc_mc_apb2lpensetr) module"]
pub type RCC_MC_APB2LPENSETR = crate::Reg<u32, _RCC_MC_APB2LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB2LPENSETR;
#[doc = "`read()` method returns [rcc_mc_apb2lpensetr::R](rcc_mc_apb2lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB2LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb2lpensetr::W](rcc_mc_apb2lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB2LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb2lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb2lpenclrr](rcc_mc_apb2lpenclrr) module"]
pub type RCC_MC_APB2LPENCLRR = crate::Reg<u32, _RCC_MC_APB2LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB2LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb2lpenclrr::R](rcc_mc_apb2lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB2LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb2lpenclrr::W](rcc_mc_apb2lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB2LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb2lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb3lpensetr](rcc_mc_apb3lpensetr) module"]
pub type RCC_MC_APB3LPENSETR = crate::Reg<u32, _RCC_MC_APB3LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB3LPENSETR;
#[doc = "`read()` method returns [rcc_mc_apb3lpensetr::R](rcc_mc_apb3lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_APB3LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb3lpensetr::W](rcc_mc_apb3lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_APB3LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_apb3lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb3lpenclrr](rcc_mc_apb3lpenclrr) module"]
pub type RCC_MC_APB3LPENCLRR = crate::Reg<u32, _RCC_MC_APB3LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_APB3LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_apb3lpenclrr::R](rcc_mc_apb3lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_APB3LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb3lpenclrr::W](rcc_mc_apb3lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_APB3LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_apb3lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb2lpensetr](rcc_mc_ahb2lpensetr) module"]
pub type RCC_MC_AHB2LPENSETR = crate::Reg<u32, _RCC_MC_AHB2LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB2LPENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb2lpensetr::R](rcc_mc_ahb2lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB2LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb2lpensetr::W](rcc_mc_ahb2lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB2LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb2lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb2lpenclrr](rcc_mc_ahb2lpenclrr) module"]
pub type RCC_MC_AHB2LPENCLRR = crate::Reg<u32, _RCC_MC_AHB2LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB2LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb2lpenclrr::R](rcc_mc_ahb2lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB2LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb2lpenclrr::W](rcc_mc_ahb2lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB2LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb2lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb3lpensetr](rcc_mc_ahb3lpensetr) module"]
pub type RCC_MC_AHB3LPENSETR = crate::Reg<u32, _RCC_MC_AHB3LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB3LPENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb3lpensetr::R](rcc_mc_ahb3lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB3LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb3lpensetr::W](rcc_mc_ahb3lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB3LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb3lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb3lpenclrr](rcc_mc_ahb3lpenclrr) module"]
pub type RCC_MC_AHB3LPENCLRR = crate::Reg<u32, _RCC_MC_AHB3LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB3LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb3lpenclrr::R](rcc_mc_ahb3lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB3LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb3lpenclrr::W](rcc_mc_ahb3lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB3LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit"]
pub mod rcc_mc_ahb3lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb4lpensetr](rcc_mc_ahb4lpensetr) module"]
pub type RCC_MC_AHB4LPENSETR = crate::Reg<u32, _RCC_MC_AHB4LPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB4LPENSETR;
#[doc = "`read()` method returns [rcc_mc_ahb4lpensetr::R](rcc_mc_ahb4lpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB4LPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb4lpensetr::W](rcc_mc_ahb4lpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB4LPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit."]
pub mod rcc_mc_ahb4lpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb4lpenclrr](rcc_mc_ahb4lpenclrr) module"]
pub type RCC_MC_AHB4LPENCLRR = crate::Reg<u32, _RCC_MC_AHB4LPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AHB4LPENCLRR;
#[doc = "`read()` method returns [rcc_mc_ahb4lpenclrr::R](rcc_mc_ahb4lpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AHB4LPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb4lpenclrr::W](rcc_mc_ahb4lpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AHB4LPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_ahb4lpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_aximlpensetr](rcc_mc_aximlpensetr) module"]
pub type RCC_MC_AXIMLPENSETR = crate::Reg<u32, _RCC_MC_AXIMLPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AXIMLPENSETR;
#[doc = "`read()` method returns [rcc_mc_aximlpensetr::R](rcc_mc_aximlpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_AXIMLPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_aximlpensetr::W](rcc_mc_aximlpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_AXIMLPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_aximlpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_aximlpenclrr](rcc_mc_aximlpenclrr) module"]
pub type RCC_MC_AXIMLPENCLRR = crate::Reg<u32, _RCC_MC_AXIMLPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_AXIMLPENCLRR;
#[doc = "`read()` method returns [rcc_mc_aximlpenclrr::R](rcc_mc_aximlpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_AXIMLPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_aximlpenclrr::W](rcc_mc_aximlpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_AXIMLPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_aximlpenclrr;
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_mlahblpensetr](rcc_mc_mlahblpensetr) module"]
pub type RCC_MC_MLAHBLPENSETR = crate::Reg<u32, _RCC_MC_MLAHBLPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_MLAHBLPENSETR;
#[doc = "`read()` method returns [rcc_mc_mlahblpensetr::R](rcc_mc_mlahblpensetr::R) reader structure"]
impl crate::Readable for RCC_MC_MLAHBLPENSETR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_mlahblpensetr::W](rcc_mc_mlahblpensetr::W) writer structure"]
impl crate::Writable for RCC_MC_MLAHBLPENSETR {}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_mlahblpensetr;
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_mlahblpenclrr](rcc_mc_mlahblpenclrr) module"]
pub type RCC_MC_MLAHBLPENCLRR = crate::Reg<u32, _RCC_MC_MLAHBLPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_MLAHBLPENCLRR;
#[doc = "`read()` method returns [rcc_mc_mlahblpenclrr::R](rcc_mc_mlahblpenclrr::R) reader structure"]
impl crate::Readable for RCC_MC_MLAHBLPENCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_mlahblpenclrr::W](rcc_mc_mlahblpenclrr::W) writer structure"]
impl crate::Writable for RCC_MC_MLAHBLPENCLRR {}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral."]
pub mod rcc_mc_mlahblpenclrr;
#[doc = "This register is used by the MCU to check the reset source.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_rstsclrr](rcc_mc_rstsclrr) module"]
pub type RCC_MC_RSTSCLRR = crate::Reg<u32, _RCC_MC_RSTSCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_RSTSCLRR;
#[doc = "`read()` method returns [rcc_mc_rstsclrr::R](rcc_mc_rstsclrr::R) reader structure"]
impl crate::Readable for RCC_MC_RSTSCLRR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_rstsclrr::W](rcc_mc_rstsclrr::W) writer structure"]
impl crate::Writable for RCC_MC_RSTSCLRR {}
#[doc = "This register is used by the MCU to check the reset source."]
pub mod rcc_mc_rstsclrr;
#[doc = "This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_cier](rcc_mc_cier) module"]
pub type RCC_MC_CIER = crate::Reg<u32, _RCC_MC_CIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_CIER;
#[doc = "`read()` method returns [rcc_mc_cier::R](rcc_mc_cier::R) reader structure"]
impl crate::Readable for RCC_MC_CIER {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_cier::W](rcc_mc_cier::W) writer structure"]
impl crate::Writable for RCC_MC_CIER {}
#[doc = "This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details."]
pub mod rcc_mc_cier;
#[doc = "This register shall be used by the MCU in order to read and clear the interrupt flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_cifr](rcc_mc_cifr) module"]
pub type RCC_MC_CIFR = crate::Reg<u32, _RCC_MC_CIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_MC_CIFR;
#[doc = "`read()` method returns [rcc_mc_cifr::R](rcc_mc_cifr::R) reader structure"]
impl crate::Readable for RCC_MC_CIFR {}
#[doc = "`write(|w| ..)` method takes [rcc_mc_cifr::W](rcc_mc_cifr::W) writer structure"]
impl crate::Writable for RCC_MC_CIFR {}
#[doc = "This register shall be used by the MCU in order to read and clear the interrupt flags."]
pub mod rcc_mc_cifr;
#[doc = "This register gives the IP version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_verr](rcc_verr) module"]
pub type RCC_VERR = crate::Reg<u32, _RCC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_VERR;
#[doc = "`read()` method returns [rcc_verr::R](rcc_verr::R) reader structure"]
impl crate::Readable for RCC_VERR {}
#[doc = "This register gives the IP version"]
pub mod rcc_verr;
#[doc = "This register gives the unique identifier of the RCC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_idr](rcc_idr) module"]
pub type RCC_IDR = crate::Reg<u32, _RCC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_IDR;
#[doc = "`read()` method returns [rcc_idr::R](rcc_idr::R) reader structure"]
impl crate::Readable for RCC_IDR {}
#[doc = "This register gives the unique identifier of the RCC"]
pub mod rcc_idr;
#[doc = "This register gives the decoding space, which is for the RCC of 4 kB.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_sidr](rcc_sidr) module"]
pub type RCC_SIDR = crate::Reg<u32, _RCC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC_SIDR;
#[doc = "`read()` method returns [rcc_sidr::R](rcc_sidr::R) reader structure"]
impl crate::Readable for RCC_SIDR {}
#[doc = "This register gives the decoding space, which is for the RCC of 4 kB."]
pub mod rcc_sidr;
