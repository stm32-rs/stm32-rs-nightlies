#[doc = "Register `PR2` reader"]
pub type R = crate::R<PR2rs>;
#[doc = "Register `PR2` writer"]
pub type W = crate::W<PR2rs>;
#[doc = "Field `PIF33` reader - Configurable event inputs x+32 Pending bit."]
pub type PIF33_R = crate::BitReader;
#[doc = "Field `PIF33` writer - Configurable event inputs x+32 Pending bit."]
pub type PIF33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIF40_41` reader - Configurable event inputs x+32 Pending bit."]
pub type PIF40_41_R = crate::FieldReader;
#[doc = "Field `PIF40_41` writer - Configurable event inputs x+32 Pending bit."]
pub type PIF40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    pub fn pif40_41(&self) -> PIF40_41_R {
        PIF40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif33(&mut self) -> PIF33_W<PR2rs> {
        PIF33_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Configurable event inputs x+32 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif40_41(&mut self) -> PIF40_41_W<PR2rs> {
        PIF40_41_W::new(self, 8)
    }
}
#[doc = "pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr2::R`](R) reader structure"]
impl crate::Readable for PR2rs {}
#[doc = "`write(|w| ..)` method takes [`pr2::W`](W) writer structure"]
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2rs {
    const RESET_VALUE: u32 = 0;
}
