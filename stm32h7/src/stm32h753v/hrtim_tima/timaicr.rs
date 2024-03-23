#[doc = "Register `TIMAICR` writer"]
pub type W = crate::W<TIMAICRrs>;
#[doc = "Field `CMP1C` writer - Compare 1 Interrupt flag Clear"]
pub type CMP1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2C` writer - Compare 2 Interrupt flag Clear"]
pub type CMP2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3C` writer - Compare 3 Interrupt flag Clear"]
pub type CMP3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP4C` writer - Compare 4 Interrupt flag Clear"]
pub type CMP4C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPC` writer - Repetition Interrupt flag Clear"]
pub type REPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDC` writer - Update Interrupt flag Clear"]
pub type UPDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPT1C` writer - Capture1 Interrupt flag Clear"]
pub type CPT1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPT2C` writer - Capture2 Interrupt flag Clear"]
pub type CPT2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET1xC` writer - Output 1 Set flag Clear"]
pub type SET1X_C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTx1C` writer - Output 1 Reset flag Clear"]
pub type RSTX1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET2xC` writer - Output 2 Set flag Clear"]
pub type SET2X_C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTx2C` writer - Output 2 Reset flag Clear"]
pub type RSTX2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTC` writer - Reset Interrupt flag Clear"]
pub type RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYPRTC` writer - Delayed Protection Flag Clear"]
pub type DLYPRTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Compare 1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1c(&mut self) -> CMP1C_W<TIMAICRrs> {
        CMP1C_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2c(&mut self) -> CMP2C_W<TIMAICRrs> {
        CMP2C_W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3c(&mut self) -> CMP3C_W<TIMAICRrs> {
        CMP3C_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4c(&mut self) -> CMP4C_W<TIMAICRrs> {
        CMP4C_W::new(self, 3)
    }
    #[doc = "Bit 4 - Repetition Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn repc(&mut self) -> REPC_W<TIMAICRrs> {
        REPC_W::new(self, 4)
    }
    #[doc = "Bit 6 - Update Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn updc(&mut self) -> UPDC_W<TIMAICRrs> {
        UPDC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture1 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt1c(&mut self) -> CPT1C_W<TIMAICRrs> {
        CPT1C_W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture2 Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn cpt2c(&mut self) -> CPT2C_W<TIMAICRrs> {
        CPT2C_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output 1 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set1x_c(&mut self) -> SET1X_C_W<TIMAICRrs> {
        SET1X_C_W::new(self, 9)
    }
    #[doc = "Bit 10 - Output 1 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx1c(&mut self) -> RSTX1C_W<TIMAICRrs> {
        RSTX1C_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output 2 Set flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn set2x_c(&mut self) -> SET2X_C_W<TIMAICRrs> {
        SET2X_C_W::new(self, 11)
    }
    #[doc = "Bit 12 - Output 2 Reset flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstx2c(&mut self) -> RSTX2C_W<TIMAICRrs> {
        RSTX2C_W::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Interrupt flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RSTC_W<TIMAICRrs> {
        RSTC_W::new(self, 13)
    }
    #[doc = "Bit 14 - Delayed Protection Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W<TIMAICRrs> {
        DLYPRTC_W::new(self, 14)
    }
}
#[doc = "Timerx Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timaicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMAICRrs;
impl crate::RegisterSpec for TIMAICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timaicr::W`](W) writer structure"]
impl crate::Writable for TIMAICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMAICR to value 0"]
impl crate::Resettable for TIMAICRrs {
    const RESET_VALUE: u32 = 0;
}
