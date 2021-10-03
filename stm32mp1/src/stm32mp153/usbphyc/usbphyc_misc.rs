#[doc = "Register `USBPHYC_MISC` reader"]
pub struct R(crate::R<USBPHYC_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYC_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYC_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYC_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHYC_MISC` writer"]
pub struct W(crate::W<USBPHYC_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYC_MISC_SPEC>;
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
impl From<crate::W<USBPHYC_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYC_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWITHOST` reader - SWITHOST"]
pub struct SWITHOST_R(crate::FieldReader<bool, bool>);
impl SWITHOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWITHOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITHOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITHOST` writer - SWITHOST"]
pub struct SWITHOST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITHOST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PPCKDIS` reader - PPCKDIS"]
pub struct PPCKDIS_R(crate::FieldReader<u8, u8>);
impl PPCKDIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPCKDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCKDIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCKDIS` writer - PPCKDIS"]
pub struct PPCKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCKDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SWITHOST"]
    #[inline(always)]
    pub fn swithost(&self) -> SWITHOST_R {
        SWITHOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - PPCKDIS"]
    #[inline(always)]
    pub fn ppckdis(&self) -> PPCKDIS_R {
        PPCKDIS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SWITHOST"]
    #[inline(always)]
    pub fn swithost(&mut self) -> SWITHOST_W {
        SWITHOST_W { w: self }
    }
    #[doc = "Bits 1:2 - PPCKDIS"]
    #[inline(always)]
    pub fn ppckdis(&mut self) -> PPCKDIS_W {
        PPCKDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the switch between controllers for the HS PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_misc](index.html) module"]
pub struct USBPHYC_MISC_SPEC;
impl crate::RegisterSpec for USBPHYC_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphyc_misc::R](R) reader structure"]
impl crate::Readable for USBPHYC_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphyc_misc::W](W) writer structure"]
impl crate::Writable for USBPHYC_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPHYC_MISC to value 0"]
impl crate::Resettable for USBPHYC_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
