#[doc = "Register `M2DEAR` reader"]
pub type R = crate::R<M2DEARrs>;
#[doc = "Field `EDEA` reader - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error."]
pub type EDEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error."]
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG memory 2 ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2dear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2DEARrs;
impl crate::RegisterSpec for M2DEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2dear::R`](R) reader structure"]
impl crate::Readable for M2DEARrs {}
#[doc = "`reset()` method sets M2DEAR to value 0"]
impl crate::Resettable for M2DEARrs {
    const RESET_VALUE: u32 = 0;
}
