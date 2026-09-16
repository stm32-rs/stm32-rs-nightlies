///Register `RXDR` reader
pub type R = crate::R<RXDRrs>;
///Field `RXDR` reader - CEC Rx Data Register
pub type RXDR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - CEC Rx Data Register
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDR").field("rxdr", &self.rxdr()).finish()
    }
}
/**Rx Data Register

You can [`read`](crate::Reg::read) this register and get [`rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#CEC:RXDR)*/
pub struct RXDRrs;
impl crate::RegisterSpec for RXDRrs {
    type Ux = u32;
}
///`read()` method returns [`rxdr::R`](R) reader structure
impl crate::Readable for RXDRrs {}
///`reset()` method sets RXDR to value 0
impl crate::Resettable for RXDRrs {}
