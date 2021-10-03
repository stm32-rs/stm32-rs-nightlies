#[doc = "Register `RCC_USBCKSELR` reader"]
pub struct R(crate::R<RCC_USBCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_USBCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_USBCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_USBCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_USBCKSELR` writer"]
pub struct W(crate::W<RCC_USBCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_USBCKSELR_SPEC>;
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
impl From<crate::W<RCC_USBCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_USBCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBPHYSRC` reader - USBPHYSRC"]
pub struct USBPHYSRC_R(crate::FieldReader<u8, u8>);
impl USBPHYSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBPHYSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBPHYSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBPHYSRC` writer - USBPHYSRC"]
pub struct USBPHYSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `USBOSRC` reader - USBOSRC"]
pub struct USBOSRC_R(crate::FieldReader<bool, bool>);
impl USBOSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBOSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBOSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOSRC` writer - USBOSRC"]
pub struct USBOSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOSRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&self) -> USBPHYSRC_R {
        USBPHYSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&self) -> USBOSRC_R {
        USBOSRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&mut self) -> USBPHYSRC_W {
        USBPHYSRC_W { w: self }
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&mut self) -> USBOSRC_W {
        USBOSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_usbckselr](index.html) module"]
pub struct RCC_USBCKSELR_SPEC;
impl crate::RegisterSpec for RCC_USBCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_usbckselr::R](R) reader structure"]
impl crate::Readable for RCC_USBCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_usbckselr::W](W) writer structure"]
impl crate::Writable for RCC_USBCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_USBCKSELR to value 0"]
impl crate::Resettable for RCC_USBCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
