///Register `BCLRFR` writer
pub type W = crate::W<BCLRFRrs>;
///Field `OVRUDR` writer - Clear overrun / underrun
pub type OVRUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUTEDET` writer - Mute detection flag
pub type MUTEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WCKCFG` writer - Clear wrong clock configuration flag
pub type WCKCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNRDY` writer - Clear codec not ready flag
pub type CNRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFSDET` writer - Clear late frame synchronization detection flag
pub type LFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BCLRFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&mut self) -> OVRUDR_W<'_, BCLRFRrs> {
        OVRUDR_W::new(self, 0)
    }
    ///Bit 1 - Mute detection flag
    #[inline(always)]
    pub fn mutedet(&mut self) -> MUTEDET_W<'_, BCLRFRrs> {
        MUTEDET_W::new(self, 1)
    }
    ///Bit 2 - Clear wrong clock configuration flag
    #[inline(always)]
    pub fn wckcfg(&mut self) -> WCKCFG_W<'_, BCLRFRrs> {
        WCKCFG_W::new(self, 2)
    }
    ///Bit 4 - Clear codec not ready flag
    #[inline(always)]
    pub fn cnrdy(&mut self) -> CNRDY_W<'_, BCLRFRrs> {
        CNRDY_W::new(self, 4)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<'_, BCLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    ///Bit 6 - Clear late frame synchronization detection flag
    #[inline(always)]
    pub fn lfsdet(&mut self) -> LFSDET_W<'_, BCLRFRrs> {
        LFSDET_W::new(self, 6)
    }
}
/**BClear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SAI:BCLRFR)*/
pub struct BCLRFRrs;
impl crate::RegisterSpec for BCLRFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bclrfr::W`](W) writer structure
impl crate::Writable for BCLRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCLRFR to value 0
impl crate::Resettable for BCLRFRrs {}
