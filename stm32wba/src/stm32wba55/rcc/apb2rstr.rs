///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
///Field `TIM1RST` reader - TIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM1RST_R = crate::BitReader;
///Field `TIM1RST` writer - TIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RST` reader - SPI1 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SPI1RST_R = crate::BitReader;
///Field `SPI1RST` writer - SPI1 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1RST` reader - USART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type USART1RST_R = crate::BitReader;
///Field `USART1RST` writer - USART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RST` reader - TIM16 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM16RST_R = crate::BitReader;
///Field `TIM16RST` writer - TIM16 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17RST` reader - TIM17 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM17RST_R = crate::BitReader;
///Field `TIM17RST` writer - TIM17 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - TIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - TIM16 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("tim1rst", &self.tim1rst())
            .field("spi1rst", &self.spi1rst())
            .field("usart1rst", &self.usart1rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI1 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 14 - USART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 17 - TIM16 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM17 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
}
/**RCC APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RCC:APB2RSTR)*/
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2rstr::R`](R) reader structure
impl crate::Readable for APB2RSTRrs {}
///`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTR to value 0
impl crate::Resettable for APB2RSTRrs {}
