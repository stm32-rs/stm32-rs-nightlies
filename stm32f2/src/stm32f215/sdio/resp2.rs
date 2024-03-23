#[doc = "Register `RESP2` reader"]
pub type R = crate::R<RESP2rs>;
#[doc = "Field `CARDSTATUS2` reader - see Table 132."]
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 132."]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP2rs;
impl crate::RegisterSpec for RESP2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp2::R`](R) reader structure"]
impl crate::Readable for RESP2rs {}
#[doc = "`reset()` method sets RESP2 to value 0"]
impl crate::Resettable for RESP2rs {
    const RESET_VALUE: u32 = 0;
}
