///Register `IER1` reader
pub type R = crate::R<IER1rs>;
///Register `IER1` writer
pub type W = crate::W<IER1rs>;
///Field `TIM2IE` reader - illegal access interrupt enable for TIM2
pub type TIM2IE_R = crate::BitReader;
///Field `TIM2IE` writer - illegal access interrupt enable for TIM2
pub type TIM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3IE` reader - illegal access interrupt enable for TIM3
pub type TIM3IE_R = crate::BitReader;
///Field `TIM3IE` writer - illegal access interrupt enable for TIM3
pub type TIM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGIE` reader - illegal access interrupt enable for WWDG
pub type WWDGIE_R = crate::BitReader;
///Field `WWDGIE` writer - illegal access interrupt enable for WWDG
pub type WWDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDGIE` reader - illegal access interrupt enable for IWDG
pub type IWDGIE_R = crate::BitReader;
///Field `IWDGIE` writer - illegal access interrupt enable for IWDG
pub type IWDGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2IE` reader - illegal access interrupt enable for USART2
pub type USART2IE_R = crate::BitReader;
///Field `USART2IE` writer - illegal access interrupt enable for USART2
pub type USART2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1IE` reader - illegal access interrupt enable for I2C1
pub type I2C1IE_R = crate::BitReader;
///Field `I2C1IE` writer - illegal access interrupt enable for I2C1
pub type I2C1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2IE` reader - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_R = crate::BitReader;
///Field `LPTIM2IE` writer - illegal access interrupt enable for LPTIM2
pub type LPTIM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for TIM2
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for TIM3
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for WWDG
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for IWDG
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for USART2
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for I2C1
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER1")
            .field("tim2ie", &self.tim2ie())
            .field("tim3ie", &self.tim3ie())
            .field("wwdgie", &self.wwdgie())
            .field("iwdgie", &self.iwdgie())
            .field("usart2ie", &self.usart2ie())
            .field("i2c1ie", &self.i2c1ie())
            .field("lptim2ie", &self.lptim2ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for TIM2
    #[inline(always)]
    pub fn tim2ie(&mut self) -> TIM2IE_W<'_, IER1rs> {
        TIM2IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for TIM3
    #[inline(always)]
    pub fn tim3ie(&mut self) -> TIM3IE_W<'_, IER1rs> {
        TIM3IE_W::new(self, 1)
    }
    ///Bit 6 - illegal access interrupt enable for WWDG
    #[inline(always)]
    pub fn wwdgie(&mut self) -> WWDGIE_W<'_, IER1rs> {
        WWDGIE_W::new(self, 6)
    }
    ///Bit 7 - illegal access interrupt enable for IWDG
    #[inline(always)]
    pub fn iwdgie(&mut self) -> IWDGIE_W<'_, IER1rs> {
        IWDGIE_W::new(self, 7)
    }
    ///Bit 9 - illegal access interrupt enable for USART2
    #[inline(always)]
    pub fn usart2ie(&mut self) -> USART2IE_W<'_, IER1rs> {
        USART2IE_W::new(self, 9)
    }
    ///Bit 13 - illegal access interrupt enable for I2C1
    #[inline(always)]
    pub fn i2c1ie(&mut self) -> I2C1IE_W<'_, IER1rs> {
        I2C1IE_W::new(self, 13)
    }
    ///Bit 17 - illegal access interrupt enable for LPTIM2
    #[inline(always)]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W<'_, IER1rs> {
        LPTIM2IE_W::new(self, 17)
    }
}
/**TZIC interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZIC:IER1)*/
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
///`read()` method returns [`ier1::R`](R) reader structure
impl crate::Readable for IER1rs {}
///`write(|w| ..)` method takes [`ier1::W`](W) writer structure
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1rs {}
