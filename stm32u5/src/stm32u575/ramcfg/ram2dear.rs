#[doc = "Register `RAM2DEAR` reader"]
pub type R = crate::R<RAM2DEARrs>;
#[doc = "Field `EDEA` reader - EDEA"]
pub type EDEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - EDEA"]
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2dear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM2DEARrs;
impl crate::RegisterSpec for RAM2DEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram2dear::R`](R) reader structure"]
impl crate::Readable for RAM2DEARrs {}
#[doc = "`reset()` method sets RAM2DEAR to value 0"]
impl crate::Resettable for RAM2DEARrs {
    const RESET_VALUE: u32 = 0;
}
