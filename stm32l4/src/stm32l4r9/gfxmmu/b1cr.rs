#[doc = "Register `B1CR` reader"]
pub struct R(crate::R<B1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B1CR` writer"]
pub struct W(crate::W<B1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B1CR_SPEC>;
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
impl From<crate::W<B1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBO` reader - Physical buffer offset"]
pub struct PBO_R(crate::FieldReader<u32, u32>);
impl PBO_R {
    pub(crate) fn new(bits: u32) -> Self {
        PBO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBO` writer - Physical buffer offset"]
pub struct PBO_W<'a> {
    w: &'a mut W,
}
impl<'a> PBO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 4)) | ((value as u32 & 0x0007_ffff) << 4);
        self.w
    }
}
#[doc = "Field `PBBA` reader - Physical buffer base address"]
pub struct PBBA_R(crate::FieldReader<u16, u16>);
impl PBBA_R {
    pub(crate) fn new(bits: u16) -> Self {
        PBBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBBA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBBA` writer - Physical buffer base address"]
pub struct PBBA_W<'a> {
    w: &'a mut W,
}
impl<'a> PBBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | ((value as u32 & 0x01ff) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:22 - Physical buffer offset"]
    #[inline(always)]
    pub fn pbo(&self) -> PBO_R {
        PBO_R::new(((self.bits >> 4) & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 23:31 - Physical buffer base address"]
    #[inline(always)]
    pub fn pbba(&self) -> PBBA_R {
        PBBA_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:22 - Physical buffer offset"]
    #[inline(always)]
    pub fn pbo(&mut self) -> PBO_W {
        PBO_W { w: self }
    }
    #[doc = "Bits 23:31 - Physical buffer base address"]
    #[inline(always)]
    pub fn pbba(&mut self) -> PBBA_W {
        PBBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU buffer 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b1cr](index.html) module"]
pub struct B1CR_SPEC;
impl crate::RegisterSpec for B1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b1cr::R](R) reader structure"]
impl crate::Readable for B1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b1cr::W](W) writer structure"]
impl crate::Writable for B1CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets B1CR to value 0"]
impl crate::Resettable for B1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
