#[doc = "Register `SUSP4R` writer"]
pub type W = crate::W<SUSP4Rrs>;
#[doc = "Field `SUSP4` writer - AES suspend"]
pub type SUSP4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - AES suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp4(&mut self) -> SUSP4_W<SUSP4Rrs> {
        SUSP4_W::new(self, 0)
    }
}
#[doc = "suspend registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp4r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUSP4Rrs;
impl crate::RegisterSpec for SUSP4Rrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`susp4r::W`](W) writer structure"]
impl crate::Writable for SUSP4Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUSP4R to value 0"]
impl crate::Resettable for SUSP4Rrs {
    const RESET_VALUE: u32 = 0;
}
