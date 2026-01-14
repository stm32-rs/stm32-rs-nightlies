///Register `APBSMENR1` reader
pub type R = crate::R<APBSMENR1rs>;
///Register `APBSMENR1` writer
pub type W = crate::W<APBSMENR1rs>;
/**TIM2 timer clock enable during Sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<TIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode
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
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN::Enabled
    }
}
///Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2SMEN>;
impl<'a, REG> TIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Enabled)
    }
}
///Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode
pub use TIM2SMEN_R as TIM3SMEN_R;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode
pub use TIM2SMEN_R as RTCAPBSMEN_R;
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode
pub use TIM2SMEN_R as WWDGSMEN_R;
///Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode
pub use TIM2SMEN_R as SPI2SMEN_R;
///Field `USART2SMEN` reader - USART2 clock enable during Sleep mode
pub use TIM2SMEN_R as USART2SMEN_R;
///Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep mode
pub use TIM2SMEN_R as LPUART1SMEN_R;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode
pub use TIM2SMEN_R as I2C1SMEN_R;
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode
pub use TIM2SMEN_R as I2C2SMEN_R;
///Field `DBGSMEN` reader - Debug support clock enable during Sleep mode
pub use TIM2SMEN_R as DBGSMEN_R;
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode
pub use TIM2SMEN_R as PWRSMEN_R;
///Field `LPTIM2SMEN` reader - Low Power Timer 2 clock enable during Sleep mode
pub use TIM2SMEN_R as LPTIM2SMEN_R;
///Field `LPTIM1SMEN` reader - Low Power Timer 1 clock enable during Sleep mode
pub use TIM2SMEN_R as LPTIM1SMEN_R;
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode
pub use TIM2SMEN_W as TIM3SMEN_W;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode
pub use TIM2SMEN_W as RTCAPBSMEN_W;
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode
pub use TIM2SMEN_W as WWDGSMEN_W;
///Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode
pub use TIM2SMEN_W as SPI2SMEN_W;
///Field `USART2SMEN` writer - USART2 clock enable during Sleep mode
pub use TIM2SMEN_W as USART2SMEN_W;
///Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep mode
pub use TIM2SMEN_W as LPUART1SMEN_W;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode
pub use TIM2SMEN_W as I2C1SMEN_W;
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode
pub use TIM2SMEN_W as I2C2SMEN_W;
///Field `DBGSMEN` writer - Debug support clock enable during Sleep mode
pub use TIM2SMEN_W as DBGSMEN_W;
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode
pub use TIM2SMEN_W as PWRSMEN_W;
///Field `LPTIM2SMEN` writer - Low Power Timer 2 clock enable during Sleep mode
pub use TIM2SMEN_W as LPTIM2SMEN_W;
///Field `LPTIM1SMEN` writer - Low Power Timer 1 clock enable during Sleep mode
pub use TIM2SMEN_W as LPTIM1SMEN_W;
impl R {
    ///Bit 0 - TIM2 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 20 - LPUART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 20) & 1) != 0)
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
    ///Bit 30 - Low Power Timer 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBSMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("tim3smen", &self.tim3smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("spi2smen", &self.spi2smen())
            .field("usart2smen", &self.usart2smen())
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("dbgsmen", &self.dbgsmen())
            .field("pwrsmen", &self.pwrsmen())
            .field("lptim2smen", &self.lptim2smen())
            .field("lptim1smen", &self.lptim1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APBSMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, APBSMENR1rs> {
        TIM3SMEN_W::new(self, 1)
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
    ///Bit 20 - LPUART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, APBSMENR1rs> {
        LPUART1SMEN_W::new(self, 20)
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
    ///Bit 30 - Low Power Timer 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<'_, APBSMENR1rs> {
        LPTIM2SMEN_W::new(self, 30)
    }
    ///Bit 31 - Low Power Timer 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APBSMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB peripheral clock enable in Sleep mode register 1

You can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#RCC:APBSMENR1)*/
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
///`reset()` method sets APBSMENR1 to value 0
impl crate::Resettable for APBSMENR1rs {}
