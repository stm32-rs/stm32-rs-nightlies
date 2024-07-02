///Register `FDCAN_TXESC` reader
pub type R = crate::R<FDCAN_TXESCrs>;
///Field `TBDS` reader - TBDS
pub type TBDS_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - TBDS
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXESC")
            .field("tbds", &self.tbds())
            .finish()
    }
}
/**Configures the number of data bytes belonging to a Tx buffer element. Data field sizes &amp;gt;8 bytes are intended for CAN FD operation only.

You can [`read`](crate::Reg::read) this register and get [`fdcan_txesc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:FDCAN_TXESC)*/
pub struct FDCAN_TXESCrs;
impl crate::RegisterSpec for FDCAN_TXESCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txesc::R`](R) reader structure
impl crate::Readable for FDCAN_TXESCrs {}
///`reset()` method sets FDCAN_TXESC to value 0
impl crate::Resettable for FDCAN_TXESCrs {
    const RESET_VALUE: u32 = 0;
}
