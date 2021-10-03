#[doc = "Register `IOPRSTR` reader"]
pub struct R(crate::R<IOPRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPRSTR` writer"]
pub struct W(crate::W<IOPRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPRSTR_SPEC>;
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
impl From<crate::W<IOPRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub struct IOPARST_R(crate::FieldReader<bool, bool>);
impl IOPARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPARST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPARST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub struct IOPARST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPARST_W<'a> {
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
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub struct IOPBRST_R(crate::FieldReader<bool, bool>);
impl IOPBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub struct IOPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBRST_W<'a> {
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
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub struct IOPCRST_R(crate::FieldReader<bool, bool>);
impl IOPCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub struct IOPCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCRST_W<'a> {
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
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub struct IOPDRST_R(crate::FieldReader<bool, bool>);
impl IOPDRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPDRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPDRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub struct IOPDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDRST_W<'a> {
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
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub struct IOPFRST_R(crate::FieldReader<bool, bool>);
impl IOPFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPFRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPFRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub struct IOPFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPFRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W {
        IOPARST_W { w: self }
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W {
        IOPBRST_W { w: self }
    }
    #[doc = "Bit 2 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W {
        IOPCRST_W { w: self }
    }
    #[doc = "Bit 3 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W {
        IOPDRST_W { w: self }
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IOPFRST_W {
        IOPFRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioprstr](index.html) module"]
pub struct IOPRSTR_SPEC;
impl crate::RegisterSpec for IOPRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioprstr::R](R) reader structure"]
impl crate::Readable for IOPRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioprstr::W](W) writer structure"]
impl crate::Writable for IOPRSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOPRSTR to value 0"]
impl crate::Resettable for IOPRSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
