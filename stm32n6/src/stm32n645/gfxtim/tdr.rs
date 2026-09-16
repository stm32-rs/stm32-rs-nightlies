///Register `TDR` writer
pub type W = crate::W<TDRrs>;
///Field `AFCDIS` writer - absolute frame counter disable
pub type AFCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALCDIS` writer - absolute line counter disable
pub type ALCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC1DIS` writer - relative frame counter 1 disable
pub type RFC1DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC2DIS` writer - relative frame counter 2 disable
pub type RFC2DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - absolute frame counter disable
    #[inline(always)]
    pub fn afcdis(&mut self) -> AFCDIS_W<'_, TDRrs> {
        AFCDIS_W::new(self, 0)
    }
    ///Bit 4 - absolute line counter disable
    #[inline(always)]
    pub fn alcdis(&mut self) -> ALCDIS_W<'_, TDRrs> {
        ALCDIS_W::new(self, 4)
    }
    ///Bit 16 - relative frame counter 1 disable
    #[inline(always)]
    pub fn rfc1dis(&mut self) -> RFC1DIS_W<'_, TDRrs> {
        RFC1DIS_W::new(self, 16)
    }
    ///Bit 20 - relative frame counter 2 disable
    #[inline(always)]
    pub fn rfc2dis(&mut self) -> RFC2DIS_W<'_, TDRrs> {
        RFC2DIS_W::new(self, 20)
    }
}
/**GFXTIM timers disable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:TDR)*/
pub struct TDRrs;
impl crate::RegisterSpec for TDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tdr::W`](W) writer structure
impl crate::Writable for TDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDR to value 0
impl crate::Resettable for TDRrs {}
