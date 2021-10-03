#[doc = "Register `MDIER` reader"]
pub struct R(crate::R<MDIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIER` writer"]
pub struct W(crate::W<MDIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIER_SPEC>;
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
impl From<crate::W<MDIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MUPDDE"]
pub type MUPDDE_A = MCMP1DE_A;
#[doc = "Field `MUPDDE` reader - MUPDDE"]
pub type MUPDDE_R = MCMP1DE_R;
#[doc = "Field `MUPDDE` writer - MUPDDE"]
pub struct MUPDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUPDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUPDDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUPDDE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUPDDE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "SYNCDE"]
pub type SYNCDE_A = MCMP1DE_A;
#[doc = "Field `SYNCDE` reader - SYNCDE"]
pub type SYNCDE_R = MCMP1DE_R;
#[doc = "Field `SYNCDE` writer - SYNCDE"]
pub struct SYNCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCDE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCDE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "MREPDE"]
pub type MREPDE_A = MCMP1DE_A;
#[doc = "Field `MREPDE` reader - MREPDE"]
pub type MREPDE_R = MCMP1DE_R;
#[doc = "Field `MREPDE` writer - MREPDE"]
pub struct MREPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREPDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MREPDE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MREPDE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "MCMP4DE"]
pub type MCMP4DE_A = MCMP1DE_A;
#[doc = "Field `MCMP4DE` reader - MCMP4DE"]
pub type MCMP4DE_R = MCMP1DE_R;
#[doc = "Field `MCMP4DE` writer - MCMP4DE"]
pub struct MCMP4DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP4DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP4DE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP4DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "MCMP3DE"]
pub type MCMP3DE_A = MCMP1DE_A;
#[doc = "Field `MCMP3DE` reader - MCMP3DE"]
pub type MCMP3DE_R = MCMP1DE_R;
#[doc = "Field `MCMP3DE` writer - MCMP3DE"]
pub struct MCMP3DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP3DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP3DE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP3DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "MCMP2DE"]
pub type MCMP2DE_A = MCMP1DE_A;
#[doc = "Field `MCMP2DE` reader - MCMP2DE"]
pub type MCMP2DE_R = MCMP1DE_R;
#[doc = "Field `MCMP2DE` writer - MCMP2DE"]
pub struct MCMP2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP2DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP2DE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP2DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "MCMP1DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCMP1DE_A {
    #[doc = "0: DMA request disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request enabled"]
    ENABLED = 1,
}
impl From<MCMP1DE_A> for bool {
    #[inline(always)]
    fn from(variant: MCMP1DE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCMP1DE` reader - MCMP1DE"]
pub struct MCMP1DE_R(crate::FieldReader<bool, MCMP1DE_A>);
impl MCMP1DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP1DE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCMP1DE_A {
        match self.bits {
            false => MCMP1DE_A::DISABLED,
            true => MCMP1DE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MCMP1DE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MCMP1DE_A::ENABLED
    }
}
impl core::ops::Deref for MCMP1DE_R {
    type Target = crate::FieldReader<bool, MCMP1DE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP1DE` writer - MCMP1DE"]
pub struct MCMP1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1DE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP1DE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP1DE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP1DE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "MUPDIE"]
pub type MUPDIE_A = MCMP1IE_A;
#[doc = "Field `MUPDIE` reader - MUPDIE"]
pub type MUPDIE_R = MCMP1IE_R;
#[doc = "Field `MUPDIE` writer - MUPDIE"]
pub struct MUPDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUPDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUPDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUPDIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUPDIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "SYNCIE"]
pub type SYNCIE_A = MCMP1IE_A;
#[doc = "Field `SYNCIE` reader - SYNCIE"]
pub type SYNCIE_R = MCMP1IE_R;
#[doc = "Field `SYNCIE` writer - SYNCIE"]
pub struct SYNCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCIE_A::ENABLED)
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
#[doc = "MREPIE"]
pub type MREPIE_A = MCMP1IE_A;
#[doc = "Field `MREPIE` reader - MREPIE"]
pub type MREPIE_R = MCMP1IE_R;
#[doc = "Field `MREPIE` writer - MREPIE"]
pub struct MREPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MREPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MREPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MREPIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MREPIE_A::ENABLED)
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
#[doc = "MCMP4IE"]
pub type MCMP4IE_A = MCMP1IE_A;
#[doc = "Field `MCMP4IE` reader - MCMP4IE"]
pub type MCMP4IE_R = MCMP1IE_R;
#[doc = "Field `MCMP4IE` writer - MCMP4IE"]
pub struct MCMP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP4IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP4IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP4IE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP4IE_A::ENABLED)
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
#[doc = "MCMP3IE"]
pub type MCMP3IE_A = MCMP1IE_A;
#[doc = "Field `MCMP3IE` reader - MCMP3IE"]
pub type MCMP3IE_R = MCMP1IE_R;
#[doc = "Field `MCMP3IE` writer - MCMP3IE"]
pub struct MCMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP3IE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP3IE_A::ENABLED)
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
#[doc = "MCMP2IE"]
pub type MCMP2IE_A = MCMP1IE_A;
#[doc = "Field `MCMP2IE` reader - MCMP2IE"]
pub type MCMP2IE_R = MCMP1IE_R;
#[doc = "Field `MCMP2IE` writer - MCMP2IE"]
pub struct MCMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP2IE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP2IE_A::ENABLED)
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
#[doc = "MCMP1IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCMP1IE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<MCMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: MCMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCMP1IE` reader - MCMP1IE"]
pub struct MCMP1IE_R(crate::FieldReader<bool, MCMP1IE_A>);
impl MCMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCMP1IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCMP1IE_A {
        match self.bits {
            false => MCMP1IE_A::DISABLED,
            true => MCMP1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MCMP1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MCMP1IE_A::ENABLED
    }
}
impl core::ops::Deref for MCMP1IE_R {
    type Target = crate::FieldReader<bool, MCMP1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCMP1IE` writer - MCMP1IE"]
pub struct MCMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMP1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCMP1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP1IE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP1IE_A::ENABLED)
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
impl R {
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    pub fn mupdde(&self) -> MUPDDE_R {
        MUPDDE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    pub fn syncde(&self) -> SYNCDE_R {
        SYNCDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    pub fn mrepde(&self) -> MREPDE_R {
        MREPDE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    pub fn mcmp4de(&self) -> MCMP4DE_R {
        MCMP4DE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    pub fn mcmp3de(&self) -> MCMP3DE_R {
        MCMP3DE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    pub fn mcmp2de(&self) -> MCMP2DE_R {
        MCMP2DE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    pub fn mcmp1de(&self) -> MCMP1DE_R {
        MCMP1DE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    pub fn mupdie(&self) -> MUPDIE_R {
        MUPDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    pub fn syncie(&self) -> SYNCIE_R {
        SYNCIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    pub fn mrepie(&self) -> MREPIE_R {
        MREPIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    pub fn mcmp4ie(&self) -> MCMP4IE_R {
        MCMP4IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    pub fn mcmp3ie(&self) -> MCMP3IE_R {
        MCMP3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    pub fn mcmp2ie(&self) -> MCMP2IE_R {
        MCMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    pub fn mcmp1ie(&self) -> MCMP1IE_R {
        MCMP1IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    pub fn mupdde(&mut self) -> MUPDDE_W {
        MUPDDE_W { w: self }
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    pub fn syncde(&mut self) -> SYNCDE_W {
        SYNCDE_W { w: self }
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    pub fn mrepde(&mut self) -> MREPDE_W {
        MREPDE_W { w: self }
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    pub fn mcmp4de(&mut self) -> MCMP4DE_W {
        MCMP4DE_W { w: self }
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    pub fn mcmp3de(&mut self) -> MCMP3DE_W {
        MCMP3DE_W { w: self }
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    pub fn mcmp2de(&mut self) -> MCMP2DE_W {
        MCMP2DE_W { w: self }
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    pub fn mcmp1de(&mut self) -> MCMP1DE_W {
        MCMP1DE_W { w: self }
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    pub fn mupdie(&mut self) -> MUPDIE_W {
        MUPDIE_W { w: self }
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    pub fn syncie(&mut self) -> SYNCIE_W {
        SYNCIE_W { w: self }
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    pub fn mrepie(&mut self) -> MREPIE_W {
        MREPIE_W { w: self }
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    pub fn mcmp4ie(&mut self) -> MCMP4IE_W {
        MCMP4IE_W { w: self }
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    pub fn mcmp3ie(&mut self) -> MCMP3IE_W {
        MCMP3IE_W { w: self }
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    pub fn mcmp2ie(&mut self) -> MCMP2IE_W {
        MCMP2IE_W { w: self }
    }
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    pub fn mcmp1ie(&mut self) -> MCMP1IE_W {
        MCMP1IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIER4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdier](index.html) module"]
pub struct MDIER_SPEC;
impl crate::RegisterSpec for MDIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdier::R](R) reader structure"]
impl crate::Readable for MDIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdier::W](W) writer structure"]
impl crate::Writable for MDIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIER to value 0"]
impl crate::Resettable for MDIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
