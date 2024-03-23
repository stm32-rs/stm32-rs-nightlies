#[doc = "Register `ALRL` writer"]
pub type W = crate::W<ALRLrs>;
#[doc = "Field `ALRL` writer - RTC alarm register low"]
pub type ALRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    #[must_use]
    pub fn alrl(&mut self) -> ALRL_W<ALRLrs> {
        ALRL_W::new(self, 0)
    }
}
#[doc = "RTC Alarm Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRLrs;
impl crate::RegisterSpec for ALRLrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrl::W`](W) writer structure"]
impl crate::Writable for ALRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRL to value 0xffff"]
impl crate::Resettable for ALRLrs {
    const RESET_VALUE: u32 = 0xffff;
}
