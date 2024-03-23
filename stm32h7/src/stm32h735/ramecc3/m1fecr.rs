#[doc = "Register `M1FECR` reader"]
pub type R = crate::R<M1FECRrs>;
#[doc = "Register `M1FECR` writer"]
pub type W = crate::W<M1FECRrs>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FEC_R = crate::FieldReader<u32>;
#[doc = "Field `FEC` writer - Failing error code"]
pub type FEC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<M1FECRrs> {
        FEC_W::new(self, 0)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1fecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1FECRrs;
impl crate::RegisterSpec for M1FECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1fecr::R`](R) reader structure"]
impl crate::Readable for M1FECRrs {}
#[doc = "`write(|w| ..)` method takes [`m1fecr::W`](W) writer structure"]
impl crate::Writable for M1FECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M1FECR to value 0"]
impl crate::Resettable for M1FECRrs {
    const RESET_VALUE: u32 = 0;
}
