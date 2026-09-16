///Register `OPTSR2_CUR` reader
pub type R = crate::R<OPTSR2_CURrs>;
///Field `SRAM2_RST` reader - SRAM2 erase when system reset
pub type SRAM2_RST_R = crate::BitReader;
///Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable
pub type BKPRAM_ECC_R = crate::BitReader;
///Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable
pub type SRAM2_ECC_R = crate::BitReader;
///Field `SRAM1_RST` reader - SRAM1 erase upon system reset
pub type SRAM1_RST_R = crate::BitReader;
///Field `SRAM1_ECC` reader - SRAM1 ECC detection and correction disable
pub type SRAM1_ECC_R = crate::BitReader;
impl R {
    ///Bit 3 - SRAM2 erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Backup RAM ECC detection and correction disable
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - SRAM2 ECC detection and correction disable
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - SRAM1 erase upon system reset
    #[inline(always)]
    pub fn sram1_rst(&self) -> SRAM1_RST_R {
        SRAM1_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM1 ECC detection and correction disable
    #[inline(always)]
    pub fn sram1_ecc(&self) -> SRAM1_ECC_R {
        SRAM1_ECC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR2_CUR")
            .field("sram2_rst", &self.sram2_rst())
            .field("bkpram_ecc", &self.bkpram_ecc())
            .field("sram2_ecc", &self.sram2_ecc())
            .field("sram1_rst", &self.sram1_rst())
            .field("sram1_ecc", &self.sram1_ecc())
            .finish()
    }
}
/**FLASH option status register 2

You can [`read`](crate::Reg::read) this register and get [`optsr2_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:OPTSR2_CUR)*/
pub struct OPTSR2_CURrs;
impl crate::RegisterSpec for OPTSR2_CURrs {
    type Ux = u32;
}
///`read()` method returns [`optsr2_cur::R`](R) reader structure
impl crate::Readable for OPTSR2_CURrs {}
///`reset()` method sets OPTSR2_CUR to value 0
impl crate::Resettable for OPTSR2_CURrs {}
