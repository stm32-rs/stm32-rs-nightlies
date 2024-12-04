///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CALRAF` writer - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register.
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSF` writer - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register.
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSOVF` writer - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear alarm A flag Writing 1 in this bit clears the ALRAF bit in the RTC_SR register.
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W<SCRrs> {
        CALRAF_W::new(self, 0)
    }
    ///Bit 3 - Clear timestamp flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register.
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W<SCRrs> {
        CTSF_W::new(self, 3)
    }
    ///Bit 4 - Clear timestamp overflow flag Writing 1 in this bit clears the TSOVF bit in the RTC_SR register. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W<SCRrs> {
        CTSOVF_W::new(self, 4)
    }
}
/**RTC status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#RTC:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
