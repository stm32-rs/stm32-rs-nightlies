#[doc = "Register `SDMMC_RESP1R` reader"]
pub type R = crate::R<SDMMC_RESP1Rrs>;
#[doc = "Field `CARDSTATUS1` reader - CARDSTATUS1"]
pub type CARDSTATUS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_RESP1Rrs;
impl crate::RegisterSpec for SDMMC_RESP1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_resp1r::R`](R) reader structure"]
impl crate::Readable for SDMMC_RESP1Rrs {}
#[doc = "`reset()` method sets SDMMC_RESP1R to value 0"]
impl crate::Resettable for SDMMC_RESP1Rrs {
    const RESET_VALUE: u32 = 0;
}
