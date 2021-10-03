#[doc = "Register `DDRCTRL_RFSHCTL3` reader"]
pub struct R(crate::R<DDRCTRL_RFSHCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_RFSHCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_RFSHCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_RFSHCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_RFSHCTL3` writer"]
pub struct W(crate::W<DDRCTRL_RFSHCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_RFSHCTL3_SPEC>;
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
impl From<crate::W<DDRCTRL_RFSHCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_RFSHCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_AUTO_REFRESH` reader - DIS_AUTO_REFRESH"]
pub struct DIS_AUTO_REFRESH_R(crate::FieldReader<bool, bool>);
impl DIS_AUTO_REFRESH_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_AUTO_REFRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_AUTO_REFRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_AUTO_REFRESH` writer - DIS_AUTO_REFRESH"]
pub struct DIS_AUTO_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_AUTO_REFRESH_W<'a> {
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
#[doc = "Field `REFRESH_UPDATE_LEVEL` reader - REFRESH_UPDATE_LEVEL"]
pub struct REFRESH_UPDATE_LEVEL_R(crate::FieldReader<bool, bool>);
impl REFRESH_UPDATE_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFRESH_UPDATE_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESH_UPDATE_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESH_UPDATE_LEVEL` writer - REFRESH_UPDATE_LEVEL"]
pub struct REFRESH_UPDATE_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_UPDATE_LEVEL_W<'a> {
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
    #[doc = "Bit 0 - DIS_AUTO_REFRESH"]
    #[inline(always)]
    pub fn dis_auto_refresh(&self) -> DIS_AUTO_REFRESH_R {
        DIS_AUTO_REFRESH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - REFRESH_UPDATE_LEVEL"]
    #[inline(always)]
    pub fn refresh_update_level(&self) -> REFRESH_UPDATE_LEVEL_R {
        REFRESH_UPDATE_LEVEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIS_AUTO_REFRESH"]
    #[inline(always)]
    pub fn dis_auto_refresh(&mut self) -> DIS_AUTO_REFRESH_W {
        DIS_AUTO_REFRESH_W { w: self }
    }
    #[doc = "Bit 1 - REFRESH_UPDATE_LEVEL"]
    #[inline(always)]
    pub fn refresh_update_level(&mut self) -> REFRESH_UPDATE_LEVEL_W {
        REFRESH_UPDATE_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL refresh control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_rfshctl3](index.html) module"]
pub struct DDRCTRL_RFSHCTL3_SPEC;
impl crate::RegisterSpec for DDRCTRL_RFSHCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_rfshctl3::R](R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_rfshctl3::W](W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_RFSHCTL3 to value 0"]
impl crate::Resettable for DDRCTRL_RFSHCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
