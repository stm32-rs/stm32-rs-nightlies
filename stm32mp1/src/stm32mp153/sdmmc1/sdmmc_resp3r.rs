#[doc = "Register `SDMMC_RESP3R` reader"]
pub type R = crate::R<SDMMC_RESP3Rrs>;
#[doc = "Field `CARDSTATUS3` reader - CARDSTATUS3"]
pub type CARDSTATUS3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS3"]
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
#[doc = "The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_resp3r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_RESP3Rrs;
impl crate::RegisterSpec for SDMMC_RESP3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_resp3r::R`](R) reader structure"]
impl crate::Readable for SDMMC_RESP3Rrs {}
#[doc = "`reset()` method sets SDMMC_RESP3R to value 0"]
impl crate::Resettable for SDMMC_RESP3Rrs {
    const RESET_VALUE: u32 = 0;
}
