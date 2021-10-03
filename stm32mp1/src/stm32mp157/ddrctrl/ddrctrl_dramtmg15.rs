#[doc = "Register `DDRCTRL_DRAMTMG15` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG15` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG15_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_STAB_X32` reader - T_STAB_X32"]
pub struct T_STAB_X32_R(crate::FieldReader<u8, u8>);
impl T_STAB_X32_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_STAB_X32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_STAB_X32_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_STAB_X32` writer - T_STAB_X32"]
pub struct T_STAB_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> T_STAB_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `EN_DFI_LP_T_STAB` reader - EN_DFI_LP_T_STAB"]
pub struct EN_DFI_LP_T_STAB_R(crate::FieldReader<bool, bool>);
impl EN_DFI_LP_T_STAB_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_DFI_LP_T_STAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_DFI_LP_T_STAB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_DFI_LP_T_STAB` writer - EN_DFI_LP_T_STAB"]
pub struct EN_DFI_LP_T_STAB_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_DFI_LP_T_STAB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    pub fn t_stab_x32(&self) -> T_STAB_X32_R {
        T_STAB_X32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&self) -> EN_DFI_LP_T_STAB_R {
        EN_DFI_LP_T_STAB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - T_STAB_X32"]
    #[inline(always)]
    pub fn t_stab_x32(&mut self) -> T_STAB_X32_W {
        T_STAB_X32_W { w: self }
    }
    #[doc = "Bit 31 - EN_DFI_LP_T_STAB"]
    #[inline(always)]
    pub fn en_dfi_lp_t_stab(&mut self) -> EN_DFI_LP_T_STAB_W {
        EN_DFI_LP_T_STAB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg15](index.html) module"]
pub struct DDRCTRL_DRAMTMG15_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg15::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg15::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG15 to value 0"]
impl crate::Resettable for DDRCTRL_DRAMTMG15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
