///Register `APBSMENR1` reader
pub type R = crate::R<APBSMENR1rs>;
///Register `APBSMENR1` writer
pub type W = crate::W<APBSMENR1rs>;
/**TIM3 timer clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3SMEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<TIM3SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM3SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode
pub type TIM3SMEN_R = crate::BitReader<TIM3SMEN>;
impl TIM3SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM3SMEN {
        match self.bits {
            false => TIM3SMEN::Disabled,
            true => TIM3SMEN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM3SMEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM3SMEN::Enabled
    }
}
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM3SMEN>;
impl<'a, REG> TIM3SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3SMEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3SMEN::Enabled)
    }
}
///Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode
pub use TIM3SMEN_R as TIM6SMEN_R;
///Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode
pub use TIM3SMEN_R as TIM7SMEN_R;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode
pub use TIM3SMEN_R as RTCAPBSMEN_R;
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode
pub use TIM3SMEN_R as WWDGSMEN_R;
///Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode
pub use TIM3SMEN_R as SPI2SMEN_R;
///Field `USART2SMEN` reader - USART2 clock enable during Sleep mode
pub use TIM3SMEN_R as USART2SMEN_R;
///Field `USART3SMEN` reader - USART3 clock enable during Sleep mode
pub use TIM3SMEN_R as USART3SMEN_R;
///Field `USART4SMEN` reader - USART4 clock enable during Sleep mode
pub use TIM3SMEN_R as USART4SMEN_R;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode
pub use TIM3SMEN_R as I2C1SMEN_R;
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode
pub use TIM3SMEN_R as I2C2SMEN_R;
///Field `DBGSMEN` reader - Debug support clock enable during Sleep mode
pub use TIM3SMEN_R as DBGSMEN_R;
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode
pub use TIM3SMEN_R as PWRSMEN_R;
///Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode
pub use TIM3SMEN_W as TIM6SMEN_W;
///Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode
pub use TIM3SMEN_W as TIM7SMEN_W;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode
pub use TIM3SMEN_W as RTCAPBSMEN_W;
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode
pub use TIM3SMEN_W as WWDGSMEN_W;
///Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode
pub use TIM3SMEN_W as SPI2SMEN_W;
///Field `USART2SMEN` writer - USART2 clock enable during Sleep mode
pub use TIM3SMEN_W as USART2SMEN_W;
///Field `USART3SMEN` writer - USART3 clock enable during Sleep mode
pub use TIM3SMEN_W as USART3SMEN_W;
///Field `USART4SMEN` writer - USART4 clock enable during Sleep mode
pub use TIM3SMEN_W as USART4SMEN_W;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode
pub use TIM3SMEN_W as I2C1SMEN_W;
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode
pub use TIM3SMEN_W as I2C2SMEN_W;
///Field `DBGSMEN` writer - Debug support clock enable during Sleep mode
pub use TIM3SMEN_W as DBGSMEN_W;
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode
pub use TIM3SMEN_W as PWRSMEN_W;
impl R {
    ///Bit 1 - TIM3 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBSMENR1")
            .field("tim3smen", &self.tim3smen())
            .field("tim6smen", &self.tim6smen())
            .field("tim7smen", &self.tim7smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("spi2smen", &self.spi2smen())
            .field("usart2smen", &self.usart2smen())
            .field("usart3smen", &self.usart3smen())
            .field("usart4smen", &self.usart4smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("dbgsmen", &self.dbgsmen())
            .field("pwrsmen", &self.pwrsmen())
            .finish()
    }
}
impl W {
    ///Bit 1 - TIM3 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, APBSMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 4 - TIM6 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<'_, APBSMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<'_, APBSMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, APBSMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APBSMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, APBSMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, APBSMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<'_, APBSMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    ///Bit 19 - USART4 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart4smen(&mut self) -> USART4SMEN_W<'_, APBSMENR1rs> {
        USART4SMEN_W::new(self, 19)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APBSMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, APBSMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<'_, APBSMENR1rs> {
        DBGSMEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<'_, APBSMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
}
/**APB peripheral clock enable in Sleep mode register 1

You can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#RCC:APBSMENR1)*/
pub struct APBSMENR1rs;
impl crate::RegisterSpec for APBSMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apbsmenr1::R`](R) reader structure
impl crate::Readable for APBSMENR1rs {}
///`write(|w| ..)` method takes [`apbsmenr1::W`](W) writer structure
impl crate::Writable for APBSMENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APBSMENR1 to value 0xffff_ffb7
impl crate::Resettable for APBSMENR1rs {
    const RESET_VALUE: u32 = 0xffff_ffb7;
}
