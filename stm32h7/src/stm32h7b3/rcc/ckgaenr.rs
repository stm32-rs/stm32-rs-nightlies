#[doc = "Register `CKGAENR` reader"]
pub struct R(crate::R<CKGAENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGAENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGAENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGAENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGAENR` writer"]
pub struct W(crate::W<CKGAENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGAENR_SPEC>;
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
impl From<crate::W<CKGAENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGAENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AXI interconnect matrix clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXICKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled"]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI interconnect matrix clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<AXICKG_A> for bool {
    #[inline(always)]
    fn from(variant: AXICKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXICKG` reader - AXI interconnect matrix clock gating This bit is set and reset by software."]
pub struct AXICKG_R(crate::FieldReader<bool, AXICKG_A>);
impl AXICKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXICKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXICKG_A {
        match self.bits {
            false => AXICKG_A::B_0X0,
            true => AXICKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AXICKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AXICKG_A::B_0X1
    }
}
impl core::ops::Deref for AXICKG_R {
    type Target = crate::FieldReader<bool, AXICKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXICKG` writer - AXI interconnect matrix clock gating This bit is set and reset by software."]
pub struct AXICKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AXICKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXICKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXICKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI interconnect matrix clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXICKG_A::B_0X1)
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
#[doc = "AXI master AHB clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHBCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix master AHB clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<AHBCKG_A> for bool {
    #[inline(always)]
    fn from(variant: AHBCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBCKG` reader - AXI master AHB clock gating This bit is set and reset by software."]
pub struct AHBCKG_R(crate::FieldReader<bool, AHBCKG_A>);
impl AHBCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBCKG_A {
        match self.bits {
            false => AHBCKG_A::B_0X0,
            true => AHBCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AHBCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AHBCKG_A::B_0X1
    }
}
impl core::ops::Deref for AHBCKG_R {
    type Target = crate::FieldReader<bool, AHBCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBCKG` writer - AXI master AHB clock gating This bit is set and reset by software."]
pub struct AHBCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHBCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AHBCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix master AHB clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AHBCKG_A::B_0X1)
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
#[doc = "AXI master CPU clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix master CPU clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<CPUCKG_A> for bool {
    #[inline(always)]
    fn from(variant: CPUCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUCKG` reader - AXI master CPU clock gating This bit is set and reset by software."]
pub struct CPUCKG_R(crate::FieldReader<bool, CPUCKG_A>);
impl CPUCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPUCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUCKG_A {
        match self.bits {
            false => CPUCKG_A::B_0X0,
            true => CPUCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CPUCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CPUCKG_A::B_0X1
    }
}
impl core::ops::Deref for CPUCKG_R {
    type Target = crate::FieldReader<bool, CPUCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUCKG` writer - AXI master CPU clock gating This bit is set and reset by software."]
pub struct CPUCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CPUCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix master CPU clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CPUCKG_A::B_0X1)
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
#[doc = "AXI master SDMMC clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMCCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix master SDMMC clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<SDMMCCKG_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMCCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMMCCKG` reader - AXI master SDMMC clock gating This bit is set and reset by software."]
pub struct SDMMCCKG_R(crate::FieldReader<bool, SDMMCCKG_A>);
impl SDMMCCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMCCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMCCKG_A {
        match self.bits {
            false => SDMMCCKG_A::B_0X0,
            true => SDMMCCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SDMMCCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SDMMCCKG_A::B_0X1
    }
}
impl core::ops::Deref for SDMMCCKG_R {
    type Target = crate::FieldReader<bool, SDMMCCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMCCKG` writer - AXI master SDMMC clock gating This bit is set and reset by software."]
pub struct SDMMCCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMCCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMCCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMCCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix master SDMMC clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMCCKG_A::B_0X1)
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
#[doc = "AXI master MDMA clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMACKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled"]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix master MDMA clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<MDMACKG_A> for bool {
    #[inline(always)]
    fn from(variant: MDMACKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMACKG` reader - AXI master MDMA clock gating This bit is set and reset by software."]
pub struct MDMACKG_R(crate::FieldReader<bool, MDMACKG_A>);
impl MDMACKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMACKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMACKG_A {
        match self.bits {
            false => MDMACKG_A::B_0X0,
            true => MDMACKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MDMACKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MDMACKG_A::B_0X1
    }
}
impl core::ops::Deref for MDMACKG_R {
    type Target = crate::FieldReader<bool, MDMACKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMACKG` writer - AXI master MDMA clock gating This bit is set and reset by software."]
pub struct MDMACKG_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMACKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMACKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MDMACKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix master MDMA clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MDMACKG_A::B_0X1)
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
#[doc = "AXI master DMA2D clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA2DCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled"]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix master DMA2D clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<DMA2DCKG_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2DCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA2DCKG` reader - AXI master DMA2D clock gating This bit is set and reset by software."]
pub struct DMA2DCKG_R(crate::FieldReader<bool, DMA2DCKG_A>);
impl DMA2DCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2DCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2DCKG_A {
        match self.bits {
            false => DMA2DCKG_A::B_0X0,
            true => DMA2DCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DMA2DCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DMA2DCKG_A::B_0X1
    }
}
impl core::ops::Deref for DMA2DCKG_R {
    type Target = crate::FieldReader<bool, DMA2DCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2DCKG` writer - AXI master DMA2D clock gating This bit is set and reset by software."]
pub struct DMA2DCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2DCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2DCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA2DCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix master DMA2D clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA2DCKG_A::B_0X1)
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
#[doc = "AXI master LTDC clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix master LTDC clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<LTDCCKG_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCCKG` reader - AXI master LTDC clock gating This bit is set and reset by software."]
pub struct LTDCCKG_R(crate::FieldReader<bool, LTDCCKG_A>);
impl LTDCCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCCKG_A {
        match self.bits {
            false => LTDCCKG_A::B_0X0,
            true => LTDCCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LTDCCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LTDCCKG_A::B_0X1
    }
}
impl core::ops::Deref for LTDCCKG_R {
    type Target = crate::FieldReader<bool, LTDCCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCCKG` writer - AXI master LTDC clock gating This bit is set and reset by software."]
pub struct LTDCCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTDCCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LTDCCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix master LTDC clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LTDCCKG_A::B_0X1)
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
#[doc = "AXI master GFXMMU clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFXMMUMCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled"]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix master GFXMMU clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<GFXMMUMCKG_A> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUMCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFXMMUMCKG` reader - AXI master GFXMMU clock gating This bit is set and reset by software."]
pub struct GFXMMUMCKG_R(crate::FieldReader<bool, GFXMMUMCKG_A>);
impl GFXMMUMCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFXMMUMCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GFXMMUMCKG_A {
        match self.bits {
            false => GFXMMUMCKG_A::B_0X0,
            true => GFXMMUMCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == GFXMMUMCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == GFXMMUMCKG_A::B_0X1
    }
}
impl core::ops::Deref for GFXMMUMCKG_R {
    type Target = crate::FieldReader<bool, GFXMMUMCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFXMMUMCKG` writer - AXI master GFXMMU clock gating This bit is set and reset by software."]
pub struct GFXMMUMCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> GFXMMUMCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFXMMUMCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GFXMMUMCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix master GFXMMU clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GFXMMUMCKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "AXI slave AHB12 clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB12CKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled"]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave AHB12 clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<AHB12CKG_A> for bool {
    #[inline(always)]
    fn from(variant: AHB12CKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB12CKG` reader - AXI slave AHB12 clock gating This bit is set and reset by software."]
pub struct AHB12CKG_R(crate::FieldReader<bool, AHB12CKG_A>);
impl AHB12CKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB12CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB12CKG_A {
        match self.bits {
            false => AHB12CKG_A::B_0X0,
            true => AHB12CKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AHB12CKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AHB12CKG_A::B_0X1
    }
}
impl core::ops::Deref for AHB12CKG_R {
    type Target = crate::FieldReader<bool, AHB12CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB12CKG` writer - AXI slave AHB12 clock gating This bit is set and reset by software."]
pub struct AHB12CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB12CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB12CKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AHB12CKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave AHB12 clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AHB12CKG_A::B_0X1)
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
#[doc = "AXI slave AHB34 clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB34CKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled"]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave AHB34 clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<AHB34CKG_A> for bool {
    #[inline(always)]
    fn from(variant: AHB34CKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB34CKG` reader - AXI slave AHB34 clock gating This bit is set and reset by software."]
pub struct AHB34CKG_R(crate::FieldReader<bool, AHB34CKG_A>);
impl AHB34CKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB34CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB34CKG_A {
        match self.bits {
            false => AHB34CKG_A::B_0X0,
            true => AHB34CKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AHB34CKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AHB34CKG_A::B_0X1
    }
}
impl core::ops::Deref for AHB34CKG_R {
    type Target = crate::FieldReader<bool, AHB34CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB34CKG` writer - AXI slave AHB34 clock gating This bit is set and reset by software."]
pub struct AHB34CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB34CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB34CKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AHB34CKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave AHB34 clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AHB34CKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLIFTCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave FLIFT clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<FLIFTCKG_A> for bool {
    #[inline(always)]
    fn from(variant: FLIFTCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLIFTCKG` reader - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
pub struct FLIFTCKG_R(crate::FieldReader<bool, FLIFTCKG_A>);
impl FLIFTCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLIFTCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLIFTCKG_A {
        match self.bits {
            false => FLIFTCKG_A::B_0X0,
            true => FLIFTCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FLIFTCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FLIFTCKG_A::B_0X1
    }
}
impl core::ops::Deref for FLIFTCKG_R {
    type Target = crate::FieldReader<bool, FLIFTCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLIFTCKG` writer - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
pub struct FLIFTCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIFTCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLIFTCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FLIFTCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave FLIFT clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FLIFTCKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "AXI slave OCTOSPI2 clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCTOSPI2CKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave OCTOSPI2 clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<OCTOSPI2CKG_A> for bool {
    #[inline(always)]
    fn from(variant: OCTOSPI2CKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTOSPI2CKG` reader - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
pub struct OCTOSPI2CKG_R(crate::FieldReader<bool, OCTOSPI2CKG_A>);
impl OCTOSPI2CKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTOSPI2CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTOSPI2CKG_A {
        match self.bits {
            false => OCTOSPI2CKG_A::B_0X0,
            true => OCTOSPI2CKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OCTOSPI2CKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OCTOSPI2CKG_A::B_0X1
    }
}
impl core::ops::Deref for OCTOSPI2CKG_R {
    type Target = crate::FieldReader<bool, OCTOSPI2CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTOSPI2CKG` writer - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
pub struct OCTOSPI2CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI2CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPI2CKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OCTOSPI2CKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave OCTOSPI2 clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OCTOSPI2CKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "AXI slave FMC clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave FMC clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<FMCCKG_A> for bool {
    #[inline(always)]
    fn from(variant: FMCCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCCKG` reader - AXI slave FMC clock gating This bit is set and reset by software."]
pub struct FMCCKG_R(crate::FieldReader<bool, FMCCKG_A>);
impl FMCCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMCCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCCKG_A {
        match self.bits {
            false => FMCCKG_A::B_0X0,
            true => FMCCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FMCCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FMCCKG_A::B_0X1
    }
}
impl core::ops::Deref for FMCCKG_R {
    type Target = crate::FieldReader<bool, FMCCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCCKG` writer - AXI slave FMC clock gating This bit is set and reset by software."]
pub struct FMCCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave FMC clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCCKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "AXI slave OCTOSPI1 clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCTOSPI1CKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave OCTOSPI1 clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<OCTOSPI1CKG_A> for bool {
    #[inline(always)]
    fn from(variant: OCTOSPI1CKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCTOSPI1CKG` reader - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
pub struct OCTOSPI1CKG_R(crate::FieldReader<bool, OCTOSPI1CKG_A>);
impl OCTOSPI1CKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTOSPI1CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCTOSPI1CKG_A {
        match self.bits {
            false => OCTOSPI1CKG_A::B_0X0,
            true => OCTOSPI1CKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OCTOSPI1CKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OCTOSPI1CKG_A::B_0X1
    }
}
impl core::ops::Deref for OCTOSPI1CKG_R {
    type Target = crate::FieldReader<bool, OCTOSPI1CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTOSPI1CKG` writer - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
pub struct OCTOSPI1CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPI1CKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OCTOSPI1CKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave OCTOSPI1 clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OCTOSPI1CKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "AXI slave SRAM1 clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIRAM1CKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave SRAM1 clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<AXIRAM1CKG_A> for bool {
    #[inline(always)]
    fn from(variant: AXIRAM1CKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXIRAM1CKG` reader - AXI slave SRAM1 clock gating This bit is set and reset by software."]
pub struct AXIRAM1CKG_R(crate::FieldReader<bool, AXIRAM1CKG_A>);
impl AXIRAM1CKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIRAM1CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIRAM1CKG_A {
        match self.bits {
            false => AXIRAM1CKG_A::B_0X0,
            true => AXIRAM1CKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AXIRAM1CKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AXIRAM1CKG_A::B_0X1
    }
}
impl core::ops::Deref for AXIRAM1CKG_R {
    type Target = crate::FieldReader<bool, AXIRAM1CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIRAM1CKG` writer - AXI slave SRAM1 clock gating This bit is set and reset by software."]
pub struct AXIRAM1CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIRAM1CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIRAM1CKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXIRAM1CKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave SRAM1 clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXIRAM1CKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "AXI matrix slave SRAM2 clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIRAM2CKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave SRAM2 clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<AXIRAM2CKG_A> for bool {
    #[inline(always)]
    fn from(variant: AXIRAM2CKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXIRAM2CKG` reader - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
pub struct AXIRAM2CKG_R(crate::FieldReader<bool, AXIRAM2CKG_A>);
impl AXIRAM2CKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIRAM2CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIRAM2CKG_A {
        match self.bits {
            false => AXIRAM2CKG_A::B_0X0,
            true => AXIRAM2CKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AXIRAM2CKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AXIRAM2CKG_A::B_0X1
    }
}
impl core::ops::Deref for AXIRAM2CKG_R {
    type Target = crate::FieldReader<bool, AXIRAM2CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIRAM2CKG` writer - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
pub struct AXIRAM2CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIRAM2CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIRAM2CKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXIRAM2CKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave SRAM2 clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXIRAM2CKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "AXI matrix slave SRAM3 clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIRAM3CKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave SRAM3 clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<AXIRAM3CKG_A> for bool {
    #[inline(always)]
    fn from(variant: AXIRAM3CKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AXIRAM3CKG` reader - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
pub struct AXIRAM3CKG_R(crate::FieldReader<bool, AXIRAM3CKG_A>);
impl AXIRAM3CKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIRAM3CKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIRAM3CKG_A {
        match self.bits {
            false => AXIRAM3CKG_A::B_0X0,
            true => AXIRAM3CKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == AXIRAM3CKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == AXIRAM3CKG_A::B_0X1
    }
}
impl core::ops::Deref for AXIRAM3CKG_R {
    type Target = crate::FieldReader<bool, AXIRAM3CKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIRAM3CKG` writer - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
pub struct AXIRAM3CKG_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIRAM3CKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIRAM3CKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXIRAM3CKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave SRAM3 clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXIRAM3CKG_A::B_0X1)
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
#[doc = "AXI matrix slave GFXMMU clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFXMMUSCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled"]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The AXI matrix slave GFXMMU clock is enabled on bus transaction request."]
    B_0X1 = 1,
}
impl From<GFXMMUSCKG_A> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUSCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFXMMUSCKG` reader - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
pub struct GFXMMUSCKG_R(crate::FieldReader<bool, GFXMMUSCKG_A>);
impl GFXMMUSCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFXMMUSCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GFXMMUSCKG_A {
        match self.bits {
            false => GFXMMUSCKG_A::B_0X0,
            true => GFXMMUSCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == GFXMMUSCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == GFXMMUSCKG_A::B_0X1
    }
}
impl core::ops::Deref for GFXMMUSCKG_R {
    type Target = crate::FieldReader<bool, GFXMMUSCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFXMMUSCKG` writer - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
pub struct GFXMMUSCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> GFXMMUSCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFXMMUSCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GFXMMUSCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The AXI matrix slave GFXMMU clock is enabled on bus transaction request."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GFXMMUSCKG_A::B_0X1)
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
#[doc = "RAM error code correction (ECC) clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCRAMCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The ECC clock is enabled only during a RAM access."]
    B_0X1 = 1,
}
impl From<ECCRAMCKG_A> for bool {
    #[inline(always)]
    fn from(variant: ECCRAMCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCRAMCKG` reader - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
pub struct ECCRAMCKG_R(crate::FieldReader<bool, ECCRAMCKG_A>);
impl ECCRAMCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCRAMCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCRAMCKG_A {
        match self.bits {
            false => ECCRAMCKG_A::B_0X0,
            true => ECCRAMCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ECCRAMCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ECCRAMCKG_A::B_0X1
    }
}
impl core::ops::Deref for ECCRAMCKG_R {
    type Target = crate::FieldReader<bool, ECCRAMCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCRAMCKG` writer - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
pub struct ECCRAMCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCRAMCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCRAMCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ECCRAMCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The ECC clock is enabled only during a RAM access."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ECCRAMCKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "EXTI clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTICKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The clock is enabled after an event detection and stopped again when the event flag is cleared."]
    B_0X1 = 1,
}
impl From<EXTICKG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTICKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTICKG` reader - EXTI clock gating This bit is set and reset by software."]
pub struct EXTICKG_R(crate::FieldReader<bool, EXTICKG_A>);
impl EXTICKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTICKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTICKG_A {
        match self.bits {
            false => EXTICKG_A::B_0X0,
            true => EXTICKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EXTICKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EXTICKG_A::B_0X1
    }
}
impl core::ops::Deref for EXTICKG_R {
    type Target = crate::FieldReader<bool, EXTICKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTICKG` writer - EXTI clock gating This bit is set and reset by software."]
pub struct EXTICKG_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTICKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTICKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EXTICKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The clock is enabled after an event detection and stopped again when the event flag is cleared."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EXTICKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "JTAG automatic clock gating This bit is set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAGCKG_A {
    #[doc = "0: The clock gating is disabled. The clock is always enabled."]
    B_0X0 = 0,
    #[doc = "1: The clock gating is enabled. The clock is disabled except if a JTAG connection has been detected"]
    B_0X1 = 1,
}
impl From<JTAGCKG_A> for bool {
    #[inline(always)]
    fn from(variant: JTAGCKG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JTAGCKG` reader - JTAG automatic clock gating This bit is set and reset by software."]
pub struct JTAGCKG_R(crate::FieldReader<bool, JTAGCKG_A>);
impl JTAGCKG_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAGCKG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAGCKG_A {
        match self.bits {
            false => JTAGCKG_A::B_0X0,
            true => JTAGCKG_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == JTAGCKG_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == JTAGCKG_A::B_0X1
    }
}
impl core::ops::Deref for JTAGCKG_R {
    type Target = crate::FieldReader<bool, JTAGCKG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAGCKG` writer - JTAG automatic clock gating This bit is set and reset by software."]
pub struct JTAGCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAGCKG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTAGCKG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is disabled. The clock is always enabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JTAGCKG_A::B_0X0)
    }
    #[doc = "The clock gating is enabled. The clock is disabled except if a JTAG connection has been detected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JTAGCKG_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AXI interconnect matrix clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axickg(&self) -> AXICKG_R {
        AXICKG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AXI master AHB clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahbckg(&self) -> AHBCKG_R {
        AHBCKG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AXI master CPU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn cpuckg(&self) -> CPUCKG_R {
        CPUCKG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXI master SDMMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn sdmmcckg(&self) -> SDMMCCKG_R {
        SDMMCCKG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AXI master MDMA clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn mdmackg(&self) -> MDMACKG_R {
        MDMACKG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AXI master DMA2D clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn dma2dckg(&self) -> DMA2DCKG_R {
        DMA2DCKG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AXI master LTDC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ltdcckg(&self) -> LTDCCKG_R {
        LTDCCKG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AXI master GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn gfxmmumckg(&self) -> GFXMMUMCKG_R {
        GFXMMUMCKG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AXI slave AHB12 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahb12ckg(&self) -> AHB12CKG_R {
        AHB12CKG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AXI slave AHB34 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahb34ckg(&self) -> AHB34CKG_R {
        AHB34CKG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn fliftckg(&self) -> FLIFTCKG_R {
        FLIFTCKG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn octospi2ckg(&self) -> OCTOSPI2CKG_R {
        OCTOSPI2CKG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AXI slave FMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn fmcckg(&self) -> FMCCKG_R {
        FMCCKG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn octospi1ckg(&self) -> OCTOSPI1CKG_R {
        OCTOSPI1CKG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AXI slave SRAM1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram1ckg(&self) -> AXIRAM1CKG_R {
        AXIRAM1CKG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram2ckg(&self) -> AXIRAM2CKG_R {
        AXIRAM2CKG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram3ckg(&self) -> AXIRAM3CKG_R {
        AXIRAM3CKG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn gfxmmusckg(&self) -> GFXMMUSCKG_R {
        GFXMMUSCKG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn eccramckg(&self) -> ECCRAMCKG_R {
        ECCRAMCKG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - EXTI clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn extickg(&self) -> EXTICKG_R {
        EXTICKG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - JTAG automatic clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn jtagckg(&self) -> JTAGCKG_R {
        JTAGCKG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXI interconnect matrix clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axickg(&mut self) -> AXICKG_W {
        AXICKG_W { w: self }
    }
    #[doc = "Bit 1 - AXI master AHB clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahbckg(&mut self) -> AHBCKG_W {
        AHBCKG_W { w: self }
    }
    #[doc = "Bit 2 - AXI master CPU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn cpuckg(&mut self) -> CPUCKG_W {
        CPUCKG_W { w: self }
    }
    #[doc = "Bit 3 - AXI master SDMMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn sdmmcckg(&mut self) -> SDMMCCKG_W {
        SDMMCCKG_W { w: self }
    }
    #[doc = "Bit 4 - AXI master MDMA clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn mdmackg(&mut self) -> MDMACKG_W {
        MDMACKG_W { w: self }
    }
    #[doc = "Bit 5 - AXI master DMA2D clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn dma2dckg(&mut self) -> DMA2DCKG_W {
        DMA2DCKG_W { w: self }
    }
    #[doc = "Bit 6 - AXI master LTDC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ltdcckg(&mut self) -> LTDCCKG_W {
        LTDCCKG_W { w: self }
    }
    #[doc = "Bit 7 - AXI master GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn gfxmmumckg(&mut self) -> GFXMMUMCKG_W {
        GFXMMUMCKG_W { w: self }
    }
    #[doc = "Bit 8 - AXI slave AHB12 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahb12ckg(&mut self) -> AHB12CKG_W {
        AHB12CKG_W { w: self }
    }
    #[doc = "Bit 9 - AXI slave AHB34 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn ahb34ckg(&mut self) -> AHB34CKG_W {
        AHB34CKG_W { w: self }
    }
    #[doc = "Bit 10 - AXI slave Flash interface (FLIFT) clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn fliftckg(&mut self) -> FLIFTCKG_W {
        FLIFTCKG_W { w: self }
    }
    #[doc = "Bit 11 - AXI slave OCTOSPI2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn octospi2ckg(&mut self) -> OCTOSPI2CKG_W {
        OCTOSPI2CKG_W { w: self }
    }
    #[doc = "Bit 12 - AXI slave FMC clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn fmcckg(&mut self) -> FMCCKG_W {
        FMCCKG_W { w: self }
    }
    #[doc = "Bit 13 - AXI slave OCTOSPI1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn octospi1ckg(&mut self) -> OCTOSPI1CKG_W {
        OCTOSPI1CKG_W { w: self }
    }
    #[doc = "Bit 14 - AXI slave SRAM1 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram1ckg(&mut self) -> AXIRAM1CKG_W {
        AXIRAM1CKG_W { w: self }
    }
    #[doc = "Bit 15 - AXI matrix slave SRAM2 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram2ckg(&mut self) -> AXIRAM2CKG_W {
        AXIRAM2CKG_W { w: self }
    }
    #[doc = "Bit 16 - AXI matrix slave SRAM3 clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn axiram3ckg(&mut self) -> AXIRAM3CKG_W {
        AXIRAM3CKG_W { w: self }
    }
    #[doc = "Bit 17 - AXI matrix slave GFXMMU clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn gfxmmusckg(&mut self) -> GFXMMUSCKG_W {
        GFXMMUSCKG_W { w: self }
    }
    #[doc = "Bit 29 - RAM error code correction (ECC) clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn eccramckg(&mut self) -> ECCRAMCKG_W {
        ECCRAMCKG_W { w: self }
    }
    #[doc = "Bit 30 - EXTI clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn extickg(&mut self) -> EXTICKG_W {
        EXTICKG_W { w: self }
    }
    #[doc = "Bit 31 - JTAG automatic clock gating This bit is set and reset by software."]
    #[inline(always)]
    pub fn jtagckg(&mut self) -> JTAGCKG_W {
        JTAGCKG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AXI clocks gating enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgaenr](index.html) module"]
pub struct CKGAENR_SPEC;
impl crate::RegisterSpec for CKGAENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgaenr::R](R) reader structure"]
impl crate::Readable for CKGAENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgaenr::W](W) writer structure"]
impl crate::Writable for CKGAENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGAENR to value 0"]
impl crate::Resettable for CKGAENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
