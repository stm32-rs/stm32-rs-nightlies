#[doc = "Register `RESP2R` reader"]
pub type R = crate::R<RESP2Rrs>;
#[doc = "Field `CARDSTATUS2` reader - see Table 347"]
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 347"]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
