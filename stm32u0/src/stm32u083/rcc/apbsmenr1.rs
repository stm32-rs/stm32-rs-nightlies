///Register `APBSMENR1` reader
pub type R = crate::R<APBSMENR1rs>;
///Register `APBSMENR1` writer
pub type W = crate::W<APBSMENR1rs>;
///Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM2SMEN_R = crate::BitReader;
///Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_R = crate::BitReader;
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM6SMEN_R = crate::BitReader;
///Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM7SMEN_R = crate::BitReader;
///Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM7SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART2SMEN` reader - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART2SMEN_R = crate::BitReader;
///Field `LPUART2SMEN` writer - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDSMEN` reader - LCD clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type LCDSMEN_R = crate::BitReader;
///Field `LCDSMEN` writer - LCD clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type LCDSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_R = crate::BitReader;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_R = crate::BitReader;
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART3SMEN` reader - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART3SMEN_R = crate::BitReader;
///Field `LPUART3SMEN` writer - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSMEN` reader - USB clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type USBSMEN_R = crate::BitReader;
///Field `USBSMEN` writer - USB clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type USBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode Set and cleared by software.
pub type SPI2SMEN_R = crate::BitReader;
///Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode Set and cleared by software.
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3SMEN` reader - SPI3 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type SPI3SMEN_R = crate::BitReader;
///Field `SPI3SMEN` writer - SPI3 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type SPI3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes<sup>(1)</sup> Set and cleared by software.
pub type CRSSMEN_R = crate::BitReader;
///Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes<sup>(1)</sup> Set and cleared by software.
pub type CRSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2SMEN` reader - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_R = crate::BitReader;
///Field `USART2SMEN` writer - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3SMEN` reader - USART3 clock enable during Sleep mode Set and cleared by software.
pub type USART3SMEN_R = crate::BitReader;
///Field `USART3SMEN` writer - USART3 clock enable during Sleep mode Set and cleared by software.
pub type USART3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4SMEN` reader - USART4 clock enable during Sleep mode Set and cleared by software.
pub type USART4SMEN_R = crate::BitReader;
///Field `USART4SMEN` writer - USART4 clock enable during Sleep mode Set and cleared by software.
pub type USART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART1SMEN_R = crate::BitReader;
///Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_R = crate::BitReader;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode Set and cleared by software.
pub type I2C2SMEN_R = crate::BitReader;
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode Set and cleared by software.
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3SMEN` reader - I2C3 clock enable during Sleep mode Set and cleared by software.
pub type I2C3SMEN_R = crate::BitReader;
///Field `I2C3SMEN` writer - I2C3 clock enable during Sleep mode Set and cleared by software.
pub type I2C3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPSMEN` reader - OPAMP clock enable during Sleep and Stop modes Set and cleared by software.
pub type OPAMPSMEN_R = crate::BitReader;
///Field `OPAMPSMEN` writer - OPAMP clock enable during Sleep and Stop modes Set and cleared by software.
pub type OPAMPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4SMEN` reader - I2C4 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type I2C4SMEN_R = crate::BitReader;
///Field `I2C4SMEN` writer - I2C4 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
pub type I2C4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3SMEN` reader - Low power timer 3 clock enable during Sleep mode Set and cleared by software.
pub type LPTIM3SMEN_R = crate::BitReader;
///Field `LPTIM3SMEN` writer - Low power timer 3 clock enable during Sleep mode Set and cleared by software.
pub type LPTIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_R = crate::BitReader;
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1SMEN` reader - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software.
pub type DAC1SMEN_R = crate::BitReader;
///Field `DAC1SMEN` writer - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software.
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2SMEN` reader - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPTIM2SMEN_R = crate::BitReader;
///Field `LPTIM2SMEN` writer - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1SMEN` reader - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPTIM1SMEN_R = crate::BitReader;
///Field `LPTIM1SMEN` writer - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart2smen(&self) -> LPUART2SMEN_R {
        LPUART2SMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LCD clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart3smen(&self) -> LPUART3SMEN_R {
        LPUART3SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USB clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CRS clock enable during Sleep and Stop modes<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OPAMP clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - I2C4 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Low power timer 3 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software.
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
            .field("tim6smen", &self.tim6smen())
            .field("tim7smen", &self.tim7smen())
            .field("lpuart2smen", &self.lpuart2smen())
            .field("lcdsmen", &self.lcdsmen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("lpuart3smen", &self.lpuart3smen())
            .field("usbsmen", &self.usbsmen())
            .field("spi2smen", &self.spi2smen())
            .field("spi3smen", &self.spi3smen())
            .field("crssmen", &self.crssmen())
            .field("usart2smen", &self.usart2smen())
            .field("usart3smen", &self.usart3smen())
            .field("usart4smen", &self.usart4smen())
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("i2c3smen", &self.i2c3smen())
            .field("opampsmen", &self.opampsmen())
            .field("i2c4smen", &self.i2c4smen())
            .field("lptim3smen", &self.lptim3smen())
            .field("pwrsmen", &self.pwrsmen())
            .field("dac1smen", &self.dac1smen())
            .field("lptim2smen", &self.lptim2smen())
            .field("lptim1smen", &self.lptim1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APBSMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, APBSMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 4 - TIM6 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<'_, APBSMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<'_, APBSMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    ///Bit 7 - LPUART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart2smen(&mut self) -> LPUART2SMEN_W<'_, APBSMENR1rs> {
        LPUART2SMEN_W::new(self, 7)
    }
    ///Bit 9 - LCD clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W<'_, APBSMENR1rs> {
        LCDSMEN_W::new(self, 9)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, APBSMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APBSMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 12 - LPUART3 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart3smen(&mut self) -> LPUART3SMEN_W<'_, APBSMENR1rs> {
        LPUART3SMEN_W::new(self, 12)
    }
    ///Bit 13 - USB clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W<'_, APBSMENR1rs> {
        USBSMEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, APBSMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<'_, APBSMENR1rs> {
        SPI3SMEN_W::new(self, 15)
    }
    ///Bit 16 - CRS clock enable during Sleep and Stop modes<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn crssmen(&mut self) -> CRSSMEN_W<'_, APBSMENR1rs> {
        CRSSMEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, APBSMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<'_, APBSMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    ///Bit 19 - USART4 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn usart4smen(&mut self) -> USART4SMEN_W<'_, APBSMENR1rs> {
        USART4SMEN_W::new(self, 19)
    }
    ///Bit 20 - LPUART1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, APBSMENR1rs> {
        LPUART1SMEN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APBSMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, APBSMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APBSMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    ///Bit 24 - OPAMP clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<'_, APBSMENR1rs> {
        OPAMPSMEN_W::new(self, 24)
    }
    ///Bit 25 - I2C4 clock enable during Sleep mode<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<'_, APBSMENR1rs> {
        I2C4SMEN_W::new(self, 25)
    }
    ///Bit 26 - Low power timer 3 clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W<'_, APBSMENR1rs> {
        LPTIM3SMEN_W::new(self, 26)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<'_, APBSMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
    ///Bit 29 - DAC1 interface clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<'_, APBSMENR1rs> {
        DAC1SMEN_W::new(self, 29)
    }
    ///Bit 30 - Low Power Timer 2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<'_, APBSMENR1rs> {
        LPTIM2SMEN_W::new(self, 30)
    }
    ///Bit 31 - Low Power Timer 1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APBSMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB peripheral clock enable in Sleep/Stop mode register 1

You can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBSMENR1)*/
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
///`reset()` method sets APBSMENR1 to value 0xff7e_4c33
impl crate::Resettable for APBSMENR1rs {
    const RESET_VALUE: u32 = 0xff7e_4c33;
}
