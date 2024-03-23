#[doc = "Register `HSEM_C2ICR` reader"]
pub type R = crate::R<HSEM_C2ICRrs>;
#[doc = "Register `HSEM_C2ICR` writer"]
pub type W = crate::W<HSEM_C2ICRrs>;
#[doc = "Field `ISC` reader - ISC"]
pub type ISC_R = crate::FieldReader<u32>;
#[doc = "Field `ISC` writer - ISC"]
pub type ISC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ISC"]
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISC"]
    #[inline(always)]
    #[must_use]
    pub fn isc(&mut self) -> ISC_W<HSEM_C2ICRrs> {
        ISC_W::new(self, 0)
    }
}
#[doc = "HSEM i2terrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c2icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c2icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C2ICRrs;
impl crate::RegisterSpec for HSEM_C2ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c2icr::R`](R) reader structure"]
impl crate::Readable for HSEM_C2ICRrs {}
#[doc = "`write(|w| ..)` method takes [`hsem_c2icr::W`](W) writer structure"]
impl crate::Writable for HSEM_C2ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSEM_C2ICR to value 0"]
impl crate::Resettable for HSEM_C2ICRrs {
    const RESET_VALUE: u32 = 0;
}
