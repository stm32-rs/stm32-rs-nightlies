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
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub type RTCAPBEN_R = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - WWDG clock enable"]
pub type WWDGEN_R = crate::BitReader;
#[doc = "Field `WWDGEN` writer - WWDG clock enable"]
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type SPI2EN_R = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type USART2EN_R = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `DBGEN` reader - Debug support clock enable"]
pub type DBGEN_R = crate::BitReader;
#[doc = "Field `DBGEN` writer - Debug support clock enable"]
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PWREN_R = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APBENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APBENR1rs> {
        USART2EN_W::new(self, 17)
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
