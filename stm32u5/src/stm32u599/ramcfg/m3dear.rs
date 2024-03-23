#[doc = "Register `M3DEAR` reader"]
pub type R = crate::R<M3DEARrs>;
#[doc = "Field `EDEA` reader - EDEA"]
pub type EDEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - EDEA"]
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3dear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3DEARrs;
impl crate::RegisterSpec for M3DEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3dear::R`](R) reader structure"]
impl crate::Readable for M3DEARrs {}
#[doc = "`reset()` method sets M3DEAR to value 0"]
impl crate::Resettable for M3DEARrs {
    const RESET_VALUE: u32 = 0;
}
