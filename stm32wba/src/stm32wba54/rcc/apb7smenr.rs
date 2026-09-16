///Register `APB7SMENR` reader
pub type R = crate::R<APB7SMENRrs>;
///Register `APB7SMENR` writer
pub type W = crate::W<APB7SMENRrs>;
///Field `SYSCFGSMEN` reader - SYSCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SYSCFGSMEN_R = crate::BitReader;
///Field `SYSCFGSMEN` writer - SYSCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3SMEN` reader - SPI3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3SMEN_R = crate::BitReader;
///Field `SPI3SMEN` writer - SPI3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1SMEN` reader - LPUART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPUART1SMEN_R = crate::BitReader;
///Field `LPUART1SMEN` writer - LPUART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3SMEN` reader - I2C3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C3SMEN_R = crate::BitReader;
///Field `I2C3SMEN` writer - I2C3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1SMEN` reader - LPTIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM1SMEN_R = crate::BitReader;
///Field `LPTIM1SMEN` writer - LPTIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBSMEN` reader - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type RTCAPBSMEN_R = crate::BitReader;
///Field `RTCAPBSMEN` writer - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - SYSCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB7SMENR")
            .field("syscfgsmen", &self.syscfgsmen())
            .field("spi3smen", &self.spi3smen())
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c3smen", &self.i2c3smen())
            .field("lptim1smen", &self.lptim1smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<'_, APB7SMENRrs> {
        SYSCFGSMEN_W::new(self, 1)
    }
    ///Bit 5 - SPI3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<'_, APB7SMENRrs> {
        SPI3SMEN_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, APB7SMENRrs> {
        LPUART1SMEN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APB7SMENRrs> {
        I2C3SMEN_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APB7SMENRrs> {
        LPTIM1SMEN_W::new(self, 11)
    }
    ///Bit 21 - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, APB7SMENRrs> {
        RTCAPBSMEN_W::new(self, 21)
    }
}
/**RCC APB7 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`apb7smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb7smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB7SMENR)*/
pub struct APB7SMENRrs;
impl crate::RegisterSpec for APB7SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb7smenr::R`](R) reader structure
impl crate::Readable for APB7SMENRrs {}
///`write(|w| ..)` method takes [`apb7smenr::W`](W) writer structure
impl crate::Writable for APB7SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB7SMENR to value 0xffff_ffff
impl crate::Resettable for APB7SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
