///Register `TXBTO` reader
pub type R = crate::R<TXBTOrs>;
///Field `TO` reader - TO
pub type TO_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - TO
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTO").field("to", &self.to()).finish()
    }
}
/**FDCAN Tx buffer transmission occurred register

You can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TXBTO)*/
pub struct TXBTOrs;
impl crate::RegisterSpec for TXBTOrs {
    type Ux = u32;
}
///`read()` method returns [`txbto::R`](R) reader structure
impl crate::Readable for TXBTOrs {}
///`reset()` method sets TXBTO to value 0
impl crate::Resettable for TXBTOrs {}
