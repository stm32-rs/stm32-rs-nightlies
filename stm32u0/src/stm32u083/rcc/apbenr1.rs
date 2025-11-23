///Register `APBENR1` reader
pub type R = crate::R<APBENR1rs>;
///Register `APBENR1` writer
pub type W = crate::W<APBENR1rs>;
///Field `TIM2EN` reader - TIM2 timer clock enable Set and cleared by software.
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - TIM2 timer clock enable Set and cleared by software.
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software.
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software.
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6EN` reader - TIM6 timer clock enable Set and cleared by software.
pub type TIM6EN_R = crate::BitReader;
///Field `TIM6EN` writer - TIM6 timer clock enable Set and cleared by software.
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7EN` reader - TIM7 timer clock enable Set and cleared by software.
pub type TIM7EN_R = crate::BitReader;
///Field `TIM7EN` writer - TIM7 timer clock enable Set and cleared by software.
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART2EN` reader - LPUART2 clock enable Set and cleared by software.
pub type LPUART2EN_R = crate::BitReader;
///Field `LPUART2EN` writer - LPUART2 clock enable Set and cleared by software.
pub type LPUART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDEN` reader - LCD clock enable<sup>(1)</sup> Set and cleared by software.
pub type LCDEN_R = crate::BitReader;
///Field `LCDEN` writer - LCD clock enable<sup>(1)</sup> Set and cleared by software.
pub type LCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software.
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software.
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART3EN` reader - LPUART3 clock enable Set and cleared by software.
pub type LPUART3EN_R = crate::BitReader;
///Field `LPUART3EN` writer - LPUART3 clock enable Set and cleared by software.
pub type LPUART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBEN` reader - USB clock enable<sup>(1)</sup> Set and cleared by software.
pub type USBEN_R = crate::BitReader;
///Field `USBEN` writer - USB clock enable<sup>(1)</sup> Set and cleared by software.
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2EN` reader - SPI2 clock enable Set and cleared by software.
pub type SPI2EN_R = crate::BitReader;
///Field `SPI2EN` writer - SPI2 clock enable Set and cleared by software.
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3EN` reader - SPI3 clock enable<sup>(1)</sup> Set and cleared by software.
pub type SPI3EN_R = crate::BitReader;
///Field `SPI3EN` writer - SPI3 clock enable<sup>(1)</sup> Set and cleared by software.
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSEN` reader - CRS clock enable<sup>(1)</sup> Set and cleared by software.
pub type CRSEN_R = crate::BitReader;
///Field `CRSEN` writer - CRS clock enable<sup>(1)</sup> Set and cleared by software.
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART2 clock enable Set and cleared by software.
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART2 clock enable Set and cleared by software.
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3EN` reader - USART3 clock enable Set and cleared by software.
pub type USART3EN_R = crate::BitReader;
///Field `USART3EN` writer - USART3 clock enable Set and cleared by software.
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4EN` reader - USART4 clock enable Set and cleared by software.
pub type USART4EN_R = crate::BitReader;
///Field `USART4EN` writer - USART4 clock enable Set and cleared by software.
pub type USART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1EN` reader - LPUART1 clock enable Set and cleared by software.
pub type LPUART1EN_R = crate::BitReader;
///Field `LPUART1EN` writer - LPUART1 clock enable Set and cleared by software.
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2EN` reader - I2C2 clock enable Set and cleared by software.
pub type I2C2EN_R = crate::BitReader;
///Field `I2C2EN` writer - I2C2 clock enable Set and cleared by software.
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3EN` reader - I2C3 clock enable Set and cleared by software.
pub type I2C3EN_R = crate::BitReader;
///Field `I2C3EN` writer - I2C3 clock enable Set and cleared by software.
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPEN` reader - OPAMP clock enable Set and cleared by software.
pub type OPAMPEN_R = crate::BitReader;
///Field `OPAMPEN` writer - OPAMP clock enable Set and cleared by software.
pub type OPAMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4EN` reader - I2C4EN clock enable<sup>(1)</sup> Set and cleared by software.
pub type I2C4EN_R = crate::BitReader;
///Field `I2C4EN` writer - I2C4EN clock enable<sup>(1)</sup> Set and cleared by software.
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3EN` reader - LPTIM3 clock enable Set and cleared by software.
pub type LPTIM3EN_R = crate::BitReader;
///Field `LPTIM3EN` writer - LPTIM3 clock enable Set and cleared by software.
pub type LPTIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWREN` reader - Power interface clock enable Set and cleared by software.
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - Power interface clock enable Set and cleared by software.
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1EN` reader - DAC1 interface clock enable Set and cleared by software.
pub type DAC1EN_R = crate::BitReader;
///Field `DAC1EN` writer - DAC1 interface clock enable Set and cleared by software.
pub type DAC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2EN` reader - LPTIM2 clock enable Set and cleared by software.
pub type LPTIM2EN_R = crate::BitReader;
///Field `LPTIM2EN` writer - LPTIM2 clock enable Set and cleared by software.
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1EN` reader - LPTIM1 clock enable Set and cleared by software.
pub type LPTIM1EN_R = crate::BitReader;
///Field `LPTIM1EN` writer - LPTIM1 clock enable Set and cleared by software.
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - LPUART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpuart2en(&self) -> LPUART2EN_R {
        LPUART2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LCD clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPUART3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpuart3en(&self) -> LPUART3EN_R {
        LPUART3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USB clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CRS clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - LPUART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OPAMP clock enable Set and cleared by software.
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - I2C4EN clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LPTIM3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - LPTIM2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LPTIM1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBENR1")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("lpuart2en", &self.lpuart2en())
            .field("lcden", &self.lcden())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("lpuart3en", &self.lpuart3en())
            .field("usben", &self.usben())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("crsen", &self.crsen())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("usart4en", &self.usart4en())
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("i2c3en", &self.i2c3en())
            .field("opampen", &self.opampen())
            .field("i2c4en", &self.i2c4en())
            .field("lptim3en", &self.lptim3en())
            .field("pwren", &self.pwren())
            .field("dac1en", &self.dac1en())
            .field("lptim2en", &self.lptim2en())
            .field("lptim1en", &self.lptim1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APBENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 4 - TIM6 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APBENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APBENR1rs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 7 - LPUART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpuart2en(&mut self) -> LPUART2EN_W<'_, APBENR1rs> {
        LPUART2EN_W::new(self, 7)
    }
    ///Bit 9 - LCD clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W<'_, APBENR1rs> {
        LCDEN_W::new(self, 9)
    }
    ///Bit 10 - RTC APB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APBENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 12 - LPUART3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpuart3en(&mut self) -> LPUART3EN_W<'_, APBENR1rs> {
        LPUART3EN_W::new(self, 12)
    }
    ///Bit 13 - USB clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<'_, APBENR1rs> {
        USBEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APBENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APBENR1rs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 16 - CRS clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W<'_, APBENR1rs> {
        CRSEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APBENR1rs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - USART4 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart4en(&mut self) -> USART4EN_W<'_, APBENR1rs> {
        USART4EN_W::new(self, 19)
    }
    ///Bit 20 - LPUART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APBENR1rs> {
        LPUART1EN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APBENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APBENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APBENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 24 - OPAMP clock enable Set and cleared by software.
    #[inline(always)]
    pub fn opampen(&mut self) -> OPAMPEN_W<'_, APBENR1rs> {
        OPAMPEN_W::new(self, 24)
    }
    ///Bit 25 - I2C4EN clock enable<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<'_, APBENR1rs> {
        I2C4EN_W::new(self, 25)
    }
    ///Bit 26 - LPTIM3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<'_, APBENR1rs> {
        LPTIM3EN_W::new(self, 26)
    }
    ///Bit 28 - Power interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APBENR1rs> {
        PWREN_W::new(self, 28)
    }
    ///Bit 29 - DAC1 interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W<'_, APBENR1rs> {
        DAC1EN_W::new(self, 29)
    }
    ///Bit 30 - LPTIM2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APBENR1rs> {
        LPTIM2EN_W::new(self, 30)
    }
    ///Bit 31 - LPTIM1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APBENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
/**APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBENR1)*/
pub struct APBENR1rs;
impl crate::RegisterSpec for APBENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apbenr1::R`](R) reader structure
impl crate::Readable for APBENR1rs {}
///`write(|w| ..)` method takes [`apbenr1::W`](W) writer structure
impl crate::Writable for APBENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APBENR1 to value 0
impl crate::Resettable for APBENR1rs {}
