///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Field `RD_LN2DPTH` reader - RD_LN2DPTH
pub type RD_LN2DPTH_R = crate::FieldReader;
///Field `NOR_BASE` reader - NOR_BASE
pub type NOR_BASE_R = crate::FieldReader;
///Field `SDRAM_RBASE` reader - SDRAM_RBASE
pub type SDRAM_RBASE_R = crate::FieldReader;
///Field `NAND_BASE` reader - NAND_BASE
pub type NAND_BASE_R = crate::FieldReader;
///Field `SDRAM1_BASE` reader - SDRAM1_BASE
pub type SDRAM1_BASE_R = crate::FieldReader;
///Field `SDRAM2_BASE` reader - SDRAM2_BASE
pub type SDRAM2_BASE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - RD_LN2DPTH
    #[inline(always)]
    pub fn rd_ln2dpth(&self) -> RD_LN2DPTH_R {
        RD_LN2DPTH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - NOR_BASE
    #[inline(always)]
    pub fn nor_base(&self) -> NOR_BASE_R {
        NOR_BASE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SDRAM_RBASE
    #[inline(always)]
    pub fn sdram_rbase(&self) -> SDRAM_RBASE_R {
        SDRAM_RBASE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - NAND_BASE
    #[inline(always)]
    pub fn nand_base(&self) -> NAND_BASE_R {
        NAND_BASE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - SDRAM1_BASE
    #[inline(always)]
    pub fn sdram1_base(&self) -> SDRAM1_BASE_R {
        SDRAM1_BASE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - SDRAM2_BASE
    #[inline(always)]
    pub fn sdram2_base(&self) -> SDRAM2_BASE_R {
        SDRAM2_BASE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR2")
            .field("rd_ln2dpth", &self.rd_ln2dpth())
            .field("nor_base", &self.nor_base())
            .field("sdram_rbase", &self.sdram_rbase())
            .field("nand_base", &self.nand_base())
            .field("sdram1_base", &self.sdram1_base())
            .field("sdram2_base", &self.sdram2_base())
            .finish()
    }
}
/**FMC Hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`reset()` method sets HWCFGR2 to value 0x00dc_8762
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x00dc_8762;
}
