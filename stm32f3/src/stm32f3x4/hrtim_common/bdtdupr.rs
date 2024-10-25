///Register `BDTDUPR` reader
pub type R = crate::R<BDTDUPRrs>;
///Register `BDTDUPR` writer
pub type W = crate::W<BDTDUPRrs>;
/**HRTIM_TIMxCR register update enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMX_CR {
    ///0: Register not updated by burst DMA access
    NotUpdated = 0,
    ///1: Register updated by burst DMA access
    Updated = 1,
}
impl From<TIMX_CR> for bool {
    #[inline(always)]
    fn from(variant: TIMX_CR) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMxCR` reader - HRTIM_TIMxCR register update enable
pub type TIMX_CR_R = crate::BitReader<TIMX_CR>;
impl TIMX_CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMX_CR {
        match self.bits {
            false => TIMX_CR::NotUpdated,
            true => TIMX_CR::Updated,
        }
    }
    ///Register not updated by burst DMA access
    #[inline(always)]
    pub fn is_not_updated(&self) -> bool {
        *self == TIMX_CR::NotUpdated
    }
    ///Register updated by burst DMA access
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == TIMX_CR::Updated
    }
}
///Field `TIMxCR` writer - HRTIM_TIMxCR register update enable
pub type TIMX_CR_W<'a, REG> = crate::BitWriter<'a, REG, TIMX_CR>;
impl<'a, REG> TIMX_CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Register not updated by burst DMA access
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut crate::W<REG> {
        self.variant(TIMX_CR::NotUpdated)
    }
    ///Register updated by burst DMA access
    #[inline(always)]
    pub fn updated(self) -> &'a mut crate::W<REG> {
        self.variant(TIMX_CR::Updated)
    }
}
///Field `TIMxICR` reader - HRTIM_TIMxICR register update enable
pub use TIMX_CR_R as TIMX_ICR_R;
///Field `TIMxDIER` reader - HRTIM_TIMxDIER register update enable
pub use TIMX_CR_R as TIMX_DIER_R;
///Field `TIMxCNT` reader - HRTIM_CNTxR register update enable
pub use TIMX_CR_R as TIMX_CNT_R;
///Field `TIMxPER` reader - HRTIM_PERxR register update enable
pub use TIMX_CR_R as TIMX_PER_R;
///Field `TIMxREP` reader - HRTIM_REPxR register update enable
pub use TIMX_CR_R as TIMX_REP_R;
///Field `TIMxCMP1` reader - HRTIM_CMP1xR register update enable
pub use TIMX_CR_R as TIMX_CMP1_R;
///Field `TIMxCMP2` reader - HRTIM_CMP2xR register update enable
pub use TIMX_CR_R as TIMX_CMP2_R;
///Field `TIMxCMP3` reader - HRTIM_CMP3xR register update enable
pub use TIMX_CR_R as TIMX_CMP3_R;
///Field `TIMxCMP4` reader - HRTIM_CMP4xR register update enable
pub use TIMX_CR_R as TIMX_CMP4_R;
///Field `TIMx_DTxR` reader - HRTIM_DTxR register update enable
pub use TIMX_CR_R as TIMX_DTX_R_R;
///Field `TIMxSET1R` reader - HRTIM_SET1xR register update enable
pub use TIMX_CR_R as TIMX_SET1R_R;
///Field `TIMxRST1R` reader - HRTIM_RST1xR register update enable
pub use TIMX_CR_R as TIMX_RST1R_R;
///Field `TIMxSET2R` reader - HRTIM_SET2xR register update enable
pub use TIMX_CR_R as TIMX_SET2R_R;
///Field `TIMxRST2R` reader - HRTIM_RST2xR register update enable
pub use TIMX_CR_R as TIMX_RST2R_R;
///Field `TIMxEEFR1` reader - HRTIM_EEFxR1 register update enable
pub use TIMX_CR_R as TIMX_EEFR1_R;
///Field `TIMxEEFR2` reader - HRTIM_EEFxR2 register update enable
pub use TIMX_CR_R as TIMX_EEFR2_R;
///Field `TIMxRSTR` reader - HRTIM_RSTxR register update enable
pub use TIMX_CR_R as TIMX_RSTR_R;
///Field `TIMxCHPR` reader - HRTIM_CHPxR register update enable
pub use TIMX_CR_R as TIMX_CHPR_R;
///Field `TIMxOUTR` reader - HRTIM_OUTxR register update enable
pub use TIMX_CR_R as TIMX_OUTR_R;
///Field `TIMxFLTR` reader - HRTIM_FLTxR register update enable
pub use TIMX_CR_R as TIMX_FLTR_R;
///Field `TIMxICR` writer - HRTIM_TIMxICR register update enable
pub use TIMX_CR_W as TIMX_ICR_W;
///Field `TIMxDIER` writer - HRTIM_TIMxDIER register update enable
pub use TIMX_CR_W as TIMX_DIER_W;
///Field `TIMxCNT` writer - HRTIM_CNTxR register update enable
pub use TIMX_CR_W as TIMX_CNT_W;
///Field `TIMxPER` writer - HRTIM_PERxR register update enable
pub use TIMX_CR_W as TIMX_PER_W;
///Field `TIMxREP` writer - HRTIM_REPxR register update enable
pub use TIMX_CR_W as TIMX_REP_W;
///Field `TIMxCMP1` writer - HRTIM_CMP1xR register update enable
pub use TIMX_CR_W as TIMX_CMP1_W;
///Field `TIMxCMP2` writer - HRTIM_CMP2xR register update enable
pub use TIMX_CR_W as TIMX_CMP2_W;
///Field `TIMxCMP3` writer - HRTIM_CMP3xR register update enable
pub use TIMX_CR_W as TIMX_CMP3_W;
///Field `TIMxCMP4` writer - HRTIM_CMP4xR register update enable
pub use TIMX_CR_W as TIMX_CMP4_W;
///Field `TIMx_DTxR` writer - HRTIM_DTxR register update enable
pub use TIMX_CR_W as TIMX_DTX_R_W;
///Field `TIMxSET1R` writer - HRTIM_SET1xR register update enable
pub use TIMX_CR_W as TIMX_SET1R_W;
///Field `TIMxRST1R` writer - HRTIM_RST1xR register update enable
pub use TIMX_CR_W as TIMX_RST1R_W;
///Field `TIMxSET2R` writer - HRTIM_SET2xR register update enable
pub use TIMX_CR_W as TIMX_SET2R_W;
///Field `TIMxRST2R` writer - HRTIM_RST2xR register update enable
pub use TIMX_CR_W as TIMX_RST2R_W;
///Field `TIMxEEFR1` writer - HRTIM_EEFxR1 register update enable
pub use TIMX_CR_W as TIMX_EEFR1_W;
///Field `TIMxEEFR2` writer - HRTIM_EEFxR2 register update enable
pub use TIMX_CR_W as TIMX_EEFR2_W;
///Field `TIMxRSTR` writer - HRTIM_RSTxR register update enable
pub use TIMX_CR_W as TIMX_RSTR_W;
///Field `TIMxCHPR` writer - HRTIM_CHPxR register update enable
pub use TIMX_CR_W as TIMX_CHPR_W;
///Field `TIMxOUTR` writer - HRTIM_OUTxR register update enable
pub use TIMX_CR_W as TIMX_OUTR_W;
///Field `TIMxFLTR` writer - HRTIM_FLTxR register update enable
pub use TIMX_CR_W as TIMX_FLTR_W;
impl R {
    ///Bit 0 - HRTIM_TIMxCR register update enable
    #[inline(always)]
    pub fn timx_cr(&self) -> TIMX_CR_R {
        TIMX_CR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HRTIM_TIMxICR register update enable
    #[inline(always)]
    pub fn timx_icr(&self) -> TIMX_ICR_R {
        TIMX_ICR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HRTIM_TIMxDIER register update enable
    #[inline(always)]
    pub fn timx_dier(&self) -> TIMX_DIER_R {
        TIMX_DIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HRTIM_CNTxR register update enable
    #[inline(always)]
    pub fn timx_cnt(&self) -> TIMX_CNT_R {
        TIMX_CNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRTIM_PERxR register update enable
    #[inline(always)]
    pub fn timx_per(&self) -> TIMX_PER_R {
        TIMX_PER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRTIM_REPxR register update enable
    #[inline(always)]
    pub fn timx_rep(&self) -> TIMX_REP_R {
        TIMX_REP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HRTIM_CMP1xR register update enable
    #[inline(always)]
    pub fn timx_cmp1(&self) -> TIMX_CMP1_R {
        TIMX_CMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - HRTIM_CMP2xR register update enable
    #[inline(always)]
    pub fn timx_cmp2(&self) -> TIMX_CMP2_R {
        TIMX_CMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HRTIM_CMP3xR register update enable
    #[inline(always)]
    pub fn timx_cmp3(&self) -> TIMX_CMP3_R {
        TIMX_CMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HRTIM_CMP4xR register update enable
    #[inline(always)]
    pub fn timx_cmp4(&self) -> TIMX_CMP4_R {
        TIMX_CMP4_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HRTIM_DTxR register update enable
    #[inline(always)]
    pub fn timx_dtx_r(&self) -> TIMX_DTX_R_R {
        TIMX_DTX_R_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HRTIM_SET1xR register update enable
    #[inline(always)]
    pub fn timx_set1r(&self) -> TIMX_SET1R_R {
        TIMX_SET1R_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HRTIM_RST1xR register update enable
    #[inline(always)]
    pub fn timx_rst1r(&self) -> TIMX_RST1R_R {
        TIMX_RST1R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HRTIM_SET2xR register update enable
    #[inline(always)]
    pub fn timx_set2r(&self) -> TIMX_SET2R_R {
        TIMX_SET2R_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HRTIM_RST2xR register update enable
    #[inline(always)]
    pub fn timx_rst2r(&self) -> TIMX_RST2R_R {
        TIMX_RST2R_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - HRTIM_EEFxR1 register update enable
    #[inline(always)]
    pub fn timx_eefr1(&self) -> TIMX_EEFR1_R {
        TIMX_EEFR1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - HRTIM_EEFxR2 register update enable
    #[inline(always)]
    pub fn timx_eefr2(&self) -> TIMX_EEFR2_R {
        TIMX_EEFR2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HRTIM_RSTxR register update enable
    #[inline(always)]
    pub fn timx_rstr(&self) -> TIMX_RSTR_R {
        TIMX_RSTR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HRTIM_CHPxR register update enable
    #[inline(always)]
    pub fn timx_chpr(&self) -> TIMX_CHPR_R {
        TIMX_CHPR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HRTIM_OUTxR register update enable
    #[inline(always)]
    pub fn timx_outr(&self) -> TIMX_OUTR_R {
        TIMX_OUTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HRTIM_FLTxR register update enable
    #[inline(always)]
    pub fn timx_fltr(&self) -> TIMX_FLTR_R {
        TIMX_FLTR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTDUPR")
            .field("timx_cr", &self.timx_cr())
            .field("timx_fltr", &self.timx_fltr())
            .field("timx_outr", &self.timx_outr())
            .field("timx_chpr", &self.timx_chpr())
            .field("timx_rstr", &self.timx_rstr())
            .field("timx_eefr2", &self.timx_eefr2())
            .field("timx_eefr1", &self.timx_eefr1())
            .field("timx_rst2r", &self.timx_rst2r())
            .field("timx_set2r", &self.timx_set2r())
            .field("timx_rst1r", &self.timx_rst1r())
            .field("timx_set1r", &self.timx_set1r())
            .field("timx_dtx_r", &self.timx_dtx_r())
            .field("timx_cmp4", &self.timx_cmp4())
            .field("timx_cmp3", &self.timx_cmp3())
            .field("timx_cmp2", &self.timx_cmp2())
            .field("timx_cmp1", &self.timx_cmp1())
            .field("timx_rep", &self.timx_rep())
            .field("timx_per", &self.timx_per())
            .field("timx_cnt", &self.timx_cnt())
            .field("timx_dier", &self.timx_dier())
            .field("timx_icr", &self.timx_icr())
            .finish()
    }
}
impl W {
    ///Bit 0 - HRTIM_TIMxCR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cr(&mut self) -> TIMX_CR_W<BDTDUPRrs> {
        TIMX_CR_W::new(self, 0)
    }
    ///Bit 1 - HRTIM_TIMxICR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_icr(&mut self) -> TIMX_ICR_W<BDTDUPRrs> {
        TIMX_ICR_W::new(self, 1)
    }
    ///Bit 2 - HRTIM_TIMxDIER register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_dier(&mut self) -> TIMX_DIER_W<BDTDUPRrs> {
        TIMX_DIER_W::new(self, 2)
    }
    ///Bit 3 - HRTIM_CNTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cnt(&mut self) -> TIMX_CNT_W<BDTDUPRrs> {
        TIMX_CNT_W::new(self, 3)
    }
    ///Bit 4 - HRTIM_PERxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_per(&mut self) -> TIMX_PER_W<BDTDUPRrs> {
        TIMX_PER_W::new(self, 4)
    }
    ///Bit 5 - HRTIM_REPxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rep(&mut self) -> TIMX_REP_W<BDTDUPRrs> {
        TIMX_REP_W::new(self, 5)
    }
    ///Bit 6 - HRTIM_CMP1xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp1(&mut self) -> TIMX_CMP1_W<BDTDUPRrs> {
        TIMX_CMP1_W::new(self, 6)
    }
    ///Bit 7 - HRTIM_CMP2xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp2(&mut self) -> TIMX_CMP2_W<BDTDUPRrs> {
        TIMX_CMP2_W::new(self, 7)
    }
    ///Bit 8 - HRTIM_CMP3xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp3(&mut self) -> TIMX_CMP3_W<BDTDUPRrs> {
        TIMX_CMP3_W::new(self, 8)
    }
    ///Bit 9 - HRTIM_CMP4xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp4(&mut self) -> TIMX_CMP4_W<BDTDUPRrs> {
        TIMX_CMP4_W::new(self, 9)
    }
    ///Bit 10 - HRTIM_DTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_dtx_r(&mut self) -> TIMX_DTX_R_W<BDTDUPRrs> {
        TIMX_DTX_R_W::new(self, 10)
    }
    ///Bit 11 - HRTIM_SET1xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_set1r(&mut self) -> TIMX_SET1R_W<BDTDUPRrs> {
        TIMX_SET1R_W::new(self, 11)
    }
    ///Bit 12 - HRTIM_RST1xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rst1r(&mut self) -> TIMX_RST1R_W<BDTDUPRrs> {
        TIMX_RST1R_W::new(self, 12)
    }
    ///Bit 13 - HRTIM_SET2xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_set2r(&mut self) -> TIMX_SET2R_W<BDTDUPRrs> {
        TIMX_SET2R_W::new(self, 13)
    }
    ///Bit 14 - HRTIM_RST2xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rst2r(&mut self) -> TIMX_RST2R_W<BDTDUPRrs> {
        TIMX_RST2R_W::new(self, 14)
    }
    ///Bit 15 - HRTIM_EEFxR1 register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_eefr1(&mut self) -> TIMX_EEFR1_W<BDTDUPRrs> {
        TIMX_EEFR1_W::new(self, 15)
    }
    ///Bit 16 - HRTIM_EEFxR2 register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_eefr2(&mut self) -> TIMX_EEFR2_W<BDTDUPRrs> {
        TIMX_EEFR2_W::new(self, 16)
    }
    ///Bit 17 - HRTIM_RSTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rstr(&mut self) -> TIMX_RSTR_W<BDTDUPRrs> {
        TIMX_RSTR_W::new(self, 17)
    }
    ///Bit 18 - HRTIM_CHPxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_chpr(&mut self) -> TIMX_CHPR_W<BDTDUPRrs> {
        TIMX_CHPR_W::new(self, 18)
    }
    ///Bit 19 - HRTIM_OUTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_outr(&mut self) -> TIMX_OUTR_W<BDTDUPRrs> {
        TIMX_OUTR_W::new(self, 19)
    }
    ///Bit 20 - HRTIM_FLTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_fltr(&mut self) -> TIMX_FLTR_W<BDTDUPRrs> {
        TIMX_FLTR_W::new(self, 20)
    }
}
/**Burst DMA Timerx update Register

You can [`read`](crate::Reg::read) this register and get [`bdtdupr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtdupr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Common:BDTDUPR)*/
pub struct BDTDUPRrs;
impl crate::RegisterSpec for BDTDUPRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtdupr::R`](R) reader structure
impl crate::Readable for BDTDUPRrs {}
///`write(|w| ..)` method takes [`bdtdupr::W`](W) writer structure
impl crate::Writable for BDTDUPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDTDUPR to value 0
impl crate::Resettable for BDTDUPRrs {
    const RESET_VALUE: u32 = 0;
}
