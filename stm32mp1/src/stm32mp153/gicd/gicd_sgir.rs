#[doc = "Register `GICD_SGIR` writer"]
pub struct W(crate::W<GICD_SGIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_SGIR_SPEC>;
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
impl From<crate::W<GICD_SGIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_SGIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGIINTID` writer - SGIINTID"]
pub struct SGIINTID_W<'a> {
    w: &'a mut W,
}
impl<'a> SGIINTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `NSATT` writer - NSATT"]
pub struct NSATT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSATT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CPUTARGETLIST` writer - CPUTARGETLIST"]
pub struct CPUTARGETLIST_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUTARGETLIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TARGETLISTFILTER` writer - TARGETLISTFILTER"]
pub struct TARGETLISTFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGETLISTFILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - SGIINTID"]
    #[inline(always)]
    pub fn sgiintid(&mut self) -> SGIINTID_W {
        SGIINTID_W { w: self }
    }
    #[doc = "Bit 15 - NSATT"]
    #[inline(always)]
    pub fn nsatt(&mut self) -> NSATT_W {
        NSATT_W { w: self }
    }
    #[doc = "Bits 16:17 - CPUTARGETLIST"]
    #[inline(always)]
    pub fn cputargetlist(&mut self) -> CPUTARGETLIST_W {
        CPUTARGETLIST_W { w: self }
    }
    #[doc = "Bits 24:25 - TARGETLISTFILTER"]
    #[inline(always)]
    pub fn targetlistfilter(&mut self) -> TARGETLISTFILTER_W {
        TARGETLISTFILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD software generated interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_sgir](index.html) module"]
pub struct GICD_SGIR_SPEC;
impl crate::RegisterSpec for GICD_SGIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gicd_sgir::W](W) writer structure"]
impl crate::Writable for GICD_SGIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_SGIR to value 0"]
impl crate::Resettable for GICD_SGIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
