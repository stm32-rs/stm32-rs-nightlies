#[doc = "Register `BDTAUPR` reader"]
pub struct R(crate::R<BDTAUPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTAUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTAUPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTAUPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTAUPR` writer"]
pub struct W(crate::W<BDTAUPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTAUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BDTAUPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTAUPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HRTIM_FLTxR register update enable"]
pub type TIMXFLTR_A = TIMXCR_A;
#[doc = "Field `TIMxFLTR` reader - HRTIM_FLTxR register update enable"]
pub type TIMXFLTR_R = TIMXCR_R;
#[doc = "Field `TIMxFLTR` writer - HRTIM_FLTxR register update enable"]
pub struct TIMXFLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXFLTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXFLTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXFLTR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXFLTR_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "HRTIM_OUTxR register update enable"]
pub type TIMXOUTR_A = TIMXCR_A;
#[doc = "Field `TIMxOUTR` reader - HRTIM_OUTxR register update enable"]
pub type TIMXOUTR_R = TIMXCR_R;
#[doc = "Field `TIMxOUTR` writer - HRTIM_OUTxR register update enable"]
pub struct TIMXOUTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXOUTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXOUTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXOUTR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXOUTR_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "HRTIM_CHPxR register update enable"]
pub type TIMXCHPR_A = TIMXCR_A;
#[doc = "Field `TIMxCHPR` reader - HRTIM_CHPxR register update enable"]
pub type TIMXCHPR_R = TIMXCR_R;
#[doc = "Field `TIMxCHPR` writer - HRTIM_CHPxR register update enable"]
pub struct TIMXCHPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCHPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXCHPR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXCHPR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXCHPR_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "HRTIM_RSTxR register update enable"]
pub type TIMXRSTR_A = TIMXCR_A;
#[doc = "Field `TIMxRSTR` reader - HRTIM_RSTxR register update enable"]
pub type TIMXRSTR_R = TIMXCR_R;
#[doc = "Field `TIMxRSTR` writer - HRTIM_RSTxR register update enable"]
pub struct TIMXRSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXRSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXRSTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXRSTR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXRSTR_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "HRTIM_EEFxR2 register update enable"]
pub type TIMXEEFR2_A = TIMXCR_A;
#[doc = "Field `TIMxEEFR2` reader - HRTIM_EEFxR2 register update enable"]
pub type TIMXEEFR2_R = TIMXCR_R;
#[doc = "Field `TIMxEEFR2` writer - HRTIM_EEFxR2 register update enable"]
pub struct TIMXEEFR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXEEFR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXEEFR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXEEFR2_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXEEFR2_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "HRTIM_EEFxR1 register update enable"]
pub type TIMXEEFR1_A = TIMXCR_A;
#[doc = "Field `TIMxEEFR1` reader - HRTIM_EEFxR1 register update enable"]
pub type TIMXEEFR1_R = TIMXCR_R;
#[doc = "Field `TIMxEEFR1` writer - HRTIM_EEFxR1 register update enable"]
pub struct TIMXEEFR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXEEFR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXEEFR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXEEFR1_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXEEFR1_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "HRTIM_RST2xR register update enable"]
pub type TIMXRST2R_A = TIMXCR_A;
#[doc = "Field `TIMxRST2R` reader - HRTIM_RST2xR register update enable"]
pub type TIMXRST2R_R = TIMXCR_R;
#[doc = "Field `TIMxRST2R` writer - HRTIM_RST2xR register update enable"]
pub struct TIMXRST2R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXRST2R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXRST2R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXRST2R_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXRST2R_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "HRTIM_SET2xR register update enable"]
pub type TIMXSET2R_A = TIMXCR_A;
#[doc = "Field `TIMxSET2R` reader - HRTIM_SET2xR register update enable"]
pub type TIMXSET2R_R = TIMXCR_R;
#[doc = "Field `TIMxSET2R` writer - HRTIM_SET2xR register update enable"]
pub struct TIMXSET2R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXSET2R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXSET2R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXSET2R_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXSET2R_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "HRTIM_RST1xR register update enable"]
pub type TIMXRST1R_A = TIMXCR_A;
#[doc = "Field `TIMxRST1R` reader - HRTIM_RST1xR register update enable"]
pub type TIMXRST1R_R = TIMXCR_R;
#[doc = "Field `TIMxRST1R` writer - HRTIM_RST1xR register update enable"]
pub struct TIMXRST1R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXRST1R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXRST1R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXRST1R_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXRST1R_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "HRTIM_SET1xR register update enable"]
pub type TIMXSET1R_A = TIMXCR_A;
#[doc = "Field `TIMxSET1R` reader - HRTIM_SET1xR register update enable"]
pub type TIMXSET1R_R = TIMXCR_R;
#[doc = "Field `TIMxSET1R` writer - HRTIM_SET1xR register update enable"]
pub struct TIMXSET1R_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXSET1R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXSET1R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXSET1R_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXSET1R_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "HRTIM_DTxR register update enable"]
pub type TIMX_DTXR_A = TIMXCR_A;
#[doc = "Field `TIMx_DTxR` reader - HRTIM_DTxR register update enable"]
pub type TIMX_DTXR_R = TIMXCR_R;
#[doc = "Field `TIMx_DTxR` writer - HRTIM_DTxR register update enable"]
pub struct TIMX_DTXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMX_DTXR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMX_DTXR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMX_DTXR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMX_DTXR_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "HRTIM_CMP4xR register update enable"]
pub type TIMXCMP4_A = TIMXCR_A;
#[doc = "Field `TIMxCMP4` reader - HRTIM_CMP4xR register update enable"]
pub type TIMXCMP4_R = TIMXCR_R;
#[doc = "Field `TIMxCMP4` writer - HRTIM_CMP4xR register update enable"]
pub struct TIMXCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXCMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXCMP4_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXCMP4_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "HRTIM_CMP3xR register update enable"]
pub type TIMXCMP3_A = TIMXCR_A;
#[doc = "Field `TIMxCMP3` reader - HRTIM_CMP3xR register update enable"]
pub type TIMXCMP3_R = TIMXCR_R;
#[doc = "Field `TIMxCMP3` writer - HRTIM_CMP3xR register update enable"]
pub struct TIMXCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXCMP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXCMP3_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXCMP3_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "HRTIM_CMP2xR register update enable"]
pub type TIMXCMP2_A = TIMXCR_A;
#[doc = "Field `TIMxCMP2` reader - HRTIM_CMP2xR register update enable"]
pub type TIMXCMP2_R = TIMXCR_R;
#[doc = "Field `TIMxCMP2` writer - HRTIM_CMP2xR register update enable"]
pub struct TIMXCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXCMP2_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXCMP2_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "HRTIM_CMP1xR register update enable"]
pub type TIMXCMP1_A = TIMXCR_A;
#[doc = "Field `TIMxCMP1` reader - HRTIM_CMP1xR register update enable"]
pub type TIMXCMP1_R = TIMXCR_R;
#[doc = "Field `TIMxCMP1` writer - HRTIM_CMP1xR register update enable"]
pub struct TIMXCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXCMP1_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXCMP1_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "HRTIM_REPxR register update enable"]
pub type TIMXREP_A = TIMXCR_A;
#[doc = "Field `TIMxREP` reader - HRTIM_REPxR register update enable"]
pub type TIMXREP_R = TIMXCR_R;
#[doc = "Field `TIMxREP` writer - HRTIM_REPxR register update enable"]
pub struct TIMXREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXREP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXREP_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXREP_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "HRTIM_PERxR register update enable"]
pub type TIMXPER_A = TIMXCR_A;
#[doc = "Field `TIMxPER` reader - HRTIM_PERxR register update enable"]
pub type TIMXPER_R = TIMXCR_R;
#[doc = "Field `TIMxPER` writer - HRTIM_PERxR register update enable"]
pub struct TIMXPER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXPER_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXPER_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "HRTIM_CNTxR register update enable"]
pub type TIMXCNT_A = TIMXCR_A;
#[doc = "Field `TIMxCNT` reader - HRTIM_CNTxR register update enable"]
pub type TIMXCNT_R = TIMXCR_R;
#[doc = "Field `TIMxCNT` writer - HRTIM_CNTxR register update enable"]
pub struct TIMXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXCNT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXCNT_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXCNT_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "HRTIM_TIMxDIER register update enable"]
pub type TIMXDIER_A = TIMXCR_A;
#[doc = "Field `TIMxDIER` reader - HRTIM_TIMxDIER register update enable"]
pub type TIMXDIER_R = TIMXCR_R;
#[doc = "Field `TIMxDIER` writer - HRTIM_TIMxDIER register update enable"]
pub struct TIMXDIER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXDIER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXDIER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXDIER_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXDIER_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "HRTIM_TIMxICR register update enable"]
pub type TIMXICR_A = TIMXCR_A;
#[doc = "Field `TIMxICR` reader - HRTIM_TIMxICR register update enable"]
pub type TIMXICR_R = TIMXCR_R;
#[doc = "Field `TIMxICR` writer - HRTIM_TIMxICR register update enable"]
pub struct TIMXICR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXICR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXICR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXICR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXICR_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "HRTIM_TIMxCR register update enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMXCR_A {
    #[doc = "0: Register not updated by burst DMA access"]
    NOTUPDATED = 0,
    #[doc = "1: Register updated by burst DMA access"]
    UPDATED = 1,
}
impl From<TIMXCR_A> for bool {
    #[inline(always)]
    fn from(variant: TIMXCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMxCR` reader - HRTIM_TIMxCR register update enable"]
pub struct TIMXCR_R(crate::FieldReader<bool, TIMXCR_A>);
impl TIMXCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMXCR_A {
        match self.bits {
            false => TIMXCR_A::NOTUPDATED,
            true => TIMXCR_A::UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTUPDATED`"]
    #[inline(always)]
    pub fn is_not_updated(&self) -> bool {
        **self == TIMXCR_A::NOTUPDATED
    }
    #[doc = "Checks if the value of the field is `UPDATED`"]
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        **self == TIMXCR_A::UPDATED
    }
}
impl core::ops::Deref for TIMXCR_R {
    type Target = crate::FieldReader<bool, TIMXCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCR` writer - HRTIM_TIMxCR register update enable"]
pub struct TIMXCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMXCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMXCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register not updated by burst DMA access"]
    #[inline(always)]
    pub fn not_updated(self) -> &'a mut W {
        self.variant(TIMXCR_A::NOTUPDATED)
    }
    #[doc = "Register updated by burst DMA access"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(TIMXCR_A::UPDATED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtaupr](index.html) module"]
pub struct BDTAUPR_SPEC;
impl crate::RegisterSpec for BDTAUPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdtaupr::R](R) reader structure"]
impl crate::Readable for BDTAUPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtaupr::W](W) writer structure"]
impl crate::Writable for BDTAUPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDTAUPR to value 0"]
impl crate::Resettable for BDTAUPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
