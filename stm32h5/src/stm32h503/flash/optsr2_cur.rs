#[doc = "Register `OPTSR2_CUR` reader"]
pub type R = crate::R<OPTSR2_CURrs>;
#[doc = "Field `SRAM2_RST` reader - SRAM2 erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader;
#[doc = "Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_R = crate::BitReader;
#[doc = "Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_R = crate::BitReader;
#[doc = "Field `SRAM1_RST` reader - SRAM1 erase upon system reset"]
pub type SRAM1_RST_R = crate::BitReader;
#[doc = "Field `SRAM1_ECC` reader - SRAM1 ECC detection and correction disable"]
pub type SRAM1_ECC_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - SRAM2 erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 erase upon system reset"]
    #[inline(always)]
    pub fn sram1_rst(&self) -> SRAM1_RST_R {
        SRAM1_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM1 ECC detection and correction disable"]
    #[inline(always)]
    pub fn sram1_ecc(&self) -> SRAM1_ECC_R {
        SRAM1_ECC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "FLASH option status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optsr2_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTSR2_CURrs;
impl crate::RegisterSpec for OPTSR2_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr2_cur::R`](R) reader structure"]
impl crate::Readable for OPTSR2_CURrs {}
#[doc = "`reset()` method sets OPTSR2_CUR to value 0"]
impl crate::Resettable for OPTSR2_CURrs {
    const RESET_VALUE: u32 = 0;
}
