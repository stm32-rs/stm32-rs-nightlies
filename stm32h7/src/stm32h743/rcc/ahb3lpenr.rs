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
#[doc = "MDMA Clock Enable During CSleep Mode\n\nValue on reset: 0"]
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
#[doc = "Field `MDMALPEN` reader - MDMA Clock Enable During CSleep Mode"]
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
#[doc = "Field `MDMALPEN` writer - MDMA Clock Enable During CSleep Mode"]
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
#[doc = "DMA2D Clock Enable During CSleep Mode"]
pub type DMA2DLPEN_A = MDMALPEN_A;
#[doc = "Field `DMA2DLPEN` reader - DMA2D Clock Enable During CSleep Mode"]
pub type DMA2DLPEN_R = MDMALPEN_R;
#[doc = "Field `DMA2DLPEN` writer - DMA2D Clock Enable During CSleep Mode"]
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
#[doc = "JPGDEC Clock Enable During CSleep Mode"]
pub type JPGDECLPEN_A = MDMALPEN_A;
#[doc = "Field `JPGDECLPEN` reader - JPGDEC Clock Enable During CSleep Mode"]
pub type JPGDECLPEN_R = MDMALPEN_R;
#[doc = "Field `JPGDECLPEN` writer - JPGDEC Clock Enable During CSleep Mode"]
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
#[doc = "FLITF Clock Enable During CSleep Mode"]
pub type FLASHLPEN_A = MDMALPEN_A;
#[doc = "Field `FLASHLPEN` reader - FLITF Clock Enable During CSleep Mode"]
pub type FLASHLPEN_R = MDMALPEN_R;
#[doc = "Field `FLASHLPEN` writer - FLITF Clock Enable During CSleep Mode"]
pub struct FLASHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHLPEN_A::ENABLED)
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
#[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
pub type FMCLPEN_A = MDMALPEN_A;
#[doc = "Field `FMCLPEN` reader - FMC Peripheral Clocks Enable During CSleep Mode"]
pub type FMCLPEN_R = MDMALPEN_R;
#[doc = "Field `FMCLPEN` writer - FMC Peripheral Clocks Enable During CSleep Mode"]
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
#[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
pub type QSPILPEN_A = MDMALPEN_A;
#[doc = "Field `QSPILPEN` reader - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
pub type QSPILPEN_R = MDMALPEN_R;
#[doc = "Field `QSPILPEN` writer - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
pub struct QSPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPILPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(QSPILPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(QSPILPEN_A::ENABLED)
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
#[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub type SDMMC1LPEN_A = MDMALPEN_A;
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub type SDMMC1LPEN_R = MDMALPEN_R;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
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
#[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
pub type D1DTCM1LPEN_A = MDMALPEN_A;
#[doc = "Field `D1DTCM1LPEN` reader - D1DTCM1 Block Clock Enable During CSleep mode"]
pub type D1DTCM1LPEN_R = MDMALPEN_R;
#[doc = "Field `D1DTCM1LPEN` writer - D1DTCM1 Block Clock Enable During CSleep mode"]
pub struct D1DTCM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D1DTCM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D1DTCM1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(D1DTCM1LPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(D1DTCM1LPEN_A::ENABLED)
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
#[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
pub type DTCM2LPEN_A = MDMALPEN_A;
#[doc = "Field `DTCM2LPEN` reader - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub type DTCM2LPEN_R = MDMALPEN_R;
#[doc = "Field `DTCM2LPEN` writer - D1 DTCM2 Block Clock Enable During CSleep mode"]
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
#[doc = "D1ITCM Block Clock Enable During CSleep mode"]
pub type ITCMLPEN_A = MDMALPEN_A;
#[doc = "Field `ITCMLPEN` reader - D1ITCM Block Clock Enable During CSleep mode"]
pub type ITCMLPEN_R = MDMALPEN_R;
#[doc = "Field `ITCMLPEN` writer - D1ITCM Block Clock Enable During CSleep mode"]
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
#[doc = "AXISRAM Block Clock Enable During CSleep mode"]
pub type AXISRAMLPEN_A = MDMALPEN_A;
#[doc = "Field `AXISRAMLPEN` reader - AXISRAM Block Clock Enable During CSleep mode"]
pub type AXISRAMLPEN_R = MDMALPEN_R;
#[doc = "Field `AXISRAMLPEN` writer - AXISRAM Block Clock Enable During CSleep mode"]
pub struct AXISRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXISRAMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXISRAMLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AXISRAMLPEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AXISRAMLPEN_A::ENABLED)
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
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - JPGDEC Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn jpgdeclpen(&self) -> JPGDECLPEN_R {
        JPGDECLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLITF Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn flashlpen(&self) -> FLASHLPEN_R {
        FLASHLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn d1dtcm1lpen(&self) -> D1DTCM1LPEN_R {
        D1DTCM1LPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn axisramlpen(&self) -> AXISRAMLPEN_R {
        AXISRAMLPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W {
        MDMALPEN_W { w: self }
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W {
        DMA2DLPEN_W { w: self }
    }
    #[doc = "Bit 5 - JPGDEC Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn jpgdeclpen(&mut self) -> JPGDECLPEN_W {
        JPGDECLPEN_W { w: self }
    }
    #[doc = "Bit 8 - FLITF Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn flashlpen(&mut self) -> FLASHLPEN_W {
        FLASHLPEN_W { w: self }
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W {
        QSPILPEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W {
        SDMMC1LPEN_W { w: self }
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn d1dtcm1lpen(&mut self) -> D1DTCM1LPEN_W {
        D1DTCM1LPEN_W { w: self }
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W {
        DTCM2LPEN_W { w: self }
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W {
        ITCMLPEN_W { w: self }
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn axisramlpen(&mut self) -> AXISRAMLPEN_W {
        AXISRAMLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB3 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3lpenr](index.html) module"]
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
#[doc = "`reset()` method sets AHB3LPENR to value 0"]
impl crate::Resettable for AHB3LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
