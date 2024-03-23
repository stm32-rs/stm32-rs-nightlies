#[doc = "Register `M3SEAR` reader"]
pub type R = crate::R<M3SEARrs>;
#[doc = "Field `ESEA` reader - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error."]
pub type ESEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error."]
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG memory 3 ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3sear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3SEARrs;
impl crate::RegisterSpec for M3SEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3sear::R`](R) reader structure"]
impl crate::Readable for M3SEARrs {}
#[doc = "`reset()` method sets M3SEAR to value 0"]
impl crate::Resettable for M3SEARrs {
    const RESET_VALUE: u32 = 0;
}
