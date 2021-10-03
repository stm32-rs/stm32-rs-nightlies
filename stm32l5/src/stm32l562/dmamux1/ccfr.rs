#[doc = "Register `CCFR` reader"]
pub struct R(crate::R<CCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFR` writer"]
pub struct W(crate::W<CCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFR_SPEC>;
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
impl From<crate::W<CCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSOF0` reader - Synchronization Clear Overrun Flag 0"]
pub struct CSOF0_R(crate::FieldReader<bool, bool>);
impl CSOF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF0` writer - Synchronization Clear Overrun Flag 0"]
pub struct CSOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF0_W<'a> {
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
#[doc = "Field `CSOF1` reader - Synchronization Clear Overrun Flag 1"]
pub struct CSOF1_R(crate::FieldReader<bool, bool>);
impl CSOF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF1` writer - Synchronization Clear Overrun Flag 1"]
pub struct CSOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF1_W<'a> {
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
#[doc = "Field `CSOF2` reader - Synchronization Clear Overrun Flag 2"]
pub struct CSOF2_R(crate::FieldReader<bool, bool>);
impl CSOF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF2` writer - Synchronization Clear Overrun Flag 2"]
pub struct CSOF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF2_W<'a> {
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
#[doc = "Field `CSOF3` reader - Synchronization Clear Overrun Flag 3"]
pub struct CSOF3_R(crate::FieldReader<bool, bool>);
impl CSOF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF3` writer - Synchronization Clear Overrun Flag 3"]
pub struct CSOF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF3_W<'a> {
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
#[doc = "Field `CSOF4` reader - Synchronization Clear Overrun Flag 4"]
pub struct CSOF4_R(crate::FieldReader<bool, bool>);
impl CSOF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF4` writer - Synchronization Clear Overrun Flag 4"]
pub struct CSOF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF4_W<'a> {
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
#[doc = "Field `CSOF5` reader - Synchronization Clear Overrun Flag 5"]
pub struct CSOF5_R(crate::FieldReader<bool, bool>);
impl CSOF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF5` writer - Synchronization Clear Overrun Flag 5"]
pub struct CSOF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF5_W<'a> {
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
#[doc = "Field `CSOF6` reader - Synchronization Clear Overrun Flag 6"]
pub struct CSOF6_R(crate::FieldReader<bool, bool>);
impl CSOF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF6` writer - Synchronization Clear Overrun Flag 6"]
pub struct CSOF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF6_W<'a> {
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
#[doc = "Field `CSOF7` reader - Synchronization Clear Overrun Flag 7"]
pub struct CSOF7_R(crate::FieldReader<bool, bool>);
impl CSOF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF7` writer - Synchronization Clear Overrun Flag 7"]
pub struct CSOF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF7_W<'a> {
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
#[doc = "Field `CSOF8` reader - Synchronization Clear Overrun Flag 8"]
pub struct CSOF8_R(crate::FieldReader<bool, bool>);
impl CSOF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF8` writer - Synchronization Clear Overrun Flag 8"]
pub struct CSOF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF8_W<'a> {
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
#[doc = "Field `CSOF9` reader - Synchronization Clear Overrun Flag 9"]
pub struct CSOF9_R(crate::FieldReader<bool, bool>);
impl CSOF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF9` writer - Synchronization Clear Overrun Flag 9"]
pub struct CSOF9_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF9_W<'a> {
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
#[doc = "Field `CSOF10` reader - Synchronization Clear Overrun Flag 10"]
pub struct CSOF10_R(crate::FieldReader<bool, bool>);
impl CSOF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF10` writer - Synchronization Clear Overrun Flag 10"]
pub struct CSOF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF10_W<'a> {
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
#[doc = "Field `CSOF11` reader - Synchronization Clear Overrun Flag 11"]
pub struct CSOF11_R(crate::FieldReader<bool, bool>);
impl CSOF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF11` writer - Synchronization Clear Overrun Flag 11"]
pub struct CSOF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF11_W<'a> {
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
#[doc = "Field `CSOF12` reader - Synchronization Clear Overrun Flag 12"]
pub struct CSOF12_R(crate::FieldReader<bool, bool>);
impl CSOF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF12` writer - Synchronization Clear Overrun Flag 12"]
pub struct CSOF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF12_W<'a> {
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
#[doc = "Field `CSOF13` reader - Synchronization Clear Overrun Flag 13"]
pub struct CSOF13_R(crate::FieldReader<bool, bool>);
impl CSOF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF13` writer - Synchronization Clear Overrun Flag 13"]
pub struct CSOF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF13_W<'a> {
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
#[doc = "Field `CSOF14` reader - Synchronization Clear Overrun Flag 13"]
pub struct CSOF14_R(crate::FieldReader<bool, bool>);
impl CSOF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF14` writer - Synchronization Clear Overrun Flag 13"]
pub struct CSOF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF14_W<'a> {
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
#[doc = "Field `CSOF15` reader - Synchronization Clear Overrun Flag 13"]
pub struct CSOF15_R(crate::FieldReader<bool, bool>);
impl CSOF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSOF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSOF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSOF15` writer - Synchronization Clear Overrun Flag 13"]
pub struct CSOF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF15_W<'a> {
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
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&self) -> CSOF0_R {
        CSOF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&self) -> CSOF1_R {
        CSOF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&self) -> CSOF2_R {
        CSOF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&self) -> CSOF3_R {
        CSOF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    pub fn csof4(&self) -> CSOF4_R {
        CSOF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    pub fn csof5(&self) -> CSOF5_R {
        CSOF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    pub fn csof6(&self) -> CSOF6_R {
        CSOF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    pub fn csof7(&self) -> CSOF7_R {
        CSOF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    pub fn csof8(&self) -> CSOF8_R {
        CSOF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    pub fn csof9(&self) -> CSOF9_R {
        CSOF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    pub fn csof10(&self) -> CSOF10_R {
        CSOF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    pub fn csof11(&self) -> CSOF11_R {
        CSOF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    pub fn csof12(&self) -> CSOF12_R {
        CSOF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof13(&self) -> CSOF13_R {
        CSOF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof14(&self) -> CSOF14_R {
        CSOF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof15(&self) -> CSOF15_R {
        CSOF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W {
        CSOF0_W { w: self }
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W {
        CSOF1_W { w: self }
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W {
        CSOF2_W { w: self }
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W {
        CSOF3_W { w: self }
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W {
        CSOF4_W { w: self }
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF5_W {
        CSOF5_W { w: self }
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF6_W {
        CSOF6_W { w: self }
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF7_W {
        CSOF7_W { w: self }
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF8_W {
        CSOF8_W { w: self }
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF9_W {
        CSOF9_W { w: self }
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF10_W {
        CSOF10_W { w: self }
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF11_W {
        CSOF11_W { w: self }
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    pub fn csof12(&mut self) -> CSOF12_W {
        CSOF12_W { w: self }
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof13(&mut self) -> CSOF13_W {
        CSOF13_W { w: self }
    }
    #[doc = "Bit 14 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof14(&mut self) -> CSOF14_W {
        CSOF14_W { w: self }
    }
    #[doc = "Bit 15 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof15(&mut self) -> CSOF15_W {
        CSOF15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Clear Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfr](index.html) module"]
pub struct CCFR_SPEC;
impl crate::RegisterSpec for CCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfr::R](R) reader structure"]
impl crate::Readable for CCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfr::W](W) writer structure"]
impl crate::Writable for CCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFR to value 0"]
impl crate::Resettable for CCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
