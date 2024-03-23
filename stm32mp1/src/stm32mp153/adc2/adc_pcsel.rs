#[doc = "Register `ADC_PCSEL` reader"]
pub type R = crate::R<ADC_PCSELrs>;
#[doc = "Register `ADC_PCSEL` writer"]
pub type W = crate::W<ADC_PCSELrs>;
#[doc = "Field `PCSEL0` reader - PCSEL0"]
pub type PCSEL0_R = crate::BitReader;
#[doc = "Field `PCSEL0` writer - PCSEL0"]
pub type PCSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL1` reader - PCSEL1"]
pub type PCSEL1_R = crate::BitReader;
#[doc = "Field `PCSEL1` writer - PCSEL1"]
pub type PCSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL2` reader - PCSEL2"]
pub type PCSEL2_R = crate::BitReader;
#[doc = "Field `PCSEL2` writer - PCSEL2"]
pub type PCSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL3` reader - PCSEL3"]
pub type PCSEL3_R = crate::BitReader;
#[doc = "Field `PCSEL3` writer - PCSEL3"]
pub type PCSEL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL4` reader - PCSEL4"]
pub type PCSEL4_R = crate::BitReader;
#[doc = "Field `PCSEL4` writer - PCSEL4"]
pub type PCSEL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL5` reader - PCSEL5"]
pub type PCSEL5_R = crate::BitReader;
#[doc = "Field `PCSEL5` writer - PCSEL5"]
pub type PCSEL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL6` reader - PCSEL6"]
pub type PCSEL6_R = crate::BitReader;
#[doc = "Field `PCSEL6` writer - PCSEL6"]
pub type PCSEL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL7` reader - PCSEL7"]
pub type PCSEL7_R = crate::BitReader;
#[doc = "Field `PCSEL7` writer - PCSEL7"]
pub type PCSEL7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL8` reader - PCSEL8"]
pub type PCSEL8_R = crate::BitReader;
#[doc = "Field `PCSEL8` writer - PCSEL8"]
pub type PCSEL8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL9` reader - PCSEL9"]
pub type PCSEL9_R = crate::BitReader;
#[doc = "Field `PCSEL9` writer - PCSEL9"]
pub type PCSEL9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL10` reader - PCSEL10"]
pub type PCSEL10_R = crate::BitReader;
#[doc = "Field `PCSEL10` writer - PCSEL10"]
pub type PCSEL10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL11` reader - PCSEL11"]
pub type PCSEL11_R = crate::BitReader;
#[doc = "Field `PCSEL11` writer - PCSEL11"]
pub type PCSEL11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL12` reader - PCSEL12"]
pub type PCSEL12_R = crate::BitReader;
#[doc = "Field `PCSEL12` writer - PCSEL12"]
pub type PCSEL12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL13` reader - PCSEL13"]
pub type PCSEL13_R = crate::BitReader;
#[doc = "Field `PCSEL13` writer - PCSEL13"]
pub type PCSEL13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL14` reader - PCSEL14"]
pub type PCSEL14_R = crate::BitReader;
#[doc = "Field `PCSEL14` writer - PCSEL14"]
pub type PCSEL14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL15` reader - PCSEL15"]
pub type PCSEL15_R = crate::BitReader;
#[doc = "Field `PCSEL15` writer - PCSEL15"]
pub type PCSEL15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL16` reader - PCSEL16"]
pub type PCSEL16_R = crate::BitReader;
#[doc = "Field `PCSEL16` writer - PCSEL16"]
pub type PCSEL16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL17` reader - PCSEL17"]
pub type PCSEL17_R = crate::BitReader;
#[doc = "Field `PCSEL17` writer - PCSEL17"]
pub type PCSEL17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL18` reader - PCSEL18"]
pub type PCSEL18_R = crate::BitReader;
#[doc = "Field `PCSEL18` writer - PCSEL18"]
pub type PCSEL18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSEL19` reader - PCSEL19"]
pub type PCSEL19_R = crate::BitReader;
#[doc = "Field `PCSEL19` writer - PCSEL19"]
pub type PCSEL19_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCSEL0"]
    #[inline(always)]
    pub fn pcsel0(&self) -> PCSEL0_R {
        PCSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCSEL1"]
    #[inline(always)]
    pub fn pcsel1(&self) -> PCSEL1_R {
        PCSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCSEL2"]
    #[inline(always)]
    pub fn pcsel2(&self) -> PCSEL2_R {
        PCSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCSEL3"]
    #[inline(always)]
    pub fn pcsel3(&self) -> PCSEL3_R {
        PCSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCSEL4"]
    #[inline(always)]
    pub fn pcsel4(&self) -> PCSEL4_R {
        PCSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCSEL5"]
    #[inline(always)]
    pub fn pcsel5(&self) -> PCSEL5_R {
        PCSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCSEL6"]
    #[inline(always)]
    pub fn pcsel6(&self) -> PCSEL6_R {
        PCSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCSEL7"]
    #[inline(always)]
    pub fn pcsel7(&self) -> PCSEL7_R {
        PCSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCSEL8"]
    #[inline(always)]
    pub fn pcsel8(&self) -> PCSEL8_R {
        PCSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCSEL9"]
    #[inline(always)]
    pub fn pcsel9(&self) -> PCSEL9_R {
        PCSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCSEL10"]
    #[inline(always)]
    pub fn pcsel10(&self) -> PCSEL10_R {
        PCSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCSEL11"]
    #[inline(always)]
    pub fn pcsel11(&self) -> PCSEL11_R {
        PCSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCSEL12"]
    #[inline(always)]
    pub fn pcsel12(&self) -> PCSEL12_R {
        PCSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCSEL13"]
    #[inline(always)]
    pub fn pcsel13(&self) -> PCSEL13_R {
        PCSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCSEL14"]
    #[inline(always)]
    pub fn pcsel14(&self) -> PCSEL14_R {
        PCSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCSEL15"]
    #[inline(always)]
    pub fn pcsel15(&self) -> PCSEL15_R {
        PCSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PCSEL16"]
    #[inline(always)]
    pub fn pcsel16(&self) -> PCSEL16_R {
        PCSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PCSEL17"]
    #[inline(always)]
    pub fn pcsel17(&self) -> PCSEL17_R {
        PCSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PCSEL18"]
    #[inline(always)]
    pub fn pcsel18(&self) -> PCSEL18_R {
        PCSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PCSEL19"]
    #[inline(always)]
    pub fn pcsel19(&self) -> PCSEL19_R {
        PCSEL19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel0(&mut self) -> PCSEL0_W<ADC_PCSELrs> {
        PCSEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - PCSEL1"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel1(&mut self) -> PCSEL1_W<ADC_PCSELrs> {
        PCSEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - PCSEL2"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel2(&mut self) -> PCSEL2_W<ADC_PCSELrs> {
        PCSEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - PCSEL3"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel3(&mut self) -> PCSEL3_W<ADC_PCSELrs> {
        PCSEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - PCSEL4"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel4(&mut self) -> PCSEL4_W<ADC_PCSELrs> {
        PCSEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - PCSEL5"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel5(&mut self) -> PCSEL5_W<ADC_PCSELrs> {
        PCSEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - PCSEL6"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel6(&mut self) -> PCSEL6_W<ADC_PCSELrs> {
        PCSEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - PCSEL7"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel7(&mut self) -> PCSEL7_W<ADC_PCSELrs> {
        PCSEL7_W::new(self, 7)
    }
    #[doc = "Bit 8 - PCSEL8"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel8(&mut self) -> PCSEL8_W<ADC_PCSELrs> {
        PCSEL8_W::new(self, 8)
    }
    #[doc = "Bit 9 - PCSEL9"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel9(&mut self) -> PCSEL9_W<ADC_PCSELrs> {
        PCSEL9_W::new(self, 9)
    }
    #[doc = "Bit 10 - PCSEL10"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel10(&mut self) -> PCSEL10_W<ADC_PCSELrs> {
        PCSEL10_W::new(self, 10)
    }
    #[doc = "Bit 11 - PCSEL11"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel11(&mut self) -> PCSEL11_W<ADC_PCSELrs> {
        PCSEL11_W::new(self, 11)
    }
    #[doc = "Bit 12 - PCSEL12"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel12(&mut self) -> PCSEL12_W<ADC_PCSELrs> {
        PCSEL12_W::new(self, 12)
    }
    #[doc = "Bit 13 - PCSEL13"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel13(&mut self) -> PCSEL13_W<ADC_PCSELrs> {
        PCSEL13_W::new(self, 13)
    }
    #[doc = "Bit 14 - PCSEL14"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel14(&mut self) -> PCSEL14_W<ADC_PCSELrs> {
        PCSEL14_W::new(self, 14)
    }
    #[doc = "Bit 15 - PCSEL15"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel15(&mut self) -> PCSEL15_W<ADC_PCSELrs> {
        PCSEL15_W::new(self, 15)
    }
    #[doc = "Bit 16 - PCSEL16"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel16(&mut self) -> PCSEL16_W<ADC_PCSELrs> {
        PCSEL16_W::new(self, 16)
    }
    #[doc = "Bit 17 - PCSEL17"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel17(&mut self) -> PCSEL17_W<ADC_PCSELrs> {
        PCSEL17_W::new(self, 17)
    }
    #[doc = "Bit 18 - PCSEL18"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel18(&mut self) -> PCSEL18_W<ADC_PCSELrs> {
        PCSEL18_W::new(self, 18)
    }
    #[doc = "Bit 19 - PCSEL19"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel19(&mut self) -> PCSEL19_W<ADC_PCSELrs> {
        PCSEL19_W::new(self, 19)
    }
}
#[doc = "ADC channel preselection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_pcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_pcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_PCSELrs;
impl crate::RegisterSpec for ADC_PCSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_pcsel::R`](R) reader structure"]
impl crate::Readable for ADC_PCSELrs {}
#[doc = "`write(|w| ..)` method takes [`adc_pcsel::W`](W) writer structure"]
impl crate::Writable for ADC_PCSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_PCSEL to value 0"]
impl crate::Resettable for ADC_PCSELrs {
    const RESET_VALUE: u32 = 0;
}
