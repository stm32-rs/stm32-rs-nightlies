///Register `APB1ENR1` reader
pub type R = crate::R<APB1ENR1rs>;
///Register `APB1ENR1` writer
pub type W = crate::W<APB1ENR1rs>;
/**CPU1 TIM2 timer clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2EN` reader - CPU1 TIM2 timer clock enable
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::Disabled,
            true => TIM2EN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
///Field `TIM2EN` writer - CPU1 TIM2 timer clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
///Field `RTCAPBEN` reader - CPU1 RTC APB clock enable
pub use TIM2EN_R as RTCAPBEN_R;
///Field `WWDGEN` reader - CPU1 Window watchdog clock enable
pub use TIM2EN_R as WWDGEN_R;
///Field `SPI2S2EN` reader - CPU1 SPI2S2 clock enable
pub use TIM2EN_R as SPI2S2EN_R;
///Field `USART2EN` reader - CPU1 USART2 clock enable
pub use TIM2EN_R as USART2EN_R;
///Field `I2C1EN` reader - CPU1 I2C1 clocks enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - CPU1 I2C2 clocks enable
pub use TIM2EN_R as I2C2EN_R;
///Field `I2C3EN` reader - CPU1 I2C3 clocks enable
pub use TIM2EN_R as I2C3EN_R;
///Field `DAC1EN` reader - CPU1 DAC1 clock enable
pub use TIM2EN_R as DAC1EN_R;
///Field `LPTIM1EN` reader - CPU1 Low power timer 1 clocks enable
pub use TIM2EN_R as LPTIM1EN_R;
///Field `RTCAPBEN` writer - CPU1 RTC APB clock enable
pub use TIM2EN_W as RTCAPBEN_W;
///Field `WWDGEN` writer - CPU1 Window watchdog clock enable
pub use TIM2EN_W as WWDGEN_W;
///Field `SPI2S2EN` writer - CPU1 SPI2S2 clock enable
pub use TIM2EN_W as SPI2S2EN_W;
///Field `USART2EN` writer - CPU1 USART2 clock enable
pub use TIM2EN_W as USART2EN_W;
///Field `I2C1EN` writer - CPU1 I2C1 clocks enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - CPU1 I2C2 clocks enable
pub use TIM2EN_W as I2C2EN_W;
///Field `I2C3EN` writer - CPU1 I2C3 clocks enable
pub use TIM2EN_W as I2C3EN_W;
///Field `DAC1EN` writer - CPU1 DAC1 clock enable
pub use TIM2EN_W as DAC1EN_W;
///Field `LPTIM1EN` writer - CPU1 Low power timer 1 clocks enable
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    ///Bit 0 - CPU1 TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 10 - CPU1 RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU1 Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - CPU1 SPI2S2 clock enable
    #[inline(always)]
    pub fn spi2s2en(&self) -> SPI2S2EN_R {
        SPI2S2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - CPU1 USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - CPU1 I2C1 clocks enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU1 I2C2 clocks enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU1 I2C3 clocks enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 29 - CPU1 DAC1 clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - CPU1 Low power timer 1 clocks enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR1")
            .field("tim2en", &self.tim2en())
            .field("lptim1en", &self.lptim1en())
            .field("dac1en", &self.dac1en())
            .field("i2c3en", &self.i2c3en())
            .field("i2c2en", &self.i2c2en())
            .field("i2c1en", &self.i2c1en())
            .field("usart2en", &self.usart2en())
            .field("spi2s2en", &self.spi2s2en())
            .field("wwdgen", &self.wwdgen())
            .field("rtcapben", &self.rtcapben())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU1 TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 10 - CPU1 RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APB1ENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - CPU1 Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - CPU1 SPI2S2 clock enable
    #[inline(always)]
    pub fn spi2s2en(&mut self) -> SPI2S2EN_W<'_, APB1ENR1rs> {
        SPI2S2EN_W::new(self, 14)
    }
    ///Bit 17 - CPU1 USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 21 - CPU1 I2C1 clocks enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - CPU1 I2C2 clocks enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1ENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - CPU1 I2C3 clocks enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB1ENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 29 - CPU1 DAC1 clock enable
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W<'_, APB1ENR1rs> {
        DAC1EN_W::new(self, 29)
    }
    ///Bit 31 - CPU1 Low power timer 1 clocks enable
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APB1ENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
/**APB1 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:APB1ENR1)*/
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
