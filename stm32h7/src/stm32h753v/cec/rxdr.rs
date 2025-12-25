///Register `RXDR` reader
pub type R = crate::R<RXDRrs>;
///Field `RXD` reader - Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line.
pub type RXD_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line.
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDR").field("rxd", &self.rxd()).finish()
    }
}
/**CEC Rx Data Register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#CEC:RXDR)*/
pub struct RXDRrs;
impl crate::RegisterSpec for RXDRrs {
    type Ux = u32;
}
///`read()` method returns [`rxdr::R`](R) reader structure
impl crate::Readable for RXDRrs {}
///`reset()` method sets RXDR to value 0
impl crate::Resettable for RXDRrs {}
