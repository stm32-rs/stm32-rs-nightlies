#[doc = "Register `RCC_MP_BOOTCR` reader"]
pub type R = crate::R<RCC_MP_BOOTCRrs>;
#[doc = "Register `RCC_MP_BOOTCR` writer"]
pub type W = crate::W<RCC_MP_BOOTCRrs>;
#[doc = "Field `MCU_BEN` reader - MCU_BEN"]
pub type MCU_BEN_R = crate::BitReader;
#[doc = "Field `MCU_BEN` writer - MCU_BEN"]
pub type MCU_BEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPU_BEN` reader - MPU_BEN"]
pub type MPU_BEN_R = crate::BitReader;
#[doc = "Field `MPU_BEN` writer - MPU_BEN"]
pub type MPU_BEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCU_BEN"]
    #[inline(always)]
    pub fn mcu_ben(&self) -> MCU_BEN_R {
        MCU_BEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPU_BEN"]
    #[inline(always)]
    pub fn mpu_ben(&self) -> MPU_BEN_R {
        MPU_BEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU_BEN"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ben(&mut self) -> MCU_BEN_W<RCC_MP_BOOTCRrs> {
        MCU_BEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MPU_BEN"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_ben(&mut self) -> MPU_BEN_W<RCC_MP_BOOTCRrs> {
        MPU_BEN_W::new(self, 1)
    }
}
#[doc = "This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_bootcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_bootcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_BOOTCRrs;
impl crate::RegisterSpec for RCC_MP_BOOTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_bootcr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_BOOTCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_bootcr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_BOOTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_BOOTCR to value 0"]
impl crate::Resettable for RCC_MP_BOOTCRrs {
    const RESET_VALUE: u32 = 0;
}
