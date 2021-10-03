#[doc = "Register `BDTFUPR` reader"]
pub struct R(crate::R<BDTFUPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTFUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTFUPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTFUPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTFUPR` writer"]
pub struct W(crate::W<BDTFUPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTFUPR_SPEC>;
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
impl From<crate::W<BDTFUPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTFUPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMxEEFR3` reader - TIMxEEFR3"]
pub struct TIMXEEFR3_R(crate::FieldReader<bool, bool>);
impl TIMXEEFR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXEEFR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXEEFR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxEEFR3` writer - TIMxEEFR3"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TIMxCR2` reader - TIMxCR2"]
pub struct TIMXCR2_R(crate::FieldReader<bool, bool>);
impl TIMXCR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCR2` writer - TIMxCR2"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TIMxFLTR` reader - HRTIM_FLTxR register update enable"]
pub struct TIMXFLTR_R(crate::FieldReader<bool, bool>);
impl TIMXFLTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXFLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXFLTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxFLTR` writer - HRTIM_FLTxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TIMxOUTR` reader - HRTIM_OUTxR register update enable"]
pub struct TIMXOUTR_R(crate::FieldReader<bool, bool>);
impl TIMXOUTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXOUTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXOUTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxOUTR` writer - HRTIM_OUTxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TIMxCHPR` reader - HRTIM_CHPxR register update enable"]
pub struct TIMXCHPR_R(crate::FieldReader<bool, bool>);
impl TIMXCHPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCHPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCHPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCHPR` writer - HRTIM_CHPxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TIMxRSTR` reader - HRTIM_RSTxR register update enable"]
pub struct TIMXRSTR_R(crate::FieldReader<bool, bool>);
impl TIMXRSTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXRSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXRSTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxRSTR` writer - HRTIM_RSTxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `TIMxEEFR2` reader - HRTIM_EEFxR2 register update enable"]
pub struct TIMXEEFR2_R(crate::FieldReader<bool, bool>);
impl TIMXEEFR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXEEFR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXEEFR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxEEFR2` writer - HRTIM_EEFxR2 register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TIMxEEFR1` reader - HRTIM_EEFxR1 register update enable"]
pub struct TIMXEEFR1_R(crate::FieldReader<bool, bool>);
impl TIMXEEFR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXEEFR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXEEFR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxEEFR1` writer - HRTIM_EEFxR1 register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TIMxRST2R` reader - HRTIM_RST2xR register update enable"]
pub struct TIMXRST2R_R(crate::FieldReader<bool, bool>);
impl TIMXRST2R_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXRST2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXRST2R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxRST2R` writer - HRTIM_RST2xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TIMxSET2R` reader - HRTIM_SET2xR register update enable"]
pub struct TIMXSET2R_R(crate::FieldReader<bool, bool>);
impl TIMXSET2R_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXSET2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXSET2R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxSET2R` writer - HRTIM_SET2xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TIMxRST1R` reader - HRTIM_RST1xR register update enable"]
pub struct TIMXRST1R_R(crate::FieldReader<bool, bool>);
impl TIMXRST1R_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXRST1R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXRST1R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxRST1R` writer - HRTIM_RST1xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TIMxSET1R` reader - HRTIM_SET1xR register update enable"]
pub struct TIMXSET1R_R(crate::FieldReader<bool, bool>);
impl TIMXSET1R_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXSET1R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXSET1R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxSET1R` writer - HRTIM_SET1xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TIMx_DTxR` reader - HRTIM_DTxR register update enable"]
pub struct TIMX_DTXR_R(crate::FieldReader<bool, bool>);
impl TIMX_DTXR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMX_DTXR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMX_DTXR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMx_DTxR` writer - HRTIM_DTxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TIMxCMP4` reader - HRTIM_CMP4xR register update enable"]
pub struct TIMXCMP4_R(crate::FieldReader<bool, bool>);
impl TIMXCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCMP4` writer - HRTIM_CMP4xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TIMxCMP3` reader - HRTIM_CMP3xR register update enable"]
pub struct TIMXCMP3_R(crate::FieldReader<bool, bool>);
impl TIMXCMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCMP3` writer - HRTIM_CMP3xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TIMxCMP2` reader - HRTIM_CMP2xR register update enable"]
pub struct TIMXCMP2_R(crate::FieldReader<bool, bool>);
impl TIMXCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCMP2` writer - HRTIM_CMP2xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TIMxCMP1` reader - HRTIM_CMP1xR register update enable"]
pub struct TIMXCMP1_R(crate::FieldReader<bool, bool>);
impl TIMXCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCMP1` writer - HRTIM_CMP1xR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TIMxREP` reader - HRTIM_REPxR register update enable"]
pub struct TIMXREP_R(crate::FieldReader<bool, bool>);
impl TIMXREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXREP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxREP` writer - HRTIM_REPxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TIMxPER` reader - HRTIM_PERxR register update enable"]
pub struct TIMXPER_R(crate::FieldReader<bool, bool>);
impl TIMXPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxPER` writer - HRTIM_PERxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TIMxCNT` reader - HRTIM_CNTxR register update enable"]
pub struct TIMXCNT_R(crate::FieldReader<bool, bool>);
impl TIMXCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxCNT` writer - HRTIM_CNTxR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TIMxDIER` reader - HRTIM_TIMxDIER register update enable"]
pub struct TIMXDIER_R(crate::FieldReader<bool, bool>);
impl TIMXDIER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXDIER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXDIER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxDIER` writer - HRTIM_TIMxDIER register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TIMxICR` reader - HRTIM_TIMxICR register update enable"]
pub struct TIMXICR_R(crate::FieldReader<bool, bool>);
impl TIMXICR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXICR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXICR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMxICR` writer - HRTIM_TIMxICR register update enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TIMxCR` reader - HRTIM_TIMxCR register update enable"]
pub struct TIMXCR_R(crate::FieldReader<bool, bool>);
impl TIMXCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMXCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMXCR_R {
    type Target = crate::FieldReader<bool, bool>;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtfupr](index.html) module"]
pub struct BDTFUPR_SPEC;
impl crate::RegisterSpec for BDTFUPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdtfupr::R](R) reader structure"]
impl crate::Readable for BDTFUPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtfupr::W](W) writer structure"]
impl crate::Writable for BDTFUPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDTFUPR to value 0"]
impl crate::Resettable for BDTFUPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
