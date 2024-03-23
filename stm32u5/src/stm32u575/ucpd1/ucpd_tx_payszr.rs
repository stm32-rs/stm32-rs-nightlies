#[doc = "Register `UCPD_TX_PAYSZR` reader"]
pub type R = crate::R<UCPD_TX_PAYSZRrs>;
#[doc = "Register `UCPD_TX_PAYSZR` writer"]
pub type W = crate::W<UCPD_TX_PAYSZRrs>;
#[doc = "Field `TXPAYSZ` reader - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
pub type TXPAYSZ_R = crate::FieldReader<u16>;
#[doc = "Field `TXPAYSZ` writer - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
pub type TXPAYSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
    #[inline(always)]
    pub fn txpaysz(&self) -> TXPAYSZ_R {
        TXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission."]
    #[inline(always)]
    #[must_use]
    pub fn txpaysz(&mut self) -> TXPAYSZ_W<UCPD_TX_PAYSZRrs> {
        TXPAYSZ_W::new(self, 0)
    }
}
#[doc = "UCPD Tx payload size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucpd_tx_payszr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucpd_tx_payszr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCPD_TX_PAYSZRrs;
impl crate::RegisterSpec for UCPD_TX_PAYSZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucpd_tx_payszr::R`](R) reader structure"]
impl crate::Readable for UCPD_TX_PAYSZRrs {}
#[doc = "`write(|w| ..)` method takes [`ucpd_tx_payszr::W`](W) writer structure"]
impl crate::Writable for UCPD_TX_PAYSZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UCPD_TX_PAYSZR to value 0"]
impl crate::Resettable for UCPD_TX_PAYSZRrs {
    const RESET_VALUE: u32 = 0;
}
