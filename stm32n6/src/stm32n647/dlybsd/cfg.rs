///Register `CFG` reader
pub type R = crate::R<CFGrs>;
///Register `CFG` writer
pub type W = crate::W<CFGrs>;
///Field `SDMMC_DLL_EN` reader - DLL enable
pub type SDMMC_DLL_EN_R = crate::BitReader;
///Field `SDMMC_DLL_EN` writer - DLL enable
pub type SDMMC_DLL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC_RX_TAP_SEL` reader - selection of RX delay
pub type SDMMC_RX_TAP_SEL_R = crate::FieldReader;
///Field `SDMMC_RX_TAP_SEL` writer - selection of RX delay
pub type SDMMC_RX_TAP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SDMMC_DLL_BYP_EN` reader - DLL configuration
pub type SDMMC_DLL_BYP_EN_R = crate::BitReader;
///Field `SDMMC_DLL_BYP_EN` writer - DLL configuration
pub type SDMMC_DLL_BYP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC_DLL_BYP_CMD` reader - Bypass command
pub type SDMMC_DLL_BYP_CMD_R = crate::FieldReader;
///Field `SDMMC_DLL_BYP_CMD` writer - Bypass command
pub type SDMMC_DLL_BYP_CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SDMMC_DLL_ANTIGLITCH_EN` reader - Antiglitch logic enabled when 1
pub type SDMMC_DLL_ANTIGLITCH_EN_R = crate::BitReader;
///Field `SDMMC_DLL_ANTIGLITCH_EN` writer - Antiglitch logic enabled when 1
pub type SDMMC_DLL_ANTIGLITCH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DLL enable
    #[inline(always)]
    pub fn sdmmc_dll_en(&self) -> SDMMC_DLL_EN_R {
        SDMMC_DLL_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:6 - selection of RX delay
    #[inline(always)]
    pub fn sdmmc_rx_tap_sel(&self) -> SDMMC_RX_TAP_SEL_R {
        SDMMC_RX_TAP_SEL_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    ///Bit 16 - DLL configuration
    #[inline(always)]
    pub fn sdmmc_dll_byp_en(&self) -> SDMMC_DLL_BYP_EN_R {
        SDMMC_DLL_BYP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - Bypass command
    #[inline(always)]
    pub fn sdmmc_dll_byp_cmd(&self) -> SDMMC_DLL_BYP_CMD_R {
        SDMMC_DLL_BYP_CMD_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 22 - Antiglitch logic enabled when 1
    #[inline(always)]
    pub fn sdmmc_dll_antiglitch_en(&self) -> SDMMC_DLL_ANTIGLITCH_EN_R {
        SDMMC_DLL_ANTIGLITCH_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("sdmmc_dll_en", &self.sdmmc_dll_en())
            .field("sdmmc_rx_tap_sel", &self.sdmmc_rx_tap_sel())
            .field("sdmmc_dll_byp_en", &self.sdmmc_dll_byp_en())
            .field("sdmmc_dll_byp_cmd", &self.sdmmc_dll_byp_cmd())
            .field("sdmmc_dll_antiglitch_en", &self.sdmmc_dll_antiglitch_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - DLL enable
    #[inline(always)]
    pub fn sdmmc_dll_en(&mut self) -> SDMMC_DLL_EN_W<'_, CFGrs> {
        SDMMC_DLL_EN_W::new(self, 0)
    }
    ///Bits 1:6 - selection of RX delay
    #[inline(always)]
    pub fn sdmmc_rx_tap_sel(&mut self) -> SDMMC_RX_TAP_SEL_W<'_, CFGrs> {
        SDMMC_RX_TAP_SEL_W::new(self, 1)
    }
    ///Bit 16 - DLL configuration
    #[inline(always)]
    pub fn sdmmc_dll_byp_en(&mut self) -> SDMMC_DLL_BYP_EN_W<'_, CFGrs> {
        SDMMC_DLL_BYP_EN_W::new(self, 16)
    }
    ///Bits 17:21 - Bypass command
    #[inline(always)]
    pub fn sdmmc_dll_byp_cmd(&mut self) -> SDMMC_DLL_BYP_CMD_W<'_, CFGrs> {
        SDMMC_DLL_BYP_CMD_W::new(self, 17)
    }
    ///Bit 22 - Antiglitch logic enabled when 1
    #[inline(always)]
    pub fn sdmmc_dll_antiglitch_en(&mut self) -> SDMMC_DLL_ANTIGLITCH_EN_W<'_, CFGrs> {
        SDMMC_DLL_ANTIGLITCH_EN_W::new(self, 22)
    }
}
/**Delay block SDMMC DLL configuration

You can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DLYBSD:CFG)*/
pub struct CFGrs;
impl crate::RegisterSpec for CFGrs {
    type Ux = u32;
}
///`read()` method returns [`cfg::R`](R) reader structure
impl crate::Readable for CFGrs {}
///`write(|w| ..)` method takes [`cfg::W`](W) writer structure
impl crate::Writable for CFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFG to value 0x0040_0000
impl crate::Resettable for CFGrs {
    const RESET_VALUE: u32 = 0x0040_0000;
}
