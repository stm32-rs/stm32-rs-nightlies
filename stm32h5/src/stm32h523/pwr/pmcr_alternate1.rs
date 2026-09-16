///Register `PMCR_ALTERNATE1` reader
pub type R = crate::R<PMCR_ALTERNATE1rs>;
///Register `PMCR_ALTERNATE1` writer
pub type W = crate::W<PMCR_ALTERNATE1rs>;
///Field `LPMS` reader - low-power mode selection
pub type LPMS_R = crate::BitReader;
///Field `LPMS` writer - low-power mode selection
pub type LPMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SVOS` reader - system Stop mode voltage scaling selection
pub type SVOS_R = crate::FieldReader;
///Field `SVOS` writer - system Stop mode voltage scaling selection
pub type SVOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CSSF` reader - clear Standby and Stop flags (always read as 0)
pub type CSSF_R = crate::BitReader;
///Field `CSSF` writer - clear Standby and Stop flags (always read as 0)
pub type CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLPS` reader - flash memory low-power mode in Stop mode
pub type FLPS_R = crate::BitReader;
///Field `FLPS` writer - flash memory low-power mode in Stop mode
pub type FLPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOSTE` reader - analog switch Vless thansub>BOOSTless than/sub> control
pub type BOOSTE_R = crate::BitReader;
///Field `BOOSTE` writer - analog switch Vless thansub>BOOSTless than/sub> control
pub type BOOSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVD_READY` reader - analog voltage ready
pub type AVD_READY_R = crate::BitReader;
///Field `AVD_READY` writer - analog voltage ready
pub type AVD_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM3SO` reader - AHB SRAM3 shut-off in Stop mode.
pub type SRAM3SO_R = crate::BitReader;
///Field `SRAM3SO` writer - AHB SRAM3 shut-off in Stop mode.
pub type SRAM3SO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_16LSO` reader - AHB SRAM2 low 16-Kbyte shut-off in Stop mode.
pub type SRAM2_16LSO_R = crate::BitReader;
///Field `SRAM2_16LSO` writer - AHB SRAM2 low 16-Kbyte shut-off in Stop mode.
pub type SRAM2_16LSO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_16HSO` reader - AHB SRAM2 high 16-Kbyte shut-off in Stop mode.
pub type SRAM2_16HSO_R = crate::BitReader;
///Field `SRAM2_16HSO` writer - AHB SRAM2 high 16-Kbyte shut-off in Stop mode.
pub type SRAM2_16HSO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2_48SO` reader - AHB SRAM2 48-Kbyte shut-off in Stop mode.
pub type SRAM2_48SO_R = crate::BitReader;
///Field `SRAM2_48SO` writer - AHB SRAM2 48-Kbyte shut-off in Stop mode.
pub type SRAM2_48SO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1SO` reader - AHB SRAM1 shut-off in Stop mode
pub type SRAM1SO_R = crate::BitReader;
///Field `SRAM1SO` writer - AHB SRAM1 shut-off in Stop mode
pub type SRAM1SO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - low-power mode selection
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - system Stop mode voltage scaling selection
    #[inline(always)]
    pub fn svos(&self) -> SVOS_R {
        SVOS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 7 - clear Standby and Stop flags (always read as 0)
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - flash memory low-power mode in Stop mode
    #[inline(always)]
    pub fn flps(&self) -> FLPS_R {
        FLPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - analog switch Vless thansub>BOOSTless than/sub> control
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - analog voltage ready
    #[inline(always)]
    pub fn avd_ready(&self) -> AVD_READY_R {
        AVD_READY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 23 - AHB SRAM3 shut-off in Stop mode.
    #[inline(always)]
    pub fn sram3so(&self) -> SRAM3SO_R {
        SRAM3SO_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - AHB SRAM2 low 16-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_16lso(&self) -> SRAM2_16LSO_R {
        SRAM2_16LSO_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AHB SRAM2 high 16-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_16hso(&self) -> SRAM2_16HSO_R {
        SRAM2_16HSO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - AHB SRAM2 48-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_48so(&self) -> SRAM2_48SO_R {
        SRAM2_48SO_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - AHB SRAM1 shut-off in Stop mode
    #[inline(always)]
    pub fn sram1so(&self) -> SRAM1SO_R {
        SRAM1SO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCR_ALTERNATE1")
            .field("lpms", &self.lpms())
            .field("svos", &self.svos())
            .field("cssf", &self.cssf())
            .field("flps", &self.flps())
            .field("booste", &self.booste())
            .field("avd_ready", &self.avd_ready())
            .field("sram3so", &self.sram3so())
            .field("sram2_16lso", &self.sram2_16lso())
            .field("sram2_16hso", &self.sram2_16hso())
            .field("sram2_48so", &self.sram2_48so())
            .field("sram1so", &self.sram1so())
            .finish()
    }
}
impl W {
    ///Bit 0 - low-power mode selection
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, PMCR_ALTERNATE1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bits 2:3 - system Stop mode voltage scaling selection
    #[inline(always)]
    pub fn svos(&mut self) -> SVOS_W<'_, PMCR_ALTERNATE1rs> {
        SVOS_W::new(self, 2)
    }
    ///Bit 7 - clear Standby and Stop flags (always read as 0)
    #[inline(always)]
    pub fn cssf(&mut self) -> CSSF_W<'_, PMCR_ALTERNATE1rs> {
        CSSF_W::new(self, 7)
    }
    ///Bit 9 - flash memory low-power mode in Stop mode
    #[inline(always)]
    pub fn flps(&mut self) -> FLPS_W<'_, PMCR_ALTERNATE1rs> {
        FLPS_W::new(self, 9)
    }
    ///Bit 12 - analog switch Vless thansub>BOOSTless than/sub> control
    #[inline(always)]
    pub fn booste(&mut self) -> BOOSTE_W<'_, PMCR_ALTERNATE1rs> {
        BOOSTE_W::new(self, 12)
    }
    ///Bit 13 - analog voltage ready
    #[inline(always)]
    pub fn avd_ready(&mut self) -> AVD_READY_W<'_, PMCR_ALTERNATE1rs> {
        AVD_READY_W::new(self, 13)
    }
    ///Bit 23 - AHB SRAM3 shut-off in Stop mode.
    #[inline(always)]
    pub fn sram3so(&mut self) -> SRAM3SO_W<'_, PMCR_ALTERNATE1rs> {
        SRAM3SO_W::new(self, 23)
    }
    ///Bit 24 - AHB SRAM2 low 16-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_16lso(&mut self) -> SRAM2_16LSO_W<'_, PMCR_ALTERNATE1rs> {
        SRAM2_16LSO_W::new(self, 24)
    }
    ///Bit 25 - AHB SRAM2 high 16-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_16hso(&mut self) -> SRAM2_16HSO_W<'_, PMCR_ALTERNATE1rs> {
        SRAM2_16HSO_W::new(self, 25)
    }
    ///Bit 26 - AHB SRAM2 48-Kbyte shut-off in Stop mode.
    #[inline(always)]
    pub fn sram2_48so(&mut self) -> SRAM2_48SO_W<'_, PMCR_ALTERNATE1rs> {
        SRAM2_48SO_W::new(self, 26)
    }
    ///Bit 27 - AHB SRAM1 shut-off in Stop mode
    #[inline(always)]
    pub fn sram1so(&mut self) -> SRAM1SO_W<'_, PMCR_ALTERNATE1rs> {
        SRAM1SO_W::new(self, 27)
    }
}
/**PWR power mode control register

You can [`read`](crate::Reg::read) this register and get [`pmcr_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#PWR:PMCR_ALTERNATE1)*/
pub struct PMCR_ALTERNATE1rs;
impl crate::RegisterSpec for PMCR_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`pmcr_alternate1::R`](R) reader structure
impl crate::Readable for PMCR_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`pmcr_alternate1::W`](W) writer structure
impl crate::Writable for PMCR_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMCR_ALTERNATE1 to value 0x0c
impl crate::Resettable for PMCR_ALTERNATE1rs {
    const RESET_VALUE: u32 = 0x0c;
}
