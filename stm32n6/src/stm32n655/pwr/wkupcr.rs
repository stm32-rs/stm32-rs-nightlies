///Register `WKUPCR` writer
pub type W = crate::W<WKUPCRrs>;
///Field `WKUPC1` writer - Clear wake-up flag for WKUP1 pin
pub type WKUPC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC2` writer - Clear wake-up flag for WKUP2 pin
pub type WKUPC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC3` writer - Clear wake-up flag for WKUP3 pin
pub type WKUPC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPC4` writer - Clear wake-up flag for WKUP4 pin
pub type WKUPC4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<WKUPCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear wake-up flag for WKUP1 pin
    #[inline(always)]
    pub fn wkupc1(&mut self) -> WKUPC1_W<'_, WKUPCRrs> {
        WKUPC1_W::new(self, 0)
    }
    ///Bit 1 - Clear wake-up flag for WKUP2 pin
    #[inline(always)]
    pub fn wkupc2(&mut self) -> WKUPC2_W<'_, WKUPCRrs> {
        WKUPC2_W::new(self, 1)
    }
    ///Bit 2 - Clear wake-up flag for WKUP3 pin
    #[inline(always)]
    pub fn wkupc3(&mut self) -> WKUPC3_W<'_, WKUPCRrs> {
        WKUPC3_W::new(self, 2)
    }
    ///Bit 3 - Clear wake-up flag for WKUP4 pin
    #[inline(always)]
    pub fn wkupc4(&mut self) -> WKUPC4_W<'_, WKUPCRrs> {
        WKUPC4_W::new(self, 3)
    }
}
/**PWR wake-up clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#PWR:WKUPCR)*/
pub struct WKUPCRrs;
impl crate::RegisterSpec for WKUPCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure
impl crate::Writable for WKUPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WKUPCR to value 0
impl crate::Resettable for WKUPCRrs {}
