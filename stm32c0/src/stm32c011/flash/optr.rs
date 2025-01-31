///Register `OPTR` reader
pub type R = crate::R<OPTRrs>;
///Register `OPTR` writer
pub type W = crate::W<OPTRrs>;
///Field `RDP` reader - Read protection level Other: Level 1, memories read protection active
pub type RDP_R = crate::FieldReader;
///Field `RDP` writer - Read protection level Other: Level 1, memories read protection active
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOR_EN` reader - Brown out reset enable
pub type BOR_EN_R = crate::BitReader;
///Field `BOR_EN` writer - Brown out reset enable
pub type BOR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BORR_LEV` reader - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
pub type BORR_LEV_R = crate::FieldReader;
///Field `BORR_LEV` writer - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
pub type BORR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BORF_LEV` reader - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
pub type BORF_LEV_R = crate::FieldReader;
///Field `BORF_LEV` writer - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
pub type BORF_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NRST_STOP` reader - None
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - None
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STDBY` reader - None
pub type NRST_STDBY_R = crate::BitReader;
///Field `NRST_STDBY` writer - None
pub type NRST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_SHDW` reader - None
pub type NRST_SHDW_R = crate::BitReader;
///Field `NRST_SHDW` writer - None
pub type NRST_SHDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_SW` reader - None
pub type IWDG_SW_R = crate::BitReader;
///Field `IWDG_SW` writer - None
pub type IWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWGD_STDBY` reader - None
pub type IWGD_STDBY_R = crate::BitReader;
///Field `IWGD_STDBY` writer - None
pub type IWGD_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSE_NOT_REMAPPED` reader - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
pub type HSE_NOT_REMAPPED_R = crate::BitReader;
///Field `HSE_NOT_REMAPPED` writer - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
pub type HSE_NOT_REMAPPED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAM_PARITY_CHECK` reader - SRAM parity check control enable/disable
pub type RAM_PARITY_CHECK_R = crate::BitReader;
///Field `RAM_PARITY_CHECK` writer - SRAM parity check control enable/disable
pub type RAM_PARITY_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECURE_MUXING_EN` reader - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
pub type SECURE_MUXING_EN_R = crate::BitReader;
///Field `SECURE_MUXING_EN` writer - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
pub type SECURE_MUXING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBOOT_SEL` reader - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
pub type NBOOT_SEL_R = crate::BitReader;
///Field `NBOOT_SEL` writer - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
pub type NBOOT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBOOT1` reader - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
pub type NBOOT1_R = crate::BitReader;
///Field `NBOOT1` writer - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
pub type NBOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBOOT0` reader - nBOOT0 option bit
pub type NBOOT0_R = crate::BitReader;
///Field `NBOOT0` writer - nBOOT0 option bit
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
    ///Bit 8 - Brown out reset enable
    #[inline(always)]
    pub fn bor_en(&self) -> BOR_EN_R {
        BOR_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
    #[inline(always)]
    pub fn borr_lev(&self) -> BORR_LEV_R {
        BORR_LEV_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
    #[inline(always)]
    pub fn borf_lev(&self) -> BORF_LEV_R {
        BORF_LEV_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - None
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - None
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - None
    #[inline(always)]
    pub fn nrst_shdw(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - None
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - None
    #[inline(always)]
    pub fn iwgd_stdby(&self) -> IWGD_STDBY_R {
        IWGD_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
    #[inline(always)]
    pub fn hse_not_remapped(&self) -> HSE_NOT_REMAPPED_R {
        HSE_NOT_REMAPPED_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRAM parity check control enable/disable
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
    #[inline(always)]
    pub fn secure_muxing_en(&self) -> SECURE_MUXING_EN_R {
        SECURE_MUXING_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
    #[inline(always)]
    pub fn nboot_sel(&self) -> NBOOT_SEL_R {
        NBOOT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
    #[inline(always)]
    pub fn nboot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - nBOOT0 option bit
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
            .field("bor_en", &self.bor_en())
            .field("borr_lev", &self.borr_lev())
            .field("borf_lev", &self.borf_lev())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stdby", &self.nrst_stdby())
            .field("nrst_shdw", &self.nrst_shdw())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("iwdg_stop", &self.iwdg_stop())
            .field("iwgd_stdby", &self.iwgd_stdby())
            .field("wwdg_sw", &self.wwdg_sw())
            .field("hse_not_remapped", &self.hse_not_remapped())
            .field("ram_parity_check", &self.ram_parity_check())
            .field("secure_muxing_en", &self.secure_muxing_en())
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
    pub fn rdp(&mut self) -> RDP_W<OPTRrs> {
        RDP_W::new(self, 0)
    }
    ///Bit 8 - Brown out reset enable
    #[inline(always)]
    pub fn bor_en(&mut self) -> BOR_EN_W<OPTRrs> {
        BOR_EN_W::new(self, 8)
    }
    ///Bits 9:10 - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
    #[inline(always)]
    pub fn borr_lev(&mut self) -> BORR_LEV_W<OPTRrs> {
        BORR_LEV_W::new(self, 9)
    }
    ///Bits 11:12 - BOR threshold at falling VDD supply Falling VDD crossings this threshold activates the reset signal.
    #[inline(always)]
    pub fn borf_lev(&mut self) -> BORF_LEV_W<OPTRrs> {
        BORF_LEV_W::new(self, 11)
    }
    ///Bit 13 - None
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<OPTRrs> {
        NRST_STOP_W::new(self, 13)
    }
    ///Bit 14 - None
    #[inline(always)]
    pub fn nrst_stdby(&mut self) -> NRST_STDBY_W<OPTRrs> {
        NRST_STDBY_W::new(self, 14)
    }
    ///Bit 15 - None
    #[inline(always)]
    pub fn nrst_shdw(&mut self) -> NRST_SHDW_W<OPTRrs> {
        NRST_SHDW_W::new(self, 15)
    }
    ///Bit 16 - None
    #[inline(always)]
    pub fn iwdg_sw(&mut self) -> IWDG_SW_W<OPTRrs> {
        IWDG_SW_W::new(self, 16)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<OPTRrs> {
        IWDG_STOP_W::new(self, 17)
    }
    ///Bit 18 - None
    #[inline(always)]
    pub fn iwgd_stdby(&mut self) -> IWGD_STDBY_W<OPTRrs> {
        IWGD_STDBY_W::new(self, 18)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<OPTRrs> {
        WWDG_SW_W::new(self, 19)
    }
    ///Bit 21 - HSE remapping enable/disable When cleared, the bit remaps the HSE clock source from PF0-OSC_IN/PF1-OSC_OUT pins to PC14-OSCX_IN/PC15-OSCX_OUT. Thus PC14-OSCX_IN/PC15-OSCX_OUT are shared by both LSE and HSE and the two clock sources cannot be use simultaneously. On packages with less than 48 pins, the remapping is always enabled (PF0-OSC_IN/PF1-OSC_OUT are not available), regardless of this bit. As all STM32C011xx packages have less than 48 pins, this bit is only applicable to STM32C031xx. Note: On 48 pins packages, when HSE_NOT_REMAPPED is reset, HSE cannot be used in bypass mode. Refer to product errata sheet for more details.
    #[inline(always)]
    pub fn hse_not_remapped(&mut self) -> HSE_NOT_REMAPPED_W<OPTRrs> {
        HSE_NOT_REMAPPED_W::new(self, 21)
    }
    ///Bit 22 - SRAM parity check control enable/disable
    #[inline(always)]
    pub fn ram_parity_check(&mut self) -> RAM_PARITY_CHECK_W<OPTRrs> {
        RAM_PARITY_CHECK_W::new(self, 22)
    }
    ///Bit 23 - Multiple-bonding security The bit allows enabling automatic I/O configuration to prevent conflicts on I/Os connected (bonded) onto the same pin. If the software sets one of the I/Os connected to the same pin as active by configuring the SYSCFG_CFGR3 register, enabling this bit automatically forces the other I/Os in digital input mode, regardless of their software configuration. When the bit is disabled, the SYSCFG_CFGR3 register setting is ignored, all GPIOs linked to a given pin are active and can be set in the mode specified by the corresponding GPIOx_MODER register. The user software must ensure that there is no conflict between GPIOs.
    #[inline(always)]
    pub fn secure_muxing_en(&mut self) -> SECURE_MUXING_EN_W<OPTRrs> {
        SECURE_MUXING_EN_W::new(self, 23)
    }
    ///Bit 24 - BOOT0 signal source selection This option bit defines the source of the BOOT0 signal.
    #[inline(always)]
    pub fn nboot_sel(&mut self) -> NBOOT_SEL_W<OPTRrs> {
        NBOOT_SEL_W::new(self, 24)
    }
    ///Bit 25 - Boot configuration Together with the BOOT0 pin or option bit nBOOT0 (depending on nBOOT_SEL option bit configuration), this bit selects boot mode from the Main flash memory, SRAM or the System memory. Refer to Section 3: Boot configuration.
    #[inline(always)]
    pub fn nboot1(&mut self) -> NBOOT1_W<OPTRrs> {
        NBOOT1_W::new(self, 25)
    }
    ///Bit 26 - nBOOT0 option bit
    #[inline(always)]
    pub fn nboot0(&mut self) -> NBOOT0_W<OPTRrs> {
        NBOOT0_W::new(self, 26)
    }
    ///Bits 27:28 - NRST pin configuration
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NRST_MODE_W<OPTRrs> {
        NRST_MODE_W::new(self, 27)
    }
    ///Bit 29 - Internal reset holder enable bit
    #[inline(always)]
    pub fn irhen(&mut self) -> IRHEN_W<OPTRrs> {
        IRHEN_W::new(self, 29)
    }
}
/**FLASH option register

You can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#FLASH:OPTR)*/
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
///`read()` method returns [`optr::R`](R) reader structure
impl crate::Readable for OPTRrs {}
///`write(|w| ..)` method takes [`optr::W`](W) writer structure
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTR to value 0
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0;
}
