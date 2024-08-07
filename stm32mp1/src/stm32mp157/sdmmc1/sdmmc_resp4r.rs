///Register `SDMMC_RESP4R` reader
pub type R = crate::R<SDMMC_RESP4Rrs>;
///Field `CARDSTATUS4` reader - CARDSTATUS4
pub type CARDSTATUS4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CARDSTATUS4
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_RESP4R")
            .field("cardstatus4", &self.cardstatus4())
            .finish()
    }
}
/**The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_resp4r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SDMMC1:SDMMC_RESP4R)*/
pub struct SDMMC_RESP4Rrs;
impl crate::RegisterSpec for SDMMC_RESP4Rrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_resp4r::R`](R) reader structure
impl crate::Readable for SDMMC_RESP4Rrs {}
///`reset()` method sets SDMMC_RESP4R to value 0
impl crate::Resettable for SDMMC_RESP4Rrs {
    const RESET_VALUE: u32 = 0;
}
