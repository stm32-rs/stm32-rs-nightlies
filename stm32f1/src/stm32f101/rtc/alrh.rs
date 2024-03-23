#[doc = "Register `ALRH` writer"]
pub type W = crate::W<ALRHrs>;
#[doc = "Field `ALRH` writer - RTC alarm register high"]
pub type ALRH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC alarm register high"]
    #[inline(always)]
    #[must_use]
    pub fn alrh(&mut self) -> ALRH_W<ALRHrs> {
        ALRH_W::new(self, 0)
    }
}
#[doc = "RTC Alarm Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRHrs;
impl crate::RegisterSpec for ALRHrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrh::W`](W) writer structure"]
impl crate::Writable for ALRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRH to value 0xffff"]
impl crate::Resettable for ALRHrs {
    const RESET_VALUE: u32 = 0xffff;
}
