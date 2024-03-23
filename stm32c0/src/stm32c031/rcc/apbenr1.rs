#[doc = "Register `APBENR1` reader"]
pub type R = crate::R<APBENR1rs>;
#[doc = "Register `APBENR1` writer"]
pub type W = crate::W<APBENR1rs>;
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software."]
pub type TIM3EN_R = crate::BitReader;
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software."]
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software."]
pub type RTCAPBEN_R = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software."]
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
pub type WWDGEN_R = crate::BitReader;
#[doc = "Field `WWDGEN` writer - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable Set and cleared by software."]
pub type USART2EN_R = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable Set and cleared by software."]
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software."]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software."]
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGEN` reader - Debug support clock enable Set and cleared by software."]
pub type DBGEN_R = crate::BitReader;
#[doc = "Field `DBGEN` writer - Debug support clock enable Set and cleared by software."]
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - Power interface clock enable Set and cleared by software."]
pub type PWREN_R = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable Set and cleared by software."]
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 10 - RTC APB clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APBENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 17 - USART2 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APBENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 27 - Debug support clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<APBENR1rs> {
        DBGEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APBENR1rs> {
        PWREN_W::new(self, 28)
    }
}
#[doc = "RCC APB peripheral clock enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
