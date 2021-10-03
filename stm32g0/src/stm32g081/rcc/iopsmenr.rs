#[doc = "Register `IOPSMENR` reader"]
pub struct R(crate::R<IOPSMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPSMENR` writer"]
pub struct W(crate::W<IOPSMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMENR_SPEC>;
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
impl From<crate::W<IOPSMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOPASMEN` reader - I/O port A clock enable during Sleep mode"]
pub struct IOPASMEN_R(crate::FieldReader<bool, bool>);
impl IOPASMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPASMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPASMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPASMEN` writer - I/O port A clock enable during Sleep mode"]
pub struct IOPASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPASMEN_W<'a> {
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
#[doc = "Field `IOPBSMEN` reader - I/O port B clock enable during Sleep mode"]
pub struct IOPBSMEN_R(crate::FieldReader<bool, bool>);
impl IOPBSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPBSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPBSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPBSMEN` writer - I/O port B clock enable during Sleep mode"]
pub struct IOPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBSMEN_W<'a> {
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
#[doc = "Field `IOPCSMEN` reader - I/O port C clock enable during Sleep mode"]
pub struct IOPCSMEN_R(crate::FieldReader<bool, bool>);
impl IOPCSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPCSMEN` writer - I/O port C clock enable during Sleep mode"]
pub struct IOPCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCSMEN_W<'a> {
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
#[doc = "Field `IOPDSMEN` reader - I/O port D clock enable during Sleep mode"]
pub struct IOPDSMEN_R(crate::FieldReader<bool, bool>);
impl IOPDSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPDSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPDSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPDSMEN` writer - I/O port D clock enable during Sleep mode"]
pub struct IOPDSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDSMEN_W<'a> {
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
#[doc = "Field `IOPFSMEN` reader - I/O port F clock enable during Sleep mode"]
pub struct IOPFSMEN_R(crate::FieldReader<bool, bool>);
impl IOPFSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPFSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOPFSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPFSMEN` writer - I/O port F clock enable during Sleep mode"]
pub struct IOPFSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPFSMEN_W<'a> {
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
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopfsmen(&self) -> IOPFSMEN_R {
        IOPFSMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopasmen(&mut self) -> IOPASMEN_W {
        IOPASMEN_W { w: self }
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W {
        IOPBSMEN_W { w: self }
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W {
        IOPCSMEN_W { w: self }
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W {
        IOPDSMEN_W { w: self }
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopfsmen(&mut self) -> IOPFSMEN_W {
        IOPFSMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO in Sleep mode clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmenr](index.html) module"]
pub struct IOPSMENR_SPEC;
impl crate::RegisterSpec for IOPSMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopsmenr::R](R) reader structure"]
impl crate::Readable for IOPSMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopsmenr::W](W) writer structure"]
impl crate::Writable for IOPSMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOPSMENR to value 0"]
impl crate::Resettable for IOPSMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
