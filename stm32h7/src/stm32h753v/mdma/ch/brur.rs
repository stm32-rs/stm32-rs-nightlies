#[doc = "Register `BRUR` reader"]
pub struct R(crate::R<BRUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRUR` writer"]
pub struct W(crate::W<BRUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRUR_SPEC>;
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
impl From<crate::W<BRUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUV` reader - source adresse update value"]
pub struct SUV_R(crate::FieldReader<u16, u16>);
impl SUV_R {
    pub(crate) fn new(bits: u16) -> Self {
        SUV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUV` writer - source adresse update value"]
pub struct SUV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DUV` reader - destination address update"]
pub struct DUV_R(crate::FieldReader<u16, u16>);
impl DUV_R {
    pub(crate) fn new(bits: u16) -> Self {
        DUV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUV` writer - destination address update"]
pub struct DUV_W<'a> {
    w: &'a mut W,
}
impl<'a> DUV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    pub fn suv(&mut self) -> SUV_W {
        SUV_W { w: self }
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    pub fn duv(&mut self) -> DUV_W {
        DUV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brur](index.html) module"]
pub struct BRUR_SPEC;
impl crate::RegisterSpec for BRUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brur::R](R) reader structure"]
impl crate::Readable for BRUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brur::W](W) writer structure"]
impl crate::Writable for BRUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRUR to value 0"]
impl crate::Resettable for BRUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
