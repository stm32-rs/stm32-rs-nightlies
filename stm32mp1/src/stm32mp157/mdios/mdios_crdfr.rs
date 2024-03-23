#[doc = "Register `MDIOS_CRDFR` reader"]
pub type R = crate::R<MDIOS_CRDFRrs>;
#[doc = "Register `MDIOS_CRDFR` writer"]
pub type W = crate::W<MDIOS_CRDFRrs>;
#[doc = "Field `CRDF` reader - CRDF"]
pub type CRDF_R = crate::FieldReader<u32>;
#[doc = "Field `CRDF` writer - CRDF"]
pub type CRDF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRDF"]
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRDF"]
    #[inline(always)]
    #[must_use]
    pub fn crdf(&mut self) -> CRDF_W<MDIOS_CRDFRrs> {
        CRDF_W::new(self, 0)
    }
}
#[doc = "MDIOS clear read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_crdfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_crdfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_CRDFRrs;
impl crate::RegisterSpec for MDIOS_CRDFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_crdfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_CRDFRrs {}
#[doc = "`write(|w| ..)` method takes [`mdios_crdfr::W`](W) writer structure"]
impl crate::Writable for MDIOS_CRDFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIOS_CRDFR to value 0"]
impl crate::Resettable for MDIOS_CRDFRrs {
    const RESET_VALUE: u32 = 0;
}
