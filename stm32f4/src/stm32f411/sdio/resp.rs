#[doc = "Register `RESP%s` reader"]
pub type R = crate::R<RESPrs>;
#[doc = "Field `CARDSTATUS` reader - Status of a card, which is part of the received response"]
pub type CARDSTATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of a card, which is part of the received response"]
    #[inline(always)]
    pub fn cardstatus(&self) -> CARDSTATUS_R {
        CARDSTATUS_R::new(self.bits)
    }
}
#[doc = "SDIO response %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPrs;
impl crate::RegisterSpec for RESPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp::R`](R) reader structure"]
impl crate::Readable for RESPrs {}
#[doc = "`reset()` method sets RESP%s to value 0"]
impl crate::Resettable for RESPrs {
    const RESET_VALUE: u32 = 0;
}
