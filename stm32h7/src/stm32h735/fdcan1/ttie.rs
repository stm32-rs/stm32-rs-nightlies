///Register `TTIE` reader
pub type R = crate::R<TTIErs>;
///Register `TTIE` writer
pub type W = crate::W<TTIErs>;
///Field `SBCE` reader - Start of Basic Cycle Interrupt Enable
pub type SBCE_R = crate::BitReader;
///Field `SBCE` writer - Start of Basic Cycle Interrupt Enable
pub type SBCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMCE` reader - Start of Matrix Cycle Interrupt Enable
pub type SMCE_R = crate::BitReader;
///Field `SMCE` writer - Start of Matrix Cycle Interrupt Enable
pub type SMCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSME` reader - Change of Synchronization Mode Interrupt Enable
pub type CSME_R = crate::BitReader;
///Field `CSME` writer - Change of Synchronization Mode Interrupt Enable
pub type CSME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOGE` reader - Start of Gap Interrupt Enable
pub type SOGE_R = crate::BitReader;
///Field `SOGE` writer - Start of Gap Interrupt Enable
pub type SOGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTMIE` reader - Register Time Mark Interrupt Enable
pub type RTMIE_R = crate::BitReader;
///Field `RTMIE` writer - Register Time Mark Interrupt Enable
pub type RTMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTMIE` reader - Trigger Time Mark Event Internal Interrupt Enable
pub type TTMIE_R = crate::BitReader;
///Field `TTMIE` writer - Trigger Time Mark Event Internal Interrupt Enable
pub type TTMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWEE` reader - Stop Watch Event Interrupt Enable
pub type SWEE_R = crate::BitReader;
///Field `SWEE` writer - Stop Watch Event Interrupt Enable
pub type SWEE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTWE` reader - Global Time Wrap Interrupt Enable
pub type GTWE_R = crate::BitReader;
///Field `GTWE` writer - Global Time Wrap Interrupt Enable
pub type GTWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTDE` reader - Global Time Discontinuity Interrupt Enable
pub type GTDE_R = crate::BitReader;
///Field `GTDE` writer - Global Time Discontinuity Interrupt Enable
pub type GTDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTEE` reader - Global Time Error Interrupt Enable
pub type GTEE_R = crate::BitReader;
///Field `GTEE` writer - Global Time Error Interrupt Enable
pub type GTEE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUE` reader - Tx Count Underflow Interrupt Enable
pub type TXUE_R = crate::BitReader;
///Field `TXUE` writer - Tx Count Underflow Interrupt Enable
pub type TXUE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOE` reader - Tx Count Overflow Interrupt Enable
pub type TXOE_R = crate::BitReader;
///Field `TXOE` writer - Tx Count Overflow Interrupt Enable
pub type TXOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE1E` reader - Scheduling Error 1 Interrupt Enable
pub type SE1E_R = crate::BitReader;
///Field `SE1E` writer - Scheduling Error 1 Interrupt Enable
pub type SE1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SE2E` reader - Scheduling Error 2 Interrupt Enable
pub type SE2E_R = crate::BitReader;
///Field `SE2E` writer - Scheduling Error 2 Interrupt Enable
pub type SE2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ELCE` reader - Change Error Level Interrupt Enable
pub type ELCE_R = crate::BitReader;
///Field `ELCE` writer - Change Error Level Interrupt Enable
pub type ELCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWTGE` reader - Initialization Watch Trigger Interrupt Enable
pub type IWTGE_R = crate::BitReader;
///Field `IWTGE` writer - Initialization Watch Trigger Interrupt Enable
pub type IWTGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WTE` reader - Watch Trigger Interrupt Enable
pub type WTE_R = crate::BitReader;
///Field `WTE` writer - Watch Trigger Interrupt Enable
pub type WTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWE` reader - Application Watchdog Interrupt Enable
pub type AWE_R = crate::BitReader;
///Field `AWE` writer - Application Watchdog Interrupt Enable
pub type AWE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CERE` reader - Configuration Error Interrupt Enable
pub type CERE_R = crate::BitReader;
///Field `CERE` writer - Configuration Error Interrupt Enable
pub type CERE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Start of Basic Cycle Interrupt Enable
    #[inline(always)]
    pub fn sbce(&self) -> SBCE_R {
        SBCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start of Matrix Cycle Interrupt Enable
    #[inline(always)]
    pub fn smce(&self) -> SMCE_R {
        SMCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Change of Synchronization Mode Interrupt Enable
    #[inline(always)]
    pub fn csme(&self) -> CSME_R {
        CSME_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start of Gap Interrupt Enable
    #[inline(always)]
    pub fn soge(&self) -> SOGE_R {
        SOGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Register Time Mark Interrupt Enable
    #[inline(always)]
    pub fn rtmie(&self) -> RTMIE_R {
        RTMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Trigger Time Mark Event Internal Interrupt Enable
    #[inline(always)]
    pub fn ttmie(&self) -> TTMIE_R {
        TTMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Stop Watch Event Interrupt Enable
    #[inline(always)]
    pub fn swee(&self) -> SWEE_R {
        SWEE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Global Time Wrap Interrupt Enable
    #[inline(always)]
    pub fn gtwe(&self) -> GTWE_R {
        GTWE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Global Time Discontinuity Interrupt Enable
    #[inline(always)]
    pub fn gtde(&self) -> GTDE_R {
        GTDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Global Time Error Interrupt Enable
    #[inline(always)]
    pub fn gtee(&self) -> GTEE_R {
        GTEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx Count Underflow Interrupt Enable
    #[inline(always)]
    pub fn txue(&self) -> TXUE_R {
        TXUE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx Count Overflow Interrupt Enable
    #[inline(always)]
    pub fn txoe(&self) -> TXOE_R {
        TXOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Scheduling Error 1 Interrupt Enable
    #[inline(always)]
    pub fn se1e(&self) -> SE1E_R {
        SE1E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Scheduling Error 2 Interrupt Enable
    #[inline(always)]
    pub fn se2e(&self) -> SE2E_R {
        SE2E_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Change Error Level Interrupt Enable
    #[inline(always)]
    pub fn elce(&self) -> ELCE_R {
        ELCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Initialization Watch Trigger Interrupt Enable
    #[inline(always)]
    pub fn iwtge(&self) -> IWTGE_R {
        IWTGE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Watch Trigger Interrupt Enable
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Application Watchdog Interrupt Enable
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Configuration Error Interrupt Enable
    #[inline(always)]
    pub fn cere(&self) -> CERE_R {
        CERE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTIE")
            .field("sbce", &self.sbce())
            .field("smce", &self.smce())
            .field("csme", &self.csme())
            .field("soge", &self.soge())
            .field("rtmie", &self.rtmie())
            .field("ttmie", &self.ttmie())
            .field("swee", &self.swee())
            .field("gtwe", &self.gtwe())
            .field("gtde", &self.gtde())
            .field("gtee", &self.gtee())
            .field("txue", &self.txue())
            .field("txoe", &self.txoe())
            .field("se1e", &self.se1e())
            .field("se2e", &self.se2e())
            .field("elce", &self.elce())
            .field("iwtge", &self.iwtge())
            .field("wte", &self.wte())
            .field("awe", &self.awe())
            .field("cere", &self.cere())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start of Basic Cycle Interrupt Enable
    #[inline(always)]
    pub fn sbce(&mut self) -> SBCE_W<'_, TTIErs> {
        SBCE_W::new(self, 0)
    }
    ///Bit 1 - Start of Matrix Cycle Interrupt Enable
    #[inline(always)]
    pub fn smce(&mut self) -> SMCE_W<'_, TTIErs> {
        SMCE_W::new(self, 1)
    }
    ///Bit 2 - Change of Synchronization Mode Interrupt Enable
    #[inline(always)]
    pub fn csme(&mut self) -> CSME_W<'_, TTIErs> {
        CSME_W::new(self, 2)
    }
    ///Bit 3 - Start of Gap Interrupt Enable
    #[inline(always)]
    pub fn soge(&mut self) -> SOGE_W<'_, TTIErs> {
        SOGE_W::new(self, 3)
    }
    ///Bit 4 - Register Time Mark Interrupt Enable
    #[inline(always)]
    pub fn rtmie(&mut self) -> RTMIE_W<'_, TTIErs> {
        RTMIE_W::new(self, 4)
    }
    ///Bit 5 - Trigger Time Mark Event Internal Interrupt Enable
    #[inline(always)]
    pub fn ttmie(&mut self) -> TTMIE_W<'_, TTIErs> {
        TTMIE_W::new(self, 5)
    }
    ///Bit 6 - Stop Watch Event Interrupt Enable
    #[inline(always)]
    pub fn swee(&mut self) -> SWEE_W<'_, TTIErs> {
        SWEE_W::new(self, 6)
    }
    ///Bit 7 - Global Time Wrap Interrupt Enable
    #[inline(always)]
    pub fn gtwe(&mut self) -> GTWE_W<'_, TTIErs> {
        GTWE_W::new(self, 7)
    }
    ///Bit 8 - Global Time Discontinuity Interrupt Enable
    #[inline(always)]
    pub fn gtde(&mut self) -> GTDE_W<'_, TTIErs> {
        GTDE_W::new(self, 8)
    }
    ///Bit 9 - Global Time Error Interrupt Enable
    #[inline(always)]
    pub fn gtee(&mut self) -> GTEE_W<'_, TTIErs> {
        GTEE_W::new(self, 9)
    }
    ///Bit 10 - Tx Count Underflow Interrupt Enable
    #[inline(always)]
    pub fn txue(&mut self) -> TXUE_W<'_, TTIErs> {
        TXUE_W::new(self, 10)
    }
    ///Bit 11 - Tx Count Overflow Interrupt Enable
    #[inline(always)]
    pub fn txoe(&mut self) -> TXOE_W<'_, TTIErs> {
        TXOE_W::new(self, 11)
    }
    ///Bit 12 - Scheduling Error 1 Interrupt Enable
    #[inline(always)]
    pub fn se1e(&mut self) -> SE1E_W<'_, TTIErs> {
        SE1E_W::new(self, 12)
    }
    ///Bit 13 - Scheduling Error 2 Interrupt Enable
    #[inline(always)]
    pub fn se2e(&mut self) -> SE2E_W<'_, TTIErs> {
        SE2E_W::new(self, 13)
    }
    ///Bit 14 - Change Error Level Interrupt Enable
    #[inline(always)]
    pub fn elce(&mut self) -> ELCE_W<'_, TTIErs> {
        ELCE_W::new(self, 14)
    }
    ///Bit 15 - Initialization Watch Trigger Interrupt Enable
    #[inline(always)]
    pub fn iwtge(&mut self) -> IWTGE_W<'_, TTIErs> {
        IWTGE_W::new(self, 15)
    }
    ///Bit 16 - Watch Trigger Interrupt Enable
    #[inline(always)]
    pub fn wte(&mut self) -> WTE_W<'_, TTIErs> {
        WTE_W::new(self, 16)
    }
    ///Bit 17 - Application Watchdog Interrupt Enable
    #[inline(always)]
    pub fn awe(&mut self) -> AWE_W<'_, TTIErs> {
        AWE_W::new(self, 17)
    }
    ///Bit 18 - Configuration Error Interrupt Enable
    #[inline(always)]
    pub fn cere(&mut self) -> CERE_W<'_, TTIErs> {
        CERE_W::new(self, 18)
    }
}
/**FDCAN TT Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ttie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FDCAN1:TTIE)*/
pub struct TTIErs;
impl crate::RegisterSpec for TTIErs {
    type Ux = u32;
}
///`read()` method returns [`ttie::R`](R) reader structure
impl crate::Readable for TTIErs {}
///`write(|w| ..)` method takes [`ttie::W`](W) writer structure
impl crate::Writable for TTIErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTIE to value 0
impl crate::Resettable for TTIErs {}
