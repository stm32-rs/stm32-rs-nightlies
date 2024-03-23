#[doc = "Register `RESP3R` reader"]
pub type R = crate::R<RESP3Rrs>;
#[doc = "Field `CARDSTATUS3` reader - see Table 347"]
pub type CARDSTATUS3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 347"]
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP3Rrs;
impl crate::RegisterSpec for RESP3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3r::R`](R) reader structure"]
impl crate::Readable for RESP3Rrs {}
#[doc = "`reset()` method sets RESP3R to value 0"]
impl crate::Resettable for RESP3Rrs {
    const RESET_VALUE: u32 = 0;
}
