///Register `SCR` writer
pub type W = crate::W<SCRrs>;
/**Clear alarm A flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRAF {
    ///1: Clear interrupt flag
    Clear = 1,
}
impl From<CALRAF> for bool {
    #[inline(always)]
    fn from(variant: CALRAF) -> Self {
        variant as u8 != 0
    }
}
///Field `CALRAF` writer - Clear alarm A flag
pub type CALRAF_W<'a, REG> = crate::BitWriter<'a, REG, CALRAF>;
impl<'a, REG> CALRAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CALRAF::Clear)
    }
}
///Field `CALRBF` writer - Clear alarm B flag
pub use CALRAF_W as CALRBF_W;
///Field `CWUTF` writer - Clear wakeup timer flag
pub use CALRAF_W as CWUTF_W;
///Field `CTSF` writer - Clear timestamp flag
pub use CALRAF_W as CTSF_W;
///Field `CTSOVF` writer - Clear timestamp overflow flag
pub use CALRAF_W as CTSOVF_W;
///Field `CITSF` writer - Clear internal timestamp flag
pub use CALRAF_W as CITSF_W;
///Field `CSSRUF` writer - Clear SSR underflow flag
pub use CALRAF_W as CSSRUF_W;
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
    ///Bit 2 - Clear wakeup timer flag
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
/**Status clear register (interrupts)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RTC:SCR)*/
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
