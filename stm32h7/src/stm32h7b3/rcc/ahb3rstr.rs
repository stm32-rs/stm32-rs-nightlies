#[doc = "Register `AHB3RSTR` reader"]
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3RSTR` writer"]
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MDMA block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMARST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<MDMARST_A> for bool {
    #[inline(always)]
    fn from(variant: MDMARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMARST` reader - MDMA block reset Set and reset by software."]
pub struct MDMARST_R(crate::FieldReader<bool, MDMARST_A>);
impl MDMARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMARST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MDMARST_A> {
        match self.bits {
            true => Some(MDMARST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == MDMARST_A::RESET
    }
}
impl core::ops::Deref for MDMARST_R {
    type Target = crate::FieldReader<bool, MDMARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMARST` writer - MDMA block reset Set and reset by software."]
pub struct MDMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MDMARST_A::RESET)
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
#[doc = "DMA2D block reset Set and reset by software."]
pub type DMA2DRST_A = MDMARST_A;
#[doc = "Field `DMA2DRST` reader - DMA2D block reset Set and reset by software."]
pub type DMA2DRST_R = MDMARST_R;
#[doc = "Field `DMA2DRST` writer - DMA2D block reset Set and reset by software."]
pub struct DMA2DRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2DRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2DRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA2DRST_A::RESET)
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
#[doc = "JPGDEC block reset Set and reset by software."]
pub type JPGDECRST_A = MDMARST_A;
#[doc = "Field `JPGDECRST` reader - JPGDEC block reset Set and reset by software."]
pub type JPGDECRST_R = MDMARST_R;
#[doc = "Field `JPGDECRST` writer - JPGDEC block reset Set and reset by software."]
pub struct JPGDECRST_W<'a> {
    w: &'a mut W,
}
impl<'a> JPGDECRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JPGDECRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(JPGDECRST_A::RESET)
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
#[doc = "FMC block reset Set and reset by software."]
pub type FMCRST_A = MDMARST_A;
#[doc = "Field `FMCRST` reader - FMC block reset Set and reset by software."]
pub type FMCRST_R = MDMARST_R;
#[doc = "Field `FMCRST` writer - FMC block reset Set and reset by software."]
pub struct FMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMCRST_A::RESET)
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
#[doc = "OCTOSPI1 and OCTOSPI1 delay blocks reset Set and reset by software."]
pub type OCTOSPI1RST_A = MDMARST_A;
#[doc = "Field `OCTOSPI1RST` reader - OCTOSPI1 and OCTOSPI1 delay blocks reset Set and reset by software."]
pub type OCTOSPI1RST_R = MDMARST_R;
#[doc = "Field `OCTOSPI1RST` writer - OCTOSPI1 and OCTOSPI1 delay blocks reset Set and reset by software."]
pub struct OCTOSPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPI1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OCTOSPI1RST_A::RESET)
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
#[doc = "SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
pub type SDMMC1RST_A = MDMARST_A;
#[doc = "Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
pub type SDMMC1RST_R = MDMARST_R;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
pub struct SDMMC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDMMC1RST_A::RESET)
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
#[doc = "OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software"]
pub type OCTOSPI2RST_A = MDMARST_A;
#[doc = "Field `OCTOSPI2RST` reader - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software"]
pub type OCTOSPI2RST_R = MDMARST_R;
#[doc = "Field `OCTOSPI2RST` writer - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software"]
pub struct OCTOSPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPI2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OCTOSPI2RST_A::RESET)
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
#[doc = "OCTOSPIM reset Set and reset by software"]
pub type OCTOSPIMRST_A = MDMARST_A;
#[doc = "Field `OCTOSPIMRST` reader - OCTOSPIM reset Set and reset by software"]
pub type OCTOSPIMRST_R = MDMARST_R;
#[doc = "Field `OCTOSPIMRST` writer - OCTOSPIM reset Set and reset by software"]
pub struct OCTOSPIMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPIMRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPIMRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OCTOSPIMRST_A::RESET)
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
#[doc = "OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
pub type OTFD1RST_A = MDMARST_A;
#[doc = "Field `OTFD1RST` reader - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
pub type OTFD1RST_R = MDMARST_R;
#[doc = "Field `OTFD1RST` writer - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
pub struct OTFD1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFD1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTFD1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTFD1RST_A::RESET)
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
#[doc = "OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
pub type OTFD2RST_A = MDMARST_A;
#[doc = "Field `OTFD2RST` reader - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
pub type OTFD2RST_R = MDMARST_R;
#[doc = "Field `OTFD2RST` writer - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
pub struct OTFD2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFD2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTFD2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTFD2RST_A::RESET)
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
#[doc = "GFXMMU reset Set and reset by software"]
pub type GFXMMURST_A = MDMARST_A;
#[doc = "Field `GFXMMURST` reader - GFXMMU reset Set and reset by software"]
pub type GFXMMURST_R = MDMARST_R;
#[doc = "Field `GFXMMURST` writer - GFXMMU reset Set and reset by software"]
pub struct GFXMMURST_W<'a> {
    w: &'a mut W,
}
impl<'a> GFXMMURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFXMMURST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GFXMMURST_A::RESET)
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
impl R {
    #[doc = "Bit 0 - MDMA block reset Set and reset by software."]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA2D block reset Set and reset by software."]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - JPGDEC block reset Set and reset by software."]
    #[inline(always)]
    pub fn jpgdecrst(&self) -> JPGDECRST_R {
        JPGDECRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMC block reset Set and reset by software."]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn octospi1rst(&self) -> OCTOSPI1RST_R {
        OCTOSPI1RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software"]
    #[inline(always)]
    pub fn octospi2rst(&self) -> OCTOSPI2RST_R {
        OCTOSPI2RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OCTOSPIM reset Set and reset by software"]
    #[inline(always)]
    pub fn octospimrst(&self) -> OCTOSPIMRST_R {
        OCTOSPIMRST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
    #[inline(always)]
    pub fn otfd1rst(&self) -> OTFD1RST_R {
        OTFD1RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
    #[inline(always)]
    pub fn otfd2rst(&self) -> OTFD2RST_R {
        OTFD2RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GFXMMU reset Set and reset by software"]
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA block reset Set and reset by software."]
    #[inline(always)]
    pub fn mdmarst(&mut self) -> MDMARST_W {
        MDMARST_W { w: self }
    }
    #[doc = "Bit 4 - DMA2D block reset Set and reset by software."]
    #[inline(always)]
    pub fn dma2drst(&mut self) -> DMA2DRST_W {
        DMA2DRST_W { w: self }
    }
    #[doc = "Bit 5 - JPGDEC block reset Set and reset by software."]
    #[inline(always)]
    pub fn jpgdecrst(&mut self) -> JPGDECRST_W {
        JPGDECRST_W { w: self }
    }
    #[doc = "Bit 12 - FMC block reset Set and reset by software."]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
    #[doc = "Bit 14 - OCTOSPI1 and OCTOSPI1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn octospi1rst(&mut self) -> OCTOSPI1RST_W {
        OCTOSPI1RST_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W {
        SDMMC1RST_W { w: self }
    }
    #[doc = "Bit 19 - OCTOSPI2 and OCTOSPI2 delay block reset Set and reset by software"]
    #[inline(always)]
    pub fn octospi2rst(&mut self) -> OCTOSPI2RST_W {
        OCTOSPI2RST_W { w: self }
    }
    #[doc = "Bit 21 - OCTOSPIM reset Set and reset by software"]
    #[inline(always)]
    pub fn octospimrst(&mut self) -> OCTOSPIMRST_W {
        OCTOSPIMRST_W { w: self }
    }
    #[doc = "Bit 22 - OTFD1 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
    #[inline(always)]
    pub fn otfd1rst(&mut self) -> OTFD1RST_W {
        OTFD1RST_W { w: self }
    }
    #[doc = "Bit 23 - OTFD2 reset Set and reset by software Take care that resetting the OTFD means loosing the decryption key loaded during secure boot."]
    #[inline(always)]
    pub fn otfd2rst(&mut self) -> OTFD2RST_W {
        OTFD2RST_W { w: self }
    }
    #[doc = "Bit 24 - GFXMMU reset Set and reset by software"]
    #[inline(always)]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W {
        GFXMMURST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](index.html) module"]
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3rstr::R](R) reader structure"]
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure"]
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
