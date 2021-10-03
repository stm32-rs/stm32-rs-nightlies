#[doc = "Register `OTG_FS_GPWRDN` reader"]
pub struct R(crate::R<OTG_FS_GPWRDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GPWRDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GPWRDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GPWRDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GPWRDN` writer"]
pub struct W(crate::W<OTG_FS_GPWRDN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GPWRDN_SPEC>;
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
impl From<crate::W<OTG_FS_GPWRDN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GPWRDN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADPMEN` reader - ADP module enable"]
pub struct ADPMEN_R(crate::FieldReader<bool, bool>);
impl ADPMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPMEN` writer - ADP module enable"]
pub struct ADPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPMEN_W<'a> {
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
#[doc = "Field `ADPIF` reader - ADP interrupt flag"]
pub struct ADPIF_R(crate::FieldReader<bool, bool>);
impl ADPIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPIF` writer - ADP interrupt flag"]
pub struct ADPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&self) -> ADPMEN_R {
        ADPMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&self) -> ADPIF_R {
        ADPIF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&mut self) -> ADPMEN_W {
        ADPMEN_W { w: self }
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&mut self) -> ADPIF_W {
        ADPIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG power down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gpwrdn](index.html) module"]
pub struct OTG_FS_GPWRDN_SPEC;
impl crate::RegisterSpec for OTG_FS_GPWRDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_gpwrdn::R](R) reader structure"]
impl crate::Readable for OTG_FS_GPWRDN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_gpwrdn::W](W) writer structure"]
impl crate::Writable for OTG_FS_GPWRDN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GPWRDN to value 0x0200_0400"]
impl crate::Resettable for OTG_FS_GPWRDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
