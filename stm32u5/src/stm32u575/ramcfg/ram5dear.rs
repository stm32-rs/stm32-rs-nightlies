#[doc = "Register `RAM5DEAR` reader"]
pub type R = crate::R<RAM5DEARrs>;
#[doc = "Field `EDEA` reader - EDEA"]
pub type EDEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - EDEA"]
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5dear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM5DEARrs;
impl crate::RegisterSpec for RAM5DEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram5dear::R`](R) reader structure"]
impl crate::Readable for RAM5DEARrs {}
#[doc = "`reset()` method sets RAM5DEAR to value 0"]
impl crate::Resettable for RAM5DEARrs {
    const RESET_VALUE: u32 = 0;
}
