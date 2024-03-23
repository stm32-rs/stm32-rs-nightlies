#[doc = "Register `ETH_MACL3A31R` reader"]
pub type R = crate::R<ETH_MACL3A31Rrs>;
#[doc = "Register `ETH_MACL3A31R` writer"]
pub type W = crate::W<ETH_MACL3A31Rrs>;
#[doc = "Field `L3A31` reader - L3A31"]
pub type L3A31_R = crate::FieldReader<u32>;
#[doc = "Field `L3A31` writer - L3A31"]
pub type L3A31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A31"]
    #[inline(always)]
    pub fn l3a31(&self) -> L3A31_R {
        L3A31_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A31"]
    #[inline(always)]
    #[must_use]
    pub fn l3a31(&mut self) -> L3A31_W<ETH_MACL3A31Rrs> {
        L3A31_W::new(self, 0)
    }
}
#[doc = "The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[127:96\\]
of 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a31r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a31r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACL3A31Rrs;
impl crate::RegisterSpec for ETH_MACL3A31Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macl3a31r::R`](R) reader structure"]
impl crate::Readable for ETH_MACL3A31Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macl3a31r::W`](W) writer structure"]
impl crate::Writable for ETH_MACL3A31Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACL3A31R to value 0"]
impl crate::Resettable for ETH_MACL3A31Rrs {
    const RESET_VALUE: u32 = 0;
}
