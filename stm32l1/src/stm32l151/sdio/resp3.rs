#[doc = "Register `RESP3` reader"]
pub type R = crate::R<RESP3rs>;
#[doc = "Field `CARDSTATUS3` reader - see Table 133."]
pub type CARDSTATUS3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 133."]
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP3rs;
impl crate::RegisterSpec for RESP3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp3::R`](R) reader structure"]
impl crate::Readable for RESP3rs {}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for RESP3rs {
    const RESET_VALUE: u32 = 0;
}
