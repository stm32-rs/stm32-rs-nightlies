#[doc = "Register `NSBOOTADD1R` writer"]
pub type W = crate::W<NSBOOTADD1Rrs>;
#[doc = "Field `NSBOOTADD1` writer - NSBOOTADD1"]
pub type NSBOOTADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl W {
    #[doc = "Bits 7:31 - NSBOOTADD1"]
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd1(&mut self) -> NSBOOTADD1_W<NSBOOTADD1Rrs> {
        NSBOOTADD1_W::new(self, 7)
    }
}
#[doc = "Flash non-secure boot address 1 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsbootadd1r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSBOOTADD1Rrs;
impl crate::RegisterSpec for NSBOOTADD1Rrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nsbootadd1r::W`](W) writer structure"]
impl crate::Writable for NSBOOTADD1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSBOOTADD1R to value 0x0f"]
impl crate::Resettable for NSBOOTADD1Rrs {
    const RESET_VALUE: u32 = 0x0f;
}
