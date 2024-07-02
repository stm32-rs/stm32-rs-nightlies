///Register `RCC_CCIPR1` reader
pub type R = crate::R<RCC_CCIPR1rs>;
///Register `RCC_CCIPR1` writer
pub type W = crate::W<RCC_CCIPR1rs>;
///Field `USART1SEL` reader - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_R = crate::FieldReader;
///Field `USART1SEL` writer - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USART2SEL` reader - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type USART2SEL_R = crate::FieldReader;
///Field `USART2SEL` writer - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USART3SEL` reader - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type USART3SEL_R = crate::FieldReader;
///Field `USART3SEL` writer - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UART4SEL` reader - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type UART4SEL_R = crate::FieldReader;
///Field `UART4SEL` writer - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type UART4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UART5SEL` reader - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type UART5SEL_R = crate::FieldReader;
///Field `UART5SEL` writer - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
pub type UART5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C1SEL` reader - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub type I2C1SEL_R = crate::FieldReader;
///Field `I2C1SEL` writer - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C2SEL` reader - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub type I2C2SEL_R = crate::FieldReader;
///Field `I2C2SEL` writer - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
pub type I2C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C4SEL` reader - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
pub type I2C4SEL_R = crate::FieldReader;
///Field `I2C4SEL` writer - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
pub type I2C4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPI2SEL` reader - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI2SEL_R = crate::FieldReader;
///Field `SPI2SEL` writer - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM2SEL` reader - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_R = crate::FieldReader;
///Field `LPTIM2SEL` writer - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPI1SEL` reader - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI1SEL_R = crate::FieldReader;
///Field `SPI1SEL` writer - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SYSTICKSEL` reader - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
pub type SYSTICKSEL_R = crate::FieldReader;
///Field `SYSTICKSEL` writer - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
pub type SYSTICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FDCAN1SEL` reader - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
pub type FDCAN1SEL_R = crate::FieldReader;
///Field `FDCAN1SEL` writer - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
pub type FDCAN1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ICLKSEL` reader - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
pub type ICLKSEL_R = crate::FieldReader;
///Field `ICLKSEL` writer - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
pub type ICLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `TIMICSEL` reader - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\]
value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.*/
pub type TIMICSEL_R = crate::FieldReader;
/**Field `TIMICSEL` writer - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\]
value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.*/
pub type TIMICSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:1 - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
    #[inline(always)]
    pub fn fdcan1sel(&self) -> FDCAN1SEL_R {
        FDCAN1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
    #[inline(always)]
    pub fn iclksel(&self) -> ICLKSEL_R {
        ICLKSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    /**Bits 29:31 - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\]
    value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.*/
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CCIPR1")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("usart3sel", &self.usart3sel())
            .field("uart4sel", &self.uart4sel())
            .field("uart5sel", &self.uart5sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("i2c4sel", &self.i2c4sel())
            .field("spi2sel", &self.spi2sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("spi1sel", &self.spi1sel())
            .field("systicksel", &self.systicksel())
            .field("fdcan1sel", &self.fdcan1sel())
            .field("iclksel", &self.iclksel())
            .field("timicsel", &self.timicsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 kernel clock source selection These bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<RCC_CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - USART2 kernel clock source selection These bits are used to select the USART2 kernel clock source. The USART2 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<RCC_CCIPR1rs> {
        USART2SEL_W::new(self, 2)
    }
    ///Bits 4:5 - USART3 kernel clock source selection These bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<RCC_CCIPR1rs> {
        USART3SEL_W::new(self, 4)
    }
    ///Bits 6:7 - UART4 kernel clock source selection These bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<RCC_CCIPR1rs> {
        UART4SEL_W::new(self, 6)
    }
    ///Bits 8:9 - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<RCC_CCIPR1rs> {
        UART5SEL_W::new(self, 8)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<RCC_CCIPR1rs> {
        I2C1SEL_W::new(self, 10)
    }
    ///Bits 12:13 - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode sonly when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<RCC_CCIPR1rs> {
        I2C2SEL_W::new(self, 12)
    }
    ///Bits 14:15 - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<RCC_CCIPR1rs> {
        I2C4SEL_W::new(self, 14)
    }
    ///Bits 16:17 - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<RCC_CCIPR1rs> {
        SPI2SEL_W::new(self, 16)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<RCC_CCIPR1rs> {
        LPTIM2SEL_W::new(self, 18)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<RCC_CCIPR1rs> {
        SPI1SEL_W::new(self, 20)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
    #[inline(always)]
    #[must_use]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<RCC_CCIPR1rs> {
        SYSTICKSEL_W::new(self, 22)
    }
    ///Bits 24:25 - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
    #[inline(always)]
    #[must_use]
    pub fn fdcan1sel(&mut self) -> FDCAN1SEL_W<RCC_CCIPR1rs> {
        FDCAN1SEL_W::new(self, 24)
    }
    ///Bits 26:27 - Intermediate clock source selection These bits are used to select the clock source for the OTG_FS, the USB, and the SDMMC.
    #[inline(always)]
    #[must_use]
    pub fn iclksel(&mut self) -> ICLKSEL_W<RCC_CCIPR1rs> {
        ICLKSEL_W::new(self, 26)
    }
    /**Bits 29:31 - Clock sources for TIM16,TIM17, and LPTIM2 internal input capture When TIMICSEL2 is set, the TIM16, TIM17, and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4, or MSI/1024. Depending on TIMICSEL\[1:0\]
    value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK, and MSIS clock sources cannot be selected as�TIM16, TIM17, or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.*/
    #[inline(always)]
    #[must_use]
    pub fn timicsel(&mut self) -> TIMICSEL_W<RCC_CCIPR1rs> {
        TIMICSEL_W::new(self, 29)
    }
}
/**RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:RCC_CCIPR1)*/
pub struct RCC_CCIPR1rs;
impl crate::RegisterSpec for RCC_CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ccipr1::R`](R) reader structure
impl crate::Readable for RCC_CCIPR1rs {}
///`write(|w| ..)` method takes [`rcc_ccipr1::W`](W) writer structure
impl crate::Writable for RCC_CCIPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CCIPR1 to value 0
impl crate::Resettable for RCC_CCIPR1rs {
    const RESET_VALUE: u32 = 0;
}
