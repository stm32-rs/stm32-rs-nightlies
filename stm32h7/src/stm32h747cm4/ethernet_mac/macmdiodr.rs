#[doc = "Register `MACMDIODR` reader"]
pub struct R(crate::R<MACMDIODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMDIODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMDIODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMDIODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACMDIODR` writer"]
pub struct W(crate::W<MACMDIODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMDIODR_SPEC>;
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
impl From<crate::W<MACMDIODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMDIODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - MII Data"]
pub struct MD_R(crate::FieldReader<u16, u16>);
impl MD_R {
    pub(crate) fn new(bits: u16) -> Self {
        MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD` writer - MII Data"]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `RA` reader - Register Address"]
pub struct RA_R(crate::FieldReader<u16, u16>);
impl RA_R {
    pub(crate) fn new(bits: u16) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Register Address"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Register Address"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
    #[doc = "Bits 16:31 - Register Address"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmdiodr](index.html) module"]
pub struct MACMDIODR_SPEC;
impl crate::RegisterSpec for MACMDIODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macmdiodr::R](R) reader structure"]
impl crate::Readable for MACMDIODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macmdiodr::W](W) writer structure"]
impl crate::Writable for MACMDIODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACMDIODR to value 0"]
impl crate::Resettable for MACMDIODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
