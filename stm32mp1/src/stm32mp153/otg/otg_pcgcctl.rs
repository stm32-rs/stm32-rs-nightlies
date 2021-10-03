#[doc = "Register `OTG_PCGCCTL` reader"]
pub struct R(crate::R<OTG_PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_PCGCCTL` writer"]
pub struct W(crate::W<OTG_PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_PCGCCTL_SPEC>;
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
impl From<crate::W<OTG_PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPPCLK` reader - STPPCLK"]
pub struct STPPCLK_R(crate::FieldReader<bool, bool>);
impl STPPCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPPCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPPCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPPCLK` writer - STPPCLK"]
pub struct STPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STPPCLK_W<'a> {
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
#[doc = "Field `GATEHCLK` reader - GATEHCLK"]
pub struct GATEHCLK_R(crate::FieldReader<bool, bool>);
impl GATEHCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GATEHCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GATEHCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GATEHCLK` writer - GATEHCLK"]
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PHYSUSP` reader - PHYSUSP"]
pub struct PHYSUSP_R(crate::FieldReader<bool, bool>);
impl PHYSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENL1GTG` reader - ENL1GTG"]
pub struct ENL1GTG_R(crate::FieldReader<bool, bool>);
impl ENL1GTG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENL1GTG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENL1GTG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENL1GTG` writer - ENL1GTG"]
pub struct ENL1GTG_W<'a> {
    w: &'a mut W,
}
impl<'a> ENL1GTG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PHYSLEEP` reader - PHYSLEEP"]
pub struct PHYSLEEP_R(crate::FieldReader<bool, bool>);
impl PHYSLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYSLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYSLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` reader - SUSP"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHYSUSP"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PHYSLEEP"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W {
        STPPCLK_W { w: self }
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W {
        ENL1GTG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is available in host and device modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_pcgcctl](index.html) module"]
pub struct OTG_PCGCCTL_SPEC;
impl crate::RegisterSpec for OTG_PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_pcgcctl::R](R) reader structure"]
impl crate::Readable for OTG_PCGCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_pcgcctl::W](W) writer structure"]
impl crate::Writable for OTG_PCGCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_PCGCCTL to value 0x200b_8000"]
impl crate::Resettable for OTG_PCGCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x200b_8000
    }
}
