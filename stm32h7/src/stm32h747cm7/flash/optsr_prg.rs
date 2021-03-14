#[doc = "Reader of register OPTSR_PRG"]
pub type R = crate::R<u32, super::OPTSR_PRG>;
#[doc = "Writer for register OPTSR_PRG"]
pub type W = crate::W<u32, super::OPTSR_PRG>;
#[doc = "Register OPTSR_PRG `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTSR_PRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWAP_BANK_OPT`"]
pub type SWAP_BANK_OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP_BANK_OPT`"]
pub struct SWAP_BANK_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_BANK_OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `IO_HSLV`"]
pub type IO_HSLV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO_HSLV`"]
pub struct IO_HSLV_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_HSLV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `NRST_STBY_D2`"]
pub type NRST_STBY_D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRST_STBY_D2`"]
pub struct NRST_STBY_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STBY_D2_W<'a> {
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
#[doc = "Reader of field `NRST_STOP_D2`"]
pub type NRST_STOP_D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRST_STOP_D2`"]
pub struct NRST_STOP_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STOP_D2_W<'a> {
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
#[doc = "Reader of field `BOOT_CM7`"]
pub type BOOT_CM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_CM7`"]
pub struct BOOT_CM7_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM7_W<'a> {
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
#[doc = "Reader of field `BOOT_CM4`"]
pub type BOOT_CM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_CM4`"]
pub struct BOOT_CM4_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_CM4_W<'a> {
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
#[doc = "Reader of field `SECURITY`"]
pub type SECURITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECURITY`"]
pub struct SECURITY_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ST_RAM_SIZE`"]
pub type ST_RAM_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ST_RAM_SIZE`"]
pub struct ST_RAM_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_RAM_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `IWDG_FZ_SDBY`"]
pub type IWDG_FZ_SDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG_FZ_SDBY`"]
pub struct IWDG_FZ_SDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_FZ_SDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `IWDG_FZ_STOP`"]
pub type IWDG_FZ_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG_FZ_STOP`"]
pub struct IWDG_FZ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_FZ_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RDP`"]
pub type RDP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDP`"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NRST_STDY_D1`"]
pub type NRST_STDY_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRST_STDY_D1`"]
pub struct NRST_STDY_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STDY_D1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `NRST_STOP_D1`"]
pub type NRST_STOP_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NRST_STOP_D1`"]
pub struct NRST_STOP_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STOP_D1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IWDG2_SW`"]
pub type IWDG2_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG2_SW`"]
pub struct IWDG2_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG2_SW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IWDG_SW`"]
pub type IWDG_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG_SW`"]
pub struct IWDG_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG_SW_W<'a> {
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
#[doc = "Reader of field `BOR_LEV`"]
pub type BOR_LEV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOR_LEV`"]
pub struct BOR_LEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_LEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stby_d2(&self) -> NRST_STBY_D2_R {
        NRST_STBY_D2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stop_d2(&self) -> NRST_STOP_D2_R {
        NRST_STOP_D2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm7(&self) -> BOOT_CM7_R {
        BOOT_CM7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm4(&self) -> BOOT_CM4_R {
        BOOT_CM4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Security enable option configuration bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - ST RAM size option configuration bits"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    pub fn iwdg_fz_stop(&self) -> IWDG_FZ_STOP_R {
        IWDG_FZ_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration bits"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stdy_d1(&self) -> NRST_STDY_D1_R {
        NRST_STDY_D1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stop_d1(&self) -> NRST_STOP_D1_R {
        NRST_STOP_D1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IWDG2 control mode option configuration bit"]
    #[inline(always)]
    pub fn iwdg2_sw(&self) -> IWDG2_SW_R {
        IWDG2_SW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IWDG control mode option configuration bit"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Brownout level option configuration bit"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W {
        SWAP_BANK_OPT_W { w: self }
    }
    #[doc = "Bit 29 - I"]
    #[inline(always)]
    pub fn io_hslv(&mut self) -> IO_HSLV_W {
        IO_HSLV_W { w: self }
    }
    #[doc = "Bit 25 - D2 domain DStandby entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stby_d2(&mut self) -> NRST_STBY_D2_W {
        NRST_STBY_D2_W { w: self }
    }
    #[doc = "Bit 24 - D2 domain DStop entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stop_d2(&mut self) -> NRST_STOP_D2_W {
        NRST_STOP_D2_W { w: self }
    }
    #[doc = "Bit 23 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm7(&mut self) -> BOOT_CM7_W {
        BOOT_CM7_W { w: self }
    }
    #[doc = "Bit 22 - Arm Cortex"]
    #[inline(always)]
    pub fn boot_cm4(&mut self) -> BOOT_CM4_W {
        BOOT_CM4_W { w: self }
    }
    #[doc = "Bit 21 - Security enable option configuration bit"]
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W {
        SECURITY_W { w: self }
    }
    #[doc = "Bits 19:20 - ST RAM size option configuration bits"]
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W {
        ST_RAM_SIZE_W { w: self }
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W {
        IWDG_FZ_SDBY_W { w: self }
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    pub fn iwdg_fz_stop(&mut self) -> IWDG_FZ_STOP_W {
        IWDG_FZ_STOP_W { w: self }
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration bits"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 7 - D1 domain DStandby entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stdy_d1(&mut self) -> NRST_STDY_D1_W {
        NRST_STDY_D1_W { w: self }
    }
    #[doc = "Bit 6 - D1 domain DStop entry reset option configuration bit"]
    #[inline(always)]
    pub fn nrst_stop_d1(&mut self) -> NRST_STOP_D1_W {
        NRST_STOP_D1_W { w: self }
    }
    #[doc = "Bit 5 - IWDG2 control mode option configuration bit"]
    #[inline(always)]
    pub fn iwdg2_sw(&mut self) -> IWDG2_SW_W {
        IWDG2_SW_W { w: self }
    }
    #[doc = "Bit 4 - IWDG control mode option configuration bit"]
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W {
        IWDG_SW_W { w: self }
    }
    #[doc = "Bits 2:3 - Brownout level option configuration bit"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W {
        BOR_LEV_W { w: self }
    }
}
