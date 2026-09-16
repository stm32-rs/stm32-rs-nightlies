///Register `APB7ENR` reader
pub type R = crate::R<APB7ENRrs>;
///Register `APB7ENR` writer
pub type W = crate::W<APB7ENRrs>;
///Field `SYSCFGEN` reader - SYSCFG bus clock enable Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSCFG bus clock enable Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3EN` reader - SPI3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SPI3EN_R = crate::BitReader;
///Field `SPI3EN` writer - SPI3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1EN` reader - LPUART1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPUART1EN_R = crate::BitReader;
///Field `LPUART1EN` writer - LPUART1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3EN` reader - I2C3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C3EN_R = crate::BitReader;
///Field `I2C3EN` writer - I2C3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1EN` reader - LPTIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPTIM1EN_R = crate::BitReader;
///Field `LPTIM1EN` writer - LPTIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTC and TAMP bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTC and TAMP bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - SYSCFG bus clock enable Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB7ENR")
            .field("syscfgen", &self.syscfgen())
            .field("spi3en", &self.spi3en())
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c3en", &self.i2c3en())
            .field("lptim1en", &self.lptim1en())
            .field("rtcapben", &self.rtcapben())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG bus clock enable Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB7ENRrs> {
        SYSCFGEN_W::new(self, 1)
    }
    ///Bit 5 - SPI3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB7ENRrs> {
        SPI3EN_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APB7ENRrs> {
        LPUART1EN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB7ENRrs> {
        I2C3EN_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APB7ENRrs> {
        LPTIM1EN_W::new(self, 11)
    }
    ///Bit 21 - RTC and TAMP bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APB7ENRrs> {
        RTCAPBEN_W::new(self, 21)
    }
}
/**RCC APB7 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb7enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb7enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB7ENR)*/
pub struct APB7ENRrs;
impl crate::RegisterSpec for APB7ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb7enr::R`](R) reader structure
impl crate::Readable for APB7ENRrs {}
///`write(|w| ..)` method takes [`apb7enr::W`](W) writer structure
impl crate::Writable for APB7ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB7ENR to value 0
impl crate::Resettable for APB7ENRrs {}
