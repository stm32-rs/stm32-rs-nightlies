///Register `RXD` reader
pub type R = crate::R<RXDrs>;
///Field `RXD` reader - Rx data
pub type RXD_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Rx data
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXD").field("rxd", &self.rxd()).finish()
    }
}
/**CEC Rx data register

You can [`read`](crate::Reg::read) this register and get [`rxd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:RXD)*/
pub struct RXDrs;
impl crate::RegisterSpec for RXDrs {
    type Ux = u32;
}
///`read()` method returns [`rxd::R`](R) reader structure
impl crate::Readable for RXDrs {}
///`reset()` method sets RXD to value 0
impl crate::Resettable for RXDrs {}
