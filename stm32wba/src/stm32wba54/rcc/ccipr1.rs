///Register `CCIPR1` reader
pub type R = crate::R<CCIPR1rs>;
///Register `CCIPR1` writer
pub type W = crate::W<CCIPR1rs>;
///Field `USART1SEL` reader - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_R = crate::FieldReader;
///Field `USART1SEL` writer - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USART2SEL` reader - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART2SEL_R = crate::FieldReader;
///Field `USART2SEL` writer - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C1SEL` reader - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
pub type I2C1SEL_R = crate::FieldReader;
///Field `I2C1SEL` writer - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM2SEL` reader - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_R = crate::FieldReader;
///Field `LPTIM2SEL` writer - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPI1SEL` reader - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
pub type SPI1SEL_R = crate::FieldReader;
///Field `SPI1SEL` writer - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SYSTICKSEL` reader - SysTick clock source selection These bits are used to select the SysTick clock source. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry.
pub type SYSTICKSEL_R = crate::FieldReader;
///Field `SYSTICKSEL` writer - SysTick clock source selection These bits are used to select the SysTick clock source. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry.
pub type SYSTICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIMICSEL` reader - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI16/256. When TIMICSEL is cleared, the HSI16, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division.
pub type TIMICSEL_R = crate::BitReader;
///Field `TIMICSEL` writer - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI16/256. When TIMICSEL is cleared, the HSI16, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division.
pub type TIMICSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry.
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 31 - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI16/256. When TIMICSEL is cleared, the HSI16, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division.
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR1")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("spi1sel", &self.spi1sel())
            .field("systicksel", &self.systicksel())
            .field("timicsel", &self.timicsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<'_, CCIPR1rs> {
        USART2SEL_W::new(self, 2)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, CCIPR1rs> {
        I2C1SEL_W::new(self, 10)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<'_, CCIPR1rs> {
        LPTIM2SEL_W::new(self, 18)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16.
    #[inline(always)]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<'_, CCIPR1rs> {
        SPI1SEL_W::new(self, 20)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry.
    #[inline(always)]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<'_, CCIPR1rs> {
        SYSTICKSEL_W::new(self, 22)
    }
    ///Bit 31 - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI16/256. When TIMICSEL is cleared, the HSI16, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division.
    #[inline(always)]
    pub fn timicsel(&mut self) -> TIMICSEL_W<'_, CCIPR1rs> {
        TIMICSEL_W::new(self, 31)
    }
}
/**RCC peripherals independent clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:CCIPR1)*/
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr1::R`](R) reader structure
impl crate::Readable for CCIPR1rs {}
///`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR1 to value 0
impl crate::Resettable for CCIPR1rs {}
