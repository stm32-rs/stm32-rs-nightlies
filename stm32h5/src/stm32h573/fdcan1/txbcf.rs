///Register `TXBCF` reader
pub type R = crate::R<TXBCFrs>;
///Field `CF` reader - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.
pub type CF_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR.
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCF").field("cf", &self.cf()).finish()
    }
}
/**FDCAN Tx buffer cancellation finished register

You can [`read`](crate::Reg::read) this register and get [`txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FDCAN1:TXBCF)*/
pub struct TXBCFrs;
impl crate::RegisterSpec for TXBCFrs {
    type Ux = u32;
}
///`read()` method returns [`txbcf::R`](R) reader structure
impl crate::Readable for TXBCFrs {}
///`reset()` method sets TXBCF to value 0
impl crate::Resettable for TXBCFrs {}
