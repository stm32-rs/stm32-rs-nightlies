#[doc = "Register `SQR2` reader"]
pub type R = crate::R<SQR2rs>;
#[doc = "Register `SQR2` writer"]
pub type W = crate::W<SQR2rs>;
#[doc = "Field `SQ19` reader - 19th conversion in regular sequence"]
pub type SQ19_R = crate::FieldReader;
#[doc = "Field `SQ19` writer - 19th conversion in regular sequence"]
pub type SQ19_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ20` reader - 20th conversion in regular sequence"]
pub type SQ20_R = crate::FieldReader;
#[doc = "Field `SQ20` writer - 20th conversion in regular sequence"]
pub type SQ20_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ21` reader - 21st conversion in regular sequence"]
pub type SQ21_R = crate::FieldReader;
#[doc = "Field `SQ21` writer - 21st conversion in regular sequence"]
pub type SQ21_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ22` reader - 22nd conversion in regular sequence"]
pub type SQ22_R = crate::FieldReader;
#[doc = "Field `SQ22` writer - 22nd conversion in regular sequence"]
pub type SQ22_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ23` reader - 23rd conversion in regular sequence"]
pub type SQ23_R = crate::FieldReader;
#[doc = "Field `SQ23` writer - 23rd conversion in regular sequence"]
pub type SQ23_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ24` reader - 24th conversion in regular sequence"]
pub type SQ24_R = crate::FieldReader;
#[doc = "Field `SQ24` writer - 24th conversion in regular sequence"]
pub type SQ24_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq19(&self) -> SQ19_R {
        SQ19_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq20(&self) -> SQ20_R {
        SQ20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq21(&self) -> SQ21_R {
        SQ21_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq22(&self) -> SQ22_R {
        SQ22_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq23(&self) -> SQ23_R {
        SQ23_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq24(&self) -> SQ24_R {
        SQ24_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq19(&mut self) -> SQ19_W<SQR2rs> {
        SQ19_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq20(&mut self) -> SQ20_W<SQR2rs> {
        SQ20_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq21(&mut self) -> SQ21_W<SQR2rs> {
        SQ21_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq22(&mut self) -> SQ22_W<SQR2rs> {
        SQ22_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq23(&mut self) -> SQ23_W<SQR2rs> {
        SQ23_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq24(&mut self) -> SQ24_W<SQR2rs> {
        SQ24_W::new(self, 25)
    }
}
#[doc = "regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR2rs;
impl crate::RegisterSpec for SQR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr2::R`](R) reader structure"]
impl crate::Readable for SQR2rs {}
#[doc = "`write(|w| ..)` method takes [`sqr2::W`](W) writer structure"]
impl crate::Writable for SQR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2rs {
    const RESET_VALUE: u32 = 0;
}
