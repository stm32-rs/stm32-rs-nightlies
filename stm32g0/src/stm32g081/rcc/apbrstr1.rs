#[doc = "Register `APBRSTR1` reader"]
pub type R = crate::R<APBRSTR1rs>;
#[doc = "Register `APBRSTR1` writer"]
pub type W = crate::W<APBRSTR1rs>;
#[doc = "Field `TIM2RST` reader - TIM2 timer reset"]
pub type TIM2RST_R = crate::BitReader;
#[doc = "Field `TIM2RST` writer - TIM2 timer reset"]
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3RST` reader - TIM3 timer reset"]
pub type TIM3RST_R = crate::BitReader;
#[doc = "Field `TIM3RST` writer - TIM3 timer reset"]
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6RST` reader - TIM6 timer reset"]
pub type TIM6RST_R = crate::BitReader;
#[doc = "Field `TIM6RST` writer - TIM6 timer reset"]
pub type TIM6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7RST` reader - TIM7 timer reset"]
pub type TIM7RST_R = crate::BitReader;
#[doc = "Field `TIM7RST` writer - TIM7 timer reset"]
pub type TIM7RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type SPI2RST_R = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type USART2RST_R = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type USART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub type USART3RST_R = crate::BitReader;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub type USART3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4RST` reader - USART4 reset"]
pub type USART4RST_R = crate::BitReader;
#[doc = "Field `USART4RST` writer - USART4 reset"]
pub type USART4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1RST` reader - LPUART1 reset"]
pub type LPUART1RST_R = crate::BitReader;
#[doc = "Field `LPUART1RST` writer - LPUART1 reset"]
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2C1RST_R = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2C2RST_R = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CECRST` reader - HDMI CEC reset"]
pub type CECRST_R = crate::BitReader;
#[doc = "Field `CECRST` writer - HDMI CEC reset"]
pub type CECRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD1RST` reader - UCPD1 reset"]
pub type UCPD1RST_R = crate::BitReader;
#[doc = "Field `UCPD1RST` writer - UCPD1 reset"]
pub type UCPD1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCPD2RST` reader - UCPD2 reset"]
pub type UCPD2RST_R = crate::BitReader;
#[doc = "Field `UCPD2RST` writer - UCPD2 reset"]
pub type UCPD2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRST` reader - Debug support reset"]
pub type DBGRST_R = crate::BitReader;
#[doc = "Field `DBGRST` writer - Debug support reset"]
pub type DBGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1RST` reader - DAC1 interface reset"]
pub type DAC1RST_R = crate::BitReader;
#[doc = "Field `DAC1RST` writer - DAC1 interface reset"]
pub type DAC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2RST` reader - Low Power Timer 2 reset"]
pub type LPTIM2RST_R = crate::BitReader;
#[doc = "Field `LPTIM2RST` writer - Low Power Timer 2 reset"]
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset"]
pub type LPTIM1RST_R = crate::BitReader;
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset"]
pub type LPTIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - HDMI CEC reset"]
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UCPD1 reset"]
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - UCPD2 reset"]
    #[inline(always)]
    pub fn ucpd2rst(&self) -> UCPD2RST_R {
        UCPD2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface reset"]
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Power Timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APBRSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APBRSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APBRSTR1rs> {
        TIM6RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<APBRSTR1rs> {
        TIM7RST_W::new(self, 5)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APBRSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APBRSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<APBRSTR1rs> {
        USART3RST_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart4rst(&mut self) -> USART4RST_W<APBRSTR1rs> {
        USART4RST_W::new(self, 19)
    }
    #[doc = "Bit 20 - LPUART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APBRSTR1rs> {
        LPUART1RST_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APBRSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APBRSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    #[doc = "Bit 24 - HDMI CEC reset"]
    #[inline(always)]
    #[must_use]
    pub fn cecrst(&mut self) -> CECRST_W<APBRSTR1rs> {
        CECRST_W::new(self, 24)
    }
    #[doc = "Bit 25 - UCPD1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<APBRSTR1rs> {
        UCPD1RST_W::new(self, 25)
    }
    #[doc = "Bit 26 - UCPD2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd2rst(&mut self) -> UCPD2RST_W<APBRSTR1rs> {
        UCPD2RST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<APBRSTR1rs> {
        DBGRST_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<APBRSTR1rs> {
        PWRRST_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<APBRSTR1rs> {
        DAC1RST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Low Power Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APBRSTR1rs> {
        LPTIM2RST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APBRSTR1rs> {
        LPTIM1RST_W::new(self, 31)
    }
}
#[doc = "APB peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRSTR1rs;
impl crate::RegisterSpec for APBRSTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr1::R`](R) reader structure"]
impl crate::Readable for APBRSTR1rs {}
#[doc = "`write(|w| ..)` method takes [`apbrstr1::W`](W) writer structure"]
impl crate::Writable for APBRSTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBRSTR1 to value 0"]
impl crate::Resettable for APBRSTR1rs {
    const RESET_VALUE: u32 = 0;
}
