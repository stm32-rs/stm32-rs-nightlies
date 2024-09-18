///Register `SDMMC_RESP2R` reader
pub type R = crate::R<SDMMC_RESP2Rrs>;
///Field `CARDSTATUS2` reader - see Table404.
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - see Table404.
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_RESP2R")
            .field("cardstatus2", &self.cardstatus2())
            .finish()
    }
}
/**The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#SDMMC1:SDMMC_RESP2R)*/
pub struct SDMMC_RESP2Rrs;
impl crate::RegisterSpec for SDMMC_RESP2Rrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_resp2r::R`](R) reader structure
impl crate::Readable for SDMMC_RESP2Rrs {}
///`reset()` method sets SDMMC_RESP2R to value 0
impl crate::Resettable for SDMMC_RESP2Rrs {
    const RESET_VALUE: u32 = 0;
}
