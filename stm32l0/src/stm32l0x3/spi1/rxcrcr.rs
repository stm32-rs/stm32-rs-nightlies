///Register `RXCRCR` reader
pub type R = crate::R<RXCRCRrs>;
///Field `RxCRC` reader - Rx CRC register
pub type RX_CRC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Rx CRC register
    #[inline(always)]
    pub fn rx_crc(&self) -> RX_CRC_R {
        RX_CRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXCRCR")
            .field("rx_crc", &self.rx_crc())
            .finish()
    }
}
/**RX CRC register

You can [`read`](crate::Reg::read) this register and get [`rxcrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#SPI1:RXCRCR)*/
pub struct RXCRCRrs;
impl crate::RegisterSpec for RXCRCRrs {
    type Ux = u16;
}
///`read()` method returns [`rxcrcr::R`](R) reader structure
impl crate::Readable for RXCRCRrs {}
///`reset()` method sets RXCRCR to value 0
impl crate::Resettable for RXCRCRrs {}
