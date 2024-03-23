#[doc = "Register `CSGCM3R` reader"]
pub type R = crate::R<CSGCM3Rrs>;
#[doc = "Register `CSGCM3R` writer"]
pub type W = crate::W<CSGCM3Rrs>;
#[doc = "Field `CSGCM3` reader - CSGCM3"]
pub type CSGCM3_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM3` writer - CSGCM3"]
pub type CSGCM3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM3"]
    #[inline(always)]
    pub fn csgcm3(&self) -> CSGCM3_R {
        CSGCM3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM3"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm3(&mut self) -> CSGCM3_W<CSGCM3Rrs> {
        CSGCM3_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcm3r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcm3r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCM3Rrs;
impl crate::RegisterSpec for CSGCM3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcm3r::R`](R) reader structure"]
impl crate::Readable for CSGCM3Rrs {}
#[doc = "`write(|w| ..)` method takes [`csgcm3r::W`](W) writer structure"]
impl crate::Writable for CSGCM3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCM3R to value 0"]
impl crate::Resettable for CSGCM3Rrs {
    const RESET_VALUE: u32 = 0;
}
