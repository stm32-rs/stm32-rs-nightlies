#[doc = "Register `L1WVPCR` reader"]
pub struct R(crate::R<L1WVPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1WVPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1WVPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1WVPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1WVPCR` writer"]
pub struct W(crate::W<L1WVPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1WVPCR_SPEC>;
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
impl From<crate::W<L1WVPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1WVPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVSTPOS` reader - Window Vertical Start Position"]
pub struct WVSTPOS_R(crate::FieldReader<u16, u16>);
impl WVSTPOS_R {
    pub(crate) fn new(bits: u16) -> Self {
        WVSTPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVSTPOS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WVSTPOS` writer - Window Vertical Start Position"]
pub struct WVSTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSTPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `WVSPPOS` reader - Window Vertical Stop Position"]
pub struct WVSPPOS_R(crate::FieldReader<u16, u16>);
impl WVSPPOS_R {
    pub(crate) fn new(bits: u16) -> Self {
        WVSPPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVSPPOS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WVSPPOS` writer - Window Vertical Stop Position"]
pub struct WVSPPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSPPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W {
        WVSTPOS_W { w: self }
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W {
        WVSPPOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Window Vertical Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1wvpcr](index.html) module"]
pub struct L1WVPCR_SPEC;
impl crate::RegisterSpec for L1WVPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1wvpcr::R](R) reader structure"]
impl crate::Readable for L1WVPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1wvpcr::W](W) writer structure"]
impl crate::Writable for L1WVPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1WVPCR to value 0"]
impl crate::Resettable for L1WVPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
