///Register `AHB1FZR` reader
pub type R = crate::R<AHB1FZRrs>;
///Register `AHB1FZR` writer
pub type W = crate::W<AHB1FZRrs>;
///Field `DBG_GPDMA1_CH0_STOP` reader - GPDMA 1 channel 0 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC0.
pub type DBG_GPDMA1_CH0_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH0_STOP` writer - GPDMA 1 channel 0 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC0.
pub type DBG_GPDMA1_CH0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_CH1_STOP` reader - GPDMA 1 channel 1 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC1.
pub type DBG_GPDMA1_CH1_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH1_STOP` writer - GPDMA 1 channel 1 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC1.
pub type DBG_GPDMA1_CH1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_CH2_STOP` reader - GPDMA 1 channel 2 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC2.
pub type DBG_GPDMA1_CH2_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH2_STOP` writer - GPDMA 1 channel 2 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC2.
pub type DBG_GPDMA1_CH2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_CH3_STOP` reader - GPDMA 1 channel 3 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC3.
pub type DBG_GPDMA1_CH3_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH3_STOP` writer - GPDMA 1 channel 3 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC3.
pub type DBG_GPDMA1_CH3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_CH4_STOP` reader - GPDMA 1 channel 4 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC4.
pub type DBG_GPDMA1_CH4_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH4_STOP` writer - GPDMA 1 channel 4 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC4.
pub type DBG_GPDMA1_CH4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_CH5_STOP` reader - GPDMA 1 channel 5 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC5.
pub type DBG_GPDMA1_CH5_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH5_STOP` writer - GPDMA 1 channel 5 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC5.
pub type DBG_GPDMA1_CH5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_CH6_STOP` reader - GPDMA 1 channel 6 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC6.
pub type DBG_GPDMA1_CH6_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH6_STOP` writer - GPDMA 1 channel 6 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC6.
pub type DBG_GPDMA1_CH6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_GPDMA1_CH7_STOP` reader - GPDMA 1 channel 7 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC7.
pub type DBG_GPDMA1_CH7_STOP_R = crate::BitReader;
///Field `DBG_GPDMA1_CH7_STOP` writer - GPDMA 1 channel 7 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC7.
pub type DBG_GPDMA1_CH7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPDMA 1 channel 0 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC0.
    #[inline(always)]
    pub fn dbg_gpdma1_ch0_stop(&self) -> DBG_GPDMA1_CH0_STOP_R {
        DBG_GPDMA1_CH0_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPDMA 1 channel 1 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC1.
    #[inline(always)]
    pub fn dbg_gpdma1_ch1_stop(&self) -> DBG_GPDMA1_CH1_STOP_R {
        DBG_GPDMA1_CH1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPDMA 1 channel 2 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC2.
    #[inline(always)]
    pub fn dbg_gpdma1_ch2_stop(&self) -> DBG_GPDMA1_CH2_STOP_R {
        DBG_GPDMA1_CH2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPDMA 1 channel 3 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC3.
    #[inline(always)]
    pub fn dbg_gpdma1_ch3_stop(&self) -> DBG_GPDMA1_CH3_STOP_R {
        DBG_GPDMA1_CH3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPDMA 1 channel 4 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC4.
    #[inline(always)]
    pub fn dbg_gpdma1_ch4_stop(&self) -> DBG_GPDMA1_CH4_STOP_R {
        DBG_GPDMA1_CH4_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPDMA 1 channel 5 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC5.
    #[inline(always)]
    pub fn dbg_gpdma1_ch5_stop(&self) -> DBG_GPDMA1_CH5_STOP_R {
        DBG_GPDMA1_CH5_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPDMA 1 channel 6 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC6.
    #[inline(always)]
    pub fn dbg_gpdma1_ch6_stop(&self) -> DBG_GPDMA1_CH6_STOP_R {
        DBG_GPDMA1_CH6_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPDMA 1 channel 7 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC7.
    #[inline(always)]
    pub fn dbg_gpdma1_ch7_stop(&self) -> DBG_GPDMA1_CH7_STOP_R {
        DBG_GPDMA1_CH7_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1FZR")
            .field("dbg_gpdma1_ch0_stop", &self.dbg_gpdma1_ch0_stop())
            .field("dbg_gpdma1_ch1_stop", &self.dbg_gpdma1_ch1_stop())
            .field("dbg_gpdma1_ch2_stop", &self.dbg_gpdma1_ch2_stop())
            .field("dbg_gpdma1_ch3_stop", &self.dbg_gpdma1_ch3_stop())
            .field("dbg_gpdma1_ch4_stop", &self.dbg_gpdma1_ch4_stop())
            .field("dbg_gpdma1_ch5_stop", &self.dbg_gpdma1_ch5_stop())
            .field("dbg_gpdma1_ch6_stop", &self.dbg_gpdma1_ch6_stop())
            .field("dbg_gpdma1_ch7_stop", &self.dbg_gpdma1_ch7_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA 1 channel 0 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC0.
    #[inline(always)]
    pub fn dbg_gpdma1_ch0_stop(&mut self) -> DBG_GPDMA1_CH0_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH0_STOP_W::new(self, 0)
    }
    ///Bit 1 - GPDMA 1 channel 1 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC1.
    #[inline(always)]
    pub fn dbg_gpdma1_ch1_stop(&mut self) -> DBG_GPDMA1_CH1_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH1_STOP_W::new(self, 1)
    }
    ///Bit 2 - GPDMA 1 channel 2 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC2.
    #[inline(always)]
    pub fn dbg_gpdma1_ch2_stop(&mut self) -> DBG_GPDMA1_CH2_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH2_STOP_W::new(self, 2)
    }
    ///Bit 3 - GPDMA 1 channel 3 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC3.
    #[inline(always)]
    pub fn dbg_gpdma1_ch3_stop(&mut self) -> DBG_GPDMA1_CH3_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH3_STOP_W::new(self, 3)
    }
    ///Bit 4 - GPDMA 1 channel 4 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC4.
    #[inline(always)]
    pub fn dbg_gpdma1_ch4_stop(&mut self) -> DBG_GPDMA1_CH4_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH4_STOP_W::new(self, 4)
    }
    ///Bit 5 - GPDMA 1 channel 5 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC5.
    #[inline(always)]
    pub fn dbg_gpdma1_ch5_stop(&mut self) -> DBG_GPDMA1_CH5_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH5_STOP_W::new(self, 5)
    }
    ///Bit 6 - GPDMA 1 channel 6 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC6.
    #[inline(always)]
    pub fn dbg_gpdma1_ch6_stop(&mut self) -> DBG_GPDMA1_CH6_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH6_STOP_W::new(self, 6)
    }
    ///Bit 7 - GPDMA 1 channel 7 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC7.
    #[inline(always)]
    pub fn dbg_gpdma1_ch7_stop(&mut self) -> DBG_GPDMA1_CH7_STOP_W<'_, AHB1FZRrs> {
        DBG_GPDMA1_CH7_STOP_W::new(self, 7)
    }
}
/**DBGMCU AHB1 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`ahb1fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#DBGMCU:AHB1FZR)*/
pub struct AHB1FZRrs;
impl crate::RegisterSpec for AHB1FZRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1fzr::R`](R) reader structure
impl crate::Readable for AHB1FZRrs {}
///`write(|w| ..)` method takes [`ahb1fzr::W`](W) writer structure
impl crate::Writable for AHB1FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1FZR to value 0
impl crate::Resettable for AHB1FZRrs {}
