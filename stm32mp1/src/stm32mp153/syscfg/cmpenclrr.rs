///Register `CMPENCLRR` reader
pub type R = crate::R<CMPENCLRRrs>;
///Register `CMPENCLRR` writer
pub type W = crate::W<CMPENCLRRrs>;
///Field `MPU_EN` reader - MPU_EN
pub type MPU_EN_R = crate::BitReader;
///Field `MPU_EN` writer - MPU_EN
pub type MPU_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_EN` reader - MCU_EN
pub type MCU_EN_R = crate::BitReader;
///Field `MCU_EN` writer - MCU_EN
pub type MCU_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MPU_EN
    #[inline(always)]
    pub fn mpu_en(&self) -> MPU_EN_R {
        MPU_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCU_EN
    #[inline(always)]
    pub fn mcu_en(&self) -> MCU_EN_R {
        MCU_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPENCLRR")
            .field("mpu_en", &self.mpu_en())
            .field("mcu_en", &self.mcu_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - MPU_EN
    #[inline(always)]
    pub fn mpu_en(&mut self) -> MPU_EN_W<'_, CMPENCLRRrs> {
        MPU_EN_W::new(self, 0)
    }
    ///Bit 1 - MCU_EN
    #[inline(always)]
    pub fn mcu_en(&mut self) -> MCU_EN_W<'_, CMPENCLRRrs> {
        MCU_EN_W::new(self, 1)
    }
}
/**SYSCFG compensation cell enable set register

You can [`read`](crate::Reg::read) this register and get [`cmpenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:CMPENCLRR)*/
pub struct CMPENCLRRrs;
impl crate::RegisterSpec for CMPENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`cmpenclrr::R`](R) reader structure
impl crate::Readable for CMPENCLRRrs {}
///`write(|w| ..)` method takes [`cmpenclrr::W`](W) writer structure
impl crate::Writable for CMPENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMPENCLRR to value 0
impl crate::Resettable for CMPENCLRRrs {}
