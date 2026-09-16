///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDE` reader - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDLS` reader - PVDLS\[2:0\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)
pub type PVDLS_R = crate::FieldReader;
///Field `PVDLS` writer - PVDLS\[2:0\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)
pub type PVDLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBGRET` reader - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP.
pub type DBGRET_R = crate::BitReader;
///Field `DBGRET` writer - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP.
pub type DBGRET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMRET1` reader - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)
pub type RAMRET1_R = crate::BitReader;
///Field `RAMRET1` writer - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)
pub type RAMRET1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPREG_FORCE_VH` reader - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it
pub type LPREG_FORCE_VH_R = crate::BitReader;
///Field `LPREG_FORCE_VH` writer - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it
pub type LPREG_FORCE_VH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPREG_VH_STATUS` reader - status LPREG VH (1.2v) during DEEPSTOP - 1: LPREG=1.2V during DEEPSTOP - 0: LPREG=1V during DEEPSTOP
pub type LPREG_VH_STATUS_R = crate::BitReader;
///Field `GPIORET` reader - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)
pub type GPIORET_R = crate::BitReader;
///Field `GPIORET` writer - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)
pub type GPIORET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENTS` reader - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled
pub type ENTS_R = crate::BitReader;
///Field `ENTS` writer - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled
pub type ENTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFREGEN` reader - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)
pub type RFREGEN_R = crate::BitReader;
///Field `RFREGEN` writer - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)
pub type RFREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFREGCEXT` reader - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only
pub type RFREGCEXT_R = crate::BitReader;
///Field `RFREGCEXT` writer - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only
pub type RFREGCEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFREGBYP` reader - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V
pub type RFREGBYP_R = crate::BitReader;
///Field `RFREGBYP` writer - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V
pub type RFREGBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFREGRDY` reader - RFDREGRDY: RF Regulator Ready flag - 1: RF Regulator is ready - 0: RF Regulator is not ready
pub type RFREGRDY_R = crate::BitReader;
///Field `RFREGON_STATUS` reader - RFREGON_STATUS: RF Regulator On Status - 1: RF Regulator is enabled - 0: RF Regulator is disabled
pub type RFREGON_STATUS_R = crate::BitReader;
impl R {
    ///Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - PVDLS\[2:0\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP.
    #[inline(always)]
    pub fn dbgret(&self) -> DBGRET_R {
        DBGRET_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)
    #[inline(always)]
    pub fn ramret1(&self) -> RAMRET1_R {
        RAMRET1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it
    #[inline(always)]
    pub fn lpreg_force_vh(&self) -> LPREG_FORCE_VH_R {
        LPREG_FORCE_VH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - status LPREG VH (1.2v) during DEEPSTOP - 1: LPREG=1.2V during DEEPSTOP - 0: LPREG=1V during DEEPSTOP
    #[inline(always)]
    pub fn lpreg_vh_status(&self) -> LPREG_VH_STATUS_R {
        LPREG_VH_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)
    #[inline(always)]
    pub fn gpioret(&self) -> GPIORET_R {
        GPIORET_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled
    #[inline(always)]
    pub fn ents(&self) -> ENTS_R {
        ENTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)
    #[inline(always)]
    pub fn rfregen(&self) -> RFREGEN_R {
        RFREGEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only
    #[inline(always)]
    pub fn rfregcext(&self) -> RFREGCEXT_R {
        RFREGCEXT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V
    #[inline(always)]
    pub fn rfregbyp(&self) -> RFREGBYP_R {
        RFREGBYP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RFDREGRDY: RF Regulator Ready flag - 1: RF Regulator is ready - 0: RF Regulator is not ready
    #[inline(always)]
    pub fn rfregrdy(&self) -> RFREGRDY_R {
        RFREGRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RFREGON_STATUS: RF Regulator On Status - 1: RF Regulator is enabled - 0: RF Regulator is disabled
    #[inline(always)]
    pub fn rfregon_status(&self) -> RFREGON_STATUS_R {
        RFREGON_STATUS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .field("dbgret", &self.dbgret())
            .field("ramret1", &self.ramret1())
            .field("lpreg_force_vh", &self.lpreg_force_vh())
            .field("lpreg_vh_status", &self.lpreg_vh_status())
            .field("gpioret", &self.gpioret())
            .field("ents", &self.ents())
            .field("rfregen", &self.rfregen())
            .field("rfregcext", &self.rfregcext())
            .field("rfregbyp", &self.rfregbyp())
            .field("rfregrdy", &self.rfregrdy())
            .field("rfregon_status", &self.rfregon_status())
            .finish()
    }
}
impl W {
    ///Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - PVDLS\[2:0\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)
    #[inline(always)]
    pub fn pvdls(&mut self) -> PVDLS_W<'_, CR2rs> {
        PVDLS_W::new(self, 1)
    }
    ///Bit 4 - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP.
    #[inline(always)]
    pub fn dbgret(&mut self) -> DBGRET_W<'_, CR2rs> {
        DBGRET_W::new(self, 4)
    }
    ///Bit 5 - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)
    #[inline(always)]
    pub fn ramret1(&mut self) -> RAMRET1_W<'_, CR2rs> {
        RAMRET1_W::new(self, 5)
    }
    ///Bit 6 - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it
    #[inline(always)]
    pub fn lpreg_force_vh(&mut self) -> LPREG_FORCE_VH_W<'_, CR2rs> {
        LPREG_FORCE_VH_W::new(self, 6)
    }
    ///Bit 8 - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)
    #[inline(always)]
    pub fn gpioret(&mut self) -> GPIORET_W<'_, CR2rs> {
        GPIORET_W::new(self, 8)
    }
    ///Bit 9 - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled
    #[inline(always)]
    pub fn ents(&mut self) -> ENTS_W<'_, CR2rs> {
        ENTS_W::new(self, 9)
    }
    ///Bit 10 - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)
    #[inline(always)]
    pub fn rfregen(&mut self) -> RFREGEN_W<'_, CR2rs> {
        RFREGEN_W::new(self, 10)
    }
    ///Bit 11 - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only
    #[inline(always)]
    pub fn rfregcext(&mut self) -> RFREGCEXT_W<'_, CR2rs> {
        RFREGCEXT_W::new(self, 11)
    }
    ///Bit 12 - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V
    #[inline(always)]
    pub fn rfregbyp(&mut self) -> RFREGBYP_W<'_, CR2rs> {
        RFREGBYP_W::new(self, 12)
    }
}
/**CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
