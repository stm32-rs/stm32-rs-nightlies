#[doc = "Register `RX_ORDEXTR1` reader"]
pub type R = crate::R<RX_ORDEXTR1rs>;
#[doc = "Register `RX_ORDEXTR1` writer"]
pub type W = crate::W<RX_ORDEXTR1rs>;
#[doc = "Field `RXSOPX1` reader - Ordered set 1 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
pub type RXSOPX1_R = crate::FieldReader<u32>;
#[doc = "Field `RXSOPX1` writer - Ordered set 1 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
pub type RXSOPX1_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Ordered set 1 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
    #[inline(always)]
    pub fn rxsopx1(&self) -> RXSOPX1_R {
        RXSOPX1_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Ordered set 1 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx1(&mut self) -> RXSOPX1_W<RX_ORDEXTR1rs> {
        RXSOPX1_W::new(self, 0)
    }
}
#[doc = "UCPD Rx ordered set extension register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordextr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordextr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ORDEXTR1rs;
impl crate::RegisterSpec for RX_ORDEXTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordextr1::R`](R) reader structure"]
impl crate::Readable for RX_ORDEXTR1rs {}
#[doc = "`write(|w| ..)` method takes [`rx_ordextr1::W`](W) writer structure"]
impl crate::Writable for RX_ORDEXTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ORDEXTR1 to value 0"]
impl crate::Resettable for RX_ORDEXTR1rs {
    const RESET_VALUE: u32 = 0;
}
