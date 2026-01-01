///Register `LMCFGR` reader
pub type R = crate::R<LMCFGRrs>;
///Register `LMCFGR` writer
pub type W = crate::W<LMCFGRrs>;
///Field `LANENB` reader - Number of lanes
pub type LANENB_R = crate::FieldReader;
///Field `LANENB` writer - Number of lanes
pub type LANENB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DL0MAP` reader - Physical mapping of logical data lane 0
pub type DL0MAP_R = crate::FieldReader;
///Field `DL0MAP` writer - Physical mapping of logical data lane 0
pub type DL0MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DL1MAP` reader - Physical mapping of logical data lane 1
pub type DL1MAP_R = crate::FieldReader;
///Field `DL1MAP` writer - Physical mapping of logical data lane 1
pub type DL1MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 8:10 - Number of lanes
    #[inline(always)]
    pub fn lanenb(&self) -> LANENB_R {
        LANENB_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:18 - Physical mapping of logical data lane 0
    #[inline(always)]
    pub fn dl0map(&self) -> DL0MAP_R {
        DL0MAP_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Physical mapping of logical data lane 1
    #[inline(always)]
    pub fn dl1map(&self) -> DL1MAP_R {
        DL1MAP_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LMCFGR")
            .field("lanenb", &self.lanenb())
            .field("dl0map", &self.dl0map())
            .field("dl1map", &self.dl1map())
            .finish()
    }
}
impl W {
    ///Bits 8:10 - Number of lanes
    #[inline(always)]
    pub fn lanenb(&mut self) -> LANENB_W<'_, LMCFGRrs> {
        LANENB_W::new(self, 8)
    }
    ///Bits 16:18 - Physical mapping of logical data lane 0
    #[inline(always)]
    pub fn dl0map(&mut self) -> DL0MAP_W<'_, LMCFGRrs> {
        DL0MAP_W::new(self, 16)
    }
    ///Bits 20:22 - Physical mapping of logical data lane 1
    #[inline(always)]
    pub fn dl1map(&mut self) -> DL1MAP_W<'_, LMCFGRrs> {
        DL1MAP_W::new(self, 20)
    }
}
/**CSI-2 Host lane merger configuration register

You can [`read`](crate::Reg::read) this register and get [`lmcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lmcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:LMCFGR)*/
pub struct LMCFGRrs;
impl crate::RegisterSpec for LMCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`lmcfgr::R`](R) reader structure
impl crate::Readable for LMCFGRrs {}
///`write(|w| ..)` method takes [`lmcfgr::W`](W) writer structure
impl crate::Writable for LMCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LMCFGR to value 0x4321_0200
impl crate::Resettable for LMCFGRrs {
    const RESET_VALUE: u32 = 0x4321_0200;
}
