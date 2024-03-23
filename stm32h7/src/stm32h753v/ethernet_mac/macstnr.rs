#[doc = "Register `MACSTNR` reader"]
pub type R = crate::R<MACSTNRrs>;
#[doc = "Field `TSSS` reader - Timestamp Sub-seconds"]
pub type TSSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub-seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "System time nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTNRrs;
impl crate::RegisterSpec for MACSTNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstnr::R`](R) reader structure"]
impl crate::Readable for MACSTNRrs {}
#[doc = "`reset()` method sets MACSTNR to value 0"]
impl crate::Resettable for MACSTNRrs {
    const RESET_VALUE: u32 = 0;
}
