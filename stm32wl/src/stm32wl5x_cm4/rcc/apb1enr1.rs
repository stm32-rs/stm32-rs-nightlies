#[doc = "Register `APB1ENR1` reader"]
pub type R = crate::R<APB1ENR1rs>;
#[doc = "Register `APB1ENR1` writer"]
pub type W = crate::W<APB1ENR1rs>;
#[doc = "CPU1 TIM2 timer clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - CPU1 TIM2 timer clock enable"]
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::Disabled,
            true => TIM2EN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
#[doc = "Field `TIM2EN` writer - CPU1 TIM2 timer clock enable"]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
#[doc = "Field `RTCAPBEN` reader - CPU1 RTC APB clock enable"]
pub use TIM2EN_R as RTCAPBEN_R;
#[doc = "Field `WWDGEN` reader - CPU1 Window watchdog clock enable"]
pub use TIM2EN_R as WWDGEN_R;
#[doc = "Field `SPI2S2EN` reader - CPU1 SPI2S2 clock enable"]
pub use TIM2EN_R as SPI2S2EN_R;
#[doc = "Field `USART2EN` reader - CPU1 USART2 clock enable"]
pub use TIM2EN_R as USART2EN_R;
#[doc = "Field `I2C1EN` reader - CPU1 I2C1 clocks enable"]
pub use TIM2EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - CPU1 I2C2 clocks enable"]
pub use TIM2EN_R as I2C2EN_R;
#[doc = "Field `I2C3EN` reader - CPU1 I2C3 clocks enable"]
pub use TIM2EN_R as I2C3EN_R;
#[doc = "Field `DAC1EN` reader - CPU1 DAC1 clock enable"]
pub use TIM2EN_R as DAC1EN_R;
#[doc = "Field `LPTIM1EN` reader - CPU1 Low power timer 1 clocks enable"]
pub use TIM2EN_R as LPTIM1EN_R;
#[doc = "Field `RTCAPBEN` writer - CPU1 RTC APB clock enable"]
pub use TIM2EN_W as RTCAPBEN_W;
#[doc = "Field `WWDGEN` writer - CPU1 Window watchdog clock enable"]
pub use TIM2EN_W as WWDGEN_W;
#[doc = "Field `SPI2S2EN` writer - CPU1 SPI2S2 clock enable"]
pub use TIM2EN_W as SPI2S2EN_W;
#[doc = "Field `USART2EN` writer - CPU1 USART2 clock enable"]
pub use TIM2EN_W as USART2EN_W;
#[doc = "Field `I2C1EN` writer - CPU1 I2C1 clocks enable"]
pub use TIM2EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - CPU1 I2C2 clocks enable"]
pub use TIM2EN_W as I2C2EN_W;
#[doc = "Field `I2C3EN` writer - CPU1 I2C3 clocks enable"]
pub use TIM2EN_W as I2C3EN_W;
#[doc = "Field `DAC1EN` writer - CPU1 DAC1 clock enable"]
pub use TIM2EN_W as DAC1EN_W;
#[doc = "Field `LPTIM1EN` writer - CPU1 Low power timer 1 clocks enable"]
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    #[doc = "Bit 0 - CPU1 TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - CPU1 RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU1 Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 SPI2S2 clock enable"]
    #[inline(always)]
    pub fn spi2s2en(&self) -> SPI2S2EN_R {
        SPI2S2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU1 USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU1 I2C1 clocks enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU1 I2C2 clocks enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU1 I2C3 clocks enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU1 DAC1 clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Low power timer 1 clocks enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU1 TIM2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 10 - CPU1 RTC APB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB1ENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU1 Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1ENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - CPU1 SPI2S2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2s2en(&mut self) -> SPI2S2EN_W<APB1ENR1rs> {
        SPI2S2EN_W::new(self, 14)
    }
    #[doc = "Bit 17 - CPU1 USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1ENR1rs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 21 - CPU1 I2C1 clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - CPU1 I2C2 clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1ENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - CPU1 I2C3 clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB1ENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    #[doc = "Bit 29 - CPU1 DAC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<APB1ENR1rs> {
        DAC1EN_W::new(self, 29)
    }
    #[doc = "Bit 31 - CPU1 Low power timer 1 clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB1ENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1ENR1rs;
impl crate::RegisterSpec for APB1ENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr1::R`](R) reader structure"]
impl crate::Readable for APB1ENR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1enr1::W`](W) writer structure"]
impl crate::Writable for APB1ENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1ENR1 to value 0"]
impl crate::Resettable for APB1ENR1rs {
    const RESET_VALUE: u32 = 0;
}
