#[doc = "Register `TIMBICR` writer"]
pub type W = crate::W<TIMBICRrs>;
#[doc = "Compare 1 Interrupt flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1C {
    #[doc = "1: Clears associated flag in ISR register"]
    Clear = 1,
}
impl From<CMP1C> for bool {
    #[inline(always)]
    fn from(variant: CMP1C) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1C` writer - Compare 1 Interrupt flag Clear"]
pub type CMP1C_W<'a, REG> = crate::BitWriter<'a, REG, CMP1C>;
impl<'a, REG> CMP1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1C::Clear)
    }
}
#[doc = "Field `CMP2C` writer - Compare 2 Interrupt flag Clear"]
pub use CMP1C_W as CMP2C_W;
#[doc = "Field `CMP3C` writer - Compare 3 Interrupt flag Clear"]
pub use CMP1C_W as CMP3C_W;
#[doc = "Field `CMP4C` writer - Compare 4 Interrupt flag Clear"]
pub use CMP1C_W as CMP4C_W;
#[doc = "Field `REPC` writer - Repetition Interrupt flag Clear"]
pub use CMP1C_W as REPC_W;
#[doc = "Field `UPDC` writer - Update Interrupt flag Clear"]
pub use CMP1C_W as UPDC_W;
#[doc = "Field `CPT1C` writer - Capture1 Interrupt flag Clear"]
pub use CMP1C_W as CPT1C_W;
#[doc = "Field `CPT2C` writer - Capture2 Interrupt flag Clear"]
pub use CMP1C_W as CPT2C_W;
#[doc = "Field `SET1xC` writer - Output 1 Set flag Clear"]
pub use CMP1C_W as SET1X_C_W;
#[doc = "Field `RSTx1C` writer - Output 1 Reset flag Clear"]
pub use CMP1C_W as RSTX1C_W;
#[doc = "Field `SET2xC` writer - Output 2 Set flag Clear"]
pub use CMP1C_W as SET2X_C_W;
#[doc = "Field `RSTx2C` writer - Output 2 Reset flag Clear"]
pub use CMP1C_W as RSTX2C_W;
#[doc = "Field `RSTC` writer - Reset Interrupt flag Clear"]
pub use CMP1C_W as RSTC_W;
#[doc = "Field `DLYPRTC` writer - Delayed Protection Flag Clear"]
pub use CMP1C_W as DLYPRTC_W;
impl W {
    #[doc = "Bit 0 - Compare 1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1c(&mut self) -> CMP1C_W<TIMBICRrs> {
        CMP1C_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2c(&mut self) -> CMP2C_W<TIMBICRrs> {
        CMP2C_W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3c(&mut self) -> CMP3C_W<TIMBICRrs> {
        CMP3C_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4c(&mut self) -> CMP4C_W<TIMBICRrs> {
        CMP4C_W::new(self, 3)
    }
    #[doc = "Bit 4 - Repetition Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn repc(&mut self) -> REPC_W<TIMBICRrs> {
        REPC_W::new(self, 4)
    }
    #[doc = "Bit 6 - Update Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn updc(&mut self) -> UPDC_W<TIMBICRrs> {
        UPDC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt1c(&mut self) -> CPT1C_W<TIMBICRrs> {
        CPT1C_W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt2c(&mut self) -> CPT2C_W<TIMBICRrs> {
        CPT2C_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output 1 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set1x_c(&mut self) -> SET1X_C_W<TIMBICRrs> {
        SET1X_C_W::new(self, 9)
    }
    #[doc = "Bit 10 - Output 1 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx1c(&mut self) -> RSTX1C_W<TIMBICRrs> {
        RSTX1C_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output 2 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set2x_c(&mut self) -> SET2X_C_W<TIMBICRrs> {
        SET2X_C_W::new(self, 11)
    }
    #[doc = "Bit 12 - Output 2 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx2c(&mut self) -> RSTX2C_W<TIMBICRrs> {
        RSTX2C_W::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RSTC_W<TIMBICRrs> {
        RSTC_W::new(self, 13)
    }
    #[doc = "Bit 14 - Delayed Protection Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W<TIMBICRrs> {
        DLYPRTC_W::new(self, 14)
    }
}
#[doc = "Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMBICRrs;
impl crate::RegisterSpec for TIMBICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timbicr::W`](W) writer structure"]
impl crate::Writable for TIMBICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMBICR to value 0"]
impl crate::Resettable for TIMBICRrs {
    const RESET_VALUE: u32 = 0;
}
