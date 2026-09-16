///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `NAND_SEL` reader - NAND_SEL
pub type NAND_SEL_R = crate::BitReader;
///Field `NAND_ECC` reader - NAND_ECC
pub type NAND_ECC_R = crate::BitReader;
///Field `SDRAM_SEL` reader - SDRAM_SEL
pub type SDRAM_SEL_R = crate::BitReader;
///Field `ID_SIZE` reader - ID_SIZE
pub type ID_SIZE_R = crate::FieldReader;
///Field `WA_LN2DPTH` reader - WA_LN2DPTH
pub type WA_LN2DPTH_R = crate::FieldReader;
///Field `WD_LN2DPTH` reader - WD_LN2DPTH
pub type WD_LN2DPTH_R = crate::FieldReader;
///Field `WR_LN2DPTH` reader - WR_LN2DPTH
pub type WR_LN2DPTH_R = crate::FieldReader;
///Field `RA_LN2DPTH` reader - RA_LN2DPTH
pub type RA_LN2DPTH_R = crate::FieldReader;
impl R {
    ///Bit 0 - NAND_SEL
    #[inline(always)]
    pub fn nand_sel(&self) -> NAND_SEL_R {
        NAND_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - NAND_ECC
    #[inline(always)]
    pub fn nand_ecc(&self) -> NAND_ECC_R {
        NAND_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SDRAM_SEL
    #[inline(always)]
    pub fn sdram_sel(&self) -> SDRAM_SEL_R {
        SDRAM_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - ID_SIZE
    #[inline(always)]
    pub fn id_size(&self) -> ID_SIZE_R {
        ID_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - WA_LN2DPTH
    #[inline(always)]
    pub fn wa_ln2dpth(&self) -> WA_LN2DPTH_R {
        WA_LN2DPTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - WD_LN2DPTH
    #[inline(always)]
    pub fn wd_ln2dpth(&self) -> WD_LN2DPTH_R {
        WD_LN2DPTH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - WR_LN2DPTH
    #[inline(always)]
    pub fn wr_ln2dpth(&self) -> WR_LN2DPTH_R {
        WR_LN2DPTH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - RA_LN2DPTH
    #[inline(always)]
    pub fn ra_ln2dpth(&self) -> RA_LN2DPTH_R {
        RA_LN2DPTH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("nand_sel", &self.nand_sel())
            .field("nand_ecc", &self.nand_ecc())
            .field("sdram_sel", &self.sdram_sel())
            .field("id_size", &self.id_size())
            .field("wa_ln2dpth", &self.wa_ln2dpth())
            .field("wd_ln2dpth", &self.wd_ln2dpth())
            .field("wr_ln2dpth", &self.wr_ln2dpth())
            .field("ra_ln2dpth", &self.ra_ln2dpth())
            .finish()
    }
}
/**FMC Hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:HWCFGR1)*/
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr1::R`](R) reader structure
impl crate::Readable for HWCFGR1rs {}
///`reset()` method sets HWCFGR1 to value 0x2232_b011
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x2232_b011;
}
