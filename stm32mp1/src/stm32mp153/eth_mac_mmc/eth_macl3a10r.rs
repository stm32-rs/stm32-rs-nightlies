#[doc = "Register `ETH_MACL3A10R` reader"]
pub type R = crate::R<ETH_MACL3A10Rrs>;
#[doc = "Register `ETH_MACL3A10R` writer"]
pub type W = crate::W<ETH_MACL3A10Rrs>;
#[doc = "Field `L3A10` reader - L3A10"]
pub type L3A10_R = crate::FieldReader<u32>;
#[doc = "Field `L3A10` writer - L3A10"]
pub type L3A10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A10"]
    #[inline(always)]
    pub fn l3a10(&self) -> L3A10_R {
        L3A10_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A10"]
    #[inline(always)]
    #[must_use]
    pub fn l3a10(&mut self) -> L3A10_W<ETH_MACL3A10Rrs> {
        L3A10_W::new(self, 0)
    }
}
#[doc = "For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\\[63:32\\]
of the 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a10r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a10r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACL3A10Rrs;
impl crate::RegisterSpec for ETH_MACL3A10Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macl3a10r::R`](R) reader structure"]
impl crate::Readable for ETH_MACL3A10Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macl3a10r::W`](W) writer structure"]
impl crate::Writable for ETH_MACL3A10Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACL3A10R to value 0"]
impl crate::Resettable for ETH_MACL3A10Rrs {
    const RESET_VALUE: u32 = 0;
}
