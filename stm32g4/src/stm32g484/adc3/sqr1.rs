#[doc = "Register `SQR1` reader"]
pub type R = crate::R<SQR1rs>;
#[doc = "Register `SQR1` writer"]
pub type W = crate::W<SQR1rs>;
#[doc = "Field `L` reader - Regular channel sequence length"]
pub type L_R = crate::FieldReader;
#[doc = "Field `L` writer - Regular channel sequence length"]
pub type L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `SQ1` reader - 1st conversion in regular sequence"]
pub type SQ1_R = crate::FieldReader;
#[doc = "Field `SQ1` writer - 1st conversion in regular sequence"]
pub type SQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ2` reader - 2nd conversion in regular sequence"]
pub type SQ2_R = crate::FieldReader;
#[doc = "Field `SQ2` writer - 2nd conversion in regular sequence"]
pub type SQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ3` reader - 3rd conversion in regular sequence"]
pub type SQ3_R = crate::FieldReader;
#[doc = "Field `SQ3` writer - 3rd conversion in regular sequence"]
pub type SQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ4` reader - 4th conversion in regular sequence"]
pub type SQ4_R = crate::FieldReader;
#[doc = "Field `SQ4` writer - 4th conversion in regular sequence"]
pub type SQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<SQR1rs> {
        L_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - 1st conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<SQR1rs> {
        SQ1_W::new(self, 6)
    }
    #[doc = "Bits 12:16 - 2nd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<SQR1rs> {
        SQ2_W::new(self, 12)
    }
    #[doc = "Bits 18:22 - 3rd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<SQR1rs> {
        SQ3_W::new(self, 18)
    }
    #[doc = "Bits 24:28 - 4th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<SQR1rs> {
        SQ4_W::new(self, 24)
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
