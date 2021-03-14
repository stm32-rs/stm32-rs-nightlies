#[doc = "Reader of register WPCR1"]
pub type R = crate::R<u32, super::WPCR1>;
#[doc = "Writer for register WPCR1"]
pub type W = crate::W<u32, super::WPCR1>;
#[doc = "Register WPCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::WPCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPRXFT`"]
pub type LPRXFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPRXFT`"]
pub struct LPRXFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRXFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `FLPRXLPM`"]
pub type FLPRXLPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLPRXLPM`"]
pub struct FLPRXLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPRXLPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `HSTXSRCDL`"]
pub type HSTXSRCDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXSRCDL`"]
pub struct HSTXSRCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `HSTXSRCCL`"]
pub type HSTXSRCCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXSRCCL`"]
pub struct HSTXSRCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRCCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDDC`"]
pub type SDDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDDC`"]
pub struct SDDC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `LPSRCDL`"]
pub type LPSRCDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPSRCDL`"]
pub struct LPSRCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRCDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `LPSRCCL`"]
pub type LPSRCCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPSRCCL`"]
pub struct LPSRCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSRCCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `HSTXDDL`"]
pub type HSTXDDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXDDL`"]
pub struct HSTXDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `HSTXDCL`"]
pub type HSTXDCL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTXDCL`"]
pub struct HSTXDCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXDCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LPRXFT_R {
        LPRXFT_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FLPRXLPM_R {
        FLPRXLPM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HSTXSRCDL_R {
        HSTXSRCDL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HSTXSRCCL_R {
        HSTXSRCCL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&self) -> SDDC_R {
        SDDC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&self) -> LPSRCDL_R {
        LPSRCDL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&self) -> LPSRCCL_R {
        LPSRCCL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&self) -> HSTXDDL_R {
        HSTXDDL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HSTXDCL_R {
        HSTXDCL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&mut self) -> LPRXFT_W {
        LPRXFT_W { w: self }
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&mut self) -> FLPRXLPM_W {
        FLPRXLPM_W { w: self }
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&mut self) -> HSTXSRCDL_W {
        HSTXSRCDL_W { w: self }
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&mut self) -> HSTXSRCCL_W {
        HSTXSRCCL_W { w: self }
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&mut self) -> SDDC_W {
        SDDC_W { w: self }
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&mut self) -> LPSRCDL_W {
        LPSRCDL_W { w: self }
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&mut self) -> LPSRCCL_W {
        LPSRCCL_W { w: self }
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&mut self) -> HSTXDDL_W {
        HSTXDDL_W { w: self }
    }
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&mut self) -> HSTXDCL_W {
        HSTXDCL_W { w: self }
    }
}
