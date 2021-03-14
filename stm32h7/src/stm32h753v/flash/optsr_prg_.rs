#[doc = "Reader of register OPTSR_PRG_"]
pub type R = crate::R<u32, super::OPTSR_PRG_>;
#[doc = "Writer for register OPTSR_PRG_"]
pub type W = crate::W<u32, super::OPTSR_PRG_>;
#[doc = "Register OPTSR_PRG_ `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTSR_PRG_ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `IWDG1_HW`"]
pub type IWDG1_HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG1_HW`"]
pub struct IWDG1_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1_HW_W<'a> {
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
#[doc = "Reader of field `nRST_STOP_D1`"]
pub type NRST_STOP_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `nRST_STOP_D1`"]
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
#[doc = "Reader of field `nRST_STBY_D1`"]
pub type NRST_STBY_D1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `nRST_STBY_D1`"]
pub struct NRST_STBY_D1_W<'a> {
    w: &'a mut W,
}
impl<'a> NRST_STBY_D1_W<'a> {
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
#[doc = "Reader of field `FZ_IWDG_STOP`"]
pub type FZ_IWDG_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FZ_IWDG_STOP`"]
pub struct FZ_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG_STOP_W<'a> {
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
#[doc = "Reader of field `FZ_IWDG_SDBY`"]
pub type FZ_IWDG_SDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FZ_IWDG_SDBY`"]
pub struct FZ_IWDG_SDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG_SDBY_W<'a> {
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
#[doc = "Reader of field `RSS1`"]
pub type RSS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSS1`"]
pub struct RSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RSS2`"]
pub type RSS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSS2`"]
pub struct RSS2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSS2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
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
impl R {
    #[doc = "Bits 2:3 - BOR reset level option configuration bits"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - IWDG1 option configuration bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&self) -> IWDG1_HW_R {
        IWDG1_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Option byte erase after D1 DStop option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&self) -> NRST_STOP_D1_R {
        NRST_STOP_D1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Option byte erase after D1 DStandby option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&self) -> NRST_STBY_D1_R {
        NRST_STBY_D1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&self) -> FZ_IWDG_STOP_R {
        FZ_IWDG_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&self) -> FZ_IWDG_SDBY_R {
        FZ_IWDG_SDBY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:20 - DTCM size select option configuration bits"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> ST_RAM_SIZE_R {
        ST_RAM_SIZE_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Security option configuration bit"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - User option configuration bit 1"]
    #[inline(always)]
    pub fn rss1(&self) -> RSS1_R {
        RSS1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - User option configuration bit 2"]
    #[inline(always)]
    pub fn rss2(&self) -> RSS2_R {
        RSS2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SWAP_BANK_OPT_R {
        SWAP_BANK_OPT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - BOR reset level option configuration bits"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W {
        BOR_LEV_W { w: self }
    }
    #[doc = "Bit 4 - IWDG1 option configuration bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&mut self) -> IWDG1_HW_W {
        IWDG1_HW_W { w: self }
    }
    #[doc = "Bit 6 - Option byte erase after D1 DStop option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&mut self) -> NRST_STOP_D1_W {
        NRST_STOP_D1_W { w: self }
    }
    #[doc = "Bit 7 - Option byte erase after D1 DStandby option configuration bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&mut self) -> NRST_STBY_D1_W {
        NRST_STBY_D1_W { w: self }
    }
    #[doc = "Bits 8:15 - Readout protection level option configuration byte"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&mut self) -> FZ_IWDG_STOP_W {
        FZ_IWDG_STOP_W { w: self }
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option configuration bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&mut self) -> FZ_IWDG_SDBY_W {
        FZ_IWDG_SDBY_W { w: self }
    }
    #[doc = "Bits 19:20 - DTCM size select option configuration bits"]
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> ST_RAM_SIZE_W {
        ST_RAM_SIZE_W { w: self }
    }
    #[doc = "Bit 21 - Security option configuration bit"]
    #[inline(always)]
    pub fn security(&mut self) -> SECURITY_W {
        SECURITY_W { w: self }
    }
    #[doc = "Bit 26 - User option configuration bit 1"]
    #[inline(always)]
    pub fn rss1(&mut self) -> RSS1_W {
        RSS1_W { w: self }
    }
    #[doc = "Bit 27 - User option configuration bit 2"]
    #[inline(always)]
    pub fn rss2(&mut self) -> RSS2_W {
        RSS2_W { w: self }
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&mut self) -> IO_HSLV_W {
        IO_HSLV_W { w: self }
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SWAP_BANK_OPT_W {
        SWAP_BANK_OPT_W { w: self }
    }
}
