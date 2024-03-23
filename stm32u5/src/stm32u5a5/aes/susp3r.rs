#[doc = "Register `SUSP3R` writer"]
pub type W = crate::W<SUSP3Rrs>;
#[doc = "Field `SUSP3` writer - AES suspend"]
pub type SUSP3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp3(&mut self) -> SUSP3_W<SUSP3Rrs> {
        SUSP3_W::new(self, 0)
    }
}
#[doc = "suspend registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp3r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP3Rrs;
impl crate::RegisterSpec for SUSP3Rrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`susp3r::W`](W) writer structure"]
impl crate::Writable for SUSP3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP3R to value 0"]
impl crate::Resettable for SUSP3Rrs {
    const RESET_VALUE: u32 = 0;
}
