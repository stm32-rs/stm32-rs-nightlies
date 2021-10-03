#[doc = "Register `RCC_MP_RSTSCLRR` reader"]
pub struct R(crate::R<RCC_MP_RSTSCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_RSTSCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_RSTSCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_RSTSCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_RSTSCLRR` writer"]
pub struct W(crate::W<RCC_MP_RSTSCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_RSTSCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_RSTSCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_RSTSCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORRSTF` reader - PORRSTF"]
pub struct PORRSTF_R(crate::FieldReader<bool, bool>);
impl PORRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORRSTF` writer - PORRSTF"]
pub struct PORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORRSTF_W<'a> {
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
#[doc = "Field `BORRSTF` reader - BORRSTF"]
pub struct BORRSTF_R(crate::FieldReader<bool, bool>);
impl BORRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BORRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORRSTF` writer - BORRSTF"]
pub struct BORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BORRSTF_W<'a> {
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
#[doc = "Field `PADRSTF` reader - PADRSTF"]
pub struct PADRSTF_R(crate::FieldReader<bool, bool>);
impl PADRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PADRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PADRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADRSTF` writer - PADRSTF"]
pub struct PADRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PADRSTF_W<'a> {
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
#[doc = "Field `HCSSRSTF` reader - HCSSRSTF"]
pub struct HCSSRSTF_R(crate::FieldReader<bool, bool>);
impl HCSSRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCSSRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCSSRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCSSRSTF` writer - HCSSRSTF"]
pub struct HCSSRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> HCSSRSTF_W<'a> {
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
#[doc = "Field `VCORERSTF` reader - VCORERSTF"]
pub struct VCORERSTF_R(crate::FieldReader<bool, bool>);
impl VCORERSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCORERSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCORERSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORERSTF` writer - VCORERSTF"]
pub struct VCORERSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORERSTF_W<'a> {
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
#[doc = "Field `MPSYSRSTF` reader - MPSYSRSTF"]
pub struct MPSYSRSTF_R(crate::FieldReader<bool, bool>);
impl MPSYSRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPSYSRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPSYSRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPSYSRSTF` writer - MPSYSRSTF"]
pub struct MPSYSRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSYSRSTF_W<'a> {
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
#[doc = "Field `MCSYSRSTF` reader - MCSYSRSTF"]
pub struct MCSYSRSTF_R(crate::FieldReader<bool, bool>);
impl MCSYSRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCSYSRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCSYSRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCSYSRSTF` writer - MCSYSRSTF"]
pub struct MCSYSRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCSYSRSTF_W<'a> {
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
#[doc = "Field `IWDG1RSTF` reader - IWDG1RSTF"]
pub struct IWDG1RSTF_R(crate::FieldReader<bool, bool>);
impl IWDG1RSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG1RSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG1RSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG1RSTF` writer - IWDG1RSTF"]
pub struct IWDG1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1RSTF_W<'a> {
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
#[doc = "Field `IWDG2RSTF` reader - IWDG2RSTF"]
pub struct IWDG2RSTF_R(crate::FieldReader<bool, bool>);
impl IWDG2RSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDG2RSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDG2RSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDG2RSTF` writer - IWDG2RSTF"]
pub struct IWDG2RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG2RSTF_W<'a> {
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
#[doc = "Field `STDBYRSTF` reader - STDBYRSTF"]
pub struct STDBYRSTF_R(crate::FieldReader<bool, bool>);
impl STDBYRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STDBYRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STDBYRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STDBYRSTF` writer - STDBYRSTF"]
pub struct STDBYRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> STDBYRSTF_W<'a> {
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
#[doc = "Field `CSTDBYRSTF` reader - CSTDBYRSTF"]
pub struct CSTDBYRSTF_R(crate::FieldReader<bool, bool>);
impl CSTDBYRSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSTDBYRSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTDBYRSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTDBYRSTF` writer - CSTDBYRSTF"]
pub struct CSTDBYRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTDBYRSTF_W<'a> {
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
#[doc = "Field `MPUP0RSTF` reader - MPUP0RSTF"]
pub struct MPUP0RSTF_R(crate::FieldReader<bool, bool>);
impl MPUP0RSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUP0RSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUP0RSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUP0RSTF` writer - MPUP0RSTF"]
pub struct MPUP0RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUP0RSTF_W<'a> {
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
#[doc = "Field `MPUP1RSTF` reader - MPUP1RSTF"]
pub struct MPUP1RSTF_R(crate::FieldReader<bool, bool>);
impl MPUP1RSTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUP1RSTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUP1RSTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUP1RSTF` writer - MPUP1RSTF"]
pub struct MPUP1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUP1RSTF_W<'a> {
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
#[doc = "Field `SPARE` reader - SPARE"]
pub struct SPARE_R(crate::FieldReader<bool, bool>);
impl SPARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE` writer - SPARE"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - STDBYRSTF"]
    #[inline(always)]
    pub fn stdbyrstf(&self) -> STDBYRSTF_R {
        STDBYRSTF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CSTDBYRSTF"]
    #[inline(always)]
    pub fn cstdbyrstf(&self) -> CSTDBYRSTF_R {
        CSTDBYRSTF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPUP0RSTF"]
    #[inline(always)]
    pub fn mpup0rstf(&self) -> MPUP0RSTF_R {
        MPUP0RSTF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPUP1RSTF"]
    #[inline(always)]
    pub fn mpup1rstf(&self) -> MPUP1RSTF_R {
        MPUP1RSTF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPARE"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W { w: self }
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W {
        BORRSTF_W { w: self }
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W {
        PADRSTF_W { w: self }
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W {
        HCSSRSTF_W { w: self }
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W {
        VCORERSTF_W { w: self }
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W {
        MPSYSRSTF_W { w: self }
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W {
        MCSYSRSTF_W { w: self }
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W {
        IWDG1RSTF_W { w: self }
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W {
        IWDG2RSTF_W { w: self }
    }
    #[doc = "Bit 11 - STDBYRSTF"]
    #[inline(always)]
    pub fn stdbyrstf(&mut self) -> STDBYRSTF_W {
        STDBYRSTF_W { w: self }
    }
    #[doc = "Bit 12 - CSTDBYRSTF"]
    #[inline(always)]
    pub fn cstdbyrstf(&mut self) -> CSTDBYRSTF_W {
        CSTDBYRSTF_W { w: self }
    }
    #[doc = "Bit 13 - MPUP0RSTF"]
    #[inline(always)]
    pub fn mpup0rstf(&mut self) -> MPUP0RSTF_W {
        MPUP0RSTF_W { w: self }
    }
    #[doc = "Bit 14 - MPUP1RSTF"]
    #[inline(always)]
    pub fn mpup1rstf(&mut self) -> MPUP1RSTF_W {
        MPUP1RSTF_W { w: self }
    }
    #[doc = "Bit 15 - SPARE"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_rstsclrr](index.html) module"]
pub struct RCC_MP_RSTSCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_RSTSCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_rstsclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_RSTSCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_rstsclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_RSTSCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_RSTSCLRR to value 0"]
impl crate::Resettable for RCC_MP_RSTSCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
