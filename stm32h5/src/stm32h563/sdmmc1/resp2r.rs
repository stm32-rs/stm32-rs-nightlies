#[doc = "Register `RESP2R` reader"]
pub type R = crate::R<RESP2Rrs>;
#[doc = "Field `CARDSTATUSx` reader - Card status x See ."]
pub type CARDSTATUSX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card status x See ."]
    #[inline(always)]
    pub fn cardstatusx(&self) -> CARDSTATUSX_R {
        CARDSTATUSX_R::new(self.bits)
    }
}
#[doc = "SDMMC response 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP2Rrs;
impl crate::RegisterSpec for RESP2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2r::R`](R) reader structure"]
impl crate::Readable for RESP2Rrs {}
#[doc = "`reset()` method sets RESP2R to value 0"]
impl crate::Resettable for RESP2Rrs {
    const RESET_VALUE: u32 = 0;
}
