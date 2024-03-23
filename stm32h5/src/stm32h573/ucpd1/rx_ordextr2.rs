#[doc = "Register `RX_ORDEXTR2` reader"]
pub type R = crate::R<RX_ORDEXTR2rs>;
#[doc = "Register `RX_ORDEXTR2` writer"]
pub type W = crate::W<RX_ORDEXTR2rs>;
#[doc = "Field `RXSOPX2` reader - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
pub type RXSOPX2_R = crate::FieldReader<u32>;
#[doc = "Field `RXSOPX2` writer - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
pub type RXSOPX2_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
    #[inline(always)]
    pub fn rxsopx2(&self) -> RXSOPX2_R {
        RXSOPX2_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Ordered set 2 received The bitfield contains a full 20-bit sequence received, consisting of four K‑codes, each of five bits. The bit 0 (bit 0 of K‑code1) is receive first, the bit 19 (bit 4 of K‑code4) last."]
    #[inline(always)]
    #[must_use]
    pub fn rxsopx2(&mut self) -> RXSOPX2_W<RX_ORDEXTR2rs> {
        RXSOPX2_W::new(self, 0)
    }
}
#[doc = "UCPD Rx ordered set extension register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ordextr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ordextr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ORDEXTR2rs;
impl crate::RegisterSpec for RX_ORDEXTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ordextr2::R`](R) reader structure"]
impl crate::Readable for RX_ORDEXTR2rs {}
#[doc = "`write(|w| ..)` method takes [`rx_ordextr2::W`](W) writer structure"]
impl crate::Writable for RX_ORDEXTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ORDEXTR2 to value 0"]
impl crate::Resettable for RX_ORDEXTR2rs {
    const RESET_VALUE: u32 = 0;
}
