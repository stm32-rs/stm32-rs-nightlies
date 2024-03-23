#[doc = "Register `APBSMENR1` reader"]
pub type R = crate::R<APBSMENR1rs>;
#[doc = "Register `APBSMENR1` writer"]
pub type W = crate::W<APBSMENR1rs>;
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode"]
pub type TIM2SMEN_R = crate::BitReader;
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode"]
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode"]
pub type TIM3SMEN_R = crate::BitReader;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode"]
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode"]
pub type TIM6SMEN_R = crate::BitReader;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode"]
pub type TIM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode"]
pub type TIM7SMEN_R = crate::BitReader;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode"]
pub type TIM7SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode"]
pub type RTCAPBSMEN_R = crate::BitReader;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode"]
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode"]
pub type WWDGSMEN_R = crate::BitReader;
#[doc = "Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode"]
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode"]
pub type SPI2SMEN_R = crate::BitReader;
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode"]
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SMEN` reader - USART2 clock enable during Sleep mode"]
pub type USART2SMEN_R = crate::BitReader;
#[doc = "Field `USART2SMEN` writer - USART2 clock enable during Sleep mode"]
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3SMEN` reader - USART3 clock enable during Sleep mode"]
pub type USART3SMEN_R = crate::BitReader;
#[doc = "Field `USART3SMEN` writer - USART3 clock enable during Sleep mode"]
pub type USART3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4SMEN` reader - USART4 clock enable during Sleep mode"]
pub type USART4SMEN_R = crate::BitReader;
#[doc = "Field `USART4SMEN` writer - USART4 clock enable during Sleep mode"]
pub type USART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep mode"]
pub type LPUART1SMEN_R = crate::BitReader;
#[doc = "Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep mode"]
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2C1SMEN_R = crate::BitReader;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode"]
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2C2SMEN_R = crate::BitReader;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode"]
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CECSMEN` reader - HDMI CEC clock enable during Sleep mode"]
pub type CECSMEN_R = crate::BitReader;
#[doc = "Field `CECSMEN` writer - HDMI CEC clock enable during Sleep mode"]
pub type CECSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1SMEN` reader - UCPD1 clock enable during Sleep mode"]
pub type UCPD1SMEN_R = crate::BitReader;
#[doc = "Field `UCPD1SMEN` writer - UCPD1 clock enable during Sleep mode"]
pub type UCPD1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD2SMEN` reader - UCPD2 clock enable during Sleep mode"]
pub type UCPD2SMEN_R = crate::BitReader;
#[doc = "Field `UCPD2SMEN` writer - UCPD2 clock enable during Sleep mode"]
pub type UCPD2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSMEN` reader - Debug support clock enable during Sleep mode"]
pub type DBGSMEN_R = crate::BitReader;
#[doc = "Field `DBGSMEN` writer - Debug support clock enable during Sleep mode"]
pub type DBGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during Sleep mode"]
pub type PWRSMEN_R = crate::BitReader;
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during Sleep mode"]
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1SMEN` reader - DAC1 interface clock enable during Sleep mode"]
pub type DAC1SMEN_R = crate::BitReader;
#[doc = "Field `DAC1SMEN` writer - DAC1 interface clock enable during Sleep mode"]
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2SMEN` reader - Low Power Timer 2 clock enable during Sleep mode"]
pub type LPTIM2SMEN_R = crate::BitReader;
#[doc = "Field `LPTIM2SMEN` writer - Low Power Timer 2 clock enable during Sleep mode"]
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1SMEN` reader - Low Power Timer 1 clock enable during Sleep mode"]
pub type LPTIM1SMEN_R = crate::BitReader;
#[doc = "Field `LPTIM1SMEN` writer - Low Power Timer 1 clock enable during Sleep mode"]
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn cecsmen(&self) -> CECSMEN_R {
        CECSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UCPD1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - UCPD2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ucpd2smen(&self) -> UCPD2SMEN_R {
        UCPD2SMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Power Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<APBSMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<APBSMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<APBSMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<APBSMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APBSMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<APBSMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<APBSMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<APBSMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<APBSMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart4smen(&mut self) -> USART4SMEN_W<APBSMENR1rs> {
        USART4SMEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - LPUART1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<APBSMENR1rs> {
        LPUART1SMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<APBSMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<APBSMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    #[doc = "Bit 24 - HDMI CEC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn cecsmen(&mut self) -> CECSMEN_W<APBSMENR1rs> {
        CECSMEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - UCPD1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<APBSMENR1rs> {
        UCPD1SMEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - UCPD2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd2smen(&mut self) -> UCPD2SMEN_W<APBSMENR1rs> {
        UCPD2SMEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<APBSMENR1rs> {
        DBGSMEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<APBSMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<APBSMENR1rs> {
        DAC1SMEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Low Power Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<APBSMENR1rs> {
        LPTIM2SMEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low Power Timer 1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<APBSMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbsmenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbsmenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBSMENR1rs;
impl crate::RegisterSpec for APBSMENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbsmenr1::R`](R) reader structure"]
impl crate::Readable for APBSMENR1rs {}
#[doc = "`write(|w| ..)` method takes [`apbsmenr1::W`](W) writer structure"]
impl crate::Writable for APBSMENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBSMENR1 to value 0"]
impl crate::Resettable for APBSMENR1rs {
    const RESET_VALUE: u32 = 0;
}
