///Register `UCPD_RX_PAYSZR` reader
pub type R = crate::R<UCPD_RX_PAYSZRrs>;
///Field `RXPAYSZ` reader - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).
pub type RXPAYSZ_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCPD_RX_PAYSZR")
            .field("rxpaysz", &self.rxpaysz())
            .finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ucpd_rx_payszr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#UCPD1:UCPD_RX_PAYSZR)*/
pub struct UCPD_RX_PAYSZRrs;
impl crate::RegisterSpec for UCPD_RX_PAYSZRrs {
    type Ux = u32;
}
///`read()` method returns [`ucpd_rx_payszr::R`](R) reader structure
impl crate::Readable for UCPD_RX_PAYSZRrs {}
///`reset()` method sets UCPD_RX_PAYSZR to value 0
impl crate::Resettable for UCPD_RX_PAYSZRrs {
    const RESET_VALUE: u32 = 0;
}
