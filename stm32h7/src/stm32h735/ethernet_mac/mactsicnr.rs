#[doc = "Register `MACTSICNR` reader"]
pub type R = crate::R<MACTSICNRrs>;
#[doc = "Register `MACTSICNR` writer"]
pub type W = crate::W<MACTSICNRrs>;
#[doc = "Field `TSIC` reader - Timestamp Ingress Correction"]
pub type TSIC_R = crate::FieldReader<u32>;
#[doc = "Field `TSIC` writer - Timestamp Ingress Correction"]
pub type TSIC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Ingress Correction"]
    #[inline(always)]
    pub fn tsic(&self) -> TSIC_R {
        TSIC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Ingress Correction"]
    #[inline(always)]
    #[must_use]
    pub fn tsic(&mut self) -> TSIC_W<MACTSICNRrs> {
        TSIC_W::new(self, 0)
    }
}
#[doc = "Timestamp Ingress correction nanosecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsicnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsicnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSICNRrs;
impl crate::RegisterSpec for MACTSICNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsicnr::R`](R) reader structure"]
impl crate::Readable for MACTSICNRrs {}
#[doc = "`write(|w| ..)` method takes [`mactsicnr::W`](W) writer structure"]
impl crate::Writable for MACTSICNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSICNR to value 0"]
impl crate::Resettable for MACTSICNRrs {
    const RESET_VALUE: u32 = 0;
}
