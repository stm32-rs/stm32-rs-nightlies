#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOF0` reader - Synchronization Overrun Flag 0"]
pub struct SOF0_R(crate::FieldReader<bool, bool>);
impl SOF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF0` writer - Synchronization Overrun Flag 0"]
pub struct SOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF0_W<'a> {
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
#[doc = "Field `SOF1` reader - Synchronization Overrun Flag 1"]
pub struct SOF1_R(crate::FieldReader<bool, bool>);
impl SOF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF1` writer - Synchronization Overrun Flag 1"]
pub struct SOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF1_W<'a> {
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
#[doc = "Field `SOF2` reader - Synchronization Overrun Flag 2"]
pub struct SOF2_R(crate::FieldReader<bool, bool>);
impl SOF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF2` writer - Synchronization Overrun Flag 2"]
pub struct SOF2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF2_W<'a> {
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
#[doc = "Field `SOF3` reader - Synchronization Overrun Flag 3"]
pub struct SOF3_R(crate::FieldReader<bool, bool>);
impl SOF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF3` writer - Synchronization Overrun Flag 3"]
pub struct SOF3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF3_W<'a> {
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
#[doc = "Field `SOF4` reader - Synchronization Overrun Flag 4"]
pub struct SOF4_R(crate::FieldReader<bool, bool>);
impl SOF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF4` writer - Synchronization Overrun Flag 4"]
pub struct SOF4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF4_W<'a> {
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
#[doc = "Field `SOF5` reader - Synchronization Overrun Flag 5"]
pub struct SOF5_R(crate::FieldReader<bool, bool>);
impl SOF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF5` writer - Synchronization Overrun Flag 5"]
pub struct SOF5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF5_W<'a> {
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
#[doc = "Field `SOF6` reader - Synchronization Overrun Flag 6"]
pub struct SOF6_R(crate::FieldReader<bool, bool>);
impl SOF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF6` writer - Synchronization Overrun Flag 6"]
pub struct SOF6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF6_W<'a> {
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
#[doc = "Field `SOF7` reader - Synchronization Overrun Flag 7"]
pub struct SOF7_R(crate::FieldReader<bool, bool>);
impl SOF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF7` writer - Synchronization Overrun Flag 7"]
pub struct SOF7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF7_W<'a> {
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
#[doc = "Field `SOF8` reader - Synchronization Overrun Flag 8"]
pub struct SOF8_R(crate::FieldReader<bool, bool>);
impl SOF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF8` writer - Synchronization Overrun Flag 8"]
pub struct SOF8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF8_W<'a> {
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
#[doc = "Field `SOF9` reader - Synchronization Overrun Flag 9"]
pub struct SOF9_R(crate::FieldReader<bool, bool>);
impl SOF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF9` writer - Synchronization Overrun Flag 9"]
pub struct SOF9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF9_W<'a> {
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
#[doc = "Field `SOF10` reader - Synchronization Overrun Flag 10"]
pub struct SOF10_R(crate::FieldReader<bool, bool>);
impl SOF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF10` writer - Synchronization Overrun Flag 10"]
pub struct SOF10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF10_W<'a> {
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
#[doc = "Field `SOF11` reader - Synchronization Overrun Flag 11"]
pub struct SOF11_R(crate::FieldReader<bool, bool>);
impl SOF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF11` writer - Synchronization Overrun Flag 11"]
pub struct SOF11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF11_W<'a> {
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
#[doc = "Field `SOF12` reader - Synchronization Overrun Flag 12"]
pub struct SOF12_R(crate::FieldReader<bool, bool>);
impl SOF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF12` writer - Synchronization Overrun Flag 12"]
pub struct SOF12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF12_W<'a> {
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
#[doc = "Field `SOF13` reader - Synchronization Overrun Flag 13"]
pub struct SOF13_R(crate::FieldReader<bool, bool>);
impl SOF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF13` writer - Synchronization Overrun Flag 13"]
pub struct SOF13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF13_W<'a> {
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
#[doc = "Field `SOF14` reader - Synchronization Overrun Flag 13"]
pub struct SOF14_R(crate::FieldReader<bool, bool>);
impl SOF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF14` writer - Synchronization Overrun Flag 13"]
pub struct SOF14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF14_W<'a> {
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
#[doc = "Field `SOF15` reader - Synchronization Overrun Flag 13"]
pub struct SOF15_R(crate::FieldReader<bool, bool>);
impl SOF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF15` writer - Synchronization Overrun Flag 13"]
pub struct SOF15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF15_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Synchronization Overrun Flag 0"]
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronization Overrun Flag 1"]
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization Overrun Flag 2"]
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronization Overrun Flag 3"]
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Synchronization Overrun Flag 4"]
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Synchronization Overrun Flag 5"]
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Synchronization Overrun Flag 6"]
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Synchronization Overrun Flag 7"]
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Synchronization Overrun Flag 8"]
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Synchronization Overrun Flag 9"]
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Synchronization Overrun Flag 10"]
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Synchronization Overrun Flag 11"]
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Synchronization Overrun Flag 12"]
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof14(&self) -> SOF14_R {
        SOF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof15(&self) -> SOF15_R {
        SOF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Overrun Flag 0"]
    #[inline(always)]
    pub fn sof0(&mut self) -> SOF0_W {
        SOF0_W { w: self }
    }
    #[doc = "Bit 1 - Synchronization Overrun Flag 1"]
    #[inline(always)]
    pub fn sof1(&mut self) -> SOF1_W {
        SOF1_W { w: self }
    }
    #[doc = "Bit 2 - Synchronization Overrun Flag 2"]
    #[inline(always)]
    pub fn sof2(&mut self) -> SOF2_W {
        SOF2_W { w: self }
    }
    #[doc = "Bit 3 - Synchronization Overrun Flag 3"]
    #[inline(always)]
    pub fn sof3(&mut self) -> SOF3_W {
        SOF3_W { w: self }
    }
    #[doc = "Bit 4 - Synchronization Overrun Flag 4"]
    #[inline(always)]
    pub fn sof4(&mut self) -> SOF4_W {
        SOF4_W { w: self }
    }
    #[doc = "Bit 5 - Synchronization Overrun Flag 5"]
    #[inline(always)]
    pub fn sof5(&mut self) -> SOF5_W {
        SOF5_W { w: self }
    }
    #[doc = "Bit 6 - Synchronization Overrun Flag 6"]
    #[inline(always)]
    pub fn sof6(&mut self) -> SOF6_W {
        SOF6_W { w: self }
    }
    #[doc = "Bit 7 - Synchronization Overrun Flag 7"]
    #[inline(always)]
    pub fn sof7(&mut self) -> SOF7_W {
        SOF7_W { w: self }
    }
    #[doc = "Bit 8 - Synchronization Overrun Flag 8"]
    #[inline(always)]
    pub fn sof8(&mut self) -> SOF8_W {
        SOF8_W { w: self }
    }
    #[doc = "Bit 9 - Synchronization Overrun Flag 9"]
    #[inline(always)]
    pub fn sof9(&mut self) -> SOF9_W {
        SOF9_W { w: self }
    }
    #[doc = "Bit 10 - Synchronization Overrun Flag 10"]
    #[inline(always)]
    pub fn sof10(&mut self) -> SOF10_W {
        SOF10_W { w: self }
    }
    #[doc = "Bit 11 - Synchronization Overrun Flag 11"]
    #[inline(always)]
    pub fn sof11(&mut self) -> SOF11_W {
        SOF11_W { w: self }
    }
    #[doc = "Bit 12 - Synchronization Overrun Flag 12"]
    #[inline(always)]
    pub fn sof12(&mut self) -> SOF12_W {
        SOF12_W { w: self }
    }
    #[doc = "Bit 13 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof13(&mut self) -> SOF13_W {
        SOF13_W { w: self }
    }
    #[doc = "Bit 14 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof14(&mut self) -> SOF14_W {
        SOF14_W { w: self }
    }
    #[doc = "Bit 15 - Synchronization Overrun Flag 13"]
    #[inline(always)]
    pub fn sof15(&mut self) -> SOF15_W {
        SOF15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Multiplexer Channel Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
