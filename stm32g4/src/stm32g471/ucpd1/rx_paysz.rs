#[doc = "Register `RX_PAYSZ` reader"]
pub type R = crate::R<RX_PAYSZrs>;
#[doc = "Field `RXPAYSZ` reader - RXPAYSZ"]
pub type RXPAYSZ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "UCPD Rx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_paysz::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_PAYSZrs;
impl crate::RegisterSpec for RX_PAYSZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_paysz::R`](R) reader structure"]
impl crate::Readable for RX_PAYSZrs {}
#[doc = "`reset()` method sets RX_PAYSZ to value 0"]
impl crate::Resettable for RX_PAYSZrs {
    const RESET_VALUE: u32 = 0;
}
