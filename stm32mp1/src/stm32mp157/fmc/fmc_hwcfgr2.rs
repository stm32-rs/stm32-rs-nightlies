#[doc = "Register `FMC_HWCFGR2` reader"]
pub type R = crate::R<FMC_HWCFGR2rs>;
#[doc = "Field `RD_LN2DPTH` reader - RD_LN2DPTH"]
pub type RD_LN2DPTH_R = crate::FieldReader;
#[doc = "Field `NOR_BASE` reader - NOR_BASE"]
pub type NOR_BASE_R = crate::FieldReader;
#[doc = "Field `SDRAM_RBASE` reader - SDRAM_RBASE"]
pub type SDRAM_RBASE_R = crate::FieldReader;
#[doc = "Field `NAND_BASE` reader - NAND_BASE"]
pub type NAND_BASE_R = crate::FieldReader;
#[doc = "Field `SDRAM1_BASE` reader - SDRAM1_BASE"]
pub type SDRAM1_BASE_R = crate::FieldReader;
#[doc = "Field `SDRAM2_BASE` reader - SDRAM2_BASE"]
pub type SDRAM2_BASE_R = crate::FieldReader;
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
#[doc = "FMC Hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_HWCFGR2rs;
impl crate::RegisterSpec for FMC_HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_hwcfgr2::R`](R) reader structure"]
impl crate::Readable for FMC_HWCFGR2rs {}
#[doc = "`reset()` method sets FMC_HWCFGR2 to value 0x00dc_8762"]
impl crate::Resettable for FMC_HWCFGR2rs {
    const RESET_VALUE: u32 = 0x00dc_8762;
}
