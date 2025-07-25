///Register `TXESC` reader
pub type R = crate::R<TXESCrs>;
///Field `TBDS` reader - Tx buffer data Field size:
pub type TBDS_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Tx buffer data Field size:
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXESC").field("tbds", &self.tbds()).finish()
    }
}
/**FDCAN Tx buffer element size configuration register

You can [`read`](crate::Reg::read) this register and get [`txesc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FDCAN1:TXESC)*/
pub struct TXESCrs;
impl crate::RegisterSpec for TXESCrs {
    type Ux = u32;
}
///`read()` method returns [`txesc::R`](R) reader structure
impl crate::Readable for TXESCrs {}
///`reset()` method sets TXESC to value 0
impl crate::Resettable for TXESCrs {}
