///Register `RX_INFO_REG` reader
pub type R = crate::R<RX_INFO_REGrs>;
///Field `RX_PCKTLEN_OUT` reader - Indicates received packet length in bytes:
pub type RX_PCKTLEN_OUT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Indicates received packet length in bytes:
    #[inline(always)]
    pub fn rx_pcktlen_out(&self) -> RX_PCKTLEN_OUT_R {
        RX_PCKTLEN_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_INFO_REG")
            .field("rx_pcktlen_out", &self.rx_pcktlen_out())
            .finish()
    }
}
/**RX_INFO_REG register

You can [`read`](crate::Reg::read) this register and get [`rx_info_reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RX_INFO_REG)*/
pub struct RX_INFO_REGrs;
impl crate::RegisterSpec for RX_INFO_REGrs {
    type Ux = u32;
}
///`read()` method returns [`rx_info_reg::R`](R) reader structure
impl crate::Readable for RX_INFO_REGrs {}
///`reset()` method sets RX_INFO_REG to value 0
impl crate::Resettable for RX_INFO_REGrs {}
