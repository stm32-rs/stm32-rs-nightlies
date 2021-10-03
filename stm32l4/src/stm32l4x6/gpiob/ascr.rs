#[doc = "Register `ASCR` reader"]
pub struct R(crate::R<ASCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASCR` writer"]
pub struct W(crate::W<ASCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASCR_SPEC>;
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
impl From<crate::W<ASCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASC0` reader - Port analog switch control"]
pub struct ASC0_R(crate::FieldReader<bool, bool>);
impl ASC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC0` writer - Port analog switch control"]
pub struct ASC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC0_W<'a> {
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
#[doc = "Field `ASC1` reader - Port analog switch control"]
pub struct ASC1_R(crate::FieldReader<bool, bool>);
impl ASC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC1` writer - Port analog switch control"]
pub struct ASC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC1_W<'a> {
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
#[doc = "Field `ASC2` reader - Port analog switch control"]
pub struct ASC2_R(crate::FieldReader<bool, bool>);
impl ASC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC2` writer - Port analog switch control"]
pub struct ASC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC2_W<'a> {
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
#[doc = "Field `ASC3` reader - Port analog switch control"]
pub struct ASC3_R(crate::FieldReader<bool, bool>);
impl ASC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC3` writer - Port analog switch control"]
pub struct ASC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC3_W<'a> {
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
#[doc = "Field `ASC4` reader - Port analog switch control"]
pub struct ASC4_R(crate::FieldReader<bool, bool>);
impl ASC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC4` writer - Port analog switch control"]
pub struct ASC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC4_W<'a> {
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
#[doc = "Field `ASC5` reader - Port analog switch control"]
pub struct ASC5_R(crate::FieldReader<bool, bool>);
impl ASC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC5` writer - Port analog switch control"]
pub struct ASC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC5_W<'a> {
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
#[doc = "Field `ASC6` reader - Port analog switch control"]
pub struct ASC6_R(crate::FieldReader<bool, bool>);
impl ASC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC6` writer - Port analog switch control"]
pub struct ASC6_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC6_W<'a> {
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
#[doc = "Field `ASC7` reader - Port analog switch control"]
pub struct ASC7_R(crate::FieldReader<bool, bool>);
impl ASC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC7` writer - Port analog switch control"]
pub struct ASC7_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC7_W<'a> {
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
#[doc = "Field `ASC8` reader - Port analog switch control"]
pub struct ASC8_R(crate::FieldReader<bool, bool>);
impl ASC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC8` writer - Port analog switch control"]
pub struct ASC8_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC8_W<'a> {
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
#[doc = "Field `ASC9` reader - Port analog switch control"]
pub struct ASC9_R(crate::FieldReader<bool, bool>);
impl ASC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC9` writer - Port analog switch control"]
pub struct ASC9_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC9_W<'a> {
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
#[doc = "Field `ASC10` reader - Port analog switch control"]
pub struct ASC10_R(crate::FieldReader<bool, bool>);
impl ASC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC10` writer - Port analog switch control"]
pub struct ASC10_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC10_W<'a> {
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
#[doc = "Field `ASC11` reader - Port analog switch control"]
pub struct ASC11_R(crate::FieldReader<bool, bool>);
impl ASC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC11` writer - Port analog switch control"]
pub struct ASC11_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC11_W<'a> {
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
#[doc = "Field `ASC12` reader - Port analog switch control"]
pub struct ASC12_R(crate::FieldReader<bool, bool>);
impl ASC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC12` writer - Port analog switch control"]
pub struct ASC12_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC12_W<'a> {
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
#[doc = "Field `ASC13` reader - Port analog switch control"]
pub struct ASC13_R(crate::FieldReader<bool, bool>);
impl ASC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC13` writer - Port analog switch control"]
pub struct ASC13_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC13_W<'a> {
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
#[doc = "Field `ASC14` reader - Port analog switch control"]
pub struct ASC14_R(crate::FieldReader<bool, bool>);
impl ASC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC14` writer - Port analog switch control"]
pub struct ASC14_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC14_W<'a> {
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
#[doc = "Field `ASC15` reader - Port analog switch control"]
pub struct ASC15_R(crate::FieldReader<bool, bool>);
impl ASC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC15` writer - Port analog switch control"]
pub struct ASC15_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC15_W<'a> {
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
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&mut self) -> ASC0_W {
        ASC0_W { w: self }
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&mut self) -> ASC1_W {
        ASC1_W { w: self }
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&mut self) -> ASC2_W {
        ASC2_W { w: self }
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&mut self) -> ASC3_W {
        ASC3_W { w: self }
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&mut self) -> ASC4_W {
        ASC4_W { w: self }
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&mut self) -> ASC5_W {
        ASC5_W { w: self }
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&mut self) -> ASC6_W {
        ASC6_W { w: self }
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&mut self) -> ASC7_W {
        ASC7_W { w: self }
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&mut self) -> ASC8_W {
        ASC8_W { w: self }
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&mut self) -> ASC9_W {
        ASC9_W { w: self }
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&mut self) -> ASC10_W {
        ASC10_W { w: self }
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&mut self) -> ASC11_W {
        ASC11_W { w: self }
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&mut self) -> ASC12_W {
        ASC12_W { w: self }
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&mut self) -> ASC13_W {
        ASC13_W { w: self }
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&mut self) -> ASC14_W {
        ASC14_W { w: self }
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&mut self) -> ASC15_W {
        ASC15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port analog switch control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr](index.html) module"]
pub struct ASCR_SPEC;
impl crate::RegisterSpec for ASCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ascr::R](R) reader structure"]
impl crate::Readable for ASCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ascr::W](W) writer structure"]
impl crate::Writable for ASCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASCR to value 0"]
impl crate::Resettable for ASCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
