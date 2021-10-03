#[doc = "Register `MACA2LR` reader"]
pub struct R(crate::R<MACA2LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA2LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA2LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA2LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA2LR` writer"]
pub struct W(crate::W<MACA2LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA2LR_SPEC>;
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
impl From<crate::W<MACA2LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA2LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA2L` reader - MACA2L"]
pub struct MACA2L_R(crate::FieldReader<u32, u32>);
impl MACA2L_R {
    pub(crate) fn new(bits: u32) -> Self {
        MACA2L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACA2L_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACA2L` writer - MACA2L"]
pub struct MACA2L_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA2L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - MACA2L"]
    #[inline(always)]
    pub fn maca2l(&self) -> MACA2L_R {
        MACA2L_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - MACA2L"]
    #[inline(always)]
    pub fn maca2l(&mut self) -> MACA2L_W {
        MACA2L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 2 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca2lr](index.html) module"]
pub struct MACA2LR_SPEC;
impl crate::RegisterSpec for MACA2LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca2lr::R](R) reader structure"]
impl crate::Readable for MACA2LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca2lr::W](W) writer structure"]
impl crate::Writable for MACA2LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA2LR to value 0xffff_ffff"]
impl crate::Resettable for MACA2LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
