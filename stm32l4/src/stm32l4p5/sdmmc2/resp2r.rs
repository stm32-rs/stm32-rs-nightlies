///Register `RESP2R` reader
pub type R = crate::R<RESP2Rrs>;
///Field `CARDSTATUS2` reader - see Table 347
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - see Table 347
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
/**response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#SDMMC2:RESP2R)*/
pub struct RESP2Rrs;
impl crate::RegisterSpec for RESP2Rrs {
    type Ux = u32;
}
///`read()` method returns [`resp2r::R`](R) reader structure
impl crate::Readable for RESP2Rrs {}
///`reset()` method sets RESP2R to value 0
impl crate::Resettable for RESP2Rrs {
    const RESET_VALUE: u32 = 0;
}
