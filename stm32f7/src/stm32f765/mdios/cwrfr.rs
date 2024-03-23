#[doc = "Register `CWRFR` reader"]
pub type R = crate::R<CWRFRrs>;
#[doc = "Register `CWRFR` writer"]
pub type W = crate::W<CWRFRrs>;
#[doc = "Field `CWRF` reader - Clear the write flag"]
pub type CWRF_R = crate::FieldReader<u32>;
#[doc = "Field `CWRF` writer - Clear the write flag"]
pub type CWRF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwrf(&mut self) -> CWRF_W<CWRFRrs> {
        CWRF_W::new(self, 0)
    }
}
#[doc = "MDIOS clear write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwrfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwrfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWRFRrs;
impl crate::RegisterSpec for CWRFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwrfr::R`](R) reader structure"]
impl crate::Readable for CWRFRrs {}
#[doc = "`write(|w| ..)` method takes [`cwrfr::W`](W) writer structure"]
impl crate::Writable for CWRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWRFR to value 0"]
impl crate::Resettable for CWRFRrs {
    const RESET_VALUE: u32 = 0;
}
