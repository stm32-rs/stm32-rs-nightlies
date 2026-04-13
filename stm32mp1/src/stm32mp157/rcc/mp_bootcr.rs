///Register `MP_BOOTCR` reader
pub type R = crate::R<MP_BOOTCRrs>;
///Register `MP_BOOTCR` writer
pub type W = crate::W<MP_BOOTCRrs>;
///Field `MCU_BEN` reader - MCU_BEN
pub type MCU_BEN_R = crate::BitReader;
///Field `MCU_BEN` writer - MCU_BEN
pub type MCU_BEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPU_BEN` reader - MPU_BEN
pub type MPU_BEN_R = crate::BitReader;
///Field `MPU_BEN` writer - MPU_BEN
pub type MPU_BEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MCU_BEN
    #[inline(always)]
    pub fn mcu_ben(&self) -> MCU_BEN_R {
        MCU_BEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MPU_BEN
    #[inline(always)]
    pub fn mpu_ben(&self) -> MPU_BEN_R {
        MPU_BEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_BOOTCR")
            .field("mcu_ben", &self.mcu_ben())
            .field("mpu_ben", &self.mpu_ben())
            .finish()
    }
}
impl W {
    ///Bit 0 - MCU_BEN
    #[inline(always)]
    pub fn mcu_ben(&mut self) -> MCU_BEN_W<'_, MP_BOOTCRrs> {
        MCU_BEN_W::new(self, 0)
    }
    ///Bit 1 - MPU_BEN
    #[inline(always)]
    pub fn mpu_ben(&mut self) -> MPU_BEN_W<'_, MP_BOOTCRrs> {
        MPU_BEN_W::new(self, 1)
    }
}
/**This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.

You can [`read`](crate::Reg::read) this register and get [`mp_bootcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_bootcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_BOOTCR)*/
pub struct MP_BOOTCRrs;
impl crate::RegisterSpec for MP_BOOTCRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_bootcr::R`](R) reader structure
impl crate::Readable for MP_BOOTCRrs {}
///`write(|w| ..)` method takes [`mp_bootcr::W`](W) writer structure
impl crate::Writable for MP_BOOTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_BOOTCR to value 0
impl crate::Resettable for MP_BOOTCRrs {}
