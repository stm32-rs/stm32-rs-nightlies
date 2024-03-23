#[doc = "Register `ETH_MACL4A1R` reader"]
pub type R = crate::R<ETH_MACL4A1Rrs>;
#[doc = "Register `ETH_MACL4A1R` writer"]
pub type W = crate::W<ETH_MACL4A1Rrs>;
#[doc = "Field `L4SP1` reader - L4SP1"]
pub type L4SP1_R = crate::FieldReader<u16>;
#[doc = "Field `L4SP1` writer - L4SP1"]
pub type L4SP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `L4DP1` reader - L4DP1"]
pub type L4DP1_R = crate::FieldReader<u16>;
#[doc = "Field `L4DP1` writer - L4DP1"]
pub type L4DP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    #[must_use]
    pub fn l4sp1(&mut self) -> L4SP1_W<ETH_MACL4A1Rrs> {
        L4SP1_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    #[must_use]
    pub fn l4dp1(&mut self) -> L4DP1_W<ETH_MACL4A1Rrs> {
        L4DP1_W::new(self, 16)
    }
}
#[doc = "The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl4a1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl4a1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACL4A1Rrs;
impl crate::RegisterSpec for ETH_MACL4A1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macl4a1r::R`](R) reader structure"]
impl crate::Readable for ETH_MACL4A1Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macl4a1r::W`](W) writer structure"]
impl crate::Writable for ETH_MACL4A1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACL4A1R to value 0"]
impl crate::Resettable for ETH_MACL4A1Rrs {
    const RESET_VALUE: u32 = 0;
}
