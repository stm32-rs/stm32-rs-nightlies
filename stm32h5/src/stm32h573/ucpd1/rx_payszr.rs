#[doc = "Register `RX_PAYSZR` reader"]
pub type R = crate::R<RX_PAYSZRrs>;
#[doc = "Field `RXPAYSZ` reader - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low)."]
pub type RXPAYSZ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low)."]
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "UCPD Rx payload size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_payszr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_PAYSZRrs;
impl crate::RegisterSpec for RX_PAYSZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_payszr::R`](R) reader structure"]
impl crate::Readable for RX_PAYSZRrs {}
#[doc = "`reset()` method sets RX_PAYSZR to value 0"]
impl crate::Resettable for RX_PAYSZRrs {
    const RESET_VALUE: u32 = 0;
}
