///Register `APB1ENR1` reader
pub type R = crate::R<APB1ENR1rs>;
///Register `APB1ENR1` writer
pub type W = crate::W<APB1ENR1rs>;
///Field `TIM2EN` reader - TIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - TIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3EN` reader - TIM3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - WWDG bus clock enable Set by software to enable the window watchdog bus clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - WWDG bus clock enable Set by software to enable the window watchdog bus clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 11 - WWDG bus clock enable Set by software to enable the window watchdog bus clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - USART2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR1")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("wwdgen", &self.wwdgen())
            .field("usart2en", &self.usart2en())
            .field("i2c1en", &self.i2c1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1ENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 11 - WWDG bus clock enable Set by software to enable the window watchdog bus clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 17 - USART2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
}
/**RCC APB1 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1ENR1)*/
pub struct APB1ENR1rs;
impl crate::RegisterSpec for APB1ENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr1::R`](R) reader structure
impl crate::Readable for APB1ENR1rs {}
///`write(|w| ..)` method takes [`apb1enr1::W`](W) writer structure
impl crate::Writable for APB1ENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR1 to value 0
impl crate::Resettable for APB1ENR1rs {}
