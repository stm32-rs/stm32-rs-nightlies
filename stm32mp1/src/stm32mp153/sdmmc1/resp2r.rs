///Register `RESP2R` reader
pub type R = crate::R<RESP2Rrs>;
///Field `CARDSTATUS2` reader - CARDSTATUS2
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CARDSTATUS2
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP2R")
            .field("cardstatus2", &self.cardstatus2())
            .finish()
    }
}
/**The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`resp2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:RESP2R)*/
pub struct RESP2Rrs;
impl crate::RegisterSpec for RESP2Rrs {
    type Ux = u32;
}
///`read()` method returns [`resp2r::R`](R) reader structure
impl crate::Readable for RESP2Rrs {}
///`reset()` method sets RESP2R to value 0
impl crate::Resettable for RESP2Rrs {}
