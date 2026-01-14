///Register `RX_CRC_REG` reader
pub type R = crate::R<RX_CRC_REGrs>;
///Field `RX_CRC_OUT` reader - CRC field of the received packet (read-only info)
pub type RX_CRC_OUT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CRC field of the received packet (read-only info)
    #[inline(always)]
    pub fn rx_crc_out(&self) -> RX_CRC_OUT_R {
        RX_CRC_OUT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CRC_REG")
            .field("rx_crc_out", &self.rx_crc_out())
            .finish()
    }
}
/**RX_CRC_REG register

You can [`read`](crate::Reg::read) this register and get [`rx_crc_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RX_CRC_REG)*/
pub struct RX_CRC_REGrs;
impl crate::RegisterSpec for RX_CRC_REGrs {
    type Ux = u32;
}
///`read()` method returns [`rx_crc_reg::R`](R) reader structure
impl crate::Readable for RX_CRC_REGrs {}
///`reset()` method sets RX_CRC_REG to value 0
impl crate::Resettable for RX_CRC_REGrs {}
