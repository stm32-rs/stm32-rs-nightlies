#[doc = "Register `MACA0HR` reader"]
pub struct R(crate::R<MACA0HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA0HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA0HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA0HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA0HR` writer"]
pub struct W(crate::W<MACA0HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA0HR_SPEC>;
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
impl From<crate::W<MACA0HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA0HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRHI` reader - MAC Address0\\[47:32\\]"]
pub struct ADDRHI_R(crate::FieldReader<u16, u16>);
impl ADDRHI_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDRHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRHI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRHI` writer - MAC Address0\\[47:32\\]"]
pub struct ADDRHI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `AE` reader - Address Enable"]
pub struct AE_R(crate::FieldReader<bool, bool>);
impl AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - MAC Address0\\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0\\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W {
        ADDRHI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address 0 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca0hr](index.html) module"]
pub struct MACA0HR_SPEC;
impl crate::RegisterSpec for MACA0HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca0hr::R](R) reader structure"]
impl crate::Readable for MACA0HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca0hr::W](W) writer structure"]
impl crate::Writable for MACA0HR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA0HR to value 0x8000_ffff"]
impl crate::Resettable for MACA0HR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_ffff
    }
}
