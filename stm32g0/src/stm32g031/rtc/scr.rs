#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALRAF` reader - CALRAF"]
pub struct CALRAF_R(crate::FieldReader<bool, bool>);
impl CALRAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALRAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALRAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALRAF` writer - CALRAF"]
pub struct CALRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRAF_W<'a> {
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
#[doc = "Field `CALRBF` reader - CALRBF"]
pub struct CALRBF_R(crate::FieldReader<bool, bool>);
impl CALRBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALRBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALRBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALRBF` writer - CALRBF"]
pub struct CALRBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRBF_W<'a> {
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
#[doc = "Field `CWUTF` reader - CWUTF"]
pub struct CWUTF_R(crate::FieldReader<bool, bool>);
impl CWUTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWUTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUTF` writer - CWUTF"]
pub struct CWUTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUTF_W<'a> {
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
#[doc = "Field `CTSF` reader - CTSF"]
pub struct CTSF_R(crate::FieldReader<bool, bool>);
impl CTSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSF` writer - CTSF"]
pub struct CTSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSF_W<'a> {
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
#[doc = "Field `CTSOVF` reader - CTSOVF"]
pub struct CTSOVF_R(crate::FieldReader<bool, bool>);
impl CTSOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSOVF` writer - CTSOVF"]
pub struct CTSOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSOVF_W<'a> {
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
#[doc = "Field `CITSF` reader - CITSF"]
pub struct CITSF_R(crate::FieldReader<bool, bool>);
impl CITSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CITSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CITSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CITSF` writer - CITSF"]
pub struct CITSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CITSF_W<'a> {
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
    #[doc = "Bit 0 - CALRAF"]
    #[inline(always)]
    pub fn calraf(&self) -> CALRAF_R {
        CALRAF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CALRBF"]
    #[inline(always)]
    pub fn calrbf(&self) -> CALRBF_R {
        CALRBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CWUTF"]
    #[inline(always)]
    pub fn cwutf(&self) -> CWUTF_R {
        CWUTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CTSF"]
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTSOVF"]
    #[inline(always)]
    pub fn ctsovf(&self) -> CTSOVF_R {
        CTSOVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CITSF"]
    #[inline(always)]
    pub fn citsf(&self) -> CITSF_R {
        CITSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CALRAF"]
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W {
        CALRAF_W { w: self }
    }
    #[doc = "Bit 1 - CALRBF"]
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W {
        CALRBF_W { w: self }
    }
    #[doc = "Bit 2 - CWUTF"]
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W {
        CWUTF_W { w: self }
    }
    #[doc = "Bit 3 - CTSF"]
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W {
        CTSF_W { w: self }
    }
    #[doc = "Bit 4 - CTSOVF"]
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W {
        CTSOVF_W { w: self }
    }
    #[doc = "Bit 5 - CITSF"]
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W {
        CITSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
