///Register `RESP1R` reader
pub type R = crate::R<RESP1Rrs>;
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
        f.debug_struct("RESP1R")
            .field("cardstatusx", &self.cardstatusx())
            .finish()
    }
}
/**SDMMC response 1 register

You can [`read`](crate::Reg::read) this register and get [`resp1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#SDMMC1:RESP1R)*/
pub struct RESP1Rrs;
impl crate::RegisterSpec for RESP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`resp1r::R`](R) reader structure
impl crate::Readable for RESP1Rrs {}
///`reset()` method sets RESP1R to value 0
impl crate::Resettable for RESP1Rrs {}
