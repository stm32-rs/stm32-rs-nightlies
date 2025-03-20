///Register `RESP1R` reader
pub type R = crate::R<RESP1Rrs>;
///Field `CARDSTATUS1` reader - see Table 347
pub type CARDSTATUS1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - see Table 347
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP1R")
            .field("cardstatus1", &self.cardstatus1())
            .finish()
    }
}
/**response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#SDMMC1:RESP1R)*/
pub struct RESP1Rrs;
impl crate::RegisterSpec for RESP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`resp1r::R`](R) reader structure
impl crate::Readable for RESP1Rrs {}
///`reset()` method sets RESP1R to value 0
impl crate::Resettable for RESP1Rrs {}
