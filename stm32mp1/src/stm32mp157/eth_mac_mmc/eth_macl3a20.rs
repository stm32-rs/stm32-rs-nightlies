#[doc = "Register `ETH_MACL3A20` reader"]
pub type R = crate::R<ETH_MACL3A20rs>;
#[doc = "Register `ETH_MACL3A20` writer"]
pub type W = crate::W<ETH_MACL3A20rs>;
#[doc = "Field `L3A20` reader - L3A20"]
pub type L3A20_R = crate::FieldReader<u32>;
#[doc = "Field `L3A20` writer - L3A20"]
pub type L3A20_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    pub fn l3a20(&self) -> L3A20_R {
        L3A20_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    #[must_use]
    pub fn l3a20(&mut self) -> L3A20_W<ETH_MACL3A20rs> {
        L3A20_W::new(self, 0)
    }
}
#[doc = "The Layer 3 Address 2 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\\[95:64\\]
of 128-bit IP Source Address or Destination Address field.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3a20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3a20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACL3A20rs;
impl crate::RegisterSpec for ETH_MACL3A20rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macl3a20::R`](R) reader structure"]
impl crate::Readable for ETH_MACL3A20rs {}
#[doc = "`write(|w| ..)` method takes [`eth_macl3a20::W`](W) writer structure"]
impl crate::Writable for ETH_MACL3A20rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACL3A20 to value 0"]
impl crate::Resettable for ETH_MACL3A20rs {
    const RESET_VALUE: u32 = 0;
}
