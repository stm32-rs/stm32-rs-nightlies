#[doc = "Register `C1_APB1HENR` reader"]
pub struct R(crate::R<C1_APB1HENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB1HENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB1HENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB1HENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB1HENR` writer"]
pub struct W(crate::W<C1_APB1HENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB1HENR_SPEC>;
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
impl From<crate::W<C1_APB1HENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB1HENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Recovery System peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSEN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<CRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSEN` reader - Clock Recovery System peripheral clock enable"]
pub struct CRSEN_R(crate::FieldReader<bool, CRSEN_A>);
impl CRSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSEN_A {
        match self.bits {
            false => CRSEN_A::DISABLED,
            true => CRSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CRSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CRSEN_A::ENABLED
    }
}
impl core::ops::Deref for CRSEN_R {
    type Target = crate::FieldReader<bool, CRSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSEN` writer - Clock Recovery System peripheral clock enable"]
pub struct CRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "SWPMI Peripheral Clocks Enable"]
pub type SWPEN_A = CRSEN_A;
#[doc = "Field `SWPEN` reader - SWPMI Peripheral Clocks Enable"]
pub type SWPEN_R = CRSEN_R;
#[doc = "Field `SWPEN` writer - SWPMI Peripheral Clocks Enable"]
pub struct SWPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "OPAMP peripheral clock enable"]
pub type OPAMPEN_A = CRSEN_A;
#[doc = "Field `OPAMPEN` reader - OPAMP peripheral clock enable"]
pub type OPAMPEN_R = CRSEN_R;
#[doc = "Field `OPAMPEN` writer - OPAMP peripheral clock enable"]
pub struct OPAMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAMPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAMPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAMPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "MDIOS peripheral clock enable"]
pub type MDIOSEN_A = CRSEN_A;
#[doc = "Field `MDIOSEN` reader - MDIOS peripheral clock enable"]
pub type MDIOSEN_R = CRSEN_R;
#[doc = "Field `MDIOSEN` writer - MDIOS peripheral clock enable"]
pub struct MDIOSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIOSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDIOSEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MDIOSEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "FDCAN Peripheral Clocks Enable"]
pub type FDCANEN_A = CRSEN_A;
#[doc = "Field `FDCANEN` reader - FDCAN Peripheral Clocks Enable"]
pub type FDCANEN_R = CRSEN_R;
#[doc = "Field `FDCANEN` writer - FDCAN Peripheral Clocks Enable"]
pub struct FDCANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FDCANEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FDCANEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn swpen(&self) -> SWPEN_R {
        SWPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    pub fn mdiosen(&self) -> MDIOSEN_R {
        MDIOSEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W {
        CRSEN_W { w: self }
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn swpen(&mut self) -> SWPEN_W {
        SWPEN_W { w: self }
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    pub fn opampen(&mut self) -> OPAMPEN_W {
        OPAMPEN_W { w: self }
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    pub fn mdiosen(&mut self) -> MDIOSEN_W {
        MDIOSEN_W { w: self }
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W {
        FDCANEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1henr](index.html) module"]
pub struct C1_APB1HENR_SPEC;
impl crate::RegisterSpec for C1_APB1HENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb1henr::R](R) reader structure"]
impl crate::Readable for C1_APB1HENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb1henr::W](W) writer structure"]
impl crate::Writable for C1_APB1HENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB1HENR to value 0"]
impl crate::Resettable for C1_APB1HENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
