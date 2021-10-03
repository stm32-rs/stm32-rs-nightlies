#[doc = "Register `AHB3LPENR` reader"]
pub struct R(crate::R<AHB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3LPENR` writer"]
pub struct W(crate::W<AHB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3LPENR_SPEC>;
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
impl From<crate::W<AHB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MDMA clock enable during CSleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMALPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    ENABLED = 1,
}
impl From<MDMALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMALPEN` reader - MDMA clock enable during CSleep mode Set and reset by software."]
pub struct MDMALPEN_R(crate::FieldReader<bool, MDMALPEN_A>);
impl MDMALPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMALPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMALPEN_A {
        match self.bits {
            false => MDMALPEN_A::DISABLED,
            true => MDMALPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MDMALPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MDMALPEN_A::ENABLED
    }
}
impl core::ops::Deref for MDMALPEN_R {
    type Target = crate::FieldReader<bool, MDMALPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMALPEN` writer - MDMA clock enable during CSleep mode Set and reset by software."]
pub struct MDMALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMALPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMALPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMALPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MDMALPEN_A::ENABLED)
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
#[doc = "DMA2D clock enable during CSleep mode Set and reset by software."]
pub type DMA2DLPEN_A = MDMALPEN_A;
#[doc = "Field `DMA2DLPEN` reader - DMA2D clock enable during CSleep mode Set and reset by software."]
pub type DMA2DLPEN_R = MDMALPEN_R;
#[doc = "Field `DMA2DLPEN` writer - DMA2D clock enable during CSleep mode Set and reset by software."]
pub struct DMA2DLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2DLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2DLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2DLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2DLPEN_A::ENABLED)
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
#[doc = "JPGDEC clock enable during CSleep mode Set and reset by software."]
pub type JPGDECLPEN_A = MDMALPEN_A;
#[doc = "Field `JPGDECLPEN` reader - JPGDEC clock enable during CSleep mode Set and reset by software."]
pub type JPGDECLPEN_R = MDMALPEN_R;
#[doc = "Field `JPGDECLPEN` writer - JPGDEC clock enable during CSleep mode Set and reset by software."]
pub struct JPGDECLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JPGDECLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JPGDECLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JPGDECLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JPGDECLPEN_A::ENABLED)
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
#[doc = "Flash interface clock enable during csleep mode"]
pub type FLASHPREN_A = MDMALPEN_A;
#[doc = "Field `FLASHPREN` reader - Flash interface clock enable during csleep mode"]
pub type FLASHPREN_R = MDMALPEN_R;
#[doc = "Field `FLASHPREN` writer - Flash interface clock enable during csleep mode"]
pub struct FLASHPREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHPREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHPREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHPREN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHPREN_A::ENABLED)
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
#[doc = "FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock."]
pub type FMCLPEN_A = MDMALPEN_A;
#[doc = "Field `FMCLPEN` reader - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock."]
pub type FMCLPEN_R = MDMALPEN_R;
#[doc = "Field `FMCLPEN` writer - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock."]
pub struct FMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FMCLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FMCLPEN_A::ENABLED)
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
#[doc = "OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software."]
pub type OCTOSPI1LPEN_A = MDMALPEN_A;
#[doc = "Field `OCTOSPI1LPEN` reader - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software."]
pub type OCTOSPI1LPEN_R = MDMALPEN_R;
#[doc = "Field `OCTOSPI1LPEN` writer - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software."]
pub struct OCTOSPI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPI1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OCTOSPI1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OCTOSPI1LPEN_A::ENABLED)
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
#[doc = "SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software."]
pub type SDMMC1LPEN_A = MDMALPEN_A;
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software."]
pub type SDMMC1LPEN_R = MDMALPEN_R;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software."]
pub struct SDMMC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDMMC1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDMMC1LPEN_A::ENABLED)
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
#[doc = "OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software."]
pub type OCTOSPI2LPEN_A = MDMALPEN_A;
#[doc = "Field `OCTOSPI2LPEN` reader - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software."]
pub type OCTOSPI2LPEN_R = MDMALPEN_R;
#[doc = "Field `OCTOSPI2LPEN` writer - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software."]
pub struct OCTOSPI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPI2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OCTOSPI2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OCTOSPI2LPEN_A::ENABLED)
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
#[doc = "OCTOSPIM block clock enable during CSleep mode Set and reset by software."]
pub type OCTOSPIMLPEN_A = MDMALPEN_A;
#[doc = "Field `OCTOSPIMLPEN` reader - OCTOSPIM block clock enable during CSleep mode Set and reset by software."]
pub type OCTOSPIMLPEN_R = MDMALPEN_R;
#[doc = "Field `OCTOSPIMLPEN` writer - OCTOSPIM block clock enable during CSleep mode Set and reset by software."]
pub struct OCTOSPIMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPIMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPIMLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OCTOSPIMLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OCTOSPIMLPEN_A::ENABLED)
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
#[doc = "OTFD1 block clock enable during CSleep mode Set and reset by software."]
pub type OTFD1LPEN_A = MDMALPEN_A;
#[doc = "Field `OTFD1LPEN` reader - OTFD1 block clock enable during CSleep mode Set and reset by software."]
pub type OTFD1LPEN_R = MDMALPEN_R;
#[doc = "Field `OTFD1LPEN` writer - OTFD1 block clock enable during CSleep mode Set and reset by software."]
pub struct OTFD1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFD1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTFD1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTFD1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTFD1LPEN_A::ENABLED)
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
#[doc = "OTFD2 block clock enable during CSleep mode Set and reset by software."]
pub type OTFD2LPEN_A = MDMALPEN_A;
#[doc = "Field `OTFD2LPEN` reader - OTFD2 block clock enable during CSleep mode Set and reset by software."]
pub type OTFD2LPEN_R = MDMALPEN_R;
#[doc = "Field `OTFD2LPEN` writer - OTFD2 block clock enable during CSleep mode Set and reset by software."]
pub struct OTFD2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFD2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTFD2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTFD2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTFD2LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "GFXMMU block clock enable during CSleep mode Set and reset by software."]
pub type GFXMMULPEN_A = MDMALPEN_A;
#[doc = "Field `GFXMMULPEN` reader - GFXMMU block clock enable during CSleep mode Set and reset by software."]
pub type GFXMMULPEN_R = MDMALPEN_R;
#[doc = "Field `GFXMMULPEN` writer - GFXMMU block clock enable during CSleep mode Set and reset by software."]
pub struct GFXMMULPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GFXMMULPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFXMMULPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GFXMMULPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GFXMMULPEN_A::ENABLED)
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
#[doc = "AXISRAM2 block clock enable during CSleep mode Set and reset by software."]
pub type AXISRAM2LPEN_A = MDMALPEN_A;
#[doc = "Field `AXISRAM2LPEN` reader - AXISRAM2 block clock enable during CSleep mode Set and reset by software."]
pub type AXISRAM2LPEN_R = MDMALPEN_R;
#[doc = "Field `AXISRAM2LPEN` writer - AXISRAM2 block clock enable during CSleep mode Set and reset by software."]
pub struct AXISRAM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXISRAM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXISRAM2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AXISRAM2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AXISRAM2LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "AXISRAM3 block clock enable during CSleep mode Set and reset by software."]
pub type AXISRAM3LPEN_A = MDMALPEN_A;
#[doc = "Field `AXISRAM3LPEN` reader - AXISRAM3 block clock enable during CSleep mode Set and reset by software."]
pub type AXISRAM3LPEN_R = MDMALPEN_R;
#[doc = "Field `AXISRAM3LPEN` writer - AXISRAM3 block clock enable during CSleep mode Set and reset by software."]
pub struct AXISRAM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXISRAM3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXISRAM3LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AXISRAM3LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AXISRAM3LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "DTCM1 block clock enable during CSleep mode Set and reset by software."]
pub type DTCM1LPEN_A = MDMALPEN_A;
#[doc = "Field `DTCM1LPEN` reader - DTCM1 block clock enable during CSleep mode Set and reset by software."]
pub type DTCM1LPEN_R = MDMALPEN_R;
#[doc = "Field `DTCM1LPEN` writer - DTCM1 block clock enable during CSleep mode Set and reset by software."]
pub struct DTCM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCM1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTCM1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTCM1LPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "DTCM2 block clock enable during CSleep mode Set and reset by software."]
pub type DTCM2LPEN_A = MDMALPEN_A;
#[doc = "Field `DTCM2LPEN` reader - DTCM2 block clock enable during CSleep mode Set and reset by software."]
pub type DTCM2LPEN_R = MDMALPEN_R;
#[doc = "Field `DTCM2LPEN` writer - DTCM2 block clock enable during CSleep mode Set and reset by software."]
pub struct DTCM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCM2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTCM2LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTCM2LPEN_A::ENABLED)
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
#[doc = "ITCM block clock enable during CSleep mode Set and reset by software."]
pub type ITCMLPEN_A = MDMALPEN_A;
#[doc = "Field `ITCMLPEN` reader - ITCM block clock enable during CSleep mode Set and reset by software."]
pub type ITCMLPEN_R = MDMALPEN_R;
#[doc = "Field `ITCMLPEN` writer - ITCM block clock enable during CSleep mode Set and reset by software."]
pub struct ITCMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITCMLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITCMLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITCMLPEN_A::ENABLED)
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
#[doc = "AXISRAM1 block clock enable during CSleep mode Set and reset by software."]
pub type AXISRAM1LPEN_A = MDMALPEN_A;
#[doc = "Field `AXISRAM1LPEN` reader - AXISRAM1 block clock enable during CSleep mode Set and reset by software."]
pub type AXISRAM1LPEN_R = MDMALPEN_R;
#[doc = "Field `AXISRAM1LPEN` writer - AXISRAM1 block clock enable during CSleep mode Set and reset by software."]
pub struct AXISRAM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXISRAM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXISRAM1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AXISRAM1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AXISRAM1LPEN_A::ENABLED)
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
    #[doc = "Bit 0 - MDMA clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA2D clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - JPGDEC clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn jpgdeclpen(&self) -> JPGDECLPEN_R {
        JPGDECLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash interface clock enable during csleep mode"]
    #[inline(always)]
    pub fn flashpren(&self) -> FLASHPREN_R {
        FLASHPREN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock."]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospi2lpen(&self) -> OCTOSPI2LPEN_R {
        OCTOSPI2LPEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospimlpen(&self) -> OCTOSPIMLPEN_R {
        OCTOSPIMLPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OTFD1 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn otfd1lpen(&self) -> OTFD1LPEN_R {
        OTFD1LPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OTFD2 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn otfd2lpen(&self) -> OTFD2LPEN_R {
        OTFD2LPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GFXMMU block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gfxmmulpen(&self) -> GFXMMULPEN_R {
        GFXMMULPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - AXISRAM2 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn axisram2lpen(&self) -> AXISRAM2LPEN_R {
        AXISRAM2LPEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - AXISRAM3 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn axisram3lpen(&self) -> AXISRAM3LPEN_R {
        AXISRAM3LPEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DTCM1 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dtcm1lpen(&self) -> DTCM1LPEN_R {
        DTCM1LPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DTCM2 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ITCM block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AXISRAM1 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn axisram1lpen(&self) -> AXISRAM1LPEN_R {
        AXISRAM1LPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W {
        MDMALPEN_W { w: self }
    }
    #[doc = "Bit 4 - DMA2D clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W {
        DMA2DLPEN_W { w: self }
    }
    #[doc = "Bit 5 - JPGDEC clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn jpgdeclpen(&mut self) -> JPGDECLPEN_W {
        JPGDECLPEN_W { w: self }
    }
    #[doc = "Bit 8 - Flash interface clock enable during csleep mode"]
    #[inline(always)]
    pub fn flashpren(&mut self) -> FLASHPREN_W {
        FLASHPREN_W { w: self }
    }
    #[doc = "Bit 12 - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock."]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W {
        OCTOSPI1LPEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W {
        SDMMC1LPEN_W { w: self }
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospi2lpen(&mut self) -> OCTOSPI2LPEN_W {
        OCTOSPI2LPEN_W { w: self }
    }
    #[doc = "Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn octospimlpen(&mut self) -> OCTOSPIMLPEN_W {
        OCTOSPIMLPEN_W { w: self }
    }
    #[doc = "Bit 22 - OTFD1 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn otfd1lpen(&mut self) -> OTFD1LPEN_W {
        OTFD1LPEN_W { w: self }
    }
    #[doc = "Bit 23 - OTFD2 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn otfd2lpen(&mut self) -> OTFD2LPEN_W {
        OTFD2LPEN_W { w: self }
    }
    #[doc = "Bit 24 - GFXMMU block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gfxmmulpen(&mut self) -> GFXMMULPEN_W {
        GFXMMULPEN_W { w: self }
    }
    #[doc = "Bit 26 - AXISRAM2 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn axisram2lpen(&mut self) -> AXISRAM2LPEN_W {
        AXISRAM2LPEN_W { w: self }
    }
    #[doc = "Bit 27 - AXISRAM3 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn axisram3lpen(&mut self) -> AXISRAM3LPEN_W {
        AXISRAM3LPEN_W { w: self }
    }
    #[doc = "Bit 28 - DTCM1 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dtcm1lpen(&mut self) -> DTCM1LPEN_W {
        DTCM1LPEN_W { w: self }
    }
    #[doc = "Bit 29 - DTCM2 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W {
        DTCM2LPEN_W { w: self }
    }
    #[doc = "Bit 30 - ITCM block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W {
        ITCMLPEN_W { w: self }
    }
    #[doc = "Bit 31 - AXISRAM1 block clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn axisram1lpen(&mut self) -> AXISRAM1LPEN_W {
        AXISRAM1LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3lpenr](index.html) module"]
pub struct AHB3LPENR_SPEC;
impl crate::RegisterSpec for AHB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3lpenr::R](R) reader structure"]
impl crate::Readable for AHB3LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3lpenr::W](W) writer structure"]
impl crate::Writable for AHB3LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB3LPENR to value 0xfde9_5131"]
impl crate::Resettable for AHB3LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfde9_5131
    }
}
