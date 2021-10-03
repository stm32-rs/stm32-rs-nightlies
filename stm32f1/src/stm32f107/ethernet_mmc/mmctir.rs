#[doc = "Register `MMCTIR` reader"]
pub struct R(crate::R<MMCTIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCTIR` writer"]
pub struct W(crate::W<MMCTIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTIR_SPEC>;
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
impl From<crate::W<MMCTIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TGFSCS` reader - Transmitted good frames single collision status"]
pub struct TGFSCS_R(crate::FieldReader<bool, bool>);
impl TGFSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFSCS` writer - Transmitted good frames single collision status"]
pub struct TGFSCS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFSCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TGFMSCS` reader - Transmitted good frames more than single collision status"]
pub struct TGFMSCS_R(crate::FieldReader<bool, bool>);
impl TGFMSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFMSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFMSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFMSCS` writer - Transmitted good frames more than single collision status"]
pub struct TGFMSCS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFMSCS_W<'a> {
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
#[doc = "Field `TGFS` reader - Transmitted good frames status"]
pub struct TGFS_R(crate::FieldReader<bool, bool>);
impl TGFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFS` writer - Transmitted good frames status"]
pub struct TGFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&mut self) -> TGFSCS_W {
        TGFSCS_W { w: self }
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&mut self) -> TGFMSCS_W {
        TGFMSCS_W { w: self }
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&mut self) -> TGFS_W {
        TGFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC transmit interrupt register (ETH_MMCTIR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctir](index.html) module"]
pub struct MMCTIR_SPEC;
impl crate::RegisterSpec for MMCTIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctir::R](R) reader structure"]
impl crate::Readable for MMCTIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmctir::W](W) writer structure"]
impl crate::Writable for MMCTIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCTIR to value 0"]
impl crate::Resettable for MMCTIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
