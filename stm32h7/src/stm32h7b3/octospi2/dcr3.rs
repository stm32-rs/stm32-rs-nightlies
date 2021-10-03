#[doc = "Register `DCR3` reader"]
pub struct R(crate::R<DCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR3` writer"]
pub struct W(crate::W<DCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR3_SPEC>;
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
impl From<crate::W<DCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXTRAN` reader - Maximum transfer"]
pub struct MAXTRAN_R(crate::FieldReader<u8, u8>);
impl MAXTRAN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAXTRAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXTRAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXTRAN` writer - Maximum transfer"]
pub struct MAXTRAN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXTRAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CSBOUND` reader - CS boundary"]
pub struct CSBOUND_R(crate::FieldReader<u8, u8>);
impl CSBOUND_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSBOUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSBOUND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSBOUND` writer - CS boundary"]
pub struct CSBOUND_W<'a> {
    w: &'a mut W,
}
impl<'a> CSBOUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum transfer"]
    #[inline(always)]
    pub fn maxtran(&self) -> MAXTRAN_R {
        MAXTRAN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - CS boundary"]
    #[inline(always)]
    pub fn csbound(&self) -> CSBOUND_R {
        CSBOUND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum transfer"]
    #[inline(always)]
    pub fn maxtran(&mut self) -> MAXTRAN_W {
        MAXTRAN_W { w: self }
    }
    #[doc = "Bits 16:20 - CS boundary"]
    #[inline(always)]
    pub fn csbound(&mut self) -> CSBOUND_W {
        CSBOUND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr3](index.html) module"]
pub struct DCR3_SPEC;
impl crate::RegisterSpec for DCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr3::R](R) reader structure"]
impl crate::Readable for DCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr3::W](W) writer structure"]
impl crate::Writable for DCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCR3 to value 0"]
impl crate::Resettable for DCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
