///Register `RX_PAYSZ` reader
pub type R = crate::R<RX_PAYSZrs>;
///Field `RXPAYSZ` reader - RXPAYSZ
pub type RXPAYSZ_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PAYSZ")
            .field("rxpaysz", &self.rxpaysz())
            .finish()
    }
}
/**UCPD Rx Paysize Register

You can [`read`](crate::Reg::read) this register and get [`rx_paysz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#UCPD1:RX_PAYSZ)*/
pub struct RX_PAYSZrs;
impl crate::RegisterSpec for RX_PAYSZrs {
    type Ux = u32;
}
///`read()` method returns [`rx_paysz::R`](R) reader structure
impl crate::Readable for RX_PAYSZrs {}
///`reset()` method sets RX_PAYSZ to value 0
impl crate::Resettable for RX_PAYSZrs {
    const RESET_VALUE: u32 = 0;
}
