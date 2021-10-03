#[doc = "Register `UR15` reader"]
pub struct R(crate::R<UR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR15` writer"]
pub struct W(crate::W<UR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR15_SPEC>;
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
impl From<crate::W<UR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D2STPRST` reader - D2 Stop Reset"]
pub struct D2STPRST_R(crate::FieldReader<bool, bool>);
impl D2STPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        D2STPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D2STPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D2STPRST` writer - D2 Stop Reset"]
pub struct D2STPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> D2STPRST_W<'a> {
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
#[doc = "Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode"]
pub struct FZIWDGSTB_R(crate::FieldReader<bool, bool>);
impl FZIWDGSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FZIWDGSTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FZIWDGSTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    pub fn d2stprst(&self) -> D2STPRST_R {
        D2STPRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Freeze independent watchdog in Standby mode"]
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    pub fn d2stprst(&mut self) -> D2STPRST_W {
        D2STPRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur15](index.html) module"]
pub struct UR15_SPEC;
impl crate::RegisterSpec for UR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur15::R](R) reader structure"]
impl crate::Readable for UR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur15::W](W) writer structure"]
impl crate::Writable for UR15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR15 to value 0"]
impl crate::Resettable for UR15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
