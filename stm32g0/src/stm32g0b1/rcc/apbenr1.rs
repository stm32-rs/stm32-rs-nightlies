#[doc = "Register `APBENR1` reader"]
pub type R = crate::R<APBENR1rs>;
#[doc = "Register `APBENR1` writer"]
pub type W = crate::W<APBENR1rs>;
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable"]
pub type TIM2EN_R = crate::BitReader;
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable"]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable"]
pub type TIM3EN_R = crate::BitReader;
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable"]
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4EN` reader - TIM4 timer clock enable"]
pub type TIM4EN_R = crate::BitReader;
#[doc = "Field `TIM4EN` writer - TIM4 timer clock enable"]
pub type TIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable"]
pub type TIM6EN_R = crate::BitReader;
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable"]
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable"]
pub type TIM7EN_R = crate::BitReader;
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable"]
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART2EN` reader - LPUART2 clock enable"]
pub type LPUART2EN_R = crate::BitReader;
#[doc = "Field `LPUART2EN` writer - LPUART2 clock enable"]
pub type LPUART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5EN` reader - USART5EN"]
pub type USART5EN_R = crate::BitReader;
#[doc = "Field `USART5EN` writer - USART5EN"]
pub type USART5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6EN` reader - USART6EN"]
pub type USART6EN_R = crate::BitReader;
#[doc = "Field `USART6EN` writer - USART6EN"]
pub type USART6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub type RTCAPBEN_R = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - WWDG clock enable"]
pub type WWDGEN_R = crate::BitReader;
#[doc = "Field `WWDGEN` writer - WWDG clock enable"]
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANEN` reader - USBEN"]
pub type FDCANEN_R = crate::BitReader;
#[doc = "Field `FDCANEN` writer - USBEN"]
pub type FDCANEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEN` reader - USBEN"]
pub type USBEN_R = crate::BitReader;
#[doc = "Field `USBEN` writer - USBEN"]
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type SPI2EN_R = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable"]
pub type SPI3EN_R = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable"]
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSEN` reader - CRSEN"]
pub type CRSEN_R = crate::BitReader;
#[doc = "Field `CRSEN` writer - CRSEN"]
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type USART2EN_R = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3EN` reader - USART3 clock enable"]
pub type USART3EN_R = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART3 clock enable"]
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4EN` reader - USART4 clock enable"]
pub type USART4EN_R = crate::BitReader;
#[doc = "Field `USART4EN` writer - USART4 clock enable"]
pub type USART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable"]
pub type LPUART1EN_R = crate::BitReader;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable"]
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub type I2C2EN_R = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable"]
pub type I2C3EN_R = crate::BitReader;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable"]
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CECEN` reader - HDMI CEC clock enable"]
pub type CECEN_R = crate::BitReader;
#[doc = "Field `CECEN` writer - HDMI CEC clock enable"]
pub type CECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1EN` reader - UCPD1 clock enable"]
pub type UCPD1EN_R = crate::BitReader;
#[doc = "Field `UCPD1EN` writer - UCPD1 clock enable"]
pub type UCPD1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD2EN` reader - UCPD2 clock enable"]
pub type UCPD2EN_R = crate::BitReader;
#[doc = "Field `UCPD2EN` writer - UCPD2 clock enable"]
pub type UCPD2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGEN` reader - Debug support clock enable"]
pub type DBGEN_R = crate::BitReader;
#[doc = "Field `DBGEN` writer - Debug support clock enable"]
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PWREN_R = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1EN` reader - DAC1 interface clock enable"]
pub type DAC1EN_R = crate::BitReader;
#[doc = "Field `DAC1EN` writer - DAC1 interface clock enable"]
pub type DAC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 clock enable"]
pub type LPTIM2EN_R = crate::BitReader;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 clock enable"]
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable"]
pub type LPTIM1EN_R = crate::BitReader;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable"]
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPUART2 clock enable"]
    #[inline(always)]
    pub fn lpuart2en(&self) -> LPUART2EN_R {
        LPUART2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USART5EN"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USBEN"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USBEN"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CRSEN"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UCPD1 clock enable"]
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - UCPD2 clock enable"]
    #[inline(always)]
    pub fn ucpd2en(&self) -> UCPD2EN_R {
        UCPD2EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LPTIM2 clock enable"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APBENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<APBENR1rs> {
        TIM4EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APBENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APBENR1rs> {
        TIM7EN_W::new(self, 5)
    }
    #[doc = "Bit 7 - LPUART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2en(&mut self) -> LPUART2EN_W<APBENR1rs> {
        LPUART2EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - USART5EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart5en(&mut self) -> USART5EN_W<APBENR1rs> {
        USART5EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - USART6EN"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<APBENR1rs> {
        USART6EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APBENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - USBEN"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<APBENR1rs> {
        FDCANEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - USBEN"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<APBENR1rs> {
        USBEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APBENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<APBENR1rs> {
        SPI3EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - CRSEN"]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<APBENR1rs> {
        CRSEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APBENR1rs> {
        USART3EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart4en(&mut self) -> USART4EN_W<APBENR1rs> {
        USART4EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - LPUART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APBENR1rs> {
        LPUART1EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APBENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APBENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APBENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<APBENR1rs> {
        CECEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - UCPD1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W<APBENR1rs> {
        UCPD1EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - UCPD2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd2en(&mut self) -> UCPD2EN_W<APBENR1rs> {
        UCPD2EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<APBENR1rs> {
        DBGEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APBENR1rs> {
        PWREN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<APBENR1rs> {
        DAC1EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - LPTIM2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<APBENR1rs> {
        LPTIM2EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - LPTIM1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APBENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
#[doc = "APB peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBENR1rs;
impl crate::RegisterSpec for APBENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbenr1::R`](R) reader structure"]
impl crate::Readable for APBENR1rs {}
#[doc = "`write(|w| ..)` method takes [`apbenr1::W`](W) writer structure"]
impl crate::Writable for APBENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBENR1 to value 0"]
impl crate::Resettable for APBENR1rs {
    const RESET_VALUE: u32 = 0;
}
