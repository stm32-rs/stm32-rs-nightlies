///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `RDP` reader - Read protection level Other: Level 1, memories read protection active
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Read protection level Other: Level 1, memories read protection active
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BORR_LEV` reader - BOR reset level
pub type BORR_LEV_R = crate::FieldReader;
///Field `BORR_LEV` writer - BOR reset level
pub type BORR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `NRST_STOP` reader - Reset generated when entering Stop mode
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - Reset generated when entering Stop mode
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDBY` reader - Reset generated when entering Standby mode
pub type NRST_STDBY_R = crate::BitReader;
///Field `NRST_STDBY` writer - Reset generated when entering Standby mode
pub type NRST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_SHDW` reader - Reset generated when entering Shutdown mode
pub type NRST_SHDW_R = crate::BitReader;
///Field `NRST_SHDW` writer - Reset generated when entering Shutdown mode
pub type NRST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - Independent watchdog selection
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - Independent watchdog selection
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BDRST` reader - Backup domain reset
pub type BDRST_R = crate::BitReader;
///Field `BDRST` writer - Backup domain reset
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAM_PARITY_CHECK` reader - SRAM parity check control enable/disable
pub type RAM_PARITY_CHECK_R = crate::BitReader;
///Field `RAM_PARITY_CHECK` writer - SRAM parity check control enable/disable
pub type RAM_PARITY_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAM_HW_ERASE_DISABLE` reader - Backup SRAM erase prevention
pub type BKPSRAM_HW_ERASE_DISABLE_R = crate::BitReader;
///Field `BKPSRAM_HW_ERASE_DISABLE` writer - Backup SRAM erase prevention
pub type BKPSRAM_HW_ERASE_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBOOT_SEL` reader - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
pub type NBOOT_SEL_R = crate::BitReader;
///Field `NBOOT_SEL` writer - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
pub type NBOOT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBOOT1` reader - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration.
pub type NBOOT1_R = crate::BitReader;
///Field `NBOOT1` writer - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration.
pub type NBOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBOOT0` reader - NBOOT0 option bit
pub type NBOOT0_R = crate::BitReader;
///Field `NBOOT0` writer - NBOOT0 option bit
pub type NBOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_MODE` reader - NRST pin configuration
pub type NRST_MODE_R = crate::FieldReader;
///Field `NRST_MODE` writer - NRST pin configuration
pub type NRST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IRHEN` reader - Internal reset holder enable bit
pub type IRHEN_R = crate::BitReader;
///Field `IRHEN` writer - Internal reset holder enable bit
pub type IRHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Read protection level Other: Level 1, memories read protection active
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - BOR reset level
    #[inline(always)]
    pub fn borr_lev(&self) -> BORR_LEV_R {
        BORR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 13 - Reset generated when entering Stop mode
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Reset generated when entering Standby mode
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Reset generated when entering Shutdown mode
    #[inline(always)]
    pub fn nrst_shdw(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Backup domain reset
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRAM parity check control enable/disable
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Backup SRAM erase prevention
    #[inline(always)]
    pub fn bkpsram_hw_erase_disable(&self) -> BKPSRAM_HW_ERASE_DISABLE_R {
        BKPSRAM_HW_ERASE_DISABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
    #[inline(always)]
    pub fn nboot_sel(&self) -> NBOOT_SEL_R {
        NBOOT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration.
    #[inline(always)]
    pub fn nboot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - NBOOT0 option bit
    #[inline(always)]
    pub fn nboot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - NRST pin configuration
    #[inline(always)]
    pub fn nrst_mode(&self) -> NRST_MODE_R {
        NRST_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&self) -> IRHEN_R {
        IRHEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTR")
            .field("rdp", &self.rdp())
            .field("borr_lev", &self.borr_lev())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stdby", &self.nrst_stdby())
            .field("nrst_shdw", &self.nrst_shdw())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwdg_stdby", &self.iwdg_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("bdrst", &self.bdrst())
            .field("ram_parity_check", &self.ram_parity_check())
            .field("bkpsram_hw_erase_disable", &self.bkpsram_hw_erase_disable())
            .field("nboot_sel", &self.nboot_sel())
            .field("nboot1", &self.nboot1())
            .field("nboot0", &self.nboot0())
            .field("nrst_mode", &self.nrst_mode())
            .field("irhen", &self.irhen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Read protection level Other: Level 1, memories read protection active
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bits 8:10 - BOR reset level
    #[inline(always)]
    pub fn borr_lev(&mut self) -> BORR_LEV_W<'_, OPTRrs> {
        BORR_LEV_W::new(self, 8)
    }
    ///Bit 13 - Reset generated when entering Stop mode
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<'_, OPTRrs> {
        NRST_STOP_W::new(self, 13)
    }
    ///Bit 14 - Reset generated when entering Standby mode
    #[inline(always)]
    pub fn nrst_stdby(&mut self) -> NRST_STDBY_W<'_, OPTRrs> {
        NRST_STDBY_W::new(self, 14)
    }
    ///Bit 15 - Reset generated when entering Shutdown mode
    #[inline(always)]
    pub fn nrst_shdw(&mut self) -> NRST_SHDW_W<'_, OPTRrs> {
        NRST_SHDW_W::new(self, 15)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<'_, OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<'_, OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<'_, OPTRrs> {
        IWDG_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<'_, OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 21 - Backup domain reset
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<'_, OPTRrs> {
        BDRST_W::new(self, 21)
    }
    ///Bit 22 - SRAM parity check control enable/disable
    #[inline(always)]
    pub fn ram_parity_check(&mut self) -> RAM_PARITY_CHECK_W<'_, OPTRrs> {
        RAM_PARITY_CHECK_W::new(self, 22)
    }
    ///Bit 23 - Backup SRAM erase prevention
    #[inline(always)]
    pub fn bkpsram_hw_erase_disable(&mut self) -> BKPSRAM_HW_ERASE_DISABLE_W<'_, OPTRrs> {
        BKPSRAM_HW_ERASE_DISABLE_W::new(self, 23)
    }
    ///Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
    #[inline(always)]
    pub fn nboot_sel(&mut self) -> NBOOT_SEL_W<'_, OPTRrs> {
        NBOOT_SEL_W::new(self, 24)
    }
    ///Bit 25 - Boot configuration Together with the BOOT0 pin or option bit NBOOT0 (depending on NBOOT_SEL option bit configuration), this bit selects boot mode from the main flash memory, SRAM or the system memory. Refer to Section12.5: Boot configuration.
    #[inline(always)]
    pub fn nboot1(&mut self) -> NBOOT1_W<'_, OPTRrs> {
        NBOOT1_W::new(self, 25)
    }
    ///Bit 26 - NBOOT0 option bit
    #[inline(always)]
    pub fn nboot0(&mut self) -> NBOOT0_W<'_, OPTRrs> {
        NBOOT0_W::new(self, 26)
    }
    ///Bits 27:28 - NRST pin configuration
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<'_, OPTRrs> {
        NRST_MODE_W::new(self, 27)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&mut self) -> IRHEN_W<'_, OPTRrs> {
        IRHEN_W::new(self, 29)
    }
}
/**FLASH option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTR to value 0
impl crate::Resettable for OPTRrs {}
