///Register `BDTxUPR` reader
pub type R = crate::R<BDTX_UPRrs>;
///Register `BDTxUPR` writer
pub type W = crate::W<BDTX_UPRrs>;
///Field `TIMxCR` reader - HRTIM_TIMxCR register update enable
pub type TIMX_CR_R = crate::BitReader;
///Field `TIMxCR` writer - HRTIM_TIMxCR register update enable
pub type TIMX_CR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxICR` reader - HRTIM_TIMxICR register update enable
pub type TIMX_ICR_R = crate::BitReader;
///Field `TIMxICR` writer - HRTIM_TIMxICR register update enable
pub type TIMX_ICR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxDIER` reader - HRTIM_TIMxDIER register update enable
pub type TIMX_DIER_R = crate::BitReader;
///Field `TIMxDIER` writer - HRTIM_TIMxDIER register update enable
pub type TIMX_DIER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxCNT` reader - HRTIM_CNTxR register update enable
pub type TIMX_CNT_R = crate::BitReader;
///Field `TIMxCNT` writer - HRTIM_CNTxR register update enable
pub type TIMX_CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxPER` reader - HRTIM_PERxR register update enable
pub type TIMX_PER_R = crate::BitReader;
///Field `TIMxPER` writer - HRTIM_PERxR register update enable
pub type TIMX_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxREP` reader - HRTIM_REPxR register update enable
pub type TIMX_REP_R = crate::BitReader;
///Field `TIMxREP` writer - HRTIM_REPxR register update enable
pub type TIMX_REP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxCMP1` reader - HRTIM_CMP1xR register update enable
pub type TIMX_CMP1_R = crate::BitReader;
///Field `TIMxCMP1` writer - HRTIM_CMP1xR register update enable
pub type TIMX_CMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxCMP2` reader - HRTIM_CMP2xR register update enable
pub type TIMX_CMP2_R = crate::BitReader;
///Field `TIMxCMP2` writer - HRTIM_CMP2xR register update enable
pub type TIMX_CMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxCMP3` reader - HRTIM_CMP3xR register update enable
pub type TIMX_CMP3_R = crate::BitReader;
///Field `TIMxCMP3` writer - HRTIM_CMP3xR register update enable
pub type TIMX_CMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxCMP4` reader - HRTIM_CMP4xR register update enable
pub type TIMX_CMP4_R = crate::BitReader;
///Field `TIMxCMP4` writer - HRTIM_CMP4xR register update enable
pub type TIMX_CMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMx_DTxR` reader - HRTIM_DTxR register update enable
pub type TIMX_DTX_R_R = crate::BitReader;
///Field `TIMx_DTxR` writer - HRTIM_DTxR register update enable
pub type TIMX_DTX_R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxSET1R` reader - HRTIM_SET1xR register update enable
pub type TIMX_SET1R_R = crate::BitReader;
///Field `TIMxSET1R` writer - HRTIM_SET1xR register update enable
pub type TIMX_SET1R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxRST1R` reader - HRTIM_RST1xR register update enable
pub type TIMX_RST1R_R = crate::BitReader;
///Field `TIMxRST1R` writer - HRTIM_RST1xR register update enable
pub type TIMX_RST1R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxSET2R` reader - HRTIM_SET2xR register update enable
pub type TIMX_SET2R_R = crate::BitReader;
///Field `TIMxSET2R` writer - HRTIM_SET2xR register update enable
pub type TIMX_SET2R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxRST2R` reader - HRTIM_RST2xR register update enable
pub type TIMX_RST2R_R = crate::BitReader;
///Field `TIMxRST2R` writer - HRTIM_RST2xR register update enable
pub type TIMX_RST2R_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxEEFR1` reader - HRTIM_EEFxR1 register update enable
pub type TIMX_EEFR1_R = crate::BitReader;
///Field `TIMxEEFR1` writer - HRTIM_EEFxR1 register update enable
pub type TIMX_EEFR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxEEFR2` reader - HRTIM_EEFxR2 register update enable
pub type TIMX_EEFR2_R = crate::BitReader;
///Field `TIMxEEFR2` writer - HRTIM_EEFxR2 register update enable
pub type TIMX_EEFR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxRSTR` reader - HRTIM_RSTxR register update enable
pub type TIMX_RSTR_R = crate::BitReader;
///Field `TIMxRSTR` writer - HRTIM_RSTxR register update enable
pub type TIMX_RSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxCHPR` reader - HRTIM_CHPxR register update enable
pub type TIMX_CHPR_R = crate::BitReader;
///Field `TIMxCHPR` writer - HRTIM_CHPxR register update enable
pub type TIMX_CHPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxOUTR` reader - HRTIM_OUTxR register update enable
pub type TIMX_OUTR_R = crate::BitReader;
///Field `TIMxOUTR` writer - HRTIM_OUTxR register update enable
pub type TIMX_OUTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMxFLTR` reader - HRTIM_FLTxR register update enable
pub type TIMX_FLTR_R = crate::BitReader;
///Field `TIMxFLTR` writer - HRTIM_FLTxR register update enable
pub type TIMX_FLTR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("BDTxUPR")
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
            .field("timx_cr", &self.timx_cr())
            .finish()
    }
}
impl W {
    ///Bit 0 - HRTIM_TIMxCR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cr(&mut self) -> TIMX_CR_W<BDTX_UPRrs> {
        TIMX_CR_W::new(self, 0)
    }
    ///Bit 1 - HRTIM_TIMxICR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_icr(&mut self) -> TIMX_ICR_W<BDTX_UPRrs> {
        TIMX_ICR_W::new(self, 1)
    }
    ///Bit 2 - HRTIM_TIMxDIER register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_dier(&mut self) -> TIMX_DIER_W<BDTX_UPRrs> {
        TIMX_DIER_W::new(self, 2)
    }
    ///Bit 3 - HRTIM_CNTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cnt(&mut self) -> TIMX_CNT_W<BDTX_UPRrs> {
        TIMX_CNT_W::new(self, 3)
    }
    ///Bit 4 - HRTIM_PERxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_per(&mut self) -> TIMX_PER_W<BDTX_UPRrs> {
        TIMX_PER_W::new(self, 4)
    }
    ///Bit 5 - HRTIM_REPxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rep(&mut self) -> TIMX_REP_W<BDTX_UPRrs> {
        TIMX_REP_W::new(self, 5)
    }
    ///Bit 6 - HRTIM_CMP1xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp1(&mut self) -> TIMX_CMP1_W<BDTX_UPRrs> {
        TIMX_CMP1_W::new(self, 6)
    }
    ///Bit 7 - HRTIM_CMP2xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp2(&mut self) -> TIMX_CMP2_W<BDTX_UPRrs> {
        TIMX_CMP2_W::new(self, 7)
    }
    ///Bit 8 - HRTIM_CMP3xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp3(&mut self) -> TIMX_CMP3_W<BDTX_UPRrs> {
        TIMX_CMP3_W::new(self, 8)
    }
    ///Bit 9 - HRTIM_CMP4xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_cmp4(&mut self) -> TIMX_CMP4_W<BDTX_UPRrs> {
        TIMX_CMP4_W::new(self, 9)
    }
    ///Bit 10 - HRTIM_DTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_dtx_r(&mut self) -> TIMX_DTX_R_W<BDTX_UPRrs> {
        TIMX_DTX_R_W::new(self, 10)
    }
    ///Bit 11 - HRTIM_SET1xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_set1r(&mut self) -> TIMX_SET1R_W<BDTX_UPRrs> {
        TIMX_SET1R_W::new(self, 11)
    }
    ///Bit 12 - HRTIM_RST1xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rst1r(&mut self) -> TIMX_RST1R_W<BDTX_UPRrs> {
        TIMX_RST1R_W::new(self, 12)
    }
    ///Bit 13 - HRTIM_SET2xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_set2r(&mut self) -> TIMX_SET2R_W<BDTX_UPRrs> {
        TIMX_SET2R_W::new(self, 13)
    }
    ///Bit 14 - HRTIM_RST2xR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rst2r(&mut self) -> TIMX_RST2R_W<BDTX_UPRrs> {
        TIMX_RST2R_W::new(self, 14)
    }
    ///Bit 15 - HRTIM_EEFxR1 register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_eefr1(&mut self) -> TIMX_EEFR1_W<BDTX_UPRrs> {
        TIMX_EEFR1_W::new(self, 15)
    }
    ///Bit 16 - HRTIM_EEFxR2 register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_eefr2(&mut self) -> TIMX_EEFR2_W<BDTX_UPRrs> {
        TIMX_EEFR2_W::new(self, 16)
    }
    ///Bit 17 - HRTIM_RSTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_rstr(&mut self) -> TIMX_RSTR_W<BDTX_UPRrs> {
        TIMX_RSTR_W::new(self, 17)
    }
    ///Bit 18 - HRTIM_CHPxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_chpr(&mut self) -> TIMX_CHPR_W<BDTX_UPRrs> {
        TIMX_CHPR_W::new(self, 18)
    }
    ///Bit 19 - HRTIM_OUTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_outr(&mut self) -> TIMX_OUTR_W<BDTX_UPRrs> {
        TIMX_OUTR_W::new(self, 19)
    }
    ///Bit 20 - HRTIM_FLTxR register update enable
    #[inline(always)]
    #[must_use]
    pub fn timx_fltr(&mut self) -> TIMX_FLTR_W<BDTX_UPRrs> {
        TIMX_FLTR_W::new(self, 20)
    }
}
/**Burst DMA Timerx update Register

You can [`read`](crate::Reg::read) this register and get [`bdtx_upr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtx_upr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HRTIM_Common:BDTxUPR)*/
pub struct BDTX_UPRrs;
impl crate::RegisterSpec for BDTX_UPRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtx_upr::R`](R) reader structure
impl crate::Readable for BDTX_UPRrs {}
///`write(|w| ..)` method takes [`bdtx_upr::W`](W) writer structure
impl crate::Writable for BDTX_UPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDTxUPR to value 0
impl crate::Resettable for BDTX_UPRrs {
    const RESET_VALUE: u32 = 0;
}
