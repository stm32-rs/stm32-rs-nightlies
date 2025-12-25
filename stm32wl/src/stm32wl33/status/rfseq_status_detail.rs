///Register `RFSEQ_STATUS_DETAIL` reader
pub type R = crate::R<RFSEQ_STATUS_DETAILrs>;
///Register `RFSEQ_STATUS_DETAIL` writer
pub type W = crate::W<RFSEQ_STATUS_DETAILrs>;
///Field `DBM_FIFO_ERROR_F` reader - Data Buffer Manager internal FIFO overflow/underflow flag.
pub type DBM_FIFO_ERROR_F_R = crate::BitReader;
///Field `DBM_FIFO_ERROR_F` writer - Data Buffer Manager internal FIFO overflow/underflow flag.
pub type DBM_FIFO_ERROR_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL_LOCK_FAIL_F` reader - PLL lock fail status flag
pub type PLL_LOCK_FAIL_F_R = crate::BitReader;
///Field `PLL_LOCK_FAIL_F` writer - PLL lock fail status flag
pub type PLL_LOCK_FAIL_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL_UNLOCK_F` reader - PLL unlock event flag
pub type PLL_UNLOCK_F_R = crate::BitReader;
///Field `PLL_UNLOCK_F` writer - PLL unlock event flag
pub type PLL_UNLOCK_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL_CALFREQ_ERROR_F` reader - VCO frequency calibration error flag
pub type PLL_CALFREQ_ERROR_F_R = crate::BitReader;
///Field `PLL_CALFREQ_ERROR_F` writer - VCO frequency calibration error flag
pub type PLL_CALFREQ_ERROR_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL_CALAMP_ERROR_F` reader - VCO amplitude calibration error flag
pub type PLL_CALAMP_ERROR_F_R = crate::BitReader;
///Field `PLL_CALAMP_ERROR_F` writer - VCO amplitude calibration error flag
pub type PLL_CALAMP_ERROR_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_ACTIONTIMEOUT_F` reader - The Sequencer has ended because the current SeqAction reached its ActionTimeout.
pub type SEQ_ACTIONTIMEOUT_F_R = crate::BitReader;
///Field `SEQ_ACTIONTIMEOUT_F` writer - The Sequencer has ended because the current SeqAction reached its ActionTimeout.
pub type SEQ_ACTIONTIMEOUT_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQ_COMPLETE_F` reader - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)
pub type SEQ_COMPLETE_F_R = crate::BitReader;
///Field `SEQ_COMPLETE_F` writer - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)
pub type SEQ_COMPLETE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - Data Buffer Manager internal FIFO overflow/underflow flag.
    #[inline(always)]
    pub fn dbm_fifo_error_f(&self) -> DBM_FIFO_ERROR_F_R {
        DBM_FIFO_ERROR_F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - PLL lock fail status flag
    #[inline(always)]
    pub fn pll_lock_fail_f(&self) -> PLL_LOCK_FAIL_F_R {
        PLL_LOCK_FAIL_F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL unlock event flag
    #[inline(always)]
    pub fn pll_unlock_f(&self) -> PLL_UNLOCK_F_R {
        PLL_UNLOCK_F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - VCO frequency calibration error flag
    #[inline(always)]
    pub fn pll_calfreq_error_f(&self) -> PLL_CALFREQ_ERROR_F_R {
        PLL_CALFREQ_ERROR_F_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - VCO amplitude calibration error flag
    #[inline(always)]
    pub fn pll_calamp_error_f(&self) -> PLL_CALAMP_ERROR_F_R {
        PLL_CALAMP_ERROR_F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - The Sequencer has ended because the current SeqAction reached its ActionTimeout.
    #[inline(always)]
    pub fn seq_actiontimeout_f(&self) -> SEQ_ACTIONTIMEOUT_F_R {
        SEQ_ACTIONTIMEOUT_F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)
    #[inline(always)]
    pub fn seq_complete_f(&self) -> SEQ_COMPLETE_F_R {
        SEQ_COMPLETE_F_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFSEQ_STATUS_DETAIL")
            .field("dbm_fifo_error_f", &self.dbm_fifo_error_f())
            .field("pll_lock_fail_f", &self.pll_lock_fail_f())
            .field("pll_unlock_f", &self.pll_unlock_f())
            .field("pll_calfreq_error_f", &self.pll_calfreq_error_f())
            .field("pll_calamp_error_f", &self.pll_calamp_error_f())
            .field("seq_actiontimeout_f", &self.seq_actiontimeout_f())
            .field("seq_complete_f", &self.seq_complete_f())
            .finish()
    }
}
impl W {
    ///Bit 5 - Data Buffer Manager internal FIFO overflow/underflow flag.
    #[inline(always)]
    pub fn dbm_fifo_error_f(&mut self) -> DBM_FIFO_ERROR_F_W<'_, RFSEQ_STATUS_DETAILrs> {
        DBM_FIFO_ERROR_F_W::new(self, 5)
    }
    ///Bit 8 - PLL lock fail status flag
    #[inline(always)]
    pub fn pll_lock_fail_f(&mut self) -> PLL_LOCK_FAIL_F_W<'_, RFSEQ_STATUS_DETAILrs> {
        PLL_LOCK_FAIL_F_W::new(self, 8)
    }
    ///Bit 9 - PLL unlock event flag
    #[inline(always)]
    pub fn pll_unlock_f(&mut self) -> PLL_UNLOCK_F_W<'_, RFSEQ_STATUS_DETAILrs> {
        PLL_UNLOCK_F_W::new(self, 9)
    }
    ///Bit 10 - VCO frequency calibration error flag
    #[inline(always)]
    pub fn pll_calfreq_error_f(&mut self) -> PLL_CALFREQ_ERROR_F_W<'_, RFSEQ_STATUS_DETAILrs> {
        PLL_CALFREQ_ERROR_F_W::new(self, 10)
    }
    ///Bit 11 - VCO amplitude calibration error flag
    #[inline(always)]
    pub fn pll_calamp_error_f(&mut self) -> PLL_CALAMP_ERROR_F_W<'_, RFSEQ_STATUS_DETAILrs> {
        PLL_CALAMP_ERROR_F_W::new(self, 11)
    }
    ///Bit 14 - The Sequencer has ended because the current SeqAction reached its ActionTimeout.
    #[inline(always)]
    pub fn seq_actiontimeout_f(&mut self) -> SEQ_ACTIONTIMEOUT_F_W<'_, RFSEQ_STATUS_DETAILrs> {
        SEQ_ACTIONTIMEOUT_F_W::new(self, 14)
    }
    ///Bit 15 - The Sequencer has ended the last defined SeqAction properly( NextAction math or null pointer)
    #[inline(always)]
    pub fn seq_complete_f(&mut self) -> SEQ_COMPLETE_F_W<'_, RFSEQ_STATUS_DETAILrs> {
        SEQ_COMPLETE_F_W::new(self, 15)
    }
}
/**RFSEQ_STATUS_DETAIL register

You can [`read`](crate::Reg::read) this register and get [`rfseq_status_detail::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfseq_status_detail::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RFSEQ_STATUS_DETAIL)*/
pub struct RFSEQ_STATUS_DETAILrs;
impl crate::RegisterSpec for RFSEQ_STATUS_DETAILrs {
    type Ux = u32;
}
///`read()` method returns [`rfseq_status_detail::R`](R) reader structure
impl crate::Readable for RFSEQ_STATUS_DETAILrs {}
///`write(|w| ..)` method takes [`rfseq_status_detail::W`](W) writer structure
impl crate::Writable for RFSEQ_STATUS_DETAILrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFSEQ_STATUS_DETAIL to value 0
impl crate::Resettable for RFSEQ_STATUS_DETAILrs {}
