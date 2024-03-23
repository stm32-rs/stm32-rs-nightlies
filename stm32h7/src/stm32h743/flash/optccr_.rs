#[doc = "Register `OPTCCR_` writer"]
pub type W = crate::W<OPTCCR_rs>;
#[doc = "Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit"]
pub type CLR_OPTCHANGEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W<OPTCCR_rs> {
        CLR_OPTCHANGEERR_W::new(self, 30)
    }
}
#[doc = "FLASH option clear control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optccr_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCCR_rs;
impl crate::RegisterSpec for OPTCCR_rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`optccr_::W`](W) writer structure"]
impl crate::Writable for OPTCCR_rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTCCR_ to value 0"]
impl crate::Resettable for OPTCCR_rs {
    const RESET_VALUE: u32 = 0;
}
