///Register `APB1SMENR1` reader
pub type R = crate::R<APB1SMENR1rs>;
///Register `APB1SMENR1` writer
pub type W = crate::W<APB1SMENR1rs>;
///Field `TIM2SMEN` reader - TIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM2SMEN_R = crate::BitReader;
///Field `TIM2SMEN` writer - TIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3SMEN` reader - TIM3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM3SMEN_R = crate::BitReader;
///Field `TIM3SMEN` writer - TIM3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGSMEN` reader - Window watchdog bus clock enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type WWDGSMEN_R = crate::BitReader;
///Field `WWDGSMEN` writer - Window watchdog bus clock enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2SMEN` reader - USART2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART2SMEN_R = crate::BitReader;
///Field `USART2SMEN` writer - USART2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SMEN` reader - I2C1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C1SMEN_R = crate::BitReader;
///Field `I2C1SMEN` writer - I2C1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 11 - Window watchdog bus clock enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - USART2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("tim3smen", &self.tim3smen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("usart2smen", &self.usart2smen())
            .field("i2c1smen", &self.i2c1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, APB1SMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 11 - Window watchdog bus clock enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 17 - USART2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
}
/**RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:APB1SMENR1)*/
pub struct APB1SMENR1rs;
impl crate::RegisterSpec for APB1SMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr1::R`](R) reader structure
impl crate::Readable for APB1SMENR1rs {}
///`write(|w| ..)` method takes [`apb1smenr1::W`](W) writer structure
impl crate::Writable for APB1SMENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR1 to value 0xffff_ffff
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
