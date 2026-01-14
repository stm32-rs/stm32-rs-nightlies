///Register `APB7RSTR` reader
pub type R = crate::R<APB7RSTRrs>;
///Register `APB7RSTR` writer
pub type W = crate::W<APB7RSTRrs>;
///Field `SYSCFGRST` reader - SYSCFG reset Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - SYSCFG reset Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3RST` reader - SPI3 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SPI3RST_R = crate::BitReader;
///Field `SPI3RST` writer - SPI3 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1RST` reader - LPUART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPUART1RST_R = crate::BitReader;
///Field `LPUART1RST` writer - LPUART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3RST` reader - I2C3 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C3RST_R = crate::BitReader;
///Field `I2C3RST` writer - I2C3 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1RST` reader - LPTIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPTIM1RST_R = crate::BitReader;
///Field `LPTIM1RST` writer - LPTIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type LPTIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - SYSCFG reset Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB7RSTR")
            .field("syscfgrst", &self.syscfgrst())
            .field("spi3rst", &self.spi3rst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c3rst", &self.i2c3rst())
            .field("lptim1rst", &self.lptim1rst())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG reset Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB7RSTRrs> {
        SYSCFGRST_W::new(self, 1)
    }
    ///Bit 5 - SPI3 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<'_, APB7RSTRrs> {
        SPI3RST_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB7RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    ///Bit 7 - I2C3 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<'_, APB7RSTRrs> {
        I2C3RST_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APB7RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
}
/**RCC APB7 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb7rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb7rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RCC:APB7RSTR)*/
pub struct APB7RSTRrs;
impl crate::RegisterSpec for APB7RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb7rstr::R`](R) reader structure
impl crate::Readable for APB7RSTRrs {}
///`write(|w| ..)` method takes [`apb7rstr::W`](W) writer structure
impl crate::Writable for APB7RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB7RSTR to value 0
impl crate::Resettable for APB7RSTRrs {}
