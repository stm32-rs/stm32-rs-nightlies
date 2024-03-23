#[doc = "Register `RNG_CR` reader"]
pub type R = crate::R<RNG_CRrs>;
#[doc = "Register `RNG_CR` writer"]
pub type W = crate::W<RNG_CRrs>;
#[doc = "Field `RNGEN` reader - RNGEN"]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - RNGEN"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - IE"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - IE"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CED` reader - CED"]
pub type CED_R = crate::BitReader;
#[doc = "Field `CED` writer - CED"]
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - RNGEN"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CED"]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RNGEN"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<RNG_CRrs> {
        RNGEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<RNG_CRrs> {
        IE_W::new(self, 3)
    }
    #[doc = "Bit 5 - CED"]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<RNG_CRrs> {
        CED_W::new(self, 5)
    }
}
#[doc = "RNG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_CRrs;
impl crate::RegisterSpec for RNG_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_cr::R`](R) reader structure"]
impl crate::Readable for RNG_CRrs {}
#[doc = "`write(|w| ..)` method takes [`rng_cr::W`](W) writer structure"]
impl crate::Writable for RNG_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_CR to value 0"]
impl crate::Resettable for RNG_CRrs {
    const RESET_VALUE: u32 = 0;
}
