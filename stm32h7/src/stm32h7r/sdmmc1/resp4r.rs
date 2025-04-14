///Register `RESP4R` reader
pub type R = crate::R<RESP4Rrs>;
///Field `CARDSTATUS` reader - Card status according table below See Table 444.
pub type CARDSTATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Card status according table below See Table 444.
    #[inline(always)]
    pub fn cardstatus(&self) -> CARDSTATUS_R {
        CARDSTATUS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP4R")
            .field("cardstatus", &self.cardstatus())
            .finish()
    }
}
/**SDMMC response 4 register

You can [`read`](crate::Reg::read) this register and get [`resp4r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SDMMC1:RESP4R)*/
pub struct RESP4Rrs;
impl crate::RegisterSpec for RESP4Rrs {
    type Ux = u32;
}
///`read()` method returns [`resp4r::R`](R) reader structure
impl crate::Readable for RESP4Rrs {}
///`reset()` method sets RESP4R to value 0
impl crate::Resettable for RESP4Rrs {}
