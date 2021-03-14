#[doc = "Reader of register FMC_HWCFGR2"]
pub type R = crate::R<u32, super::FMC_HWCFGR2>;
#[doc = "Reader of field `RD_LN2DPTH`"]
pub type RD_LN2DPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `NOR_BASE`"]
pub type NOR_BASE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SDRAM_RBASE`"]
pub type SDRAM_RBASE_R = crate::R<u8, u8>;
#[doc = "Reader of field `NAND_BASE`"]
pub type NAND_BASE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SDRAM1_BASE`"]
pub type SDRAM1_BASE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SDRAM2_BASE`"]
pub type SDRAM2_BASE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - RD_LN2DPTH"]
    #[inline(always)]
    pub fn rd_ln2dpth(&self) -> RD_LN2DPTH_R {
        RD_LN2DPTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - NOR_BASE"]
    #[inline(always)]
    pub fn nor_base(&self) -> NOR_BASE_R {
        NOR_BASE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDRAM_RBASE"]
    #[inline(always)]
    pub fn sdram_rbase(&self) -> SDRAM_RBASE_R {
        SDRAM_RBASE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NAND_BASE"]
    #[inline(always)]
    pub fn nand_base(&self) -> NAND_BASE_R {
        NAND_BASE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SDRAM1_BASE"]
    #[inline(always)]
    pub fn sdram1_base(&self) -> SDRAM1_BASE_R {
        SDRAM1_BASE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SDRAM2_BASE"]
    #[inline(always)]
    pub fn sdram2_base(&self) -> SDRAM2_BASE_R {
        SDRAM2_BASE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
