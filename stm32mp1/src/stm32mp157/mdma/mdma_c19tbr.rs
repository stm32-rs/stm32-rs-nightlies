#[doc = "Register `MDMA_C19TBR` reader"]
pub struct R(crate::R<MDMA_C19TBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C19TBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C19TBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C19TBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C19TBR` writer"]
pub struct W(crate::W<MDMA_C19TBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C19TBR_SPEC>;
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
impl From<crate::W<MDMA_C19TBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C19TBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEL` reader - TSEL"]
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
#[doc = "Field `TSEL` writer - TSEL"]
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
#[doc = "Field `SBUS` reader - SBUS"]
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
#[doc = "Field `SBUS` writer - SBUS"]
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
#[doc = "Field `DBUS` reader - DBUS"]
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
#[doc = "Field `DBUS` writer - DBUS"]
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
    #[doc = "Bits 0:5 - TSEL"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SBUS"]
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DBUS"]
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - TSEL"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W {
        TSEL_W { w: self }
    }
    #[doc = "Bit 16 - SBUS"]
    #[inline(always)]
    pub fn sbus(&mut self) -> SBUS_W {
        SBUS_W { w: self }
    }
    #[doc = "Bit 17 - DBUS"]
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
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x18).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c19tbr](index.html) module"]
pub struct MDMA_C19TBR_SPEC;
impl crate::RegisterSpec for MDMA_C19TBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c19tbr::R](R) reader structure"]
impl crate::Readable for MDMA_C19TBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c19tbr::W](W) writer structure"]
impl crate::Writable for MDMA_C19TBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMA_C19TBR to value 0"]
impl crate::Resettable for MDMA_C19TBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
