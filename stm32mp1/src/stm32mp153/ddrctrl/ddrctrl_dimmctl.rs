#[doc = "Register `DDRCTRL_DIMMCTL` reader"]
pub struct R(crate::R<DDRCTRL_DIMMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DIMMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DIMMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DIMMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DIMMCTL` writer"]
pub struct W(crate::W<DDRCTRL_DIMMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DIMMCTL_SPEC>;
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
impl From<crate::W<DDRCTRL_DIMMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DIMMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIMM_STAGGER_CS_EN` reader - DIMM_STAGGER_CS_EN"]
pub struct DIMM_STAGGER_CS_EN_R(crate::FieldReader<bool, bool>);
impl DIMM_STAGGER_CS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIMM_STAGGER_CS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIMM_STAGGER_CS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIMM_STAGGER_CS_EN` writer - DIMM_STAGGER_CS_EN"]
pub struct DIMM_STAGGER_CS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIMM_STAGGER_CS_EN_W<'a> {
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
#[doc = "Field `DIMM_ADDR_MIRR_EN` reader - DIMM_ADDR_MIRR_EN"]
pub struct DIMM_ADDR_MIRR_EN_R(crate::FieldReader<bool, bool>);
impl DIMM_ADDR_MIRR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIMM_ADDR_MIRR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIMM_ADDR_MIRR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIMM_ADDR_MIRR_EN` writer - DIMM_ADDR_MIRR_EN"]
pub struct DIMM_ADDR_MIRR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIMM_ADDR_MIRR_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DIMM_STAGGER_CS_EN"]
    #[inline(always)]
    pub fn dimm_stagger_cs_en(&self) -> DIMM_STAGGER_CS_EN_R {
        DIMM_STAGGER_CS_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIMM_ADDR_MIRR_EN"]
    #[inline(always)]
    pub fn dimm_addr_mirr_en(&self) -> DIMM_ADDR_MIRR_EN_R {
        DIMM_ADDR_MIRR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIMM_STAGGER_CS_EN"]
    #[inline(always)]
    pub fn dimm_stagger_cs_en(&mut self) -> DIMM_STAGGER_CS_EN_W {
        DIMM_STAGGER_CS_EN_W { w: self }
    }
    #[doc = "Bit 1 - DIMM_ADDR_MIRR_EN"]
    #[inline(always)]
    pub fn dimm_addr_mirr_en(&mut self) -> DIMM_ADDR_MIRR_EN_W {
        DIMM_ADDR_MIRR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DIMM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dimmctl](index.html) module"]
pub struct DDRCTRL_DIMMCTL_SPEC;
impl crate::RegisterSpec for DDRCTRL_DIMMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dimmctl::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DIMMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dimmctl::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DIMMCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DIMMCTL to value 0"]
impl crate::Resettable for DDRCTRL_DIMMCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
