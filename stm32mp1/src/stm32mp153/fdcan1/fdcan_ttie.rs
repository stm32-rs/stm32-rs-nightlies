#[doc = "Register `FDCAN_TTIE` reader"]
pub type R = crate::R<FDCAN_TTIErs>;
#[doc = "Register `FDCAN_TTIE` writer"]
pub type W = crate::W<FDCAN_TTIErs>;
#[doc = "Field `SBCE` reader - SBCE"]
pub type SBCE_R = crate::BitReader;
#[doc = "Field `SBCE` writer - SBCE"]
pub type SBCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCE` reader - SMCE"]
pub type SMCE_R = crate::BitReader;
#[doc = "Field `SMCE` writer - SMCE"]
pub type SMCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSME` reader - CSME"]
pub type CSME_R = crate::BitReader;
#[doc = "Field `CSME` writer - CSME"]
pub type CSME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOGE` reader - SOGE"]
pub type SOGE_R = crate::BitReader;
#[doc = "Field `SOGE` writer - SOGE"]
pub type SOGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMIE` reader - RTMIE"]
pub type RTMIE_R = crate::BitReader;
#[doc = "Field `RTMIE` writer - RTMIE"]
pub type RTMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMIE` reader - TTMIE"]
pub type TTMIE_R = crate::BitReader;
#[doc = "Field `TTMIE` writer - TTMIE"]
pub type TTMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEE` reader - SWEE"]
pub type SWEE_R = crate::BitReader;
#[doc = "Field `SWEE` writer - SWEE"]
pub type SWEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTWE` reader - GTWE"]
pub type GTWE_R = crate::BitReader;
#[doc = "Field `GTWE` writer - GTWE"]
pub type GTWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTDE` reader - GTDE"]
pub type GTDE_R = crate::BitReader;
#[doc = "Field `GTDE` writer - GTDE"]
pub type GTDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTEE` reader - GTEE"]
pub type GTEE_R = crate::BitReader;
#[doc = "Field `GTEE` writer - GTEE"]
pub type GTEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUE` reader - TXUE"]
pub type TXUE_R = crate::BitReader;
#[doc = "Field `TXUE` writer - TXUE"]
pub type TXUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOE` reader - TXOE"]
pub type TXOE_R = crate::BitReader;
#[doc = "Field `TXOE` writer - TXOE"]
pub type TXOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1E` reader - SE1E"]
pub type SE1E_R = crate::BitReader;
#[doc = "Field `SE1E` writer - SE1E"]
pub type SE1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2E` reader - SE2E"]
pub type SE2E_R = crate::BitReader;
#[doc = "Field `SE2E` writer - SE2E"]
pub type SE2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELCE` reader - ELCE"]
pub type ELCE_R = crate::BitReader;
#[doc = "Field `ELCE` writer - ELCE"]
pub type ELCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTE` reader - IWTE"]
pub type IWTE_R = crate::BitReader;
#[doc = "Field `IWTE` writer - IWTE"]
pub type IWTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTE` reader - WTE"]
pub type WTE_R = crate::BitReader;
#[doc = "Field `WTE` writer - WTE"]
pub type WTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWE` reader - AWE"]
pub type AWE_R = crate::BitReader;
#[doc = "Field `AWE` writer - AWE"]
pub type AWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERE` reader - CERE"]
pub type CERE_R = crate::BitReader;
#[doc = "Field `CERE` writer - CERE"]
pub type CERE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SBCE"]
    #[inline(always)]
    pub fn sbce(&self) -> SBCE_R {
        SBCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMCE"]
    #[inline(always)]
    pub fn smce(&self) -> SMCE_R {
        SMCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSME"]
    #[inline(always)]
    pub fn csme(&self) -> CSME_R {
        CSME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOGE"]
    #[inline(always)]
    pub fn soge(&self) -> SOGE_R {
        SOGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTMIE"]
    #[inline(always)]
    pub fn rtmie(&self) -> RTMIE_R {
        RTMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TTMIE"]
    #[inline(always)]
    pub fn ttmie(&self) -> TTMIE_R {
        TTMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWEE"]
    #[inline(always)]
    pub fn swee(&self) -> SWEE_R {
        SWEE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTWE"]
    #[inline(always)]
    pub fn gtwe(&self) -> GTWE_R {
        GTWE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTDE"]
    #[inline(always)]
    pub fn gtde(&self) -> GTDE_R {
        GTDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTEE"]
    #[inline(always)]
    pub fn gtee(&self) -> GTEE_R {
        GTEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUE"]
    #[inline(always)]
    pub fn txue(&self) -> TXUE_R {
        TXUE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXOE"]
    #[inline(always)]
    pub fn txoe(&self) -> TXOE_R {
        TXOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SE1E"]
    #[inline(always)]
    pub fn se1e(&self) -> SE1E_R {
        SE1E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SE2E"]
    #[inline(always)]
    pub fn se2e(&self) -> SE2E_R {
        SE2E_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ELCE"]
    #[inline(always)]
    pub fn elce(&self) -> ELCE_R {
        ELCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IWTE"]
    #[inline(always)]
    pub fn iwte(&self) -> IWTE_R {
        IWTE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WTE"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWE"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CERE"]
    #[inline(always)]
    pub fn cere(&self) -> CERE_R {
        CERE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBCE"]
    #[inline(always)]
    #[must_use]
    pub fn sbce(&mut self) -> SBCE_W<FDCAN_TTIErs> {
        SBCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SMCE"]
    #[inline(always)]
    #[must_use]
    pub fn smce(&mut self) -> SMCE_W<FDCAN_TTIErs> {
        SMCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CSME"]
    #[inline(always)]
    #[must_use]
    pub fn csme(&mut self) -> CSME_W<FDCAN_TTIErs> {
        CSME_W::new(self, 2)
    }
    #[doc = "Bit 3 - SOGE"]
    #[inline(always)]
    #[must_use]
    pub fn soge(&mut self) -> SOGE_W<FDCAN_TTIErs> {
        SOGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTMIE"]
    #[inline(always)]
    #[must_use]
    pub fn rtmie(&mut self) -> RTMIE_W<FDCAN_TTIErs> {
        RTMIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - TTMIE"]
    #[inline(always)]
    #[must_use]
    pub fn ttmie(&mut self) -> TTMIE_W<FDCAN_TTIErs> {
        TTMIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - SWEE"]
    #[inline(always)]
    #[must_use]
    pub fn swee(&mut self) -> SWEE_W<FDCAN_TTIErs> {
        SWEE_W::new(self, 6)
    }
    #[doc = "Bit 7 - GTWE"]
    #[inline(always)]
    #[must_use]
    pub fn gtwe(&mut self) -> GTWE_W<FDCAN_TTIErs> {
        GTWE_W::new(self, 7)
    }
    #[doc = "Bit 8 - GTDE"]
    #[inline(always)]
    #[must_use]
    pub fn gtde(&mut self) -> GTDE_W<FDCAN_TTIErs> {
        GTDE_W::new(self, 8)
    }
    #[doc = "Bit 9 - GTEE"]
    #[inline(always)]
    #[must_use]
    pub fn gtee(&mut self) -> GTEE_W<FDCAN_TTIErs> {
        GTEE_W::new(self, 9)
    }
    #[doc = "Bit 10 - TXUE"]
    #[inline(always)]
    #[must_use]
    pub fn txue(&mut self) -> TXUE_W<FDCAN_TTIErs> {
        TXUE_W::new(self, 10)
    }
    #[doc = "Bit 11 - TXOE"]
    #[inline(always)]
    #[must_use]
    pub fn txoe(&mut self) -> TXOE_W<FDCAN_TTIErs> {
        TXOE_W::new(self, 11)
    }
    #[doc = "Bit 12 - SE1E"]
    #[inline(always)]
    #[must_use]
    pub fn se1e(&mut self) -> SE1E_W<FDCAN_TTIErs> {
        SE1E_W::new(self, 12)
    }
    #[doc = "Bit 13 - SE2E"]
    #[inline(always)]
    #[must_use]
    pub fn se2e(&mut self) -> SE2E_W<FDCAN_TTIErs> {
        SE2E_W::new(self, 13)
    }
    #[doc = "Bit 14 - ELCE"]
    #[inline(always)]
    #[must_use]
    pub fn elce(&mut self) -> ELCE_W<FDCAN_TTIErs> {
        ELCE_W::new(self, 14)
    }
    #[doc = "Bit 15 - IWTE"]
    #[inline(always)]
    #[must_use]
    pub fn iwte(&mut self) -> IWTE_W<FDCAN_TTIErs> {
        IWTE_W::new(self, 15)
    }
    #[doc = "Bit 16 - WTE"]
    #[inline(always)]
    #[must_use]
    pub fn wte(&mut self) -> WTE_W<FDCAN_TTIErs> {
        WTE_W::new(self, 16)
    }
    #[doc = "Bit 17 - AWE"]
    #[inline(always)]
    #[must_use]
    pub fn awe(&mut self) -> AWE_W<FDCAN_TTIErs> {
        AWE_W::new(self, 17)
    }
    #[doc = "Bit 18 - CERE"]
    #[inline(always)]
    #[must_use]
    pub fn cere(&mut self) -> CERE_W<FDCAN_TTIErs> {
        CERE_W::new(self, 18)
    }
}
#[doc = "The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTIErs;
impl crate::RegisterSpec for FDCAN_TTIErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttie::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTIErs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttie::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTIE to value 0"]
impl crate::Resettable for FDCAN_TTIErs {
    const RESET_VALUE: u32 = 0;
}
