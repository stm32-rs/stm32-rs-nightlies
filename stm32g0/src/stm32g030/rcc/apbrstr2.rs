#[doc = "Register `APBRSTR2` reader"]
pub type R = crate::R<APBRSTR2rs>;
#[doc = "Register `APBRSTR2` writer"]
pub type W = crate::W<APBRSTR2rs>;
#[doc = "Field `SYSCFGRST` reader - SYSCFG, COMP and VREFBUF reset"]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFG, COMP and VREFBUF reset"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14RST` reader - TIM14 timer reset"]
pub type TIM14RST_R = crate::BitReader;
#[doc = "Field `TIM14RST` writer - TIM14 timer reset"]
pub type TIM14RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub type TIM16RST_R = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub type TIM17RST_R = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type ADCRST_R = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APBRSTR2rs> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APBRSTR2rs> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APBRSTR2rs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APBRSTR2rs> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim14rst(&mut self) -> TIM14RST_W<APBRSTR2rs> {
        TIM14RST_W::new(self, 15)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APBRSTR2rs> {
        TIM16RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<APBRSTR2rs> {
        TIM17RST_W::new(self, 18)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<APBRSTR2rs> {
        ADCRST_W::new(self, 20)
    }
}
#[doc = "APB peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbrstr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbrstr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRSTR2rs;
impl crate::RegisterSpec for APBRSTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr2::R`](R) reader structure"]
impl crate::Readable for APBRSTR2rs {}
#[doc = "`write(|w| ..)` method takes [`apbrstr2::W`](W) writer structure"]
impl crate::Writable for APBRSTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBRSTR2 to value 0"]
impl crate::Resettable for APBRSTR2rs {
    const RESET_VALUE: u32 = 0;
}
