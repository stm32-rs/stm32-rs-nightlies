#[doc = "Register `C1_APB1HLPENR` reader"]
pub struct R(crate::R<C1_APB1HLPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB1HLPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB1HLPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB1HLPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB1HLPENR` writer"]
pub struct W(crate::W<C1_APB1HLPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB1HLPENR_SPEC>;
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
impl From<crate::W<C1_APB1HLPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB1HLPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Recovery System peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSLPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<CRSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSLPEN` reader - Clock Recovery System peripheral clock enable during CSleep mode"]
pub struct CRSLPEN_R(crate::FieldReader<bool, CRSLPEN_A>);
impl CRSLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSLPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSLPEN_A {
        match self.bits {
            false => CRSLPEN_A::DISABLED,
            true => CRSLPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CRSLPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CRSLPEN_A::ENABLED
    }
}
impl core::ops::Deref for CRSLPEN_R {
    type Target = crate::FieldReader<bool, CRSLPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSLPEN` writer - Clock Recovery System peripheral clock enable during CSleep mode"]
pub struct CRSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRSLPEN_A::ENABLED)
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
#[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub type SWPMILPEN_A = CRSLPEN_A;
#[doc = "Field `SWPMILPEN` reader - SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub type SWPMILPEN_R = CRSLPEN_R;
#[doc = "Field `SWPMILPEN` writer - SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub struct SWPMILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPMILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPMILPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWPMILPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWPMILPEN_A::ENABLED)
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
#[doc = "OPAMP peripheral clock enable during CSleep mode"]
pub type OPAMPLPEN_A = CRSLPEN_A;
#[doc = "Field `OPAMPLPEN` reader - OPAMP peripheral clock enable during CSleep mode"]
pub type OPAMPLPEN_R = CRSLPEN_R;
#[doc = "Field `OPAMPLPEN` writer - OPAMP peripheral clock enable during CSleep mode"]
pub struct OPAMPLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAMPLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAMPLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAMPLPEN_A::ENABLED)
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
#[doc = "MDIOS peripheral clock enable during CSleep mode"]
pub type MDIOSLPEN_A = CRSLPEN_A;
#[doc = "Field `MDIOSLPEN` reader - MDIOS peripheral clock enable during CSleep mode"]
pub type MDIOSLPEN_R = CRSLPEN_R;
#[doc = "Field `MDIOSLPEN` writer - MDIOS peripheral clock enable during CSleep mode"]
pub struct MDIOSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIOSLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDIOSLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MDIOSLPEN_A::ENABLED)
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
#[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub type FDCANLPEN_A = CRSLPEN_A;
#[doc = "Field `FDCANLPEN` reader - FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub type FDCANLPEN_R = CRSLPEN_R;
#[doc = "Field `FDCANLPEN` writer - FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub struct FDCANLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FDCANLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FDCANLPEN_A::ENABLED)
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
#[doc = "TIM23 block enable during CSleep Mode"]
pub type TIM23LPEN_A = CRSLPEN_A;
#[doc = "Field `TIM23LPEN` reader - TIM23 block enable during CSleep Mode"]
pub type TIM23LPEN_R = CRSLPEN_R;
#[doc = "Field `TIM23LPEN` writer - TIM23 block enable during CSleep Mode"]
pub struct TIM23LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM23LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM23LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM23LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM23LPEN_A::ENABLED)
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
#[doc = "TIM24 block enable during CSleep Mode"]
pub type TIM24LPEN_A = CRSLPEN_A;
#[doc = "Field `TIM24LPEN` reader - TIM24 block enable during CSleep Mode"]
pub type TIM24LPEN_R = CRSLPEN_R;
#[doc = "Field `TIM24LPEN` writer - TIM24 block enable during CSleep Mode"]
pub struct TIM24LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM24LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM24LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM24LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM24LPEN_A::ENABLED)
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
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn swpmilpen(&self) -> SWPMILPEN_R {
        SWPMILPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TIM23 block enable during CSleep Mode"]
    #[inline(always)]
    pub fn tim23lpen(&self) -> TIM23LPEN_R {
        TIM23LPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TIM24 block enable during CSleep Mode"]
    #[inline(always)]
    pub fn tim24lpen(&self) -> TIM24LPEN_R {
        TIM24LPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crslpen(&mut self) -> CRSLPEN_W {
        CRSLPEN_W { w: self }
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn swpmilpen(&mut self) -> SWPMILPEN_W {
        SWPMILPEN_W { w: self }
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W {
        OPAMPLPEN_W { w: self }
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W {
        MDIOSLPEN_W { w: self }
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W {
        FDCANLPEN_W { w: self }
    }
    #[doc = "Bit 24 - TIM23 block enable during CSleep Mode"]
    #[inline(always)]
    pub fn tim23lpen(&mut self) -> TIM23LPEN_W {
        TIM23LPEN_W { w: self }
    }
    #[doc = "Bit 25 - TIM24 block enable during CSleep Mode"]
    #[inline(always)]
    pub fn tim24lpen(&mut self) -> TIM24LPEN_W {
        TIM24LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 High Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1hlpenr](index.html) module"]
pub struct C1_APB1HLPENR_SPEC;
impl crate::RegisterSpec for C1_APB1HLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb1hlpenr::R](R) reader structure"]
impl crate::Readable for C1_APB1HLPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb1hlpenr::W](W) writer structure"]
impl crate::Writable for C1_APB1HLPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB1HLPENR to value 0"]
impl crate::Resettable for C1_APB1HLPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
