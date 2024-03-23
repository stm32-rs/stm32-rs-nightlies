#[doc = "Register `RESP1R` reader"]
pub type R = crate::R<RESP1Rrs>;
#[doc = "Field `CARDSTATUS1` reader - see Table 432"]
pub type CARDSTATUS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table 432"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP1Rrs;
impl crate::RegisterSpec for RESP1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1r::R`](R) reader structure"]
impl crate::Readable for RESP1Rrs {}
#[doc = "`reset()` method sets RESP1R to value 0"]
impl crate::Resettable for RESP1Rrs {
    const RESET_VALUE: u32 = 0;
}
