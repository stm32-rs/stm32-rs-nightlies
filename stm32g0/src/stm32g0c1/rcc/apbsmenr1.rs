///Register `APBSMENR1` reader
pub type R = crate::R<APBSMENR1rs>;
///Register `APBSMENR1` writer
pub type W = crate::W<APBSMENR1rs>;
/**TIM2 timer clock enable during Sleep mode

Value on reset: 1*/
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
///Field `TIM4SMEN` reader - TIM4 timer clock enable during Sleep mode
pub use TIM2SMEN_R as TIM4SMEN_R;
///Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode
pub use TIM2SMEN_R as TIM6SMEN_R;
///Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode
pub use TIM2SMEN_R as TIM7SMEN_R;
///Field `LPUART2SMEN` reader - LPUART2 clock enable
pub use TIM2SMEN_R as LPUART2SMEN_R;
///Field `USART5SMEN` reader - USART5 clock enable
pub use TIM2SMEN_R as USART5SMEN_R;
///Field `USART6SMEN` reader - USART6 clock enable
pub use TIM2SMEN_R as USART6SMEN_R;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode
pub use TIM2SMEN_R as RTCAPBSMEN_R;
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode
pub use TIM2SMEN_R as WWDGSMEN_R;
///Field `FDCANSMEN` reader - FDCAN clock enable during Sleep mode
pub use TIM2SMEN_R as FDCANSMEN_R;
///Field `USBSMEN` reader - USB clock enable during Sleep mode
pub use TIM2SMEN_R as USBSMEN_R;
///Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode
pub use TIM2SMEN_R as SPI2SMEN_R;
///Field `SPI3SMEN` reader - SPI3 clock enable during Sleep mode
pub use TIM2SMEN_R as SPI3SMEN_R;
///Field `CRSSSMEN` reader - CRSS clock enable during Sleep mode
pub use TIM2SMEN_R as CRSSSMEN_R;
///Field `USART2SMEN` reader - USART2 clock enable during Sleep mode
pub use TIM2SMEN_R as USART2SMEN_R;
///Field `USART3SMEN` reader - USART3 clock enable during Sleep mode
pub use TIM2SMEN_R as USART3SMEN_R;
///Field `USART4SMEN` reader - USART4 clock enable during Sleep mode
pub use TIM2SMEN_R as USART4SMEN_R;
///Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep mode
pub use TIM2SMEN_R as LPUART1SMEN_R;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode
pub use TIM2SMEN_R as I2C1SMEN_R;
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode
pub use TIM2SMEN_R as I2C2SMEN_R;
///Field `I2C3SMEN` reader - I2C3 clock enable during Sleep mode
pub use TIM2SMEN_R as I2C3SMEN_R;
///Field `CECSMEN` reader - HDMI CEC clock enable during Sleep mode
pub use TIM2SMEN_R as CECSMEN_R;
///Field `UCPD1SMEN` reader - UCPD1 clock enable during Sleep mode
pub use TIM2SMEN_R as UCPD1SMEN_R;
///Field `UCPD2SMEN` reader - UCPD2 clock enable during Sleep mode
pub use TIM2SMEN_R as UCPD2SMEN_R;
///Field `DBGSMEN` reader - Debug support clock enable during Sleep mode
pub use TIM2SMEN_R as DBGSMEN_R;
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode
pub use TIM2SMEN_R as PWRSMEN_R;
///Field `DAC1SMEN` reader - DAC1 interface clock enable during Sleep mode
pub use TIM2SMEN_R as DAC1SMEN_R;
///Field `LPTIM2SMEN` reader - Low Power Timer 2 clock enable during Sleep mode
pub use TIM2SMEN_R as LPTIM2SMEN_R;
///Field `LPTIM1SMEN` reader - Low Power Timer 1 clock enable during Sleep mode
pub use TIM2SMEN_R as LPTIM1SMEN_R;
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode
pub use TIM2SMEN_W as TIM3SMEN_W;
///Field `TIM4SMEN` writer - TIM4 timer clock enable during Sleep mode
pub use TIM2SMEN_W as TIM4SMEN_W;
///Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode
pub use TIM2SMEN_W as TIM6SMEN_W;
///Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode
pub use TIM2SMEN_W as TIM7SMEN_W;
///Field `LPUART2SMEN` writer - LPUART2 clock enable
pub use TIM2SMEN_W as LPUART2SMEN_W;
///Field `USART5SMEN` writer - USART5 clock enable
pub use TIM2SMEN_W as USART5SMEN_W;
///Field `USART6SMEN` writer - USART6 clock enable
pub use TIM2SMEN_W as USART6SMEN_W;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode
pub use TIM2SMEN_W as RTCAPBSMEN_W;
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode
pub use TIM2SMEN_W as WWDGSMEN_W;
///Field `FDCANSMEN` writer - FDCAN clock enable during Sleep mode
pub use TIM2SMEN_W as FDCANSMEN_W;
///Field `USBSMEN` writer - USB clock enable during Sleep mode
pub use TIM2SMEN_W as USBSMEN_W;
///Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode
pub use TIM2SMEN_W as SPI2SMEN_W;
///Field `SPI3SMEN` writer - SPI3 clock enable during Sleep mode
pub use TIM2SMEN_W as SPI3SMEN_W;
///Field `CRSSSMEN` writer - CRSS clock enable during Sleep mode
pub use TIM2SMEN_W as CRSSSMEN_W;
///Field `USART2SMEN` writer - USART2 clock enable during Sleep mode
pub use TIM2SMEN_W as USART2SMEN_W;
///Field `USART3SMEN` writer - USART3 clock enable during Sleep mode
pub use TIM2SMEN_W as USART3SMEN_W;
///Field `USART4SMEN` writer - USART4 clock enable during Sleep mode
pub use TIM2SMEN_W as USART4SMEN_W;
///Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep mode
pub use TIM2SMEN_W as LPUART1SMEN_W;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode
pub use TIM2SMEN_W as I2C1SMEN_W;
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode
pub use TIM2SMEN_W as I2C2SMEN_W;
///Field `I2C3SMEN` writer - I2C3 clock enable during Sleep mode
pub use TIM2SMEN_W as I2C3SMEN_W;
///Field `CECSMEN` writer - HDMI CEC clock enable during Sleep mode
pub use TIM2SMEN_W as CECSMEN_W;
///Field `UCPD1SMEN` writer - UCPD1 clock enable during Sleep mode
pub use TIM2SMEN_W as UCPD1SMEN_W;
///Field `UCPD2SMEN` writer - UCPD2 clock enable during Sleep mode
pub use TIM2SMEN_W as UCPD2SMEN_W;
///Field `DBGSMEN` writer - Debug support clock enable during Sleep mode
pub use TIM2SMEN_W as DBGSMEN_W;
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode
pub use TIM2SMEN_W as PWRSMEN_W;
///Field `DAC1SMEN` writer - DAC1 interface clock enable during Sleep mode
pub use TIM2SMEN_W as DAC1SMEN_W;
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
    ///Bit 2 - TIM4 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
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
    ///Bit 7 - LPUART2 clock enable
    #[inline(always)]
    pub fn lpuart2smen(&self) -> LPUART2SMEN_R {
        LPUART2SMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - USART5 clock enable
    #[inline(always)]
    pub fn usart5smen(&self) -> USART5SMEN_R {
        USART5SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - USART6 clock enable
    #[inline(always)]
    pub fn usart6smen(&self) -> USART6SMEN_R {
        USART6SMEN_R::new(((self.bits >> 9) & 1) != 0)
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
    ///Bit 12 - FDCAN clock enable during Sleep mode
    #[inline(always)]
    pub fn fdcansmen(&self) -> FDCANSMEN_R {
        FDCANSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USB clock enable during Sleep mode
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CRSS clock enable during Sleep mode
    #[inline(always)]
    pub fn crsssmen(&self) -> CRSSSMEN_R {
        CRSSSMEN_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 23 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - HDMI CEC clock enable during Sleep mode
    #[inline(always)]
    pub fn cecsmen(&self) -> CECSMEN_R {
        CECSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - UCPD1 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - UCPD2 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd2smen(&self) -> UCPD2SMEN_R {
        UCPD2SMEN_R::new(((self.bits >> 26) & 1) != 0)
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
    ///Bit 29 - DAC1 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
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
            .field("tim4smen", &self.tim4smen())
            .field("tim6smen", &self.tim6smen())
            .field("tim7smen", &self.tim7smen())
            .field("lpuart2smen", &self.lpuart2smen())
            .field("usart5smen", &self.usart5smen())
            .field("usart6smen", &self.usart6smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("fdcansmen", &self.fdcansmen())
            .field("usbsmen", &self.usbsmen())
            .field("spi2smen", &self.spi2smen())
            .field("spi3smen", &self.spi3smen())
            .field("crsssmen", &self.crsssmen())
            .field("usart2smen", &self.usart2smen())
            .field("usart3smen", &self.usart3smen())
            .field("usart4smen", &self.usart4smen())
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("i2c3smen", &self.i2c3smen())
            .field("cecsmen", &self.cecsmen())
            .field("ucpd1smen", &self.ucpd1smen())
            .field("ucpd2smen", &self.ucpd2smen())
            .field("dbgsmen", &self.dbgsmen())
            .field("pwrsmen", &self.pwrsmen())
            .field("dac1smen", &self.dac1smen())
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
    ///Bit 2 - TIM4 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<'_, APBSMENR1rs> {
        TIM4SMEN_W::new(self, 2)
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
    ///Bit 7 - LPUART2 clock enable
    #[inline(always)]
    pub fn lpuart2smen(&mut self) -> LPUART2SMEN_W<'_, APBSMENR1rs> {
        LPUART2SMEN_W::new(self, 7)
    }
    ///Bit 8 - USART5 clock enable
    #[inline(always)]
    pub fn usart5smen(&mut self) -> USART5SMEN_W<'_, APBSMENR1rs> {
        USART5SMEN_W::new(self, 8)
    }
    ///Bit 9 - USART6 clock enable
    #[inline(always)]
    pub fn usart6smen(&mut self) -> USART6SMEN_W<'_, APBSMENR1rs> {
        USART6SMEN_W::new(self, 9)
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
    ///Bit 12 - FDCAN clock enable during Sleep mode
    #[inline(always)]
    pub fn fdcansmen(&mut self) -> FDCANSMEN_W<'_, APBSMENR1rs> {
        FDCANSMEN_W::new(self, 12)
    }
    ///Bit 13 - USB clock enable during Sleep mode
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W<'_, APBSMENR1rs> {
        USBSMEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, APBSMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<'_, APBSMENR1rs> {
        SPI3SMEN_W::new(self, 15)
    }
    ///Bit 16 - CRSS clock enable during Sleep mode
    #[inline(always)]
    pub fn crsssmen(&mut self) -> CRSSSMEN_W<'_, APBSMENR1rs> {
        CRSSSMEN_W::new(self, 16)
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
    ///Bit 23 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APBSMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    ///Bit 24 - HDMI CEC clock enable during Sleep mode
    #[inline(always)]
    pub fn cecsmen(&mut self) -> CECSMEN_W<'_, APBSMENR1rs> {
        CECSMEN_W::new(self, 24)
    }
    ///Bit 25 - UCPD1 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<'_, APBSMENR1rs> {
        UCPD1SMEN_W::new(self, 25)
    }
    ///Bit 26 - UCPD2 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd2smen(&mut self) -> UCPD2SMEN_W<'_, APBSMENR1rs> {
        UCPD2SMEN_W::new(self, 26)
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
    ///Bit 29 - DAC1 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<'_, APBSMENR1rs> {
        DAC1SMEN_W::new(self, 29)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#RCC:APBSMENR1)*/
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
