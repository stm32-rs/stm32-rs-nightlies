///Register `TXCRCR` reader
pub type R = crate::R<TXCRCRrs>;
///Field `TxCRC` reader - Tx CRC register
pub type TX_CRC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Tx CRC register
    #[inline(always)]
    pub fn tx_crc(&self) -> TX_CRC_R {
        TX_CRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXCRCR")
            .field("tx_crc", &self.tx_crc())
            .finish()
    }
}
/**TX CRC register

You can [`read`](crate::Reg::read) this register and get [`txcrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#SPI1:TXCRCR)*/
pub struct TXCRCRrs;
impl crate::RegisterSpec for TXCRCRrs {
    type Ux = u16;
}
///`read()` method returns [`txcrcr::R`](R) reader structure
impl crate::Readable for TXCRCRrs {}
///`reset()` method sets TXCRCR to value 0
impl crate::Resettable for TXCRCRrs {}
