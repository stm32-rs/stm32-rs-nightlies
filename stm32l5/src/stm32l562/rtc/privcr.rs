#[doc = "Register `PRIVCR` reader"]
pub struct R(crate::R<PRIVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCR` writer"]
pub struct W(crate::W<PRIVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCR_SPEC>;
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
impl From<crate::W<PRIVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV` reader - PRIV"]
pub struct PRIV_R(crate::FieldReader<bool, bool>);
impl PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV` writer - PRIV"]
pub struct PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_W<'a> {
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
#[doc = "Field `INITPRIV` reader - INITPRIV"]
pub struct INITPRIV_R(crate::FieldReader<bool, bool>);
impl INITPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITPRIV` writer - INITPRIV"]
pub struct INITPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INITPRIV_W<'a> {
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
#[doc = "Field `CALPRIV` reader - CALPRIV"]
pub struct CALPRIV_R(crate::FieldReader<bool, bool>);
impl CALPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALPRIV` writer - CALPRIV"]
pub struct CALPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CALPRIV_W<'a> {
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
#[doc = "Field `TSPRIV` reader - TSPRIV"]
pub struct TSPRIV_R(crate::FieldReader<bool, bool>);
impl TSPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSPRIV` writer - TSPRIV"]
pub struct TSPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSPRIV_W<'a> {
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
#[doc = "Field `WUTPRIV` reader - WUTPRIV"]
pub struct WUTPRIV_R(crate::FieldReader<bool, bool>);
impl WUTPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTPRIV` writer - WUTPRIV"]
pub struct WUTPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTPRIV_W<'a> {
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
#[doc = "Field `ALRBPRIV` reader - ALRBPRIV"]
pub struct ALRBPRIV_R(crate::FieldReader<bool, bool>);
impl ALRBPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBPRIV` writer - ALRBPRIV"]
pub struct ALRBPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBPRIV_W<'a> {
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
#[doc = "Field `ALRAPRIV` reader - ALRAPRIV"]
pub struct ALRAPRIV_R(crate::FieldReader<bool, bool>);
impl ALRAPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRAPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRAPRIV` writer - ALRAPRIV"]
pub struct ALRAPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAPRIV_W<'a> {
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
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - PRIV"]
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W {
        PRIV_W { w: self }
    }
    #[doc = "Bit 14 - INITPRIV"]
    #[inline(always)]
    pub fn initpriv(&mut self) -> INITPRIV_W {
        INITPRIV_W { w: self }
    }
    #[doc = "Bit 13 - CALPRIV"]
    #[inline(always)]
    pub fn calpriv(&mut self) -> CALPRIV_W {
        CALPRIV_W { w: self }
    }
    #[doc = "Bit 3 - TSPRIV"]
    #[inline(always)]
    pub fn tspriv(&mut self) -> TSPRIV_W {
        TSPRIV_W { w: self }
    }
    #[doc = "Bit 2 - WUTPRIV"]
    #[inline(always)]
    pub fn wutpriv(&mut self) -> WUTPRIV_W {
        WUTPRIV_W { w: self }
    }
    #[doc = "Bit 1 - ALRBPRIV"]
    #[inline(always)]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W {
        ALRBPRIV_W { w: self }
    }
    #[doc = "Bit 0 - ALRAPRIV"]
    #[inline(always)]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W {
        ALRAPRIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC privilege mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcr](index.html) module"]
pub struct PRIVCR_SPEC;
impl crate::RegisterSpec for PRIVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcr::R](R) reader structure"]
impl crate::Readable for PRIVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcr::W](W) writer structure"]
impl crate::Writable for PRIVCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIVCR to value 0"]
impl crate::Resettable for PRIVCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
