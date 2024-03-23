#[doc = "Register `SUSP2R` writer"]
pub type W = crate::W<SUSP2Rrs>;
#[doc = "Field `SUSP2` writer - AES suspend"]
pub type SUSP2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp2(&mut self) -> SUSP2_W<SUSP2Rrs> {
        SUSP2_W::new(self, 0)
    }
}
#[doc = "suspend registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp2r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP2Rrs;
impl crate::RegisterSpec for SUSP2Rrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`susp2r::W`](W) writer structure"]
impl crate::Writable for SUSP2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP2R to value 0"]
impl crate::Resettable for SUSP2Rrs {
    const RESET_VALUE: u32 = 0;
}
