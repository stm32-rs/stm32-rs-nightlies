#[doc = "Register `SDMMC_RESP2R` reader"]
pub type R = crate::R<SDMMC_RESP2Rrs>;
#[doc = "Field `CARDSTATUS2` reader - see Table404."]
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp2r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_RESP2Rrs;
impl crate::RegisterSpec for SDMMC_RESP2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_resp2r::R`](R) reader structure"]
impl crate::Readable for SDMMC_RESP2Rrs {}
#[doc = "`reset()` method sets SDMMC_RESP2R to value 0"]
impl crate::Resettable for SDMMC_RESP2Rrs {
    const RESET_VALUE: u32 = 0;
}
