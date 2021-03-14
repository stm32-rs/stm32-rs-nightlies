#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BREN`"]
pub type BREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BREN`"]
pub struct BREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREN_W<'a> {
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
#[doc = "Reader of field `MONEN`"]
pub type MONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONEN`"]
pub struct MONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONEN_W<'a> {
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
#[doc = "Reader of field `BRRDY`"]
pub type BRRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBATL`"]
pub type VBATL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBATH`"]
pub type VBATH_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMPL`"]
pub type TEMPL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMPH`"]
pub type TEMPH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VBAT level monitoring versus low threshold"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VBAT level monitoring versus high threshold"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W {
        BREN_W { w: self }
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W {
        MONEN_W { w: self }
    }
}
