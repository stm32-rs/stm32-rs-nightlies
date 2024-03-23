#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCRrs>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCRrs>;
#[doc = "Field `CK_PSC` reader - HRTIM Master Clock prescaler"]
pub type CK_PSC_R = crate::FieldReader;
#[doc = "Field `CK_PSC` writer - HRTIM Master Clock prescaler"]
pub type CK_PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONT` reader - Master Continuous mode"]
pub type CONT_R = crate::BitReader;
#[doc = "Field `CONT` writer - Master Continuous mode"]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRIG` reader - Master Re-triggerable mode"]
pub type RETRIG_R = crate::BitReader;
#[doc = "Field `RETRIG` writer - Master Re-triggerable mode"]
pub type RETRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALF` reader - Half mode enable"]
pub type HALF_R = crate::BitReader;
#[doc = "Field `HALF` writer - Half mode enable"]
pub type HALF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTLVD` reader - Interleaved mode"]
pub type INTLVD_R = crate::FieldReader;
#[doc = "Field `INTLVD` writer - Interleaved mode"]
pub type INTLVD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC_IN` reader - synchronization input"]
pub type SYNC_IN_R = crate::FieldReader;
#[doc = "Field `SYNC_IN` writer - synchronization input"]
pub type SYNC_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNCRSTM` reader - Synchronization Resets Master"]
pub type SYNCRSTM_R = crate::BitReader;
#[doc = "Field `SYNCRSTM` writer - Synchronization Resets Master"]
pub type SYNCRSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCSTRTM` reader - Synchronization Starts Master"]
pub type SYNCSTRTM_R = crate::BitReader;
#[doc = "Field `SYNCSTRTM` writer - Synchronization Starts Master"]
pub type SYNCSTRTM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_OUT` reader - Synchronization output"]
pub type SYNC_OUT_R = crate::FieldReader;
#[doc = "Field `SYNC_OUT` writer - Synchronization output"]
pub type SYNC_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC_SRC` reader - Synchronization source"]
pub type SYNC_SRC_R = crate::FieldReader;
#[doc = "Field `SYNC_SRC` writer - Synchronization source"]
pub type SYNC_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCEN` reader - Master Counter enable"]
pub type MCEN_R = crate::BitReader;
#[doc = "Field `MCEN` writer - Master Counter enable"]
pub type MCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACEN` reader - Timer A counter enable"]
pub type TACEN_R = crate::BitReader;
#[doc = "Field `TACEN` writer - Timer A counter enable"]
pub type TACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCEN` reader - Timer B counter enable"]
pub type TBCEN_R = crate::BitReader;
#[doc = "Field `TBCEN` writer - Timer B counter enable"]
pub type TBCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCEN` reader - Timer C counter enable"]
pub type TCCEN_R = crate::BitReader;
#[doc = "Field `TCCEN` writer - Timer C counter enable"]
pub type TCCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCEN` reader - Timer D counter enable"]
pub type TDCEN_R = crate::BitReader;
#[doc = "Field `TDCEN` writer - Timer D counter enable"]
pub type TDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TECEN` reader - Timer E counter enable"]
pub type TECEN_R = crate::BitReader;
#[doc = "Field `TECEN` writer - Timer E counter enable"]
pub type TECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCEN` reader - Timer F counter enable"]
pub type TFCEN_R = crate::BitReader;
#[doc = "Field `TFCEN` writer - Timer F counter enable"]
pub type TFCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub type DACSYNC_R = crate::FieldReader;
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub type DACSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREEN` reader - Preload enable"]
pub type PREEN_R = crate::BitReader;
#[doc = "Field `PREEN` writer - Preload enable"]
pub type PREEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPU` reader - Master Timer Repetition update"]
pub type MREPU_R = crate::BitReader;
#[doc = "Field `MREPU` writer - Master Timer Repetition update"]
pub type MREPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSTDMA` reader - Burst DMA Update"]
pub type BRSTDMA_R = crate::FieldReader;
#[doc = "Field `BRSTDMA` writer - Burst DMA Update"]
pub type BRSTDMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    pub fn ck_psc(&self) -> CK_PSC_R {
        CK_PSC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Interleaved mode"]
    #[inline(always)]
    pub fn intlvd(&self) -> INTLVD_R {
        INTLVD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - synchronization input"]
    #[inline(always)]
    pub fn sync_in(&self) -> SYNC_IN_R {
        SYNC_IN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    pub fn syncrstm(&self) -> SYNCRSTM_R {
        SYNCRSTM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    pub fn syncstrtm(&self) -> SYNCSTRTM_R {
        SYNCSTRTM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    pub fn sync_out(&self) -> SYNC_OUT_R {
        SYNC_OUT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    pub fn sync_src(&self) -> SYNC_SRC_R {
        SYNC_SRC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    pub fn tacen(&self) -> TACEN_R {
        TACEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    pub fn tbcen(&self) -> TBCEN_R {
        TBCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    pub fn tccen(&self) -> TCCEN_R {
        TCCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    pub fn tecen(&self) -> TECEN_R {
        TECEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer F counter enable"]
    #[inline(always)]
    pub fn tfcen(&self) -> TFCEN_R {
        TFCEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    pub fn mrepu(&self) -> MREPU_R {
        MREPU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    pub fn brstdma(&self) -> BRSTDMA_R {
        BRSTDMA_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Master Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ck_psc(&mut self) -> CK_PSC_W<MCRrs> {
        CK_PSC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Master Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<MCRrs> {
        CONT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<MCRrs> {
        RETRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<MCRrs> {
        HALF_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Interleaved mode"]
    #[inline(always)]
    #[must_use]
    pub fn intlvd(&mut self) -> INTLVD_W<MCRrs> {
        INTLVD_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - synchronization input"]
    #[inline(always)]
    #[must_use]
    pub fn sync_in(&mut self) -> SYNC_IN_W<MCRrs> {
        SYNC_IN_W::new(self, 8)
    }
    #[doc = "Bit 10 - Synchronization Resets Master"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstm(&mut self) -> SYNCRSTM_W<MCRrs> {
        SYNCRSTM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Starts Master"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtm(&mut self) -> SYNCSTRTM_W<MCRrs> {
        SYNCSTRTM_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Synchronization output"]
    #[inline(always)]
    #[must_use]
    pub fn sync_out(&mut self) -> SYNC_OUT_W<MCRrs> {
        SYNC_OUT_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Synchronization source"]
    #[inline(always)]
    #[must_use]
    pub fn sync_src(&mut self) -> SYNC_SRC_W<MCRrs> {
        SYNC_SRC_W::new(self, 14)
    }
    #[doc = "Bit 16 - Master Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcen(&mut self) -> MCEN_W<MCRrs> {
        MCEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer A counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tacen(&mut self) -> TACEN_W<MCRrs> {
        TACEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbcen(&mut self) -> TBCEN_W<MCRrs> {
        TBCEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer C counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tccen(&mut self) -> TCCEN_W<MCRrs> {
        TCCEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer D counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcen(&mut self) -> TDCEN_W<MCRrs> {
        TDCEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Timer E counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tecen(&mut self) -> TECEN_W<MCRrs> {
        TECEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Timer F counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfcen(&mut self) -> TFCEN_W<MCRrs> {
        TFCEN_W::new(self, 22)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<MCRrs> {
        DACSYNC_W::new(self, 25)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<MCRrs> {
        PREEN_W::new(self, 27)
    }
    #[doc = "Bit 29 - Master Timer Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn mrepu(&mut self) -> MREPU_W<MCRrs> {
        MREPU_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Burst DMA Update"]
    #[inline(always)]
    #[must_use]
    pub fn brstdma(&mut self) -> BRSTDMA_W<MCRrs> {
        BRSTDMA_W::new(self, 30)
    }
}
#[doc = "Master Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCRrs {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0;
}
