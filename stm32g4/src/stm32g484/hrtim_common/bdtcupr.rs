#[doc = "Reader of register BDTCUPR"]
pub type R = crate::R<u32, super::BDTCUPR>;
#[doc = "Writer for register BDTCUPR"]
pub type W = crate::W<u32, super::BDTCUPR>;
#[doc = "Register BDTCUPR `reset()`'s with value 0"]
impl crate::ResetValue for super::BDTCUPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMxEEFR3`"]
pub type TIMXEEFR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxEEFR3`"]
pub struct TIMXEEFR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXEEFR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TIMxCR2`"]
pub type TIMXCR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCR2`"]
pub struct TIMXCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TIMxFLTR`"]
pub type TIMXFLTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxFLTR`"]
pub struct TIMXFLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXFLTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TIMxOUTR`"]
pub type TIMXOUTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxOUTR`"]
pub struct TIMXOUTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXOUTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TIMxCHPR`"]
pub type TIMXCHPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCHPR`"]
pub struct TIMXCHPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCHPR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TIMxRSTR`"]
pub type TIMXRSTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxRSTR`"]
pub struct TIMXRSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXRSTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TIMxEEFR2`"]
pub type TIMXEEFR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxEEFR2`"]
pub struct TIMXEEFR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXEEFR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TIMxEEFR1`"]
pub type TIMXEEFR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxEEFR1`"]
pub struct TIMXEEFR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXEEFR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TIMxRST2R`"]
pub type TIMXRST2R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxRST2R`"]
pub struct TIMXRST2R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXRST2R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TIMxSET2R`"]
pub type TIMXSET2R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxSET2R`"]
pub struct TIMXSET2R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXSET2R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIMxRST1R`"]
pub type TIMXRST1R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxRST1R`"]
pub struct TIMXRST1R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXRST1R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TIMxSET1R`"]
pub type TIMXSET1R_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxSET1R`"]
pub struct TIMXSET1R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXSET1R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TIMx_DTxR`"]
pub type TIMX_DTXR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMx_DTxR`"]
pub struct TIMX_DTXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMX_DTXR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TIMxCMP4`"]
pub type TIMXCMP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCMP4`"]
pub struct TIMXCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TIMxCMP3`"]
pub type TIMXCMP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCMP3`"]
pub struct TIMXCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMxCMP2`"]
pub type TIMXCMP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCMP2`"]
pub struct TIMXCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TIMxCMP1`"]
pub type TIMXCMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCMP1`"]
pub struct TIMXCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TIMxREP`"]
pub type TIMXREP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxREP`"]
pub struct TIMXREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXREP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TIMxPER`"]
pub type TIMXPER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxPER`"]
pub struct TIMXPER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXPER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TIMxCNT`"]
pub type TIMXCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCNT`"]
pub struct TIMXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TIMxDIER`"]
pub type TIMXDIER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxDIER`"]
pub struct TIMXDIER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXDIER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TIMxICR`"]
pub type TIMXICR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxICR`"]
pub struct TIMXICR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXICR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TIMxCR`"]
pub type TIMXCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMxCR`"]
pub struct TIMXCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 22 - TIMxEEFR3"]
    #[inline(always)]
    pub fn timx_eefr3(&self) -> TIMXEEFR3_R {
        TIMXEEFR3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TIMxCR2"]
    #[inline(always)]
    pub fn timx_cr2(&self) -> TIMXCR2_R {
        TIMXCR2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HRTIM_FLTxR register update enable"]
    #[inline(always)]
    pub fn timx_fltr(&self) -> TIMXFLTR_R {
        TIMXFLTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HRTIM_OUTxR register update enable"]
    #[inline(always)]
    pub fn timx_outr(&self) -> TIMXOUTR_R {
        TIMXOUTR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HRTIM_CHPxR register update enable"]
    #[inline(always)]
    pub fn timx_chpr(&self) -> TIMXCHPR_R {
        TIMXCHPR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HRTIM_RSTxR register update enable"]
    #[inline(always)]
    pub fn timx_rstr(&self) -> TIMXRSTR_R {
        TIMXRSTR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HRTIM_EEFxR2 register update enable"]
    #[inline(always)]
    pub fn timx_eefr2(&self) -> TIMXEEFR2_R {
        TIMXEEFR2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - HRTIM_EEFxR1 register update enable"]
    #[inline(always)]
    pub fn timx_eefr1(&self) -> TIMXEEFR1_R {
        TIMXEEFR1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - HRTIM_RST2xR register update enable"]
    #[inline(always)]
    pub fn timx_rst2r(&self) -> TIMXRST2R_R {
        TIMXRST2R_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HRTIM_SET2xR register update enable"]
    #[inline(always)]
    pub fn timx_set2r(&self) -> TIMXSET2R_R {
        TIMXSET2R_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HRTIM_RST1xR register update enable"]
    #[inline(always)]
    pub fn timx_rst1r(&self) -> TIMXRST1R_R {
        TIMXRST1R_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HRTIM_SET1xR register update enable"]
    #[inline(always)]
    pub fn timx_set1r(&self) -> TIMXSET1R_R {
        TIMXSET1R_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HRTIM_DTxR register update enable"]
    #[inline(always)]
    pub fn timx_dtx_r(&self) -> TIMX_DTXR_R {
        TIMX_DTXR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HRTIM_CMP4xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp4(&self) -> TIMXCMP4_R {
        TIMXCMP4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HRTIM_CMP3xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp3(&self) -> TIMXCMP3_R {
        TIMXCMP3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HRTIM_CMP2xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp2(&self) -> TIMXCMP2_R {
        TIMXCMP2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HRTIM_CMP1xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp1(&self) -> TIMXCMP1_R {
        TIMXCMP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRTIM_REPxR register update enable"]
    #[inline(always)]
    pub fn timx_rep(&self) -> TIMXREP_R {
        TIMXREP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRTIM_PERxR register update enable"]
    #[inline(always)]
    pub fn timx_per(&self) -> TIMXPER_R {
        TIMXPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HRTIM_CNTxR register update enable"]
    #[inline(always)]
    pub fn timx_cnt(&self) -> TIMXCNT_R {
        TIMXCNT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HRTIM_TIMxDIER register update enable"]
    #[inline(always)]
    pub fn timx_dier(&self) -> TIMXDIER_R {
        TIMXDIER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - HRTIM_TIMxICR register update enable"]
    #[inline(always)]
    pub fn timx_icr(&self) -> TIMXICR_R {
        TIMXICR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - HRTIM_TIMxCR register update enable"]
    #[inline(always)]
    pub fn timx_cr(&self) -> TIMXCR_R {
        TIMXCR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - TIMxEEFR3"]
    #[inline(always)]
    pub fn timx_eefr3(&mut self) -> TIMXEEFR3_W {
        TIMXEEFR3_W { w: self }
    }
    #[doc = "Bit 21 - TIMxCR2"]
    #[inline(always)]
    pub fn timx_cr2(&mut self) -> TIMXCR2_W {
        TIMXCR2_W { w: self }
    }
    #[doc = "Bit 20 - HRTIM_FLTxR register update enable"]
    #[inline(always)]
    pub fn timx_fltr(&mut self) -> TIMXFLTR_W {
        TIMXFLTR_W { w: self }
    }
    #[doc = "Bit 19 - HRTIM_OUTxR register update enable"]
    #[inline(always)]
    pub fn timx_outr(&mut self) -> TIMXOUTR_W {
        TIMXOUTR_W { w: self }
    }
    #[doc = "Bit 18 - HRTIM_CHPxR register update enable"]
    #[inline(always)]
    pub fn timx_chpr(&mut self) -> TIMXCHPR_W {
        TIMXCHPR_W { w: self }
    }
    #[doc = "Bit 17 - HRTIM_RSTxR register update enable"]
    #[inline(always)]
    pub fn timx_rstr(&mut self) -> TIMXRSTR_W {
        TIMXRSTR_W { w: self }
    }
    #[doc = "Bit 16 - HRTIM_EEFxR2 register update enable"]
    #[inline(always)]
    pub fn timx_eefr2(&mut self) -> TIMXEEFR2_W {
        TIMXEEFR2_W { w: self }
    }
    #[doc = "Bit 15 - HRTIM_EEFxR1 register update enable"]
    #[inline(always)]
    pub fn timx_eefr1(&mut self) -> TIMXEEFR1_W {
        TIMXEEFR1_W { w: self }
    }
    #[doc = "Bit 14 - HRTIM_RST2xR register update enable"]
    #[inline(always)]
    pub fn timx_rst2r(&mut self) -> TIMXRST2R_W {
        TIMXRST2R_W { w: self }
    }
    #[doc = "Bit 13 - HRTIM_SET2xR register update enable"]
    #[inline(always)]
    pub fn timx_set2r(&mut self) -> TIMXSET2R_W {
        TIMXSET2R_W { w: self }
    }
    #[doc = "Bit 12 - HRTIM_RST1xR register update enable"]
    #[inline(always)]
    pub fn timx_rst1r(&mut self) -> TIMXRST1R_W {
        TIMXRST1R_W { w: self }
    }
    #[doc = "Bit 11 - HRTIM_SET1xR register update enable"]
    #[inline(always)]
    pub fn timx_set1r(&mut self) -> TIMXSET1R_W {
        TIMXSET1R_W { w: self }
    }
    #[doc = "Bit 10 - HRTIM_DTxR register update enable"]
    #[inline(always)]
    pub fn timx_dtx_r(&mut self) -> TIMX_DTXR_W {
        TIMX_DTXR_W { w: self }
    }
    #[doc = "Bit 9 - HRTIM_CMP4xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp4(&mut self) -> TIMXCMP4_W {
        TIMXCMP4_W { w: self }
    }
    #[doc = "Bit 8 - HRTIM_CMP3xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp3(&mut self) -> TIMXCMP3_W {
        TIMXCMP3_W { w: self }
    }
    #[doc = "Bit 7 - HRTIM_CMP2xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp2(&mut self) -> TIMXCMP2_W {
        TIMXCMP2_W { w: self }
    }
    #[doc = "Bit 6 - HRTIM_CMP1xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp1(&mut self) -> TIMXCMP1_W {
        TIMXCMP1_W { w: self }
    }
    #[doc = "Bit 5 - HRTIM_REPxR register update enable"]
    #[inline(always)]
    pub fn timx_rep(&mut self) -> TIMXREP_W {
        TIMXREP_W { w: self }
    }
    #[doc = "Bit 4 - HRTIM_PERxR register update enable"]
    #[inline(always)]
    pub fn timx_per(&mut self) -> TIMXPER_W {
        TIMXPER_W { w: self }
    }
    #[doc = "Bit 3 - HRTIM_CNTxR register update enable"]
    #[inline(always)]
    pub fn timx_cnt(&mut self) -> TIMXCNT_W {
        TIMXCNT_W { w: self }
    }
    #[doc = "Bit 2 - HRTIM_TIMxDIER register update enable"]
    #[inline(always)]
    pub fn timx_dier(&mut self) -> TIMXDIER_W {
        TIMXDIER_W { w: self }
    }
    #[doc = "Bit 1 - HRTIM_TIMxICR register update enable"]
    #[inline(always)]
    pub fn timx_icr(&mut self) -> TIMXICR_W {
        TIMXICR_W { w: self }
    }
    #[doc = "Bit 0 - HRTIM_TIMxCR register update enable"]
    #[inline(always)]
    pub fn timx_cr(&mut self) -> TIMXCR_W {
        TIMXCR_W { w: self }
    }
}
