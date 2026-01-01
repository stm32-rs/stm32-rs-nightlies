///Register `APB1SMENR1` reader
pub type R = crate::R<APB1SMENR1rs>;
///Register `APB1SMENR1` writer
pub type W = crate::W<APB1SMENR1rs>;
/**TIM2 timer clock enable during CPU1 CSleep mode.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<TIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clock enable during CPU1 CSleep mode.
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN>;
impl TIM2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2SMEN {
        match self.bits {
            false => TIM2SMEN::Disabled,
            true => TIM2SMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN::Enabled
    }
}
///Field `TIM2SMEN` writer - TIM2 timer clock enable during CPU1 CSleep mode.
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2SMEN>;
impl<'a, REG> TIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Enabled)
    }
}
///Field `RTCAPBSMEN` reader - RTC bus clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_R as RTCAPBSMEN_R;
///Field `WWDGSMEN` reader - Window watchdog clocks enable during CPU1 CSleep mode.
pub use TIM2SMEN_R as WWDGSMEN_R;
///Field `SPI2S2SMEN` reader - SPI2S2 clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_R as SPI2S2SMEN_R;
///Field `USART2SMEN` reader - USART2 clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_R as USART2SMEN_R;
///Field `I2C1SMEN` reader - I2C1 clock enable during CPU1 Csleep and CStop modes
pub use TIM2SMEN_R as I2C1SMEN_R;
///Field `I2C2SMEN` reader - I2C2 clock enable during CPU1 Csleep and CStop modes
pub use TIM2SMEN_R as I2C2SMEN_R;
///Field `I2C3SMEN` reader - I2C3 clock enable during CPU1 Csleep and CStop modes
pub use TIM2SMEN_R as I2C3SMEN_R;
///Field `DACSMEN` reader - DAC1 clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_R as DACSMEN_R;
///Field `LPTIM1SMEN` reader - Low power timer 1 clock enable during CPU1 Csleep and CStop mode
pub use TIM2SMEN_R as LPTIM1SMEN_R;
///Field `RTCAPBSMEN` writer - RTC bus clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_W as RTCAPBSMEN_W;
///Field `WWDGSMEN` writer - Window watchdog clocks enable during CPU1 CSleep mode.
pub use TIM2SMEN_W as WWDGSMEN_W;
///Field `SPI2S2SMEN` writer - SPI2S2 clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_W as SPI2S2SMEN_W;
///Field `USART2SMEN` writer - USART2 clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_W as USART2SMEN_W;
///Field `I2C1SMEN` writer - I2C1 clock enable during CPU1 Csleep and CStop modes
pub use TIM2SMEN_W as I2C1SMEN_W;
///Field `I2C2SMEN` writer - I2C2 clock enable during CPU1 Csleep and CStop modes
pub use TIM2SMEN_W as I2C2SMEN_W;
///Field `I2C3SMEN` writer - I2C3 clock enable during CPU1 Csleep and CStop modes
pub use TIM2SMEN_W as I2C3SMEN_W;
///Field `DACSMEN` writer - DAC1 clock enable during CPU1 CSleep mode.
pub use TIM2SMEN_W as DACSMEN_W;
///Field `LPTIM1SMEN` writer - Low power timer 1 clock enable during CPU1 Csleep and CStop mode
pub use TIM2SMEN_W as LPTIM1SMEN_W;
impl R {
    ///Bit 0 - TIM2 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 10 - RTC bus clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clocks enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2S2 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn spi2s2smen(&self) -> SPI2S2SMEN_R {
        SPI2S2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 29 - DAC1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dacsmen(&self) -> DACSMEN_R {
        DACSMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Low power timer 1 clock enable during CPU1 Csleep and CStop mode
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("lptim1smen", &self.lptim1smen())
            .field("dacsmen", &self.dacsmen())
            .field("i2c3smen", &self.i2c3smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("usart2smen", &self.usart2smen())
            .field("spi2s2smen", &self.spi2s2smen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 10 - RTC bus clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clocks enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2S2 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn spi2s2smen(&mut self) -> SPI2S2SMEN_W<'_, APB1SMENR1rs> {
        SPI2S2SMEN_W::new(self, 14)
    }
    ///Bit 17 - USART2 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, APB1SMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clock enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    ///Bit 29 - DAC1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn dacsmen(&mut self) -> DACSMEN_W<'_, APB1SMENR1rs> {
        DACSMEN_W::new(self, 29)
    }
    ///Bit 31 - Low power timer 1 clock enable during CPU1 Csleep and CStop mode
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APB1SMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB1 peripheral clocks enable in Sleep mode register 1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RCC:APB1SMENR1)*/
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
///`reset()` method sets APB1SMENR1 to value 0xa0e2_4c01
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xa0e2_4c01;
}
