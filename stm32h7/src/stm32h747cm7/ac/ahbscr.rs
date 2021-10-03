#[doc = "Register `AHBSCR` reader"]
pub struct R(crate::R<AHBSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBSCR` writer"]
pub struct W(crate::W<AHBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBSCR_SPEC>;
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
impl From<crate::W<AHBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTL` reader - CTL"]
pub struct CTL_R(crate::FieldReader<u8, u8>);
impl CTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTL` writer - CTL"]
pub struct CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TPRI` reader - TPRI"]
pub struct TPRI_R(crate::FieldReader<u16, u16>);
impl TPRI_R {
    pub(crate) fn new(bits: u16) -> Self {
        TPRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPRI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPRI` writer - TPRI"]
pub struct TPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> TPRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | ((value as u32 & 0x01ff) << 2);
        self.w
    }
}
#[doc = "Field `INITCOUNT` reader - INITCOUNT"]
pub struct INITCOUNT_R(crate::FieldReader<u8, u8>);
impl INITCOUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        INITCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITCOUNT` writer - INITCOUNT"]
pub struct INITCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INITCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CTL"]
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:10 - TPRI"]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:15 - INITCOUNT"]
    #[inline(always)]
    pub fn initcount(&self) -> INITCOUNT_R {
        INITCOUNT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTL"]
    #[inline(always)]
    pub fn ctl(&mut self) -> CTL_W {
        CTL_W { w: self }
    }
    #[doc = "Bits 2:10 - TPRI"]
    #[inline(always)]
    pub fn tpri(&mut self) -> TPRI_W {
        TPRI_W { w: self }
    }
    #[doc = "Bits 11:15 - INITCOUNT"]
    #[inline(always)]
    pub fn initcount(&mut self) -> INITCOUNT_W {
        INITCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Slave Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbscr](index.html) module"]
pub struct AHBSCR_SPEC;
impl crate::RegisterSpec for AHBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbscr::R](R) reader structure"]
impl crate::Readable for AHBSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbscr::W](W) writer structure"]
impl crate::Writable for AHBSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBSCR to value 0"]
impl crate::Resettable for AHBSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
