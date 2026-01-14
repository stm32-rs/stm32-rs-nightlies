///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `CALRAF` writer - Clear alarm A flag
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CALRBF` writer - Clear alarm B flag
pub type CALRBF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWUTF` writer - Clear wake-up timer flag
pub type CWUTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSF` writer - Clear timestamp flag
pub type CTSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSOVF` writer - Clear timestamp overflow flag
pub type CTSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CITSF` writer - Clear internal timestamp flag
pub type CITSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSRUF` writer - Clear SSR underflow flag
pub type CSSRUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear alarm A flag
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W<'_, SCRrs> {
        CALRAF_W::new(self, 0)
    }
    ///Bit 1 - Clear alarm B flag
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W<'_, SCRrs> {
        CALRBF_W::new(self, 1)
    }
    ///Bit 2 - Clear wake-up timer flag
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W<'_, SCRrs> {
        CWUTF_W::new(self, 2)
    }
    ///Bit 3 - Clear timestamp flag
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W<'_, SCRrs> {
        CTSF_W::new(self, 3)
    }
    ///Bit 4 - Clear timestamp overflow flag
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W<'_, SCRrs> {
        CTSOVF_W::new(self, 4)
    }
    ///Bit 5 - Clear internal timestamp flag
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W<'_, SCRrs> {
        CITSF_W::new(self, 5)
    }
    ///Bit 6 - Clear SSR underflow flag
    #[inline(always)]
    pub fn cssruf(&mut self) -> CSSRUF_W<'_, SCRrs> {
        CSSRUF_W::new(self, 6)
    }
}
/**RTC status clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RTC:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
