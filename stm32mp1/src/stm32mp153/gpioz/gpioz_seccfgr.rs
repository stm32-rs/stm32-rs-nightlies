///Register `GPIOZ_SECCFGR` writer
pub type W = crate::W<GPIOZ_SECCFGRrs>;
///Field `SEC0` writer - SEC0
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC1` writer - SEC1
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC2` writer - SEC2
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC3` writer - SEC3
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC4` writer - SEC4
pub type SEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC5` writer - SEC5
pub type SEC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC6` writer - SEC6
pub type SEC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC7` writer - SEC7
pub type SEC7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GPIOZ_SECCFGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - SEC0
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W<'_, GPIOZ_SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W<'_, GPIOZ_SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W<'_, GPIOZ_SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W<'_, GPIOZ_SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    ///Bit 4 - SEC4
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W<'_, GPIOZ_SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    ///Bit 5 - SEC5
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W<'_, GPIOZ_SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    ///Bit 6 - SEC6
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W<'_, GPIOZ_SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    ///Bit 7 - SEC7
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W<'_, GPIOZ_SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
}
/**This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioz_seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_SECCFGR)*/
pub struct GPIOZ_SECCFGRrs;
impl crate::RegisterSpec for GPIOZ_SECCFGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gpioz_seccfgr::W`](W) writer structure
impl crate::Writable for GPIOZ_SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOZ_SECCFGR to value 0xff
impl crate::Resettable for GPIOZ_SECCFGRrs {
    const RESET_VALUE: u32 = 0xff;
}
