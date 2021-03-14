#[doc = "Reader of register FMC_HWCFGR1"]
pub type R = crate::R<u32, super::FMC_HWCFGR1>;
#[doc = "Reader of field `NAND_SEL`"]
pub type NAND_SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAND_ECC`"]
pub type NAND_ECC_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDRAM_SEL`"]
pub type SDRAM_SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ID_SIZE`"]
pub type ID_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `WA_LN2DPTH`"]
pub type WA_LN2DPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `WD_LN2DPTH`"]
pub type WD_LN2DPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `WR_LN2DPTH`"]
pub type WR_LN2DPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `RA_LN2DPTH`"]
pub type RA_LN2DPTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - NAND_SEL"]
    #[inline(always)]
    pub fn nand_sel(&self) -> NAND_SEL_R {
        NAND_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAND_ECC"]
    #[inline(always)]
    pub fn nand_ecc(&self) -> NAND_ECC_R {
        NAND_ECC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SDRAM_SEL"]
    #[inline(always)]
    pub fn sdram_sel(&self) -> SDRAM_SEL_R {
        SDRAM_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - ID_SIZE"]
    #[inline(always)]
    pub fn id_size(&self) -> ID_SIZE_R {
        ID_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - WA_LN2DPTH"]
    #[inline(always)]
    pub fn wa_ln2dpth(&self) -> WA_LN2DPTH_R {
        WA_LN2DPTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - WD_LN2DPTH"]
    #[inline(always)]
    pub fn wd_ln2dpth(&self) -> WD_LN2DPTH_R {
        WD_LN2DPTH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - WR_LN2DPTH"]
    #[inline(always)]
    pub fn wr_ln2dpth(&self) -> WR_LN2DPTH_R {
        WR_LN2DPTH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - RA_LN2DPTH"]
    #[inline(always)]
    pub fn ra_ln2dpth(&self) -> RA_LN2DPTH_R {
        RA_LN2DPTH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
