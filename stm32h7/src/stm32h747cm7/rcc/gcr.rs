#[doc = "Register `GCR` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WWDG1 reset scope control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WW1RSC_A {
    #[doc = "0: Clear WWDG1 scope control"]
    CLEAR = 0,
    #[doc = "1: Set WWDG1 scope control"]
    SET = 1,
}
impl From<WW1RSC_A> for bool {
    #[inline(always)]
    fn from(variant: WW1RSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WW1RSC` reader - WWDG1 reset scope control"]
pub struct WW1RSC_R(crate::FieldReader<bool, WW1RSC_A>);
impl WW1RSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WW1RSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WW1RSC_A {
        match self.bits {
            false => WW1RSC_A::CLEAR,
            true => WW1RSC_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == WW1RSC_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == WW1RSC_A::SET
    }
}
impl core::ops::Deref for WW1RSC_R {
    type Target = crate::FieldReader<bool, WW1RSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WW1RSC` writer - WWDG1 reset scope control"]
pub struct WW1RSC_W<'a> {
    w: &'a mut W,
}
impl<'a> WW1RSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WW1RSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear WWDG1 scope control"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WW1RSC_A::CLEAR)
    }
    #[doc = "Set WWDG1 scope control"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(WW1RSC_A::SET)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `WW2RSC` reader - WWDG2 reset scope control"]
pub struct WW2RSC_R(crate::FieldReader<bool, bool>);
impl WW2RSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WW2RSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WW2RSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WW2RSC` writer - WWDG2 reset scope control"]
pub struct WW2RSC_W<'a> {
    w: &'a mut W,
}
impl<'a> WW2RSC_W<'a> {
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
#[doc = "Field `BOOT_C1` reader - Force allow CPU1 to boot"]
pub struct BOOT_C1_R(crate::FieldReader<bool, bool>);
impl BOOT_C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_C1` writer - Force allow CPU1 to boot"]
pub struct BOOT_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_C1_W<'a> {
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
#[doc = "Field `BOOT_C2` reader - Force allow CPU2 to boot"]
pub struct BOOT_C2_R(crate::FieldReader<bool, bool>);
impl BOOT_C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_C2` writer - Force allow CPU2 to boot"]
pub struct BOOT_C2_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_C2_W<'a> {
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
    #[doc = "Bit 0 - WWDG1 reset scope control"]
    #[inline(always)]
    pub fn ww1rsc(&self) -> WW1RSC_R {
        WW1RSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WWDG2 reset scope control"]
    #[inline(always)]
    pub fn ww2rsc(&self) -> WW2RSC_R {
        WW2RSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force allow CPU1 to boot"]
    #[inline(always)]
    pub fn boot_c1(&self) -> BOOT_C1_R {
        BOOT_C1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force allow CPU2 to boot"]
    #[inline(always)]
    pub fn boot_c2(&self) -> BOOT_C2_R {
        BOOT_C2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDG1 reset scope control"]
    #[inline(always)]
    pub fn ww1rsc(&mut self) -> WW1RSC_W {
        WW1RSC_W { w: self }
    }
    #[doc = "Bit 1 - WWDG2 reset scope control"]
    #[inline(always)]
    pub fn ww2rsc(&mut self) -> WW2RSC_W {
        WW2RSC_W { w: self }
    }
    #[doc = "Bit 2 - Force allow CPU1 to boot"]
    #[inline(always)]
    pub fn boot_c1(&mut self) -> BOOT_C1_W {
        BOOT_C1_W { w: self }
    }
    #[doc = "Bit 3 - Force allow CPU2 to boot"]
    #[inline(always)]
    pub fn boot_c2(&mut self) -> BOOT_C2_W {
        BOOT_C2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
