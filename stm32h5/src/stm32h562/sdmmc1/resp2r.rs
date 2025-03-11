///Register `RESP2R` reader
pub type R = crate::R<RESP2Rrs>;
///Field `CARDSTATUSx` reader - Card status x See .
pub type CARDSTATUSX_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Card status x See .
    #[inline(always)]
    pub fn cardstatusx(&self) -> CARDSTATUSX_R {
        CARDSTATUSX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP2R")
            .field("cardstatusx", &self.cardstatusx())
            .finish()
    }
}
/**SDMMC response 2 register

You can [`read`](crate::Reg::read) this register and get [`resp2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#SDMMC1:RESP2R)*/
pub struct RESP2Rrs;
impl crate::RegisterSpec for RESP2Rrs {
    type Ux = u32;
}
///`read()` method returns [`resp2r::R`](R) reader structure
impl crate::Readable for RESP2Rrs {}
///`reset()` method sets RESP2R to value 0
impl crate::Resettable for RESP2Rrs {}
