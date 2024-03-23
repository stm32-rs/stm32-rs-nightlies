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
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type SPI2RST_R = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type USART2RST_R = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type USART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2C1RST_R = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2C2RST_R = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRST` reader - Debug support reset"]
pub type DBGRST_R = crate::BitReader;
#[doc = "Field `DBGRST` writer - Debug support reset"]
pub type DBGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
