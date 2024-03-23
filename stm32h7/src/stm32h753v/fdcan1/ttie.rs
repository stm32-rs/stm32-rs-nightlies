#[doc = "Register `TTIE` reader"]
pub type R = crate::R<TTIErs>;
#[doc = "Register `TTIE` writer"]
pub type W = crate::W<TTIErs>;
#[doc = "Field `SBCE` reader - Start of Basic Cycle Interrupt Enable"]
pub type SBCE_R = crate::BitReader;
#[doc = "Field `SBCE` writer - Start of Basic Cycle Interrupt Enable"]
pub type SBCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCE` reader - Start of Matrix Cycle Interrupt Enable"]
pub type SMCE_R = crate::BitReader;
#[doc = "Field `SMCE` writer - Start of Matrix Cycle Interrupt Enable"]
pub type SMCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSME` reader - Change of Synchronization Mode Interrupt Enable"]
pub type CSME_R = crate::BitReader;
#[doc = "Field `CSME` writer - Change of Synchronization Mode Interrupt Enable"]
pub type CSME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOGE` reader - Start of Gap Interrupt Enable"]
pub type SOGE_R = crate::BitReader;
#[doc = "Field `SOGE` writer - Start of Gap Interrupt Enable"]
pub type SOGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMIE` reader - Register Time Mark Interrupt Enable"]
pub type RTMIE_R = crate::BitReader;
#[doc = "Field `RTMIE` writer - Register Time Mark Interrupt Enable"]
pub type RTMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMIE` reader - Trigger Time Mark Event Internal Interrupt Enable"]
pub type TTMIE_R = crate::BitReader;
#[doc = "Field `TTMIE` writer - Trigger Time Mark Event Internal Interrupt Enable"]
pub type TTMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEE` reader - Stop Watch Event Interrupt Enable"]
pub type SWEE_R = crate::BitReader;
#[doc = "Field `SWEE` writer - Stop Watch Event Interrupt Enable"]
pub type SWEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTWE` reader - Global Time Wrap Interrupt Enable"]
pub type GTWE_R = crate::BitReader;
#[doc = "Field `GTWE` writer - Global Time Wrap Interrupt Enable"]
pub type GTWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTDE` reader - Global Time Discontinuity Interrupt Enable"]
pub type GTDE_R = crate::BitReader;
#[doc = "Field `GTDE` writer - Global Time Discontinuity Interrupt Enable"]
pub type GTDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTEE` reader - Global Time Error Interrupt Enable"]
pub type GTEE_R = crate::BitReader;
#[doc = "Field `GTEE` writer - Global Time Error Interrupt Enable"]
pub type GTEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUE` reader - Tx Count Underflow Interrupt Enable"]
pub type TXUE_R = crate::BitReader;
#[doc = "Field `TXUE` writer - Tx Count Underflow Interrupt Enable"]
pub type TXUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOE` reader - Tx Count Overflow Interrupt Enable"]
pub type TXOE_R = crate::BitReader;
#[doc = "Field `TXOE` writer - Tx Count Overflow Interrupt Enable"]
pub type TXOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1E` reader - Scheduling Error 1 Interrupt Enable"]
pub type SE1E_R = crate::BitReader;
#[doc = "Field `SE1E` writer - Scheduling Error 1 Interrupt Enable"]
pub type SE1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2E` reader - Scheduling Error 2 Interrupt Enable"]
pub type SE2E_R = crate::BitReader;
#[doc = "Field `SE2E` writer - Scheduling Error 2 Interrupt Enable"]
pub type SE2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELCE` reader - Change Error Level Interrupt Enable"]
pub type ELCE_R = crate::BitReader;
#[doc = "Field `ELCE` writer - Change Error Level Interrupt Enable"]
pub type ELCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTGE` reader - Initialization Watch Trigger Interrupt Enable"]
pub type IWTGE_R = crate::BitReader;
#[doc = "Field `IWTGE` writer - Initialization Watch Trigger Interrupt Enable"]
pub type IWTGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTE` reader - Watch Trigger Interrupt Enable"]
pub type WTE_R = crate::BitReader;
#[doc = "Field `WTE` writer - Watch Trigger Interrupt Enable"]
pub type WTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWE` reader - Application Watchdog Interrupt Enable"]
pub type AWE_R = crate::BitReader;
#[doc = "Field `AWE` writer - Application Watchdog Interrupt Enable"]
pub type AWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERE` reader - Configuration Error Interrupt Enable"]
pub type CERE_R = crate::BitReader;
#[doc = "Field `CERE` writer - Configuration Error Interrupt Enable"]
pub type CERE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn sbce(&self) -> SBCE_R {
        SBCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn smce(&self) -> SMCE_R {
        SMCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn csme(&self) -> CSME_R {
        CSME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn soge(&self) -> SOGE_R {
        SOGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn rtmie(&self) -> RTMIE_R {
        RTMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    pub fn ttmie(&self) -> TTMIE_R {
        TTMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&self) -> SWEE_R {
        SWEE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn gtwe(&self) -> GTWE_R {
        GTWE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn gtde(&self) -> GTDE_R {
        GTDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn gtee(&self) -> GTEE_R {
        GTEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txue(&self) -> TXUE_R {
        TXUE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txoe(&self) -> TXOE_R {
        TXOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn se1e(&self) -> SE1E_R {
        SE1E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn se2e(&self) -> SE2E_R {
        SE2E_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn elce(&self) -> ELCE_R {
        ELCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn iwtge(&self) -> IWTGE_R {
        IWTGE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn cere(&self) -> CERE_R {
        CERE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbce(&mut self) -> SBCE_W<TTIErs> {
        SBCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smce(&mut self) -> SMCE_W<TTIErs> {
        SMCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csme(&mut self) -> CSME_W<TTIErs> {
        CSME_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn soge(&mut self) -> SOGE_W<TTIErs> {
        SOGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtmie(&mut self) -> RTMIE_W<TTIErs> {
        RTMIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ttmie(&mut self) -> TTMIE_W<TTIErs> {
        TTMIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swee(&mut self) -> SWEE_W<TTIErs> {
        SWEE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtwe(&mut self) -> GTWE_W<TTIErs> {
        GTWE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtde(&mut self) -> GTDE_W<TTIErs> {
        GTDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtee(&mut self) -> GTEE_W<TTIErs> {
        GTEE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txue(&mut self) -> TXUE_W<TTIErs> {
        TXUE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txoe(&mut self) -> TXOE_W<TTIErs> {
        TXOE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se1e(&mut self) -> SE1E_W<TTIErs> {
        SE1E_W::new(self, 12)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se2e(&mut self) -> SE2E_W<TTIErs> {
        SE2E_W::new(self, 13)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn elce(&mut self) -> ELCE_W<TTIErs> {
        ELCE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iwtge(&mut self) -> IWTGE_W<TTIErs> {
        IWTGE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wte(&mut self) -> WTE_W<TTIErs> {
        WTE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn awe(&mut self) -> AWE_W<TTIErs> {
        AWE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cere(&mut self) -> CERE_W<TTIErs> {
        CERE_W::new(self, 18)
    }
}
#[doc = "FDCAN TT Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTIErs;
impl crate::RegisterSpec for TTIErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttie::R`](R) reader structure"]
impl crate::Readable for TTIErs {}
#[doc = "`write(|w| ..)` method takes [`ttie::W`](W) writer structure"]
impl crate::Writable for TTIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTIE to value 0"]
impl crate::Resettable for TTIErs {
    const RESET_VALUE: u32 = 0;
}
