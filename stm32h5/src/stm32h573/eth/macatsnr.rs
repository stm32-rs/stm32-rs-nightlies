#[doc = "Register `MACATSNR` reader"]
pub type R = crate::R<MACATSNRrs>;
#[doc = "Field `AUXTSLO` reader - Auxiliary Timestamp Contains the lower 31 bits (nanoseconds field) of the auxiliary timestamp."]
pub type AUXTSLO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Auxiliary Timestamp Contains the lower 31 bits (nanoseconds field) of the auxiliary timestamp."]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "Auxiliary timestamp nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macatsnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACATSNRrs;
impl crate::RegisterSpec for MACATSNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macatsnr::R`](R) reader structure"]
impl crate::Readable for MACATSNRrs {}
#[doc = "`reset()` method sets MACATSNR to value 0"]
impl crate::Resettable for MACATSNRrs {
    const RESET_VALUE: u32 = 0;
}
