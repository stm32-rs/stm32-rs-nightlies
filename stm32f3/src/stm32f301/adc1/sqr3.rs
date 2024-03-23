#[doc = "Register `SQR3` reader"]
pub type R = crate::R<SQR3rs>;
#[doc = "Register `SQR3` writer"]
pub type W = crate::W<SQR3rs>;
#[doc = "Field `SQ10` reader - 10th conversion in regular sequence"]
pub type SQ10_R = crate::FieldReader;
#[doc = "Field `SQ10` writer - 10th conversion in regular sequence"]
pub type SQ10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ11` reader - 11th conversion in regular sequence"]
pub type SQ11_R = crate::FieldReader;
#[doc = "Field `SQ11` writer - 11th conversion in regular sequence"]
pub type SQ11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ12` reader - 13th conversion in regular sequence"]
pub type SQ12_R = crate::FieldReader;
#[doc = "Field `SQ12` writer - 13th conversion in regular sequence"]
pub type SQ12_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ13` reader - 13th conversion in regular sequence"]
pub type SQ13_R = crate::FieldReader;
#[doc = "Field `SQ13` writer - 13th conversion in regular sequence"]
pub type SQ13_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ14` reader - 14th conversion in regular sequence"]
pub type SQ14_R = crate::FieldReader;
#[doc = "Field `SQ14` writer - 14th conversion in regular sequence"]
pub type SQ14_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 10th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq10(&mut self) -> SQ10_W<SQR3rs> {
        SQ10_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - 11th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq11(&mut self) -> SQ11_W<SQR3rs> {
        SQ11_W::new(self, 6)
    }
    #[doc = "Bits 12:16 - 13th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq12(&mut self) -> SQ12_W<SQR3rs> {
        SQ12_W::new(self, 12)
    }
    #[doc = "Bits 18:22 - 13th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> SQ13_W<SQR3rs> {
        SQ13_W::new(self, 18)
    }
    #[doc = "Bits 24:28 - 14th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> SQ14_W<SQR3rs> {
        SQ14_W::new(self, 24)
    }
}
#[doc = "ADC regular sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR3rs;
impl crate::RegisterSpec for SQR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr3::R`](R) reader structure"]
impl crate::Readable for SQR3rs {}
#[doc = "`write(|w| ..)` method takes [`sqr3::W`](W) writer structure"]
impl crate::Writable for SQR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR3 to value 0"]
impl crate::Resettable for SQR3rs {
    const RESET_VALUE: u32 = 0;
}
