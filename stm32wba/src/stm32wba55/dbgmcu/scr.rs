///Register `SCR` reader
pub type R = crate::R<SCRrs>;
///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `DBG_STOP` reader - Allows debug in Stop mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillators is used as system clock during Stop debug mode, allowing CPU debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.
pub type DBG_STOP_R = crate::BitReader;
///Field `DBG_STOP` writer - Allows debug in Stop mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillators is used as system clock during Stop debug mode, allowing CPU debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STANDBY` reader - Allows debug in Standby mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillator is used as system clock, the supply and SRAM memory content is maintained during Standby debug mode, allowing CPU debug capability. On exit from Standby mode, a standby reset is performed.
pub type DBG_STANDBY_R = crate::BitReader;
///Field `DBG_STANDBY` writer - Allows debug in Standby mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillator is used as system clock, the supply and SRAM memory content is maintained during Standby debug mode, allowing CPU debug capability. On exit from Standby mode, a standby reset is performed.
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPMS` reader - Device low power mode selected 10x: Standby mode others reserved
pub type LPMS_R = crate::FieldReader;
///Field `STOPF` reader - Device Stop flag
pub type STOPF_R = crate::BitReader;
///Field `SBF` reader - Device Standby flag
pub type SBF_R = crate::BitReader;
///Field `CS` reader - CPU Sleep
pub type CS_R = crate::BitReader;
///Field `CDS` reader - CPU DeepSleep
pub type CDS_R = crate::BitReader;
impl R {
    ///Bit 1 - Allows debug in Stop mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillators is used as system clock during Stop debug mode, allowing CPU debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Allows debug in Standby mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillator is used as system clock, the supply and SRAM memory content is maintained during Standby debug mode, allowing CPU debug capability. On exit from Standby mode, a standby reset is performed.
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 16:18 - Device low power mode selected 10x: Standby mode others reserved
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Device Stop flag
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Device Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - CPU Sleep
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CPU DeepSleep
    #[inline(always)]
    pub fn cds(&self) -> CDS_R {
        CDS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("lpms", &self.lpms())
            .field("stopf", &self.stopf())
            .field("sbf", &self.sbf())
            .field("cs", &self.cs())
            .field("cds", &self.cds())
            .finish()
    }
}
impl W {
    ///Bit 1 - Allows debug in Stop mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillators is used as system clock during Stop debug mode, allowing CPU debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<'_, SCRrs> {
        DBG_STOP_W::new(self, 1)
    }
    ///Bit 2 - Allows debug in Standby mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and DBGMCU clocks remain active and the HSI16 oscillator is used as system clock, the supply and SRAM memory content is maintained during Standby debug mode, allowing CPU debug capability. On exit from Standby mode, a standby reset is performed.
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<'_, SCRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
}
/**DBGMCU status and configuration register

You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#DBGMCU:SCR)*/
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for SCRrs {}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {}
