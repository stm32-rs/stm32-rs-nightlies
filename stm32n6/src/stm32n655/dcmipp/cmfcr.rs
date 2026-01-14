///Register `CMFCR` writer
pub type W = crate::W<CMFCRrs>;
///Field `CATXERRF` writer - AXI transfer error interrupt status clear
pub type CATXERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPRERRF` writer - Synchronization error interrupt status clear
pub type CPRERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP0LINEF` writer - Multi-line capture complete interrupt status clear
pub type CP0LINEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP0FRAMEF` writer - Frame capture complete interrupt status clear
pub type CP0FRAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP0VSYNCF` writer - Vertical synchronization interrupt status clear
pub type CP0VSYNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP0LIMITF` writer - limit interrupt status clear
pub type CP0LIMITF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP0OVRF` writer - Overrun interrupt status clear
pub type CP0OVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP1LINEF` writer - Multi-line capture complete interrupt status clear
pub type CP1LINEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP1FRAMEF` writer - Frame capture complete interrupt status clear
pub type CP1FRAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP1VSYNCF` writer - Vertical synchronization interrupt status clear
pub type CP1VSYNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP1OVRF` writer - Overrun interrupt status clear
pub type CP1OVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP2LINEF` writer - Multi-line capture complete interrupt status clear
pub type CP2LINEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP2FRAMEF` writer - Frame capture complete interrupt status clear
pub type CP2FRAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP2VSYNCF` writer - Vertical synchronization interrupt status clear
pub type CP2VSYNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CP2OVRF` writer - Overrun interrupt status clear
pub type CP2OVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - AXI transfer error interrupt status clear
    #[inline(always)]
    pub fn catxerrf(&mut self) -> CATXERRF_W<'_, CMFCRrs> {
        CATXERRF_W::new(self, 5)
    }
    ///Bit 6 - Synchronization error interrupt status clear
    #[inline(always)]
    pub fn cprerrf(&mut self) -> CPRERRF_W<'_, CMFCRrs> {
        CPRERRF_W::new(self, 6)
    }
    ///Bit 8 - Multi-line capture complete interrupt status clear
    #[inline(always)]
    pub fn cp0linef(&mut self) -> CP0LINEF_W<'_, CMFCRrs> {
        CP0LINEF_W::new(self, 8)
    }
    ///Bit 9 - Frame capture complete interrupt status clear
    #[inline(always)]
    pub fn cp0framef(&mut self) -> CP0FRAMEF_W<'_, CMFCRrs> {
        CP0FRAMEF_W::new(self, 9)
    }
    ///Bit 10 - Vertical synchronization interrupt status clear
    #[inline(always)]
    pub fn cp0vsyncf(&mut self) -> CP0VSYNCF_W<'_, CMFCRrs> {
        CP0VSYNCF_W::new(self, 10)
    }
    ///Bit 14 - limit interrupt status clear
    #[inline(always)]
    pub fn cp0limitf(&mut self) -> CP0LIMITF_W<'_, CMFCRrs> {
        CP0LIMITF_W::new(self, 14)
    }
    ///Bit 15 - Overrun interrupt status clear
    #[inline(always)]
    pub fn cp0ovrf(&mut self) -> CP0OVRF_W<'_, CMFCRrs> {
        CP0OVRF_W::new(self, 15)
    }
    ///Bit 16 - Multi-line capture complete interrupt status clear
    #[inline(always)]
    pub fn cp1linef(&mut self) -> CP1LINEF_W<'_, CMFCRrs> {
        CP1LINEF_W::new(self, 16)
    }
    ///Bit 17 - Frame capture complete interrupt status clear
    #[inline(always)]
    pub fn cp1framef(&mut self) -> CP1FRAMEF_W<'_, CMFCRrs> {
        CP1FRAMEF_W::new(self, 17)
    }
    ///Bit 18 - Vertical synchronization interrupt status clear
    #[inline(always)]
    pub fn cp1vsyncf(&mut self) -> CP1VSYNCF_W<'_, CMFCRrs> {
        CP1VSYNCF_W::new(self, 18)
    }
    ///Bit 23 - Overrun interrupt status clear
    #[inline(always)]
    pub fn cp1ovrf(&mut self) -> CP1OVRF_W<'_, CMFCRrs> {
        CP1OVRF_W::new(self, 23)
    }
    ///Bit 24 - Multi-line capture complete interrupt status clear
    #[inline(always)]
    pub fn cp2linef(&mut self) -> CP2LINEF_W<'_, CMFCRrs> {
        CP2LINEF_W::new(self, 24)
    }
    ///Bit 25 - Frame capture complete interrupt status clear
    #[inline(always)]
    pub fn cp2framef(&mut self) -> CP2FRAMEF_W<'_, CMFCRrs> {
        CP2FRAMEF_W::new(self, 25)
    }
    ///Bit 26 - Vertical synchronization interrupt status clear
    #[inline(always)]
    pub fn cp2vsyncf(&mut self) -> CP2VSYNCF_W<'_, CMFCRrs> {
        CP2VSYNCF_W::new(self, 26)
    }
    ///Bit 31 - Overrun interrupt status clear
    #[inline(always)]
    pub fn cp2ovrf(&mut self) -> CP2OVRF_W<'_, CMFCRrs> {
        CP2OVRF_W::new(self, 31)
    }
}
/**DCMIPP common interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmfcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:CMFCR)*/
pub struct CMFCRrs;
impl crate::RegisterSpec for CMFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmfcr::W`](W) writer structure
impl crate::Writable for CMFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMFCR to value 0
impl crate::Resettable for CMFCRrs {}
