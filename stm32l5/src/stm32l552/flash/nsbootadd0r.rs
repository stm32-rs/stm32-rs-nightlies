#[doc = "Register `NSBOOTADD0R` writer"]
pub type W = crate::W<NSBOOTADD0Rrs>;
#[doc = "Field `NSBOOTADD0` writer - NSBOOTADD0"]
pub type NSBOOTADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl W {
    #[doc = "Bits 7:31 - NSBOOTADD0"]
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd0(&mut self) -> NSBOOTADD0_W<NSBOOTADD0Rrs> {
        NSBOOTADD0_W::new(self, 7)
    }
}
#[doc = "Flash non-secure boot address 0 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsbootadd0r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSBOOTADD0Rrs;
impl crate::RegisterSpec for NSBOOTADD0Rrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nsbootadd0r::W`](W) writer structure"]
impl crate::Writable for NSBOOTADD0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSBOOTADD0R to value 0x0f"]
impl crate::Resettable for NSBOOTADD0Rrs {
    const RESET_VALUE: u32 = 0x0f;
}
