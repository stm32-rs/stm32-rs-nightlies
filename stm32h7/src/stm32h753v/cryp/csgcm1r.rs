#[doc = "Register `CSGCM1R` reader"]
pub type R = crate::R<CSGCM1Rrs>;
#[doc = "Register `CSGCM1R` writer"]
pub type W = crate::W<CSGCM1Rrs>;
#[doc = "Field `CSGCM1` reader - CSGCM1"]
pub type CSGCM1_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM1` writer - CSGCM1"]
pub type CSGCM1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM1"]
    #[inline(always)]
    pub fn csgcm1(&self) -> CSGCM1_R {
        CSGCM1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM1"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm1(&mut self) -> CSGCM1_W<CSGCM1Rrs> {
        CSGCM1_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCM1Rrs;
impl crate::RegisterSpec for CSGCM1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm1r::R`](R) reader structure"]
impl crate::Readable for CSGCM1Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcm1r::W`](W) writer structure"]
impl crate::Writable for CSGCM1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCM1R to value 0"]
impl crate::Resettable for CSGCM1Rrs {
    const RESET_VALUE: u32 = 0;
}
