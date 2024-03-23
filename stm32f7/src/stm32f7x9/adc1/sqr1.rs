#[doc = "Register `SQR1` reader"]
pub type R = crate::R<SQR1rs>;
#[doc = "Register `SQR1` writer"]
pub type W = crate::W<SQR1rs>;
#[doc = "Field `SQ13` reader - 13th conversion in regular sequence"]
pub type SQ13_R = crate::FieldReader;
#[doc = "Field `SQ13` writer - 13th conversion in regular sequence"]
pub type SQ13_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ14` reader - 14th conversion in regular sequence"]
pub type SQ14_R = crate::FieldReader;
#[doc = "Field `SQ14` writer - 14th conversion in regular sequence"]
pub type SQ14_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ15` reader - 15th conversion in regular sequence"]
pub type SQ15_R = crate::FieldReader;
#[doc = "Field `SQ15` writer - 15th conversion in regular sequence"]
pub type SQ15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ16` reader - 16th conversion in regular sequence"]
pub type SQ16_R = crate::FieldReader;
#[doc = "Field `SQ16` writer - 16th conversion in regular sequence"]
pub type SQ16_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L` reader - Regular channel sequence length"]
pub type L_R = crate::FieldReader;
#[doc = "Field `L` writer - Regular channel sequence length"]
pub type L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> SQ13_W<SQR1rs> {
        SQ13_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> SQ14_W<SQR1rs> {
        SQ14_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<SQR1rs> {
        SQ15_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<SQR1rs> {
        SQ16_W::new(self, 15)
    }
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<SQR1rs> {
        L_W::new(self, 20)
    }
}
#[doc = "regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR1rs;
impl crate::RegisterSpec for SQR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr1::R`](R) reader structure"]
impl crate::Readable for SQR1rs {}
#[doc = "`write(|w| ..)` method takes [`sqr1::W`](W) writer structure"]
impl crate::Writable for SQR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1rs {
    const RESET_VALUE: u32 = 0;
}
