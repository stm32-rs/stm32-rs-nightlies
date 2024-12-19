///Register `RTC_SCR` writer
pub type W = crate::W<RTC_SCRrs>;
///Field `CALRAF` writer - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register.
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALRBF` writer - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register.
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUTF` writer - Clear wake-up timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register.
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSF` writer - Clear timestamp flag Writing 1 in this bit clears the TSF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF.
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSOVF` writer - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITSF` writer - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register.
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSRUF` writer - Clear SSR underflow flag Writing 1 in this bit clears the SSRUF in the RTC_SR register.
pub type CSSRUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RTC_SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register.
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W<RTC_SCRrs> {
        CALRAF_W::new(self, 0)
    }
    ///Bit 1 - Clear alarm B flag Writing 1 in this bit clears the ALRBF bit in the RTC_SR register.
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W<RTC_SCRrs> {
        CALRBF_W::new(self, 1)
    }
    ///Bit 2 - Clear wake-up timer flag Writing 1 in this bit clears the WUTF bit in the RTC_SR register.
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W<RTC_SCRrs> {
        CWUTF_W::new(self, 2)
    }
    ///Bit 3 - Clear timestamp flag Writing 1 in this bit clears the TSF bit in the RTC_SR register. If ITSF flag is set, TSF must be cleared together with ITSF by setting CRSF and CITSF.
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W<RTC_SCRrs> {
        CTSF_W::new(self, 3)
    }
    ///Bit 4 - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W<RTC_SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    ///Bit 5 - Clear internal timestamp flag Writing 1 in this bit clears the ITSF bit in the RTC_SR register.
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W<RTC_SCRrs> {
        CITSF_W::new(self, 5)
    }
    ///Bit 6 - Clear SSR underflow flag Writing 1 in this bit clears the SSRUF in the RTC_SR register.
    #[inline(always)]
    pub fn cssruf(&mut self) -> CSSRUF_W<RTC_SCRrs> {
        CSSRUF_W::new(self, 6)
    }
}
/**RTC status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RTC:RTC_SCR)*/
pub struct RTC_SCRrs;
impl crate::RegisterSpec for RTC_SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rtc_scr::W`](W) writer structure
impl crate::Writable for RTC_SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RTC_SCR to value 0
impl crate::Resettable for RTC_SCRrs {
    const RESET_VALUE: u32 = 0;
}
