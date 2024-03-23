#[doc = "Register `RX_PAYSZ` reader"]
pub type R = crate::R<RX_PAYSZrs>;
#[doc = "Register `RX_PAYSZ` writer"]
pub type W = crate::W<RX_PAYSZrs>;
#[doc = "Field `RXPAYSZ` reader - RXPAYSZ"]
pub type RXPAYSZ_R = crate::FieldReader<u16>;
#[doc = "Field `RXPAYSZ` writer - RXPAYSZ"]
pub type RXPAYSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    #[must_use]
    pub fn rxpaysz(&mut self) -> RXPAYSZ_W<RX_PAYSZrs> {
        RXPAYSZ_W::new(self, 0)
    }
}
#[doc = "UCPD Rx Paysize Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_paysz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_paysz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_PAYSZrs;
impl crate::RegisterSpec for RX_PAYSZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_paysz::R`](R) reader structure"]
impl crate::Readable for RX_PAYSZrs {}
#[doc = "`write(|w| ..)` method takes [`rx_paysz::W`](W) writer structure"]
impl crate::Writable for RX_PAYSZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_PAYSZ to value 0"]
impl crate::Resettable for RX_PAYSZrs {
    const RESET_VALUE: u32 = 0;
}
