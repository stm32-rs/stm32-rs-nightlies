#[doc = "Register `TBR` reader"]
pub struct R(crate::R<TBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBR` writer"]
pub struct W(crate::W<TBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBR_SPEC>;
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
impl From<crate::W<TBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEL` reader - Trigger selection"]
pub struct TSEL_R(crate::FieldReader<u8, u8>);
impl TSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL` writer - Trigger selection"]
pub struct TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `SBUS` reader - Source BUS select This bit is protected and can be written only if EN is 0."]
pub struct SBUS_R(crate::FieldReader<bool, bool>);
impl SBUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBUS` writer - Source BUS select This bit is protected and can be written only if EN is 0."]
pub struct SBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBUS_W<'a> {
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
#[doc = "Field `DBUS` reader - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub struct DBUS_R(crate::FieldReader<bool, bool>);
impl DBUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS` writer - Destination BUS slect This bit is protected and can be written only if EN is 0."]
pub struct DBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger selection"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W {
        TSEL_W { w: self }
    }
    #[doc = "Bit 16 - Source BUS select This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn sbus(&mut self) -> SBUS_W {
        SBUS_W { w: self }
    }
    #[doc = "Bit 17 - Destination BUS slect This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn dbus(&mut self) -> DBUS_W {
        DBUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbr](index.html) module"]
pub struct TBR_SPEC;
impl crate::RegisterSpec for TBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbr::R](R) reader structure"]
impl crate::Readable for TBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbr::W](W) writer structure"]
impl crate::Writable for TBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBR to value 0"]
impl crate::Resettable for TBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
