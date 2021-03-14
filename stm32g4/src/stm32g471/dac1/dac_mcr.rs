#[doc = "Reader of register DAC_MCR"]
pub type R = crate::R<u32, super::DAC_MCR>;
#[doc = "Writer for register DAC_MCR"]
pub type W = crate::W<u32, super::DAC_MCR>;
#[doc = "Register DAC_MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE1`"]
pub type MODE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE1`"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DMADOUBLE1`"]
pub type DMADOUBLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMADOUBLE1`"]
pub struct DMADOUBLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADOUBLE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SINFORMAT1`"]
pub type SINFORMAT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINFORMAT1`"]
pub struct SINFORMAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SINFORMAT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `HFSEL`"]
pub type HFSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFSEL`"]
pub struct HFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MODE2`"]
pub type MODE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE2`"]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMADOUBLE2`"]
pub type DMADOUBLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMADOUBLE2`"]
pub struct DMADOUBLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADOUBLE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SINFORMAT2`"]
pub type SINFORMAT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINFORMAT2`"]
pub struct SINFORMAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SINFORMAT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble1(&self) -> DMADOUBLE1_R {
        DMADOUBLE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn sinformat1(&self) -> SINFORMAT1_R {
        SINFORMAT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble2(&self) -> DMADOUBLE2_R {
        DMADOUBLE2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn sinformat2(&self) -> SINFORMAT2_R {
        SINFORMAT2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble1(&mut self) -> DMADOUBLE1_W {
        DMADOUBLE1_W { w: self }
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn sinformat1(&mut self) -> SINFORMAT1_W {
        SINFORMAT1_W { w: self }
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn hfsel(&mut self) -> HFSEL_W {
        HFSEL_W { w: self }
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble2(&mut self) -> DMADOUBLE2_W {
        DMADOUBLE2_W { w: self }
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn sinformat2(&mut self) -> SINFORMAT2_W {
        SINFORMAT2_W { w: self }
    }
}
