#[doc = "Register `VDSSA` reader"]
pub struct R(crate::R<VDSSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDSSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDSSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDSSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDSSA` writer"]
pub struct W(crate::W<VDSSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDSSA_SPEC>;
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
impl From<crate::W<VDSSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDSSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD` reader - Volatile data segment start address"]
pub struct ADD_R(crate::FieldReader<u16, u16>);
impl ADD_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD` writer - Volatile data segment start address"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | ((value as u32 & 0x03ff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Volatile data segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdssa](index.html) module"]
pub struct VDSSA_SPEC;
impl crate::RegisterSpec for VDSSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdssa::R](R) reader structure"]
impl crate::Readable for VDSSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdssa::W](W) writer structure"]
impl crate::Writable for VDSSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDSSA to value 0"]
impl crate::Resettable for VDSSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
