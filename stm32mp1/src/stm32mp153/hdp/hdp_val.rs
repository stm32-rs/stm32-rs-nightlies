#[doc = "Register `HDP_VAL` reader"]
pub type R = crate::R<HDP_VALrs>;
#[doc = "Field `HDPVAL` reader - HDPVAL"]
pub type HDPVAL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - HDPVAL"]
    #[inline(always)]
    pub fn hdpval(&self) -> HDPVAL_R {
        HDPVAL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "HDP value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp_val::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP_VALrs;
impl crate::RegisterSpec for HDP_VALrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp_val::R`](R) reader structure"]
impl crate::Readable for HDP_VALrs {}
#[doc = "`reset()` method sets HDP_VAL to value 0"]
impl crate::Resettable for HDP_VALrs {
    const RESET_VALUE: u32 = 0;
}
