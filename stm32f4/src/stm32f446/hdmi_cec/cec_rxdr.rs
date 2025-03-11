///Register `CEC_RXDR` reader
pub type R = crate::R<CEC_RXDRrs>;
///Field `RXD` reader - Rx Data register
pub type RXD_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Rx Data register
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEC_RXDR")
            .field("rxd", &self.rxd())
            .finish()
    }
}
/**CEC Rx Data Register

You can [`read`](crate::Reg::read) this register and get [`cec_rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_RXDR)*/
pub struct CEC_RXDRrs;
impl crate::RegisterSpec for CEC_RXDRrs {
    type Ux = u32;
}
///`read()` method returns [`cec_rxdr::R`](R) reader structure
impl crate::Readable for CEC_RXDRrs {}
///`reset()` method sets CEC_RXDR to value 0
impl crate::Resettable for CEC_RXDRrs {}
