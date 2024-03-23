#[doc = "Register `PRLH` writer"]
pub type W = crate::W<PRLHrs>;
#[doc = "Field `PRLH` writer - RTC Prescaler Load Register High"]
pub type PRLH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC Prescaler Load Register High"]
    #[inline(always)]
    #[must_use]
    pub fn prlh(&mut self) -> PRLH_W<PRLHrs> {
        PRLH_W::new(self, 0)
    }
}
#[doc = "RTC Prescaler Load Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prlh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRLHrs;
impl crate::RegisterSpec for PRLHrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prlh::W`](W) writer structure"]
impl crate::Writable for PRLHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRLH to value 0"]
impl crate::Resettable for PRLHrs {
    const RESET_VALUE: u32 = 0;
}
