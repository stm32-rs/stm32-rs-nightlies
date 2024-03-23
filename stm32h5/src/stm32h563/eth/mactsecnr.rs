#[doc = "Register `MACTSECNR` reader"]
pub type R = crate::R<MACTSECNRrs>;
#[doc = "Register `MACTSECNR` writer"]
pub type W = crate::W<MACTSECNRrs>;
#[doc = "Field `TSEC` reader - Timestamp Egress Correction This field contains the nanoseconds part of the egress path correction value as defined by the Egress Correction expression."]
pub type TSEC_R = crate::FieldReader<u32>;
#[doc = "Field `TSEC` writer - Timestamp Egress Correction This field contains the nanoseconds part of the egress path correction value as defined by the Egress Correction expression."]
pub type TSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Egress Correction This field contains the nanoseconds part of the egress path correction value as defined by the Egress Correction expression."]
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Egress Correction This field contains the nanoseconds part of the egress path correction value as defined by the Egress Correction expression."]
    #[inline(always)]
    #[must_use]
    pub fn tsec(&mut self) -> TSEC_W<MACTSECNRrs> {
        TSEC_W::new(self, 0)
    }
}
#[doc = "Timestamp Egress correction nanosecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsecnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsecnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSECNRrs;
impl crate::RegisterSpec for MACTSECNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsecnr::R`](R) reader structure"]
impl crate::Readable for MACTSECNRrs {}
#[doc = "`write(|w| ..)` method takes [`mactsecnr::W`](W) writer structure"]
impl crate::Writable for MACTSECNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSECNR to value 0"]
impl crate::Resettable for MACTSECNRrs {
    const RESET_VALUE: u32 = 0;
}
