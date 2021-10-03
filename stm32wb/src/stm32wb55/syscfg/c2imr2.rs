#[doc = "Register `C2IMR2` reader"]
pub struct R(crate::R<C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR2` writer"]
pub struct W(crate::W<C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR2_SPEC>;
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
impl From<crate::W<C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1_CH1_IM` reader - Peripheral DMA1 CH1 interrupt mask to CPU2"]
pub struct DMA1_CH1_IM_R(crate::FieldReader<bool, bool>);
impl DMA1_CH1_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH1_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH1_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_CH1_IM` writer - Peripheral DMA1 CH1 interrupt mask to CPU2"]
pub struct DMA1_CH1_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH1_IM_W<'a> {
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
#[doc = "Field `DMA1_CH2_IM` reader - Peripheral DMA1 CH2 interrupt mask to CPU2"]
pub struct DMA1_CH2_IM_R(crate::FieldReader<bool, bool>);
impl DMA1_CH2_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH2_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH2_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_CH2_IM` writer - Peripheral DMA1 CH2 interrupt mask to CPU2"]
pub struct DMA1_CH2_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH2_IM_W<'a> {
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
#[doc = "Field `DMA1_CH3_IM` reader - Peripheral DMA1 CH3 interrupt mask to CPU2"]
pub struct DMA1_CH3_IM_R(crate::FieldReader<bool, bool>);
impl DMA1_CH3_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH3_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH3_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_CH3_IM` writer - Peripheral DMA1 CH3 interrupt mask to CPU2"]
pub struct DMA1_CH3_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH3_IM_W<'a> {
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
#[doc = "Field `DMA1_CH4_IM` reader - Peripheral DMA1 CH4 interrupt mask to CPU2"]
pub struct DMA1_CH4_IM_R(crate::FieldReader<bool, bool>);
impl DMA1_CH4_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH4_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH4_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_CH4_IM` writer - Peripheral DMA1 CH4 interrupt mask to CPU2"]
pub struct DMA1_CH4_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH4_IM_W<'a> {
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
#[doc = "Field `DMA1_CH5_IM` reader - Peripheral DMA1 CH5 interrupt mask to CPU2"]
pub struct DMA1_CH5_IM_R(crate::FieldReader<bool, bool>);
impl DMA1_CH5_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH5_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH5_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_CH5_IM` writer - Peripheral DMA1 CH5 interrupt mask to CPU2"]
pub struct DMA1_CH5_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH5_IM_W<'a> {
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
#[doc = "Field `DMA1_CH6_IM` reader - Peripheral DMA1 CH6 interrupt mask to CPU2"]
pub struct DMA1_CH6_IM_R(crate::FieldReader<bool, bool>);
impl DMA1_CH6_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH6_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH6_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_CH6_IM` writer - Peripheral DMA1 CH6 interrupt mask to CPU2"]
pub struct DMA1_CH6_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH6_IM_W<'a> {
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
#[doc = "Field `DMA1_CH7_IM` reader - Peripheral DMA1 CH7 interrupt mask to CPU2"]
pub struct DMA1_CH7_IM_R(crate::FieldReader<bool, bool>);
impl DMA1_CH7_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_CH7_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1_CH7_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_CH7_IM` writer - Peripheral DMA1 CH7 interrupt mask to CPU2"]
pub struct DMA1_CH7_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_CH7_IM_W<'a> {
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
#[doc = "Field `DMA2_CH1_IM` reader - Peripheral DMA2 CH1 interrupt mask to CPU1"]
pub struct DMA2_CH1_IM_R(crate::FieldReader<bool, bool>);
impl DMA2_CH1_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2_CH1_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2_CH1_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2_CH1_IM` writer - Peripheral DMA2 CH1 interrupt mask to CPU1"]
pub struct DMA2_CH1_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH1_IM_W<'a> {
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
#[doc = "Field `DMA2_CH2_IM` reader - Peripheral DMA2 CH2 interrupt mask to CPU1"]
pub struct DMA2_CH2_IM_R(crate::FieldReader<bool, bool>);
impl DMA2_CH2_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2_CH2_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2_CH2_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2_CH2_IM` writer - Peripheral DMA2 CH2 interrupt mask to CPU1"]
pub struct DMA2_CH2_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH2_IM_W<'a> {
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
#[doc = "Field `DMA2_CH3_IM` reader - Peripheral DMA2 CH3 interrupt mask to CPU1"]
pub struct DMA2_CH3_IM_R(crate::FieldReader<bool, bool>);
impl DMA2_CH3_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2_CH3_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2_CH3_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2_CH3_IM` writer - Peripheral DMA2 CH3 interrupt mask to CPU1"]
pub struct DMA2_CH3_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH3_IM_W<'a> {
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
#[doc = "Field `DMA2_CH4_IM` reader - Peripheral DMA2 CH4 interrupt mask to CPU1"]
pub struct DMA2_CH4_IM_R(crate::FieldReader<bool, bool>);
impl DMA2_CH4_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2_CH4_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2_CH4_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2_CH4_IM` writer - Peripheral DMA2 CH4 interrupt mask to CPU1"]
pub struct DMA2_CH4_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH4_IM_W<'a> {
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
#[doc = "Field `DMA2_CH5_IM` reader - Peripheral DMA2 CH5 interrupt mask to CPU1"]
pub struct DMA2_CH5_IM_R(crate::FieldReader<bool, bool>);
impl DMA2_CH5_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2_CH5_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2_CH5_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2_CH5_IM` writer - Peripheral DMA2 CH5 interrupt mask to CPU1"]
pub struct DMA2_CH5_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH5_IM_W<'a> {
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
#[doc = "Field `DMA2_CH6_IM` reader - Peripheral DMA2 CH6 interrupt mask to CPU1"]
pub struct DMA2_CH6_IM_R(crate::FieldReader<bool, bool>);
impl DMA2_CH6_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2_CH6_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2_CH6_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2_CH6_IM` writer - Peripheral DMA2 CH6 interrupt mask to CPU1"]
pub struct DMA2_CH6_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH6_IM_W<'a> {
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
#[doc = "Field `DMA2_CH7_IM` reader - Peripheral DMA2 CH7 interrupt mask to CPU1"]
pub struct DMA2_CH7_IM_R(crate::FieldReader<bool, bool>);
impl DMA2_CH7_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2_CH7_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2_CH7_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2_CH7_IM` writer - Peripheral DMA2 CH7 interrupt mask to CPU1"]
pub struct DMA2_CH7_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2_CH7_IM_W<'a> {
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
#[doc = "Field `DMAM_UX1_IM` reader - Peripheral DMAM UX1 interrupt mask to CPU1"]
pub struct DMAM_UX1_IM_R(crate::FieldReader<bool, bool>);
impl DMAM_UX1_IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAM_UX1_IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAM_UX1_IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAM_UX1_IM` writer - Peripheral DMAM UX1 interrupt mask to CPU1"]
pub struct DMAM_UX1_IM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAM_UX1_IM_W<'a> {
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
#[doc = "Field `PVM1IM` reader - Peripheral PVM1IM interrupt mask to CPU1"]
pub struct PVM1IM_R(crate::FieldReader<bool, bool>);
impl PVM1IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVM1IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVM1IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVM1IM` writer - Peripheral PVM1IM interrupt mask to CPU1"]
pub struct PVM1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVM1IM_W<'a> {
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
#[doc = "Field `PVM3IM` reader - Peripheral PVM3IM interrupt mask to CPU1"]
pub struct PVM3IM_R(crate::FieldReader<bool, bool>);
impl PVM3IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVM3IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVM3IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVM3IM` writer - Peripheral PVM3IM interrupt mask to CPU1"]
pub struct PVM3IM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVM3IM_W<'a> {
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
#[doc = "Field `PVDIM` reader - Peripheral PVDIM interrupt mask to CPU1"]
pub struct PVDIM_R(crate::FieldReader<bool, bool>);
impl PVDIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDIM` writer - Peripheral PVDIM interrupt mask to CPU1"]
pub struct PVDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDIM_W<'a> {
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
#[doc = "Field `TSCIM` reader - Peripheral TSCIM interrupt mask to CPU1"]
pub struct TSCIM_R(crate::FieldReader<bool, bool>);
impl TSCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCIM` writer - Peripheral TSCIM interrupt mask to CPU1"]
pub struct TSCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCIM_W<'a> {
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
#[doc = "Field `LCDIM` reader - Peripheral LCDIM interrupt mask to CPU1"]
pub struct LCDIM_R(crate::FieldReader<bool, bool>);
impl LCDIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDIM` writer - Peripheral LCDIM interrupt mask to CPU1"]
pub struct LCDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch1_im(&self) -> DMA1_CH1_IM_R {
        DMA1_CH1_IM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch2_im(&self) -> DMA1_CH2_IM_R {
        DMA1_CH2_IM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch3_im(&self) -> DMA1_CH3_IM_R {
        DMA1_CH3_IM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch4_im(&self) -> DMA1_CH4_IM_R {
        DMA1_CH4_IM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch5_im(&self) -> DMA1_CH5_IM_R {
        DMA1_CH5_IM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch6_im(&self) -> DMA1_CH6_IM_R {
        DMA1_CH6_IM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch7_im(&self) -> DMA1_CH7_IM_R {
        DMA1_CH7_IM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch1_im(&self) -> DMA2_CH1_IM_R {
        DMA2_CH1_IM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch2_im(&self) -> DMA2_CH2_IM_R {
        DMA2_CH2_IM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch3_im(&self) -> DMA2_CH3_IM_R {
        DMA2_CH3_IM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch4_im(&self) -> DMA2_CH4_IM_R {
        DMA2_CH4_IM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch5_im(&self) -> DMA2_CH5_IM_R {
        DMA2_CH5_IM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch6_im(&self) -> DMA2_CH6_IM_R {
        DMA2_CH6_IM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch7_im(&self) -> DMA2_CH7_IM_R {
        DMA2_CH7_IM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dmam_ux1_im(&self) -> DMAM_UX1_IM_R {
        DMAM_UX1_IM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&self) -> PVM1IM_R {
        PVM1IM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&self) -> PVM3IM_R {
        PVM3IM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&self) -> PVDIM_R {
        PVDIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tscim(&self) -> TSCIM_R {
        TSCIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn lcdim(&self) -> LCDIM_R {
        LCDIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch1_im(&mut self) -> DMA1_CH1_IM_W {
        DMA1_CH1_IM_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch2_im(&mut self) -> DMA1_CH2_IM_W {
        DMA1_CH2_IM_W { w: self }
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch3_im(&mut self) -> DMA1_CH3_IM_W {
        DMA1_CH3_IM_W { w: self }
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch4_im(&mut self) -> DMA1_CH4_IM_W {
        DMA1_CH4_IM_W { w: self }
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch5_im(&mut self) -> DMA1_CH5_IM_W {
        DMA1_CH5_IM_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch6_im(&mut self) -> DMA1_CH6_IM_W {
        DMA1_CH6_IM_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn dma1_ch7_im(&mut self) -> DMA1_CH7_IM_W {
        DMA1_CH7_IM_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch1_im(&mut self) -> DMA2_CH1_IM_W {
        DMA2_CH1_IM_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch2_im(&mut self) -> DMA2_CH2_IM_W {
        DMA2_CH2_IM_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch3_im(&mut self) -> DMA2_CH3_IM_W {
        DMA2_CH3_IM_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch4_im(&mut self) -> DMA2_CH4_IM_W {
        DMA2_CH4_IM_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch5_im(&mut self) -> DMA2_CH5_IM_W {
        DMA2_CH5_IM_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch6_im(&mut self) -> DMA2_CH6_IM_W {
        DMA2_CH6_IM_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dma2_ch7_im(&mut self) -> DMA2_CH7_IM_W {
        DMA2_CH7_IM_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline(always)]
    pub fn dmam_ux1_im(&mut self) -> DMAM_UX1_IM_W {
        DMAM_UX1_IM_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm1im(&mut self) -> PVM1IM_W {
        PVM1IM_W { w: self }
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvm3im(&mut self) -> PVM3IM_W {
        PVM3IM_W { w: self }
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn pvdim(&mut self) -> PVDIM_W {
        PVDIM_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn tscim(&mut self) -> TSCIM_W {
        TSCIM_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline(always)]
    pub fn lcdim(&mut self) -> LCDIM_W {
        LCDIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](index.html) module"]
pub struct C2IMR2_SPEC;
impl crate::RegisterSpec for C2IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr2::R](R) reader structure"]
impl crate::Readable for C2IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](W) writer structure"]
impl crate::Writable for C2IMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2IMR2 to value 0"]
impl crate::Resettable for C2IMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
