#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CALRAF` writer - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register."]
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRBF` writer - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register."]
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUTF` writer - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register."]
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSF` writer - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF."]
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSOVF` writer - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITSF` writer - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register."]
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSRUF` writer - Clear SSR underflow flag Writing ‘1’ in this bit clears the SSRUF in the RTC_SR register."]
pub type CSSRUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CALRAF_W<SCRrs> {
        CALRAF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CALRBF_W<SCRrs> {
        CALRBF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CWUTF_W<SCRrs> {
        CWUTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF."]
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<SCRrs> {
        CTSF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CTSOVF_W<SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CITSF_W<SCRrs> {
        CITSF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear SSR underflow flag Writing ‘1’ in this bit clears the SSRUF in the RTC_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn cssruf(&mut self) -> CSSRUF_W<SCRrs> {
        CSSRUF_W::new(self, 6)
    }
}
#[doc = "RTC status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
