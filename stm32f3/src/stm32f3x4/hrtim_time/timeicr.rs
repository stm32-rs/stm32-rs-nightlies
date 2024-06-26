///Register `TIMEICR` writer
pub type W = crate::W<TIMEICRrs>;
/**Compare 1 Interrupt flag Clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1C {
    ///1: Clears associated flag in ISR register
    Clear = 1,
}
impl From<CMP1C> for bool {
    #[inline(always)]
    fn from(variant: CMP1C) -> Self {
        variant as u8 != 0
    }
}
///Field `CMP1C` writer - Compare 1 Interrupt flag Clear
pub type CMP1C_W<'a, REG> = crate::BitWriter<'a, REG, CMP1C>;
impl<'a, REG> CMP1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears associated flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1C::Clear)
    }
}
///Field `CMP2C` writer - Compare 2 Interrupt flag Clear
pub use CMP1C_W as CMP2C_W;
///Field `CMP3C` writer - Compare 3 Interrupt flag Clear
pub use CMP1C_W as CMP3C_W;
///Field `CMP4C` writer - Compare 4 Interrupt flag Clear
pub use CMP1C_W as CMP4C_W;
///Field `REPC` writer - Repetition Interrupt flag Clear
pub use CMP1C_W as REPC_W;
///Field `UPDC` writer - Update Interrupt flag Clear
pub use CMP1C_W as UPDC_W;
///Field `CPT1C` writer - Capture1 Interrupt flag Clear
pub use CMP1C_W as CPT1C_W;
///Field `CPT2C` writer - Capture2 Interrupt flag Clear
pub use CMP1C_W as CPT2C_W;
///Field `SET1xC` writer - Output 1 Set flag Clear
pub use CMP1C_W as SET1X_C_W;
///Field `RSTx1C` writer - Output 1 Reset flag Clear
pub use CMP1C_W as RSTX1C_W;
///Field `SET2xC` writer - Output 2 Set flag Clear
pub use CMP1C_W as SET2X_C_W;
///Field `RSTx2C` writer - Output 2 Reset flag Clear
pub use CMP1C_W as RSTX2C_W;
///Field `RSTC` writer - Reset Interrupt flag Clear
pub use CMP1C_W as RSTC_W;
///Field `DLYPRTC` writer - Delayed Protection Flag Clear
pub use CMP1C_W as DLYPRTC_W;
impl core::fmt::Debug for crate::generic::Reg<TIMEICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Compare 1 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp1c(&mut self) -> CMP1C_W<TIMEICRrs> {
        CMP1C_W::new(self, 0)
    }
    ///Bit 1 - Compare 2 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp2c(&mut self) -> CMP2C_W<TIMEICRrs> {
        CMP2C_W::new(self, 1)
    }
    ///Bit 2 - Compare 3 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp3c(&mut self) -> CMP3C_W<TIMEICRrs> {
        CMP3C_W::new(self, 2)
    }
    ///Bit 3 - Compare 4 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp4c(&mut self) -> CMP4C_W<TIMEICRrs> {
        CMP4C_W::new(self, 3)
    }
    ///Bit 4 - Repetition Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn repc(&mut self) -> REPC_W<TIMEICRrs> {
        REPC_W::new(self, 4)
    }
    ///Bit 6 - Update Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn updc(&mut self) -> UPDC_W<TIMEICRrs> {
        UPDC_W::new(self, 6)
    }
    ///Bit 7 - Capture1 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cpt1c(&mut self) -> CPT1C_W<TIMEICRrs> {
        CPT1C_W::new(self, 7)
    }
    ///Bit 8 - Capture2 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cpt2c(&mut self) -> CPT2C_W<TIMEICRrs> {
        CPT2C_W::new(self, 8)
    }
    ///Bit 9 - Output 1 Set flag Clear
    #[inline(always)]
    #[must_use]
    pub fn set1x_c(&mut self) -> SET1X_C_W<TIMEICRrs> {
        SET1X_C_W::new(self, 9)
    }
    ///Bit 10 - Output 1 Reset flag Clear
    #[inline(always)]
    #[must_use]
    pub fn rstx1c(&mut self) -> RSTX1C_W<TIMEICRrs> {
        RSTX1C_W::new(self, 10)
    }
    ///Bit 11 - Output 2 Set flag Clear
    #[inline(always)]
    #[must_use]
    pub fn set2x_c(&mut self) -> SET2X_C_W<TIMEICRrs> {
        SET2X_C_W::new(self, 11)
    }
    ///Bit 12 - Output 2 Reset flag Clear
    #[inline(always)]
    #[must_use]
    pub fn rstx2c(&mut self) -> RSTX2C_W<TIMEICRrs> {
        RSTX2C_W::new(self, 12)
    }
    ///Bit 13 - Reset Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RSTC_W<TIMEICRrs> {
        RSTC_W::new(self, 13)
    }
    ///Bit 14 - Delayed Protection Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W<TIMEICRrs> {
        DLYPRTC_W::new(self, 14)
    }
}
/**Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIME:TIMEICR)*/
pub struct TIMEICRrs;
impl crate::RegisterSpec for TIMEICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`timeicr::W`](W) writer structure
impl crate::Writable for TIMEICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMEICR to value 0
impl crate::Resettable for TIMEICRrs {
    const RESET_VALUE: u32 = 0;
}
