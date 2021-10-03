#[doc = "Register `DDRCTRL_HWLPCTL` reader"]
pub struct R(crate::R<DDRCTRL_HWLPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_HWLPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_HWLPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_HWLPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_HWLPCTL` writer"]
pub struct W(crate::W<DDRCTRL_HWLPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_HWLPCTL_SPEC>;
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
impl From<crate::W<DDRCTRL_HWLPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_HWLPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HW_LP_EN` reader - HW_LP_EN"]
pub struct HW_LP_EN_R(crate::FieldReader<bool, bool>);
impl HW_LP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HW_LP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_LP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_LP_EN` writer - HW_LP_EN"]
pub struct HW_LP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LP_EN_W<'a> {
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
#[doc = "Field `HW_LP_EXIT_IDLE_EN` reader - HW_LP_EXIT_IDLE_EN"]
pub struct HW_LP_EXIT_IDLE_EN_R(crate::FieldReader<bool, bool>);
impl HW_LP_EXIT_IDLE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HW_LP_EXIT_IDLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_LP_EXIT_IDLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_LP_EXIT_IDLE_EN` writer - HW_LP_EXIT_IDLE_EN"]
pub struct HW_LP_EXIT_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LP_EXIT_IDLE_EN_W<'a> {
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
#[doc = "Field `HW_LP_IDLE_X32` reader - HW_LP_IDLE_X32"]
pub struct HW_LP_IDLE_X32_R(crate::FieldReader<u16, u16>);
impl HW_LP_IDLE_X32_R {
    pub(crate) fn new(bits: u16) -> Self {
        HW_LP_IDLE_X32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_LP_IDLE_X32_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_LP_IDLE_X32` writer - HW_LP_IDLE_X32"]
pub struct HW_LP_IDLE_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LP_IDLE_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    pub fn hw_lp_en(&self) -> HW_LP_EN_R {
        HW_LP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&self) -> HW_LP_EXIT_IDLE_EN_R {
        HW_LP_EXIT_IDLE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    pub fn hw_lp_idle_x32(&self) -> HW_LP_IDLE_X32_R {
        HW_LP_IDLE_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    pub fn hw_lp_en(&mut self) -> HW_LP_EN_W {
        HW_LP_EN_W { w: self }
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&mut self) -> HW_LP_EXIT_IDLE_EN_W {
        HW_LP_EXIT_IDLE_EN_W { w: self }
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    pub fn hw_lp_idle_x32(&mut self) -> HW_LP_IDLE_X32_W {
        HW_LP_IDLE_X32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL hardware low power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_hwlpctl](index.html) module"]
pub struct DDRCTRL_HWLPCTL_SPEC;
impl crate::RegisterSpec for DDRCTRL_HWLPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_hwlpctl::R](R) reader structure"]
impl crate::Readable for DDRCTRL_HWLPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_hwlpctl::W](W) writer structure"]
impl crate::Writable for DDRCTRL_HWLPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_HWLPCTL to value 0x03"]
impl crate::Resettable for DDRCTRL_HWLPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
