///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `PVDE` reader - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
pub type PVDE_R = crate::BitReader;
///Field `PVDE` writer - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDLS` reader - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
pub type PVDLS_R = crate::FieldReader;
///Field `PVDLS` writer - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
pub type PVDLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DBGRET` reader - DBGRET: PA2 and PA3 retention enable after DEEPSTOP 0: PA2, PA3 don't retain their status exiting from DEEPSTOP. (default) 1: PA2, PA3 retain their status exiting from DEEPSTOP.
pub type DBGRET_R = crate::BitReader;
///Field `DBGRET` writer - DBGRET: PA2 and PA3 retention enable after DEEPSTOP 0: PA2, PA3 don't retain their status exiting from DEEPSTOP. (default) 1: PA2, PA3 retain their status exiting from DEEPSTOP.
pub type DBGRET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMRET1` reader - RAMRET1: RAM1 retention during low power mode
pub type RAMRET1_R = crate::BitReader;
///Field `RAMRET1` writer - RAMRET1: RAM1 retention during low power mode
pub type RAMRET1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIORET` reader - GPIORET: GPIO retention enable. 0: GPIO don't retain their status during DEEPSTOP and exiting from DEEPSTOP (default) 1: GPIO retain their status during DEEPSTOP and exiting from DEEPSTOP. Note: it's mandatory to ensure this bit is set before entering DEEPSTOP unless DBRG.DEEPSTOP2 bit is set.
pub type GPIORET_R = crate::BitReader;
///Field `GPIORET` writer - GPIORET: GPIO retention enable. 0: GPIO don't retain their status during DEEPSTOP and exiting from DEEPSTOP (default) 1: GPIO retain their status during DEEPSTOP and exiting from DEEPSTOP. Note: it's mandatory to ensure this bit is set before entering DEEPSTOP unless DBRG.DEEPSTOP2 bit is set.
pub type GPIORET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENTS` reader - ENTS: Enable Temperature Sensor
pub type ENTS_R = crate::BitReader;
///Field `ENTS` writer - ENTS: Enable Temperature Sensor
pub type ENTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - DBGRET: PA2 and PA3 retention enable after DEEPSTOP 0: PA2, PA3 don't retain their status exiting from DEEPSTOP. (default) 1: PA2, PA3 retain their status exiting from DEEPSTOP.
    #[inline(always)]
    pub fn dbgret(&self) -> DBGRET_R {
        DBGRET_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RAMRET1: RAM1 retention during low power mode
    #[inline(always)]
    pub fn ramret1(&self) -> RAMRET1_R {
        RAMRET1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - GPIORET: GPIO retention enable. 0: GPIO don't retain their status during DEEPSTOP and exiting from DEEPSTOP (default) 1: GPIO retain their status during DEEPSTOP and exiting from DEEPSTOP. Note: it's mandatory to ensure this bit is set before entering DEEPSTOP unless DBRG.DEEPSTOP2 bit is set.
    #[inline(always)]
    pub fn gpioret(&self) -> GPIORET_R {
        GPIORET_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ENTS: Enable Temperature Sensor
    #[inline(always)]
    pub fn ents(&self) -> ENTS_R {
        ENTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("pvde", &self.pvde())
            .field("pvdls", &self.pvdls())
            .field("dbgret", &self.dbgret())
            .field("ramret1", &self.ramret1())
            .field("gpioret", &self.gpioret())
            .field("ents", &self.ents())
            .finish()
    }
}
impl W {
    ///Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, CR2rs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - PVDLS\[2:0\] Programmable Voltage Detector Level selection then PVDO=1)
    #[inline(always)]
    pub fn pvdls(&mut self) -> PVDLS_W<'_, CR2rs> {
        PVDLS_W::new(self, 1)
    }
    ///Bit 4 - DBGRET: PA2 and PA3 retention enable after DEEPSTOP 0: PA2, PA3 don't retain their status exiting from DEEPSTOP. (default) 1: PA2, PA3 retain their status exiting from DEEPSTOP.
    #[inline(always)]
    pub fn dbgret(&mut self) -> DBGRET_W<'_, CR2rs> {
        DBGRET_W::new(self, 4)
    }
    ///Bit 5 - RAMRET1: RAM1 retention during low power mode
    #[inline(always)]
    pub fn ramret1(&mut self) -> RAMRET1_W<'_, CR2rs> {
        RAMRET1_W::new(self, 5)
    }
    ///Bit 8 - GPIORET: GPIO retention enable. 0: GPIO don't retain their status during DEEPSTOP and exiting from DEEPSTOP (default) 1: GPIO retain their status during DEEPSTOP and exiting from DEEPSTOP. Note: it's mandatory to ensure this bit is set before entering DEEPSTOP unless DBRG.DEEPSTOP2 bit is set.
    #[inline(always)]
    pub fn gpioret(&mut self) -> GPIORET_W<'_, CR2rs> {
        GPIORET_W::new(self, 8)
    }
    ///Bit 9 - ENTS: Enable Temperature Sensor
    #[inline(always)]
    pub fn ents(&mut self) -> ENTS_W<'_, CR2rs> {
        ENTS_W::new(self, 9)
    }
}
/**CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:CR2)*/
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
