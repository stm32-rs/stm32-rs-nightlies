///Register `TXBTO` reader
pub type R = crate::R<TXBTOrs>;
///Field `TO` reader - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.
pub type TO_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Transmission occurred. Each Tx buffer has its own TO bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTO").field("to", &self.to()).finish()
    }
}
/**FDCAN Tx buffer transmission occurred register

You can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FDCAN1:TXBTO)*/
pub struct TXBTOrs;
impl crate::RegisterSpec for TXBTOrs {
    type Ux = u32;
}
///`read()` method returns [`txbto::R`](R) reader structure
impl crate::Readable for TXBTOrs {}
///`reset()` method sets TXBTO to value 0
impl crate::Resettable for TXBTOrs {}
