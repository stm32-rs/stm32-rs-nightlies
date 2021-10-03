#[doc = "Register `DDRCTRL_PCCFG` reader"]
pub struct R(crate::R<DDRCTRL_PCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCCFG` writer"]
pub struct W(crate::W<DDRCTRL_PCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCCFG_SPEC>;
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
impl From<crate::W<DDRCTRL_PCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GO2CRITICAL_EN` reader - GO2CRITICAL_EN"]
pub struct GO2CRITICAL_EN_R(crate::FieldReader<bool, bool>);
impl GO2CRITICAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GO2CRITICAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GO2CRITICAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GO2CRITICAL_EN` writer - GO2CRITICAL_EN"]
pub struct GO2CRITICAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GO2CRITICAL_EN_W<'a> {
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
#[doc = "Field `PAGEMATCH_LIMIT` reader - PAGEMATCH_LIMIT"]
pub struct PAGEMATCH_LIMIT_R(crate::FieldReader<bool, bool>);
impl PAGEMATCH_LIMIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGEMATCH_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGEMATCH_LIMIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGEMATCH_LIMIT` writer - PAGEMATCH_LIMIT"]
pub struct PAGEMATCH_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGEMATCH_LIMIT_W<'a> {
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
#[doc = "Field `BL_EXP_MODE` reader - BL_EXP_MODE"]
pub struct BL_EXP_MODE_R(crate::FieldReader<bool, bool>);
impl BL_EXP_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BL_EXP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BL_EXP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BL_EXP_MODE` writer - BL_EXP_MODE"]
pub struct BL_EXP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_EXP_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    pub fn go2critical_en(&self) -> GO2CRITICAL_EN_R {
        GO2CRITICAL_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    pub fn pagematch_limit(&self) -> PAGEMATCH_LIMIT_R {
        PAGEMATCH_LIMIT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    pub fn bl_exp_mode(&self) -> BL_EXP_MODE_R {
        BL_EXP_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    pub fn go2critical_en(&mut self) -> GO2CRITICAL_EN_W {
        GO2CRITICAL_EN_W { w: self }
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    pub fn pagematch_limit(&mut self) -> PAGEMATCH_LIMIT_W {
        PAGEMATCH_LIMIT_W { w: self }
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    pub fn bl_exp_mode(&mut self) -> BL_EXP_MODE_W {
        BL_EXP_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pccfg](index.html) module"]
pub struct DDRCTRL_PCCFG_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pccfg::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pccfg::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_PCCFG to value 0"]
impl crate::Resettable for DDRCTRL_PCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
