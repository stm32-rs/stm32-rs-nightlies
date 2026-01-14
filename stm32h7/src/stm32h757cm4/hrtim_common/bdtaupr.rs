///Register `BDTAUPR` reader
pub type R = crate::R<BDTAUPRrs>;
///Register `BDTAUPR` writer
pub type W = crate::W<BDTAUPRrs>;
/**HRTIM_TIMxCR register update enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CR {
    ///0: Register not updated by burst DMA access
    NotUpdated = 0,
    ///1: Register updated by burst DMA access
    Updated = 1,
}
impl From<CR> for bool {
    #[inline(always)]
    fn from(variant: CR) -> Self {
        variant as u8 != 0
    }
}
///Field `CR` reader - HRTIM_TIMxCR register update enable
pub type CR_R = crate::BitReader<CR>;
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CR {
        match self.bits {
            false => CR::NotUpdated,
            true => CR::Updated,
        }
    }
    ///Register not updated by burst DMA access
    #[inline(always)]
    pub fn is_not_updated(&self) -> bool {
        *self == CR::NotUpdated
    }
    ///Register updated by burst DMA access
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == CR::Updated
    }
}
///Field `CR` writer - HRTIM_TIMxCR register update enable
pub type CR_W<'a, REG> = crate::BitWriter<'a, REG, CR>;
impl<'a, REG> CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Register not updated by burst DMA access
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut crate::W<REG> {
        self.variant(CR::NotUpdated)
    }
    ///Register updated by burst DMA access
    #[inline(always)]
    pub fn updated(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Updated)
    }
}
///Field `ICR` reader - HRTIM_TIMxICR register update enable
pub use CR_R as ICR_R;
///Field `DIER` reader - HRTIM_TIMxDIER register update enable
pub use CR_R as DIER_R;
///Field `CNT` reader - HRTIM_CNTxR register update enable
pub use CR_R as CNT_R;
///Field `PER` reader - HRTIM_PERxR register update enable
pub use CR_R as PER_R;
///Field `REP` reader - HRTIM_REPxR register update enable
pub use CR_R as REP_R;
///Field `CMP1` reader - HRTIM_CMP1xR register update enable
pub use CR_R as CMP1_R;
///Field `CMP2` reader - HRTIM_CMP2xR register update enable
pub use CR_R as CMP2_R;
///Field `CMP3` reader - HRTIM_CMP3xR register update enable
pub use CR_R as CMP3_R;
///Field `CMP4` reader - HRTIM_CMP4xR register update enable
pub use CR_R as CMP4_R;
///Field `DTR` reader - HRTIM_DTxR register update enable
pub use CR_R as DTR_R;
///Field `SET1R` reader - HRTIM_SET1xR register update enable
pub use CR_R as SET1R_R;
///Field `RST1R` reader - HRTIM_RST1xR register update enable
pub use CR_R as RST1R_R;
///Field `SET2R` reader - HRTIM_SET2xR register update enable
pub use CR_R as SET2R_R;
///Field `RST2R` reader - HRTIM_RST2xR register update enable
pub use CR_R as RST2R_R;
///Field `EEFR1` reader - HRTIM_EEFxR1 register update enable
pub use CR_R as EEFR1_R;
///Field `EEFR2` reader - HRTIM_EEFxR2 register update enable
pub use CR_R as EEFR2_R;
///Field `RSTR` reader - HRTIM_RSTxR register update enable
pub use CR_R as RSTR_R;
///Field `CHPR` reader - HRTIM_CHPxR register update enable
pub use CR_R as CHPR_R;
///Field `OUTR` reader - HRTIM_OUTxR register update enable
pub use CR_R as OUTR_R;
///Field `FLTR` reader - HRTIM_FLTxR register update enable
pub use CR_R as FLTR_R;
///Field `ICR` writer - HRTIM_TIMxICR register update enable
pub use CR_W as ICR_W;
///Field `DIER` writer - HRTIM_TIMxDIER register update enable
pub use CR_W as DIER_W;
///Field `CNT` writer - HRTIM_CNTxR register update enable
pub use CR_W as CNT_W;
///Field `PER` writer - HRTIM_PERxR register update enable
pub use CR_W as PER_W;
///Field `REP` writer - HRTIM_REPxR register update enable
pub use CR_W as REP_W;
///Field `CMP1` writer - HRTIM_CMP1xR register update enable
pub use CR_W as CMP1_W;
///Field `CMP2` writer - HRTIM_CMP2xR register update enable
pub use CR_W as CMP2_W;
///Field `CMP3` writer - HRTIM_CMP3xR register update enable
pub use CR_W as CMP3_W;
///Field `CMP4` writer - HRTIM_CMP4xR register update enable
pub use CR_W as CMP4_W;
///Field `DTR` writer - HRTIM_DTxR register update enable
pub use CR_W as DTR_W;
///Field `SET1R` writer - HRTIM_SET1xR register update enable
pub use CR_W as SET1R_W;
///Field `RST1R` writer - HRTIM_RST1xR register update enable
pub use CR_W as RST1R_W;
///Field `SET2R` writer - HRTIM_SET2xR register update enable
pub use CR_W as SET2R_W;
///Field `RST2R` writer - HRTIM_RST2xR register update enable
pub use CR_W as RST2R_W;
///Field `EEFR1` writer - HRTIM_EEFxR1 register update enable
pub use CR_W as EEFR1_W;
///Field `EEFR2` writer - HRTIM_EEFxR2 register update enable
pub use CR_W as EEFR2_W;
///Field `RSTR` writer - HRTIM_RSTxR register update enable
pub use CR_W as RSTR_W;
///Field `CHPR` writer - HRTIM_CHPxR register update enable
pub use CR_W as CHPR_W;
///Field `OUTR` writer - HRTIM_OUTxR register update enable
pub use CR_W as OUTR_W;
///Field `FLTR` writer - HRTIM_FLTxR register update enable
pub use CR_W as FLTR_W;
impl R {
    ///Bit 0 - HRTIM_TIMxCR register update enable
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HRTIM_TIMxICR register update enable
    #[inline(always)]
    pub fn icr(&self) -> ICR_R {
        ICR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HRTIM_TIMxDIER register update enable
    #[inline(always)]
    pub fn dier(&self) -> DIER_R {
        DIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HRTIM_CNTxR register update enable
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRTIM_PERxR register update enable
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRTIM_REPxR register update enable
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HRTIM_CMP1xR register update enable
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - HRTIM_CMP2xR register update enable
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HRTIM_CMP3xR register update enable
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HRTIM_CMP4xR register update enable
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HRTIM_DTxR register update enable
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HRTIM_SET1xR register update enable
    #[inline(always)]
    pub fn set1r(&self) -> SET1R_R {
        SET1R_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HRTIM_RST1xR register update enable
    #[inline(always)]
    pub fn rst1r(&self) -> RST1R_R {
        RST1R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HRTIM_SET2xR register update enable
    #[inline(always)]
    pub fn set2r(&self) -> SET2R_R {
        SET2R_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HRTIM_RST2xR register update enable
    #[inline(always)]
    pub fn rst2r(&self) -> RST2R_R {
        RST2R_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - HRTIM_EEFxR1 register update enable
    #[inline(always)]
    pub fn eefr1(&self) -> EEFR1_R {
        EEFR1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - HRTIM_EEFxR2 register update enable
    #[inline(always)]
    pub fn eefr2(&self) -> EEFR2_R {
        EEFR2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HRTIM_RSTxR register update enable
    #[inline(always)]
    pub fn rstr(&self) -> RSTR_R {
        RSTR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HRTIM_CHPxR register update enable
    #[inline(always)]
    pub fn chpr(&self) -> CHPR_R {
        CHPR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HRTIM_OUTxR register update enable
    #[inline(always)]
    pub fn outr(&self) -> OUTR_R {
        OUTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HRTIM_FLTxR register update enable
    #[inline(always)]
    pub fn fltr(&self) -> FLTR_R {
        FLTR_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDTAUPR")
            .field("cr", &self.cr())
            .field("fltr", &self.fltr())
            .field("outr", &self.outr())
            .field("chpr", &self.chpr())
            .field("rstr", &self.rstr())
            .field("eefr2", &self.eefr2())
            .field("eefr1", &self.eefr1())
            .field("rst2r", &self.rst2r())
            .field("set2r", &self.set2r())
            .field("rst1r", &self.rst1r())
            .field("set1r", &self.set1r())
            .field("dtr", &self.dtr())
            .field("cmp4", &self.cmp4())
            .field("cmp3", &self.cmp3())
            .field("cmp2", &self.cmp2())
            .field("cmp1", &self.cmp1())
            .field("rep", &self.rep())
            .field("per", &self.per())
            .field("cnt", &self.cnt())
            .field("dier", &self.dier())
            .field("icr", &self.icr())
            .finish()
    }
}
impl W {
    ///Bit 0 - HRTIM_TIMxCR register update enable
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, BDTAUPRrs> {
        CR_W::new(self, 0)
    }
    ///Bit 1 - HRTIM_TIMxICR register update enable
    #[inline(always)]
    pub fn icr(&mut self) -> ICR_W<'_, BDTAUPRrs> {
        ICR_W::new(self, 1)
    }
    ///Bit 2 - HRTIM_TIMxDIER register update enable
    #[inline(always)]
    pub fn dier(&mut self) -> DIER_W<'_, BDTAUPRrs> {
        DIER_W::new(self, 2)
    }
    ///Bit 3 - HRTIM_CNTxR register update enable
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, BDTAUPRrs> {
        CNT_W::new(self, 3)
    }
    ///Bit 4 - HRTIM_PERxR register update enable
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, BDTAUPRrs> {
        PER_W::new(self, 4)
    }
    ///Bit 5 - HRTIM_REPxR register update enable
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<'_, BDTAUPRrs> {
        REP_W::new(self, 5)
    }
    ///Bit 6 - HRTIM_CMP1xR register update enable
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W<'_, BDTAUPRrs> {
        CMP1_W::new(self, 6)
    }
    ///Bit 7 - HRTIM_CMP2xR register update enable
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W<'_, BDTAUPRrs> {
        CMP2_W::new(self, 7)
    }
    ///Bit 8 - HRTIM_CMP3xR register update enable
    #[inline(always)]
    pub fn cmp3(&mut self) -> CMP3_W<'_, BDTAUPRrs> {
        CMP3_W::new(self, 8)
    }
    ///Bit 9 - HRTIM_CMP4xR register update enable
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W<'_, BDTAUPRrs> {
        CMP4_W::new(self, 9)
    }
    ///Bit 10 - HRTIM_DTxR register update enable
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W<'_, BDTAUPRrs> {
        DTR_W::new(self, 10)
    }
    ///Bit 11 - HRTIM_SET1xR register update enable
    #[inline(always)]
    pub fn set1r(&mut self) -> SET1R_W<'_, BDTAUPRrs> {
        SET1R_W::new(self, 11)
    }
    ///Bit 12 - HRTIM_RST1xR register update enable
    #[inline(always)]
    pub fn rst1r(&mut self) -> RST1R_W<'_, BDTAUPRrs> {
        RST1R_W::new(self, 12)
    }
    ///Bit 13 - HRTIM_SET2xR register update enable
    #[inline(always)]
    pub fn set2r(&mut self) -> SET2R_W<'_, BDTAUPRrs> {
        SET2R_W::new(self, 13)
    }
    ///Bit 14 - HRTIM_RST2xR register update enable
    #[inline(always)]
    pub fn rst2r(&mut self) -> RST2R_W<'_, BDTAUPRrs> {
        RST2R_W::new(self, 14)
    }
    ///Bit 15 - HRTIM_EEFxR1 register update enable
    #[inline(always)]
    pub fn eefr1(&mut self) -> EEFR1_W<'_, BDTAUPRrs> {
        EEFR1_W::new(self, 15)
    }
    ///Bit 16 - HRTIM_EEFxR2 register update enable
    #[inline(always)]
    pub fn eefr2(&mut self) -> EEFR2_W<'_, BDTAUPRrs> {
        EEFR2_W::new(self, 16)
    }
    ///Bit 17 - HRTIM_RSTxR register update enable
    #[inline(always)]
    pub fn rstr(&mut self) -> RSTR_W<'_, BDTAUPRrs> {
        RSTR_W::new(self, 17)
    }
    ///Bit 18 - HRTIM_CHPxR register update enable
    #[inline(always)]
    pub fn chpr(&mut self) -> CHPR_W<'_, BDTAUPRrs> {
        CHPR_W::new(self, 18)
    }
    ///Bit 19 - HRTIM_OUTxR register update enable
    #[inline(always)]
    pub fn outr(&mut self) -> OUTR_W<'_, BDTAUPRrs> {
        OUTR_W::new(self, 19)
    }
    ///Bit 20 - HRTIM_FLTxR register update enable
    #[inline(always)]
    pub fn fltr(&mut self) -> FLTR_W<'_, BDTAUPRrs> {
        FLTR_W::new(self, 20)
    }
}
/**Burst DMA Timerx update Register

You can [`read`](crate::Reg::read) this register and get [`bdtaupr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtaupr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HRTIM_Common:BDTAUPR)*/
pub struct BDTAUPRrs;
impl crate::RegisterSpec for BDTAUPRrs {
    type Ux = u32;
}
///`read()` method returns [`bdtaupr::R`](R) reader structure
impl crate::Readable for BDTAUPRrs {}
///`write(|w| ..)` method takes [`bdtaupr::W`](W) writer structure
impl crate::Writable for BDTAUPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDTAUPR to value 0
impl crate::Resettable for BDTAUPRrs {}
