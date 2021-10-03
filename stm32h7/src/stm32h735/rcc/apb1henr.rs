#[doc = "Register `APB1HENR` reader"]
pub struct R(crate::R<APB1HENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HENR` writer"]
pub struct W(crate::W<APB1HENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HENR_SPEC>;
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
impl From<crate::W<APB1HENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HENR_SPEC>) -> Self {
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
pub type SWPMIEN_A = CRSEN_A;
#[doc = "Field `SWPMIEN` reader - SWPMI Peripheral Clocks Enable"]
pub type SWPMIEN_R = CRSEN_R;
#[doc = "Field `SWPMIEN` writer - SWPMI Peripheral Clocks Enable"]
pub struct SWPMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPMIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWPMIEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWPMIEN_A::ENABLED)
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
#[doc = "TIM23 block enable"]
pub type TIM23EN_A = CRSEN_A;
#[doc = "Field `TIM23EN` reader - TIM23 block enable"]
pub type TIM23EN_R = CRSEN_R;
#[doc = "Field `TIM23EN` writer - TIM23 block enable"]
pub struct TIM23EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM23EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM23EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM23EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM23EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "TIM24 block enable"]
pub type TIM24EN_A = CRSEN_A;
#[doc = "Field `TIM24EN` reader - TIM24 block enable"]
pub type TIM24EN_R = CRSEN_R;
#[doc = "Field `TIM24EN` writer - TIM24 block enable"]
pub struct TIM24EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM24EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM24EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM24EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM24EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
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
    pub fn swpmien(&self) -> SWPMIEN_R {
        SWPMIEN_R::new(((self.bits >> 2) & 0x01) != 0)
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
    #[doc = "Bit 24 - TIM23 block enable"]
    #[inline(always)]
    pub fn tim23en(&self) -> TIM23EN_R {
        TIM23EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TIM24 block enable"]
    #[inline(always)]
    pub fn tim24en(&self) -> TIM24EN_R {
        TIM24EN_R::new(((self.bits >> 25) & 0x01) != 0)
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
    pub fn swpmien(&mut self) -> SWPMIEN_W {
        SWPMIEN_W { w: self }
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
    #[doc = "Bit 24 - TIM23 block enable"]
    #[inline(always)]
    pub fn tim23en(&mut self) -> TIM23EN_W {
        TIM23EN_W { w: self }
    }
    #[doc = "Bit 25 - TIM24 block enable"]
    #[inline(always)]
    pub fn tim24en(&mut self) -> TIM24EN_W {
        TIM24EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1henr](index.html) module"]
pub struct APB1HENR_SPEC;
impl crate::RegisterSpec for APB1HENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1henr::R](R) reader structure"]
impl crate::Readable for APB1HENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1henr::W](W) writer structure"]
impl crate::Writable for APB1HENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1HENR to value 0"]
impl crate::Resettable for APB1HENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
