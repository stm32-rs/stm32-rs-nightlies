///Register `FDCAN_TDCR` reader
pub type R = crate::R<FDCAN_TDCRrs>;
///Field `TDCF` reader - Transmitter Delay Compensation Filter Window Length
pub type TDCF_R = crate::FieldReader;
///Field `TDCO` reader - Transmitter Delay Compensation Offset
pub type TDCO_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - Transmitter Delay Compensation Filter Window Length
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - Transmitter Delay Compensation Offset
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TDCR")
            .field("tdcf", &self.tdcf())
            .field("tdco", &self.tdco())
            .finish()
    }
}
/**FDCAN Transmitter Delay Compensation Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tdcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#CAN_CCU:FDCAN_TDCR)*/
pub struct FDCAN_TDCRrs;
impl crate::RegisterSpec for FDCAN_TDCRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_tdcr::R`](R) reader structure
impl crate::Readable for FDCAN_TDCRrs {}
///`reset()` method sets FDCAN_TDCR to value 0
impl crate::Resettable for FDCAN_TDCRrs {
    const RESET_VALUE: u32 = 0;
}
