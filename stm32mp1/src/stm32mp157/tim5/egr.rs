///Register `EGR` writer
pub type W = crate::W<EGRrs>;
///Field `UG` writer - UG
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1G` writer - CC1G
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2G` writer - CC2G
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3G` writer - CC3G
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4G` writer - CC4G
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMG` writer - COMG
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TG` writer - TG
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BG` writer - BG
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2G` writer - B2G
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - UG
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - CC1G
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<'_, EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - CC2G
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<'_, EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 3 - CC3G
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<'_, EGRrs> {
        CC3G_W::new(self, 3)
    }
    ///Bit 4 - CC4G
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<'_, EGRrs> {
        CC4G_W::new(self, 4)
    }
    ///Bit 5 - COMG
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<'_, EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 6 - TG
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<'_, EGRrs> {
        TG_W::new(self, 6)
    }
    ///Bit 7 - BG
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<'_, EGRrs> {
        BG_W::new(self, 7)
    }
    ///Bit 8 - B2G
    #[inline(always)]
    pub fn b2g(&mut self) -> B2G_W<'_, EGRrs> {
        B2G_W::new(self, 8)
    }
}
/**TIM5 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM5:EGR)*/
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`egr::W`](W) writer structure
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EGR to value 0
impl crate::Resettable for EGRrs {}
