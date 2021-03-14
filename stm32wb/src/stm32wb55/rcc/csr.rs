#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0x0c00_0000"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c00_0000
    }
}
#[doc = "Reader of field `LPWRRSTF`"]
pub type LPWRRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WWDGRSTF`"]
pub type WWDGRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `IWDGRSTF`"]
pub type IWDGRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SFTRSTF`"]
pub type SFTRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BORRSTF`"]
pub type BORRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PINRSTF`"]
pub type PINRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OBLRSTF`"]
pub type OBLRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RMVF`"]
pub type RMVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMVF`"]
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RFWKPSEL`"]
pub type RFWKPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFWKPSEL`"]
pub struct RFWKPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFWKPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `LSI2BW`"]
pub type LSI2BW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSI2BW`"]
pub struct LSI2BW_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI2BW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LSI2TRIMOK`"]
pub type LSI2TRIMOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSI2TRIMEN`"]
pub type LSI2TRIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSI2TRIMEN`"]
pub struct LSI2TRIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI2TRIMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `LSI2RDY`"]
pub type LSI2RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSI2ON`"]
pub type LSI2ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSI2ON`"]
pub struct LSI2ON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI2ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LSI1RDY`"]
pub type LSI1RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSI1ON`"]
pub type LSI1ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSI1ON`"]
pub struct LSI1ON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI1ON_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RFRSTS`"]
pub type RFRSTS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - BOR flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - RF system wakeup clock source selection"]
    #[inline(always)]
    pub fn rfwkpsel(&self) -> RFWKPSEL_R {
        RFWKPSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - LSI2 oscillator bias configuration"]
    #[inline(always)]
    pub fn lsi2bw(&self) -> LSI2BW_R {
        LSI2BW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - LSI2 oscillator trim OK"]
    #[inline(always)]
    pub fn lsi2trimok(&self) -> LSI2TRIMOK_R {
        LSI2TRIMOK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LSI2 oscillator trimming enable"]
    #[inline(always)]
    pub fn lsi2trimen(&self) -> LSI2TRIMEN_R {
        LSI2TRIMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSI2 oscillator ready"]
    #[inline(always)]
    pub fn lsi2rdy(&self) -> LSI2RDY_R {
        LSI2RDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LSI2 oscillator enabled"]
    #[inline(always)]
    pub fn lsi2on(&self) -> LSI2ON_R {
        LSI2ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSI1 oscillator ready"]
    #[inline(always)]
    pub fn lsi1rdy(&self) -> LSI1RDY_R {
        LSI1RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI1 oscillator enabled"]
    #[inline(always)]
    pub fn lsi1on(&self) -> LSI1ON_R {
        LSI1ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Radio system BLE and 802.15.4 reset status"]
    #[inline(always)]
    pub fn rfrsts(&self) -> RFRSTS_R {
        RFRSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W { w: self }
    }
    #[doc = "Bits 14:15 - RF system wakeup clock source selection"]
    #[inline(always)]
    pub fn rfwkpsel(&mut self) -> RFWKPSEL_W {
        RFWKPSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - LSI2 oscillator bias configuration"]
    #[inline(always)]
    pub fn lsi2bw(&mut self) -> LSI2BW_W {
        LSI2BW_W { w: self }
    }
    #[doc = "Bit 4 - LSI2 oscillator trimming enable"]
    #[inline(always)]
    pub fn lsi2trimen(&mut self) -> LSI2TRIMEN_W {
        LSI2TRIMEN_W { w: self }
    }
    #[doc = "Bit 2 - LSI2 oscillator enabled"]
    #[inline(always)]
    pub fn lsi2on(&mut self) -> LSI2ON_W {
        LSI2ON_W { w: self }
    }
    #[doc = "Bit 0 - LSI1 oscillator enabled"]
    #[inline(always)]
    pub fn lsi1on(&mut self) -> LSI1ON_W {
        LSI1ON_W { w: self }
    }
}
