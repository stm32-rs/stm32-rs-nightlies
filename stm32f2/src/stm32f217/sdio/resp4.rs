#[doc = "Register `RESP4` reader"]
pub type R = crate::R<RESP4rs>;
#[doc = "Field `CARDSTATUS4` reader - see Table 132."]
pub type CARDSTATUS4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 132."]
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP4rs;
impl crate::RegisterSpec for RESP4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp4::R`](R) reader structure"]
impl crate::Readable for RESP4rs {}
#[doc = "`reset()` method sets RESP4 to value 0"]
impl crate::Resettable for RESP4rs {
    const RESET_VALUE: u32 = 0;
}
