///Register `OPTSR_CUR` reader
pub type R = crate::R<OPTSR_CURrs>;
///Field `BOR_LEV` reader - Brownout level option status bit These bits reflects the power level that generates a system reset. 00 or 11: BOR Level 1, the threshold level is low (around 2.1V)
pub type BOR_LEV_R = crate::FieldReader;
///Field `BORH_EN` reader - Brownout high enable
pub type BORH_EN_R = crate::BitReader;
///Field `IWDG_SW` reader - IWDG control mode option status bit
pub type IWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` reader - WWDG control mode option status bit
pub type WWDG_SW_R = crate::BitReader;
///Field `NRST_STOP` reader - Core domain Stop entry reset option status bit
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STDBY` reader - Core domain Standby entry reset option status bit
pub type NRST_STDBY_R = crate::BitReader;
///Field `PRODUCT_STATE` reader - Life state code (based on Hamming 8,4). More information in Section7.6.11: Product state transitions.
pub type PRODUCT_STATE_R = crate::FieldReader;
///Field `IO_VDD_HSLV` reader - High-speed IO at low V DD voltage configuration bit. This bit can be set only with V DD below 2.7V.
pub type IO_VDD_HSLV_R = crate::BitReader;
///Field `IO_VDDIO2_HSLV` reader - High-speed IO at low V DDIO2 voltage configuration bit. This bit can be set only with V DDIO2 below 2.7V.
pub type IO_VDDIO2_HSLV_R = crate::BitReader;
///Field `IWDG_STOP` reader - IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode.
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STDBY` reader - IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode.
pub type IWDG_STDBY_R = crate::BitReader;
///Field `BOOT_UBE` reader - Available only on cryptography enabled devices. Unique boot entry control, selects either ST or OEM iRoT for secure boot.
pub type BOOT_UBE_R = crate::FieldReader;
///Field `SWAP_BANK` reader - Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset.
pub type SWAP_BANK_R = crate::BitReader;
impl R {
    ///Bits 0:1 - Brownout level option status bit These bits reflects the power level that generates a system reset. 00 or 11: BOR Level 1, the threshold level is low (around 2.1V)
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Brownout high enable
    #[inline(always)]
    pub fn borh_en(&self) -> BORH_EN_R {
        BORH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IWDG control mode option status bit
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WWDG control mode option status bit
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Core domain Stop entry reset option status bit
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Core domain Standby entry reset option status bit
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Life state code (based on Hamming 8,4). More information in Section7.6.11: Product state transitions.
    #[inline(always)]
    pub fn product_state(&self) -> PRODUCT_STATE_R {
        PRODUCT_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - High-speed IO at low V DD voltage configuration bit. This bit can be set only with V DD below 2.7V.
    #[inline(always)]
    pub fn io_vdd_hslv(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - High-speed IO at low V DDIO2 voltage configuration bit. This bit can be set only with V DDIO2 below 2.7V.
    #[inline(always)]
    pub fn io_vddio2_hslv(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode.
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode.
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:29 - Available only on cryptography enabled devices. Unique boot entry control, selects either ST or OEM iRoT for secure boot.
    #[inline(always)]
    pub fn boot_ube(&self) -> BOOT_UBE_R {
        BOOT_UBE_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    ///Bit 31 - Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset.
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR_CUR")
            .field("bor_lev", &self.bor_lev())
            .field("borh_en", &self.borh_en())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stdby", &self.nrst_stdby())
            .field("product_state", &self.product_state())
            .field("io_vdd_hslv", &self.io_vdd_hslv())
            .field("io_vddio2_hslv", &self.io_vddio2_hslv())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("boot_ube", &self.boot_ube())
            .field("swap_bank", &self.swap_bank())
            .finish()
    }
}
/**FLASH option status register

You can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FLASH:OPTSR_CUR)*/
pub struct OPTSR_CURrs;
impl crate::RegisterSpec for OPTSR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`optsr_cur::R`](R) reader structure
impl crate::Readable for OPTSR_CURrs {}
///`reset()` method sets OPTSR_CUR to value 0
impl crate::Resettable for OPTSR_CURrs {}
