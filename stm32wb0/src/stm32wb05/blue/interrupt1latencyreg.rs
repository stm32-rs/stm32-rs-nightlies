///Register `INTERRUPT1LATENCYREG` reader
pub type R = crate::R<INTERRUPT1LATENCYREGrs>;
///Field `INTERRUPT1LATENCY` reader - relative time counter started on irq_BLE_int1 (BLE_TXRX) occurrence.
pub type INTERRUPT1LATENCY_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - relative time counter started on irq_BLE_int1 (BLE_TXRX) occurrence.
    #[inline(always)]
    pub fn interrupt1latency(&self) -> INTERRUPT1LATENCY_R {
        INTERRUPT1LATENCY_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT1LATENCYREG")
            .field("interrupt1latency", &self.interrupt1latency())
            .finish()
    }
}
/**INTERRUPT1LATENCYREG register

You can [`read`](crate::Reg::read) this register and get [`interrupt1latencyreg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:INTERRUPT1LATENCYREG)*/
pub struct INTERRUPT1LATENCYREGrs;
impl crate::RegisterSpec for INTERRUPT1LATENCYREGrs {
    type Ux = u32;
}
///`read()` method returns [`interrupt1latencyreg::R`](R) reader structure
impl crate::Readable for INTERRUPT1LATENCYREGrs {}
///`reset()` method sets INTERRUPT1LATENCYREG to value 0
impl crate::Resettable for INTERRUPT1LATENCYREGrs {}
