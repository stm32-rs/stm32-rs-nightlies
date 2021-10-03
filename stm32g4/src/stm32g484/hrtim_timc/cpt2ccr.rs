#[doc = "Register `CPT2CCR` reader"]
pub struct R(crate::R<CPT2CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT2CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT2CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT2CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPT2CCR` writer"]
pub struct W(crate::W<CPT2CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPT2CCR_SPEC>;
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
impl From<crate::W<CPT2CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPT2CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TECMP2` reader - Timer E Compare 2"]
pub struct TECMP2_R(crate::FieldReader<bool, bool>);
impl TECMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TECMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TECMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TECMP2` writer - Timer E Compare 2"]
pub struct TECMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `TECMP1` reader - Timer E Compare 1"]
pub struct TECMP1_R(crate::FieldReader<bool, bool>);
impl TECMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TECMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TECMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TECMP1` writer - Timer E Compare 1"]
pub struct TECMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `TE1RST` reader - Timer E output 1 Reset"]
pub struct TE1RST_R(crate::FieldReader<bool, bool>);
impl TE1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE1RST` writer - Timer E output 1 Reset"]
pub struct TE1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TE1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `TE1SET` reader - Timer E output 1 Set"]
pub struct TE1SET_R(crate::FieldReader<bool, bool>);
impl TE1SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        TE1SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE1SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE1SET` writer - Timer E output 1 Set"]
pub struct TE1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TE1SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `TDCMP2` reader - Timer D Compare 2"]
pub struct TDCMP2_R(crate::FieldReader<bool, bool>);
impl TDCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCMP2` writer - Timer D Compare 2"]
pub struct TDCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `TDCMP1` reader - Timer D Compare 1"]
pub struct TDCMP1_R(crate::FieldReader<bool, bool>);
impl TDCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCMP1` writer - Timer D Compare 1"]
pub struct TDCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TD1RST` reader - Timer D output 1 Reset"]
pub struct TD1RST_R(crate::FieldReader<bool, bool>);
impl TD1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TD1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TD1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TD1RST` writer - Timer D output 1 Reset"]
pub struct TD1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TD1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `TD1SET` reader - Timer D output 1 Set"]
pub struct TD1SET_R(crate::FieldReader<bool, bool>);
impl TD1SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        TD1SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TD1SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TD1SET` writer - Timer D output 1 Set"]
pub struct TD1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TD1SET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `TF1CMP2` reader - TF1CMP2"]
pub struct TF1CMP2_R(crate::FieldReader<bool, bool>);
impl TF1CMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF1CMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF1CMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF1CMP2` writer - TF1CMP2"]
pub struct TF1CMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1CMP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TF1CMP1` reader - TF1CMP1"]
pub struct TF1CMP1_R(crate::FieldReader<bool, bool>);
impl TF1CMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF1CMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF1CMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF1CMP1` writer - TF1CMP1"]
pub struct TF1CMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1CMP1_W<'a> {
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
#[doc = "Field `TF1RST` reader - TF1RST"]
pub struct TF1RST_R(crate::FieldReader<bool, bool>);
impl TF1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF1RST` writer - TF1RST"]
pub struct TF1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1RST_W<'a> {
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
#[doc = "Field `TF1SET` reader - TF1SET"]
pub struct TF1SET_R(crate::FieldReader<bool, bool>);
impl TF1SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        TF1SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF1SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF1SET` writer - TF1SET"]
pub struct TF1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TF1SET_W<'a> {
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
#[doc = "Field `TBCMP2` reader - Timer B Compare 2"]
pub struct TBCMP2_R(crate::FieldReader<bool, bool>);
impl TBCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBCMP2` writer - Timer B Compare 2"]
pub struct TBCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP2_W<'a> {
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
#[doc = "Field `TBCMP1` reader - Timer B Compare 1"]
pub struct TBCMP1_R(crate::FieldReader<bool, bool>);
impl TBCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBCMP1` writer - Timer B Compare 1"]
pub struct TBCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP1_W<'a> {
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
#[doc = "Field `TB1RST` reader - Timer B output 1 Reset"]
pub struct TB1RST_R(crate::FieldReader<bool, bool>);
impl TB1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TB1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TB1RST` writer - Timer B output 1 Reset"]
pub struct TB1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TB1RST_W<'a> {
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
#[doc = "Field `TB1SET` reader - Timer B output 1 Set"]
pub struct TB1SET_R(crate::FieldReader<bool, bool>);
impl TB1SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        TB1SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TB1SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TB1SET` writer - Timer B output 1 Set"]
pub struct TB1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TB1SET_W<'a> {
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
#[doc = "Field `TACMP2` reader - Timer A Compare 2"]
pub struct TACMP2_R(crate::FieldReader<bool, bool>);
impl TACMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACMP2` writer - Timer A Compare 2"]
pub struct TACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP2_W<'a> {
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
#[doc = "Field `TACMP1` reader - Timer A Compare 1"]
pub struct TACMP1_R(crate::FieldReader<bool, bool>);
impl TACMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACMP1` writer - Timer A Compare 1"]
pub struct TACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP1_W<'a> {
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
#[doc = "Field `TA1RST` reader - Timer A output 1 Reset"]
pub struct TA1RST_R(crate::FieldReader<bool, bool>);
impl TA1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TA1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA1RST` writer - Timer A output 1 Reset"]
pub struct TA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TA1RST_W<'a> {
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
#[doc = "Field `TA1SET` reader - Timer A output 1 Set"]
pub struct TA1SET_R(crate::FieldReader<bool, bool>);
impl TA1SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        TA1SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA1SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA1SET` writer - Timer A output 1 Set"]
pub struct TA1SET_W<'a> {
    w: &'a mut W,
}
impl<'a> TA1SET_W<'a> {
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
#[doc = "Field `EXEV10CPT` reader - External Event 10 Capture"]
pub struct EXEV10CPT_R(crate::FieldReader<bool, bool>);
impl EXEV10CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV10CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV10CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV10CPT` writer - External Event 10 Capture"]
pub struct EXEV10CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV10CPT_W<'a> {
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
#[doc = "Field `EXEV9CPT` reader - External Event 9 Capture"]
pub struct EXEV9CPT_R(crate::FieldReader<bool, bool>);
impl EXEV9CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV9CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV9CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV9CPT` writer - External Event 9 Capture"]
pub struct EXEV9CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV9CPT_W<'a> {
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
#[doc = "Field `EXEV8CPT` reader - External Event 8 Capture"]
pub struct EXEV8CPT_R(crate::FieldReader<bool, bool>);
impl EXEV8CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV8CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV8CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV8CPT` writer - External Event 8 Capture"]
pub struct EXEV8CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV8CPT_W<'a> {
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
#[doc = "Field `EXEV7CPT` reader - External Event 7 Capture"]
pub struct EXEV7CPT_R(crate::FieldReader<bool, bool>);
impl EXEV7CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV7CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV7CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV7CPT` writer - External Event 7 Capture"]
pub struct EXEV7CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV7CPT_W<'a> {
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
#[doc = "Field `EXEV6CPT` reader - External Event 6 Capture"]
pub struct EXEV6CPT_R(crate::FieldReader<bool, bool>);
impl EXEV6CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV6CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV6CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV6CPT` writer - External Event 6 Capture"]
pub struct EXEV6CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV6CPT_W<'a> {
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
#[doc = "Field `EXEV5CPT` reader - External Event 5 Capture"]
pub struct EXEV5CPT_R(crate::FieldReader<bool, bool>);
impl EXEV5CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV5CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV5CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV5CPT` writer - External Event 5 Capture"]
pub struct EXEV5CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV5CPT_W<'a> {
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
#[doc = "Field `EXEV4CPT` reader - External Event 4 Capture"]
pub struct EXEV4CPT_R(crate::FieldReader<bool, bool>);
impl EXEV4CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV4CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV4CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV4CPT` writer - External Event 4 Capture"]
pub struct EXEV4CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV4CPT_W<'a> {
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
#[doc = "Field `EXEV3CPT` reader - External Event 3 Capture"]
pub struct EXEV3CPT_R(crate::FieldReader<bool, bool>);
impl EXEV3CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV3CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV3CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV3CPT` writer - External Event 3 Capture"]
pub struct EXEV3CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV3CPT_W<'a> {
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
#[doc = "Field `EXEV2CPT` reader - External Event 2 Capture"]
pub struct EXEV2CPT_R(crate::FieldReader<bool, bool>);
impl EXEV2CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV2CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV2CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV2CPT` writer - External Event 2 Capture"]
pub struct EXEV2CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV2CPT_W<'a> {
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
#[doc = "Field `EXEV1CPT` reader - External Event 1 Capture"]
pub struct EXEV1CPT_R(crate::FieldReader<bool, bool>);
impl EXEV1CPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXEV1CPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXEV1CPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXEV1CPT` writer - External Event 1 Capture"]
pub struct EXEV1CPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXEV1CPT_W<'a> {
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
#[doc = "Field `UDPCPT` reader - Update Capture"]
pub struct UDPCPT_R(crate::FieldReader<bool, bool>);
impl UDPCPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDPCPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDPCPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDPCPT` writer - Update Capture"]
pub struct UDPCPT_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPCPT_W<'a> {
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
#[doc = "Field `SWCPT` reader - Software Capture"]
pub struct SWCPT_R(crate::FieldReader<bool, bool>);
impl SWCPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWCPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWCPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWCPT` writer - Software Capture"]
pub struct SWCPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCPT_W<'a> {
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
    #[doc = "Bit 31 - Timer E Compare 2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Timer E Compare 1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Timer E output 1 Reset"]
    #[inline(always)]
    pub fn te1rst(&self) -> TE1RST_R {
        TE1RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Timer E output 1 Set"]
    #[inline(always)]
    pub fn te1set(&self) -> TE1SET_R {
        TE1SET_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    pub fn td1rst(&self) -> TD1RST_R {
        TD1RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    pub fn td1set(&self) -> TD1SET_R {
        TD1SET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TF1CMP2"]
    #[inline(always)]
    pub fn tf1cmp2(&self) -> TF1CMP2_R {
        TF1CMP2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TF1CMP1"]
    #[inline(always)]
    pub fn tf1cmp1(&self) -> TF1CMP1_R {
        TF1CMP1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TF1RST"]
    #[inline(always)]
    pub fn tf1rst(&self) -> TF1RST_R {
        TF1RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TF1SET"]
    #[inline(always)]
    pub fn tf1set(&self) -> TF1SET_R {
        TF1SET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    pub fn tb1rst(&self) -> TB1RST_R {
        TB1RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    pub fn tb1set(&self) -> TB1SET_R {
        TB1SET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    pub fn ta1rst(&self) -> TA1RST_R {
        TA1RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    pub fn ta1set(&self) -> TA1SET_R {
        TA1SET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    pub fn exev10cpt(&self) -> EXEV10CPT_R {
        EXEV10CPT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    pub fn exev9cpt(&self) -> EXEV9CPT_R {
        EXEV9CPT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    pub fn exev8cpt(&self) -> EXEV8CPT_R {
        EXEV8CPT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    pub fn exev7cpt(&self) -> EXEV7CPT_R {
        EXEV7CPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    pub fn exev6cpt(&self) -> EXEV6CPT_R {
        EXEV6CPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    pub fn exev5cpt(&self) -> EXEV5CPT_R {
        EXEV5CPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    pub fn exev4cpt(&self) -> EXEV4CPT_R {
        EXEV4CPT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    pub fn exev3cpt(&self) -> EXEV3CPT_R {
        EXEV3CPT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    pub fn exev2cpt(&self) -> EXEV2CPT_R {
        EXEV2CPT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    pub fn exev1cpt(&self) -> EXEV1CPT_R {
        EXEV1CPT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    pub fn udpcpt(&self) -> UDPCPT_R {
        UDPCPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    pub fn swcpt(&self) -> SWCPT_R {
        SWCPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Timer E Compare 2"]
    #[inline(always)]
    pub fn tecmp2(&mut self) -> TECMP2_W {
        TECMP2_W { w: self }
    }
    #[doc = "Bit 30 - Timer E Compare 1"]
    #[inline(always)]
    pub fn tecmp1(&mut self) -> TECMP1_W {
        TECMP1_W { w: self }
    }
    #[doc = "Bit 29 - Timer E output 1 Reset"]
    #[inline(always)]
    pub fn te1rst(&mut self) -> TE1RST_W {
        TE1RST_W { w: self }
    }
    #[doc = "Bit 28 - Timer E output 1 Set"]
    #[inline(always)]
    pub fn te1set(&mut self) -> TE1SET_W {
        TE1SET_W { w: self }
    }
    #[doc = "Bit 27 - Timer D Compare 2"]
    #[inline(always)]
    pub fn tdcmp2(&mut self) -> TDCMP2_W {
        TDCMP2_W { w: self }
    }
    #[doc = "Bit 26 - Timer D Compare 1"]
    #[inline(always)]
    pub fn tdcmp1(&mut self) -> TDCMP1_W {
        TDCMP1_W { w: self }
    }
    #[doc = "Bit 25 - Timer D output 1 Reset"]
    #[inline(always)]
    pub fn td1rst(&mut self) -> TD1RST_W {
        TD1RST_W { w: self }
    }
    #[doc = "Bit 24 - Timer D output 1 Set"]
    #[inline(always)]
    pub fn td1set(&mut self) -> TD1SET_W {
        TD1SET_W { w: self }
    }
    #[doc = "Bit 23 - TF1CMP2"]
    #[inline(always)]
    pub fn tf1cmp2(&mut self) -> TF1CMP2_W {
        TF1CMP2_W { w: self }
    }
    #[doc = "Bit 22 - TF1CMP1"]
    #[inline(always)]
    pub fn tf1cmp1(&mut self) -> TF1CMP1_W {
        TF1CMP1_W { w: self }
    }
    #[doc = "Bit 21 - TF1RST"]
    #[inline(always)]
    pub fn tf1rst(&mut self) -> TF1RST_W {
        TF1RST_W { w: self }
    }
    #[doc = "Bit 20 - TF1SET"]
    #[inline(always)]
    pub fn tf1set(&mut self) -> TF1SET_W {
        TF1SET_W { w: self }
    }
    #[doc = "Bit 19 - Timer B Compare 2"]
    #[inline(always)]
    pub fn tbcmp2(&mut self) -> TBCMP2_W {
        TBCMP2_W { w: self }
    }
    #[doc = "Bit 18 - Timer B Compare 1"]
    #[inline(always)]
    pub fn tbcmp1(&mut self) -> TBCMP1_W {
        TBCMP1_W { w: self }
    }
    #[doc = "Bit 17 - Timer B output 1 Reset"]
    #[inline(always)]
    pub fn tb1rst(&mut self) -> TB1RST_W {
        TB1RST_W { w: self }
    }
    #[doc = "Bit 16 - Timer B output 1 Set"]
    #[inline(always)]
    pub fn tb1set(&mut self) -> TB1SET_W {
        TB1SET_W { w: self }
    }
    #[doc = "Bit 15 - Timer A Compare 2"]
    #[inline(always)]
    pub fn tacmp2(&mut self) -> TACMP2_W {
        TACMP2_W { w: self }
    }
    #[doc = "Bit 14 - Timer A Compare 1"]
    #[inline(always)]
    pub fn tacmp1(&mut self) -> TACMP1_W {
        TACMP1_W { w: self }
    }
    #[doc = "Bit 13 - Timer A output 1 Reset"]
    #[inline(always)]
    pub fn ta1rst(&mut self) -> TA1RST_W {
        TA1RST_W { w: self }
    }
    #[doc = "Bit 12 - Timer A output 1 Set"]
    #[inline(always)]
    pub fn ta1set(&mut self) -> TA1SET_W {
        TA1SET_W { w: self }
    }
    #[doc = "Bit 11 - External Event 10 Capture"]
    #[inline(always)]
    pub fn exev10cpt(&mut self) -> EXEV10CPT_W {
        EXEV10CPT_W { w: self }
    }
    #[doc = "Bit 10 - External Event 9 Capture"]
    #[inline(always)]
    pub fn exev9cpt(&mut self) -> EXEV9CPT_W {
        EXEV9CPT_W { w: self }
    }
    #[doc = "Bit 9 - External Event 8 Capture"]
    #[inline(always)]
    pub fn exev8cpt(&mut self) -> EXEV8CPT_W {
        EXEV8CPT_W { w: self }
    }
    #[doc = "Bit 8 - External Event 7 Capture"]
    #[inline(always)]
    pub fn exev7cpt(&mut self) -> EXEV7CPT_W {
        EXEV7CPT_W { w: self }
    }
    #[doc = "Bit 7 - External Event 6 Capture"]
    #[inline(always)]
    pub fn exev6cpt(&mut self) -> EXEV6CPT_W {
        EXEV6CPT_W { w: self }
    }
    #[doc = "Bit 6 - External Event 5 Capture"]
    #[inline(always)]
    pub fn exev5cpt(&mut self) -> EXEV5CPT_W {
        EXEV5CPT_W { w: self }
    }
    #[doc = "Bit 5 - External Event 4 Capture"]
    #[inline(always)]
    pub fn exev4cpt(&mut self) -> EXEV4CPT_W {
        EXEV4CPT_W { w: self }
    }
    #[doc = "Bit 4 - External Event 3 Capture"]
    #[inline(always)]
    pub fn exev3cpt(&mut self) -> EXEV3CPT_W {
        EXEV3CPT_W { w: self }
    }
    #[doc = "Bit 3 - External Event 2 Capture"]
    #[inline(always)]
    pub fn exev2cpt(&mut self) -> EXEV2CPT_W {
        EXEV2CPT_W { w: self }
    }
    #[doc = "Bit 2 - External Event 1 Capture"]
    #[inline(always)]
    pub fn exev1cpt(&mut self) -> EXEV1CPT_W {
        EXEV1CPT_W { w: self }
    }
    #[doc = "Bit 1 - Update Capture"]
    #[inline(always)]
    pub fn udpcpt(&mut self) -> UDPCPT_W {
        UDPCPT_W { w: self }
    }
    #[doc = "Bit 0 - Software Capture"]
    #[inline(always)]
    pub fn swcpt(&mut self) -> SWCPT_W {
        SWCPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPT2xCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2ccr](index.html) module"]
pub struct CPT2CCR_SPEC;
impl crate::RegisterSpec for CPT2CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpt2ccr::R](R) reader structure"]
impl crate::Readable for CPT2CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpt2ccr::W](W) writer structure"]
impl crate::Writable for CPT2CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPT2CCR to value 0"]
impl crate::Resettable for CPT2CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
