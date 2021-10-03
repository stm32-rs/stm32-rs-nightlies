#[doc = "Register `LDO` reader"]
pub struct R(crate::R<LDO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO` writer"]
pub struct W(crate::W<LDO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_SPEC>;
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
impl From<crate::W<LDO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDO_USED` reader - Indicates the presence of the LDO in the chip"]
pub struct LDO_USED_R(crate::FieldReader<bool, bool>);
impl LDO_USED_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO_USED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_USED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_STATUS` reader - Monitors the status of the PHY's LDO"]
pub struct LDO_STATUS_R(crate::FieldReader<bool, bool>);
impl LDO_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DISABLE` reader - Controls disable of the High Speed PHY's LDO"]
pub struct LDO_DISABLE_R(crate::FieldReader<bool, bool>);
impl LDO_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DISABLE` writer - Controls disable of the High Speed PHY's LDO"]
pub struct LDO_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates the presence of the LDO in the chip"]
    #[inline(always)]
    pub fn ldo_used(&self) -> LDO_USED_R {
        LDO_USED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Monitors the status of the PHY's LDO"]
    #[inline(always)]
    pub fn ldo_status(&self) -> LDO_STATUS_R {
        LDO_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls disable of the High Speed PHY's LDO"]
    #[inline(always)]
    pub fn ldo_disable(&self) -> LDO_DISABLE_R {
        LDO_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Controls disable of the High Speed PHY's LDO"]
    #[inline(always)]
    pub fn ldo_disable(&mut self) -> LDO_DISABLE_W {
        LDO_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBPHYC LDO control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo](index.html) module"]
pub struct LDO_SPEC;
impl crate::RegisterSpec for LDO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo::R](R) reader structure"]
impl crate::Readable for LDO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo::W](W) writer structure"]
impl crate::Writable for LDO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDO to value 0x01"]
impl crate::Resettable for LDO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
