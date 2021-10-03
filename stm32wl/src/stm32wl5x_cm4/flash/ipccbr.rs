#[doc = "Register `IPCCBR` reader"]
pub struct R(crate::R<IPCCBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCCBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCCBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCCBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCCBR` writer"]
pub struct W(crate::W<IPCCBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCCBR_SPEC>;
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
impl From<crate::W<IPCCBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCCBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPCCDBA` reader - IPCCDBA"]
pub struct IPCCDBA_R(crate::FieldReader<u16, u16>);
impl IPCCDBA_R {
    pub(crate) fn new(bits: u16) -> Self {
        IPCCDBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPCCDBA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPCCDBA` writer - IPCCDBA"]
pub struct IPCCDBA_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCDBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - IPCCDBA"]
    #[inline(always)]
    pub fn ipccdba(&self) -> IPCCDBA_R {
        IPCCDBA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - IPCCDBA"]
    #[inline(always)]
    pub fn ipccdba(&mut self) -> IPCCDBA_W {
        IPCCDBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash IPCC data buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipccbr](index.html) module"]
pub struct IPCCBR_SPEC;
impl crate::RegisterSpec for IPCCBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipccbr::R](R) reader structure"]
impl crate::Readable for IPCCBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipccbr::W](W) writer structure"]
impl crate::Writable for IPCCBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPCCBR to value 0xffff_ffff"]
impl crate::Resettable for IPCCBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
