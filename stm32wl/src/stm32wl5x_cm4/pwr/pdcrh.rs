#[doc = "Register `PDCRH` reader"]
pub struct R(crate::R<PDCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRH` writer"]
pub struct W(crate::W<PDCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRH_SPEC>;
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
impl From<crate::W<PDCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD3_A {
    #[doc = "0: Disable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    DISABLED = 0,
    #[doc = "1: Enable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    ENABLED = 1,
}
impl From<PD3_A> for bool {
    #[inline(always)]
    fn from(variant: PD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD3` reader - pull-down"]
pub struct PD3_R(crate::FieldReader<bool, PD3_A>);
impl PD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD3_A {
        match self.bits {
            false => PD3_A::DISABLED,
            true => PD3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PD3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PD3_A::ENABLED
    }
}
impl core::ops::Deref for PD3_R {
    type Target = crate::FieldReader<bool, PD3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD3` writer - pull-down"]
pub struct PD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD3_A::DISABLED)
    }
    #[doc = "Enable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD3_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - pull-down"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - pull-down"]
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port H pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrh](index.html) module"]
pub struct PDCRH_SPEC;
impl crate::RegisterSpec for PDCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcrh::R](R) reader structure"]
impl crate::Readable for PDCRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcrh::W](W) writer structure"]
impl crate::Writable for PDCRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDCRH to value 0"]
impl crate::Resettable for PDCRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
