#[doc = "Register `FDCAN_TTOCN` reader"]
pub type R = crate::R<FDCAN_TTOCNrs>;
#[doc = "Register `FDCAN_TTOCN` writer"]
pub type W = crate::W<FDCAN_TTOCNrs>;
#[doc = "Field `SGT` reader - SGT"]
pub type SGT_R = crate::BitReader;
#[doc = "Field `SGT` writer - SGT"]
pub type SGT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECS` reader - ECS"]
pub type ECS_R = crate::BitReader;
#[doc = "Field `ECS` writer - ECS"]
pub type ECS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWP` reader - SWP"]
pub type SWP_R = crate::BitReader;
#[doc = "Field `SWP` writer - SWP"]
pub type SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWS` reader - SWS"]
pub type SWS_R = crate::FieldReader;
#[doc = "Field `SWS` writer - SWS"]
pub type SWS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTIE` reader - RTIE"]
pub type RTIE_R = crate::BitReader;
#[doc = "Field `RTIE` writer - RTIE"]
pub type RTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMC` reader - TMC"]
pub type TMC_R = crate::FieldReader;
#[doc = "Field `TMC` writer - TMC"]
pub type TMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TTIE` reader - TTIE"]
pub type TTIE_R = crate::BitReader;
#[doc = "Field `TTIE` writer - TTIE"]
pub type TTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCS` reader - GCS"]
pub type GCS_R = crate::BitReader;
#[doc = "Field `GCS` writer - GCS"]
pub type GCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGP` reader - FGP"]
pub type FGP_R = crate::BitReader;
#[doc = "Field `FGP` writer - FGP"]
pub type FGP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMG` reader - TMG"]
pub type TMG_R = crate::BitReader;
#[doc = "Field `TMG` writer - TMG"]
pub type TMG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIG` reader - NIG"]
pub type NIG_R = crate::BitReader;
#[doc = "Field `NIG` writer - NIG"]
pub type NIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCN` reader - ESCN"]
pub type ESCN_R = crate::BitReader;
#[doc = "Field `ESCN` writer - ESCN"]
pub type ESCN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKC` reader - LCKC"]
pub type LCKC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SGT"]
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECS"]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWP"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - SWS"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - RTIE"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TMC"]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - TTIE"]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GCS"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FGP"]
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TMG"]
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NIG"]
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ESCN"]
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - LCKC"]
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SGT"]
    #[inline(always)]
    #[must_use]
    pub fn sgt(&mut self) -> SGT_W<FDCAN_TTOCNrs> {
        SGT_W::new(self, 0)
    }
    #[doc = "Bit 1 - ECS"]
    #[inline(always)]
    #[must_use]
    pub fn ecs(&mut self) -> ECS_W<FDCAN_TTOCNrs> {
        ECS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SWP"]
    #[inline(always)]
    #[must_use]
    pub fn swp(&mut self) -> SWP_W<FDCAN_TTOCNrs> {
        SWP_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - SWS"]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<FDCAN_TTOCNrs> {
        SWS_W::new(self, 3)
    }
    #[doc = "Bit 5 - RTIE"]
    #[inline(always)]
    #[must_use]
    pub fn rtie(&mut self) -> RTIE_W<FDCAN_TTOCNrs> {
        RTIE_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - TMC"]
    #[inline(always)]
    #[must_use]
    pub fn tmc(&mut self) -> TMC_W<FDCAN_TTOCNrs> {
        TMC_W::new(self, 6)
    }
    #[doc = "Bit 8 - TTIE"]
    #[inline(always)]
    #[must_use]
    pub fn ttie(&mut self) -> TTIE_W<FDCAN_TTOCNrs> {
        TTIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - GCS"]
    #[inline(always)]
    #[must_use]
    pub fn gcs(&mut self) -> GCS_W<FDCAN_TTOCNrs> {
        GCS_W::new(self, 9)
    }
    #[doc = "Bit 10 - FGP"]
    #[inline(always)]
    #[must_use]
    pub fn fgp(&mut self) -> FGP_W<FDCAN_TTOCNrs> {
        FGP_W::new(self, 10)
    }
    #[doc = "Bit 11 - TMG"]
    #[inline(always)]
    #[must_use]
    pub fn tmg(&mut self) -> TMG_W<FDCAN_TTOCNrs> {
        TMG_W::new(self, 11)
    }
    #[doc = "Bit 12 - NIG"]
    #[inline(always)]
    #[must_use]
    pub fn nig(&mut self) -> NIG_W<FDCAN_TTOCNrs> {
        NIG_W::new(self, 12)
    }
    #[doc = "Bit 13 - ESCN"]
    #[inline(always)]
    #[must_use]
    pub fn escn(&mut self) -> ESCN_W<FDCAN_TTOCNrs> {
        ESCN_W::new(self, 13)
    }
}
#[doc = "FDCAN TT operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttocn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttocn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTOCNrs;
impl crate::RegisterSpec for FDCAN_TTOCNrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttocn::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTOCNrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttocn::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTOCNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTOCN to value 0"]
impl crate::Resettable for FDCAN_TTOCNrs {
    const RESET_VALUE: u32 = 0;
}
