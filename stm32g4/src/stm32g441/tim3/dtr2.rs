#[doc = "Register `DTR2` reader"]
pub struct R(crate::R<DTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTR2` writer"]
pub struct W(crate::W<DTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTR2_SPEC>;
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
impl From<crate::W<DTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTPE` reader - Deadtime Preload Enable"]
pub struct DTPE_R(crate::FieldReader<bool, bool>);
impl DTPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTPE` writer - Deadtime Preload Enable"]
pub struct DTPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPE_W<'a> {
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
#[doc = "Field `DTAE` reader - Deadtime Asymmetric Enable"]
pub struct DTAE_R(crate::FieldReader<bool, bool>);
impl DTAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTAE` writer - Deadtime Asymmetric Enable"]
pub struct DTAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTAE_W<'a> {
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
#[doc = "Field `DTGF` reader - Dead-time falling edge generator setup"]
pub struct DTGF_R(crate::FieldReader<u8, u8>);
impl DTGF_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTGF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTGF` writer - Dead-time falling edge generator setup"]
pub struct DTGF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Deadtime Preload Enable"]
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Deadtime Asymmetric Enable"]
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup"]
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 17 - Deadtime Preload Enable"]
    #[inline(always)]
    pub fn dtpe(&mut self) -> DTPE_W {
        DTPE_W { w: self }
    }
    #[doc = "Bit 16 - Deadtime Asymmetric Enable"]
    #[inline(always)]
    pub fn dtae(&mut self) -> DTAE_W {
        DTAE_W { w: self }
    }
    #[doc = "Bits 0:7 - Dead-time falling edge generator setup"]
    #[inline(always)]
    pub fn dtgf(&mut self) -> DTGF_W {
        DTGF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "timer Deadtime Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtr2](index.html) module"]
pub struct DTR2_SPEC;
impl crate::RegisterSpec for DTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtr2::R](R) reader structure"]
impl crate::Readable for DTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtr2::W](W) writer structure"]
impl crate::Writable for DTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTR2 to value 0"]
impl crate::Resettable for DTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
