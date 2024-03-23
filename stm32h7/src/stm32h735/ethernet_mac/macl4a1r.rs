#[doc = "Register `MACL4A1R` reader"]
pub type R = crate::R<MACL4A1Rrs>;
#[doc = "Register `MACL4A1R` writer"]
pub type W = crate::W<MACL4A1Rrs>;
#[doc = "Field `L4SP1` reader - Layer 4 Source Port Number Field"]
pub type L4SP1_R = crate::FieldReader<u16>;
#[doc = "Field `L4SP1` writer - Layer 4 Source Port Number Field"]
pub type L4SP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `L4DP1` reader - Layer 4 Destination Port Number Field"]
pub type L4DP1_R = crate::FieldReader<u16>;
#[doc = "Field `L4DP1` writer - Layer 4 Destination Port Number Field"]
pub type L4DP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field"]
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field"]
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field"]
    #[inline(always)]
    #[must_use]
    pub fn l4sp1(&mut self) -> L4SP1_W<MACL4A1Rrs> {
        L4SP1_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field"]
    #[inline(always)]
    #[must_use]
    pub fn l4dp1(&mut self) -> L4DP1_W<MACL4A1Rrs> {
        L4DP1_W::new(self, 16)
    }
}
#[doc = "Layer 4 address filter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl4a1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl4a1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL4A1Rrs;
impl crate::RegisterSpec for MACL4A1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl4a1r::R`](R) reader structure"]
impl crate::Readable for MACL4A1Rrs {}
#[doc = "`write(|w| ..)` method takes [`macl4a1r::W`](W) writer structure"]
impl crate::Writable for MACL4A1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACL4A1R to value 0"]
impl crate::Resettable for MACL4A1Rrs {
    const RESET_VALUE: u32 = 0;
}
