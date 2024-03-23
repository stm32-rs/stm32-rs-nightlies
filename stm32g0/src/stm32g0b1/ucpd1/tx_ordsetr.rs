#[doc = "Register `TX_ORDSETR` reader"]
pub type R = crate::R<TX_ORDSETRrs>;
#[doc = "Register `TX_ORDSETR` writer"]
pub type W = crate::W<TX_ORDSETRrs>;
#[doc = "Field `TXORDSET` reader - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
pub type TXORDSET_R = crate::FieldReader<u32>;
#[doc = "Field `TXORDSET` writer - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
pub type TXORDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
    #[inline(always)]
    #[must_use]
    pub fn txordset(&mut self) -> TXORDSET_W<TX_ORDSETRrs> {
        TXORDSET_W::new(self, 0)
    }
}
#[doc = "UCPD Tx ordered set type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ordsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ordsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ORDSETRrs;
impl crate::RegisterSpec for TX_ORDSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ordsetr::R`](R) reader structure"]
impl crate::Readable for TX_ORDSETRrs {}
#[doc = "`write(|w| ..)` method takes [`tx_ordsetr::W`](W) writer structure"]
impl crate::Writable for TX_ORDSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_ORDSETR to value 0"]
impl crate::Resettable for TX_ORDSETRrs {
    const RESET_VALUE: u32 = 0;
}
