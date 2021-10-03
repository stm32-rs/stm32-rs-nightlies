#[doc = "Register `SYSCFG_BOOTR` reader"]
pub struct R(crate::R<SYSCFG_BOOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_BOOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_BOOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_BOOTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_BOOTR` writer"]
pub struct W(crate::W<SYSCFG_BOOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_BOOTR_SPEC>;
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
impl From<crate::W<SYSCFG_BOOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_BOOTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT0` reader - BOOT0"]
pub struct BOOT0_R(crate::FieldReader<bool, bool>);
impl BOOT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT1` reader - BOOT1"]
pub struct BOOT1_R(crate::FieldReader<bool, bool>);
impl BOOT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT2` reader - BOOT2"]
pub struct BOOT2_R(crate::FieldReader<bool, bool>);
impl BOOT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT0_PD` reader - BOOT0_PD"]
pub struct BOOT0_PD_R(crate::FieldReader<bool, bool>);
impl BOOT0_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT0_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT0_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT0_PD` writer - BOOT0_PD"]
pub struct BOOT0_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT0_PD_W<'a> {
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
#[doc = "Field `BOOT1_PD` reader - BOOT1_PD"]
pub struct BOOT1_PD_R(crate::FieldReader<bool, bool>);
impl BOOT1_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT1_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT1_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT1_PD` writer - BOOT1_PD"]
pub struct BOOT1_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT1_PD_W<'a> {
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
#[doc = "Field `BOOT2_PD` reader - BOOT2_PD"]
pub struct BOOT2_PD_R(crate::FieldReader<bool, bool>);
impl BOOT2_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT2_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT2_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT2_PD` writer - BOOT2_PD"]
pub struct BOOT2_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT2_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BOOT0"]
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOOT1"]
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BOOT2"]
    #[inline(always)]
    pub fn boot2(&self) -> BOOT2_R {
        BOOT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BOOT0_PD"]
    #[inline(always)]
    pub fn boot0_pd(&self) -> BOOT0_PD_R {
        BOOT0_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BOOT1_PD"]
    #[inline(always)]
    pub fn boot1_pd(&self) -> BOOT1_PD_R {
        BOOT1_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOOT2_PD"]
    #[inline(always)]
    pub fn boot2_pd(&self) -> BOOT2_PD_R {
        BOOT2_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - BOOT0_PD"]
    #[inline(always)]
    pub fn boot0_pd(&mut self) -> BOOT0_PD_W {
        BOOT0_PD_W { w: self }
    }
    #[doc = "Bit 5 - BOOT1_PD"]
    #[inline(always)]
    pub fn boot1_pd(&mut self) -> BOOT1_PD_W {
        BOOT1_PD_W { w: self }
    }
    #[doc = "Bit 6 - BOOT2_PD"]
    #[inline(always)]
    pub fn boot2_pd(&mut self) -> BOOT2_PD_W {
        BOOT2_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_bootr](index.html) module"]
pub struct SYSCFG_BOOTR_SPEC;
impl crate::RegisterSpec for SYSCFG_BOOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_bootr::R](R) reader structure"]
impl crate::Readable for SYSCFG_BOOTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_bootr::W](W) writer structure"]
impl crate::Writable for SYSCFG_BOOTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG_BOOTR to value 0"]
impl crate::Resettable for SYSCFG_BOOTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
