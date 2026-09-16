///Register `RESP3R` reader
pub type R = crate::R<RESP3Rrs>;
///Field `CARDSTATUS3` reader - CARDSTATUS3
pub type CARDSTATUS3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CARDSTATUS3
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP3R")
            .field("cardstatus3", &self.cardstatus3())
            .finish()
    }
}
/**The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.

You can [`read`](crate::Reg::read) this register and get [`resp3r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:RESP3R)*/
pub struct RESP3Rrs;
impl crate::RegisterSpec for RESP3Rrs {
    type Ux = u32;
}
///`read()` method returns [`resp3r::R`](R) reader structure
impl crate::Readable for RESP3Rrs {}
///`reset()` method sets RESP3R to value 0
impl crate::Resettable for RESP3Rrs {}
