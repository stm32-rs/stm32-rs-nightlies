///Register `APB1RSTR1` reader
pub type R = crate::R<APB1RSTR1rs>;
///Register `APB1RSTR1` writer
pub type W = crate::W<APB1RSTR1rs>;
///Field `TIM2RST` reader - TIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM2RST_R = crate::BitReader;
///Field `TIM2RST` writer - TIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3RST` reader - TIM3 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM3RST_R = crate::BitReader;
///Field `TIM3RST` writer - TIM3 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2RST` reader - USART2 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type USART2RST_R = crate::BitReader;
///Field `USART2RST` writer - USART2 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type USART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1RST` reader - I2C1 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C1RST_R = crate::BitReader;
///Field `I2C1RST` writer - I2C1 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 17 - USART2 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR1")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("usart2rst", &self.usart2rst())
            .field("i2c1rst", &self.i2c1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, APB1RSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM3 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<'_, APB1RSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 17 - USART2 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<'_, APB1RSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, APB1RSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
}
/**RCC APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1RSTR1)*/
pub struct APB1RSTR1rs;
impl crate::RegisterSpec for APB1RSTR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr1::R`](R) reader structure
impl crate::Readable for APB1RSTR1rs {}
///`write(|w| ..)` method takes [`apb1rstr1::W`](W) writer structure
impl crate::Writable for APB1RSTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR1 to value 0
impl crate::Resettable for APB1RSTR1rs {}
