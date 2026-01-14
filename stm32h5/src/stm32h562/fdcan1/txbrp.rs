///Register `TXBRP` reader
pub type R = crate::R<TXBRPrs>;
///Field `TRP` reader - Transmission request pending Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions.
pub type TRP_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Transmission request pending Each Tx buffer has its own transmission request pending bit. The bits are set via register TXBAR. The bits are reset after a requested transmission has completed or has been canceled via register TXBCR. After a TXBRP bit has been set, a Tx scan is started to check for the pending Tx request with the highest priority (Tx buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signaled via TXBCF after successful transmission together with the corresponding TXBTO bit when the transmission has not yet been started at the point of cancellation when the transmission has been aborted due to lost arbitration when an error occurred during frame transmission In DAR mode all transmissions are automatically canceled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions.
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBRP").field("trp", &self.trp()).finish()
    }
}
/**FDCAN Tx buffer request pending register

You can [`read`](crate::Reg::read) this register and get [`txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:TXBRP)*/
pub struct TXBRPrs;
impl crate::RegisterSpec for TXBRPrs {
    type Ux = u32;
}
///`read()` method returns [`txbrp::R`](R) reader structure
impl crate::Readable for TXBRPrs {}
///`reset()` method sets TXBRP to value 0
impl crate::Resettable for TXBRPrs {}
