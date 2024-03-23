#[doc = "Register `ETH_MACL4A0R` reader"]
pub type R = crate::R<ETH_MACL4A0Rrs>;
#[doc = "Register `ETH_MACL4A0R` writer"]
pub type W = crate::W<ETH_MACL4A0Rrs>;
#[doc = "Field `L4SP0` reader - L4SP0"]
pub type L4SP0_R = crate::FieldReader<u16>;
#[doc = "Field `L4SP0` writer - L4SP0"]
pub type L4SP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `L4DP0` reader - L4DP0"]
pub type L4DP0_R = crate::FieldReader<u16>;
#[doc = "Field `L4DP0` writer - L4DP0"]
pub type L4DP0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    #[must_use]
    pub fn l4sp0(&mut self) -> L4SP0_W<ETH_MACL4A0Rrs> {
        L4SP0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    #[must_use]
    pub fn l4dp0(&mut self) -> L4DP0_W<ETH_MACL4A0Rrs> {
        L4DP0_W::new(self, 16)
    }
}
#[doc = "Layer4 address filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl4a0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl4a0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACL4A0Rrs;
impl crate::RegisterSpec for ETH_MACL4A0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macl4a0r::R`](R) reader structure"]
impl crate::Readable for ETH_MACL4A0Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macl4a0r::W`](W) writer structure"]
impl crate::Writable for ETH_MACL4A0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACL4A0R to value 0"]
impl crate::Resettable for ETH_MACL4A0Rrs {
    const RESET_VALUE: u32 = 0;
}
