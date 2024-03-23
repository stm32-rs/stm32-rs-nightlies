#[doc = "Register `SYSCFG_CMPENCLRR` reader"]
pub type R = crate::R<SYSCFG_CMPENCLRRrs>;
#[doc = "Register `SYSCFG_CMPENCLRR` writer"]
pub type W = crate::W<SYSCFG_CMPENCLRRrs>;
#[doc = "Field `MPU_EN` reader - MPU_EN"]
pub type MPU_EN_R = crate::BitReader;
#[doc = "Field `MPU_EN` writer - MPU_EN"]
pub type MPU_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_EN` reader - MCU_EN"]
pub type MCU_EN_R = crate::BitReader;
#[doc = "Field `MCU_EN` writer - MCU_EN"]
pub type MCU_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MPU_EN"]
    #[inline(always)]
    pub fn mpu_en(&self) -> MPU_EN_R {
        MPU_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCU_EN"]
    #[inline(always)]
    pub fn mcu_en(&self) -> MCU_EN_R {
        MCU_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU_EN"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_en(&mut self) -> MPU_EN_W<SYSCFG_CMPENCLRRrs> {
        MPU_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MCU_EN"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_en(&mut self) -> MCU_EN_W<SYSCFG_CMPENCLRRrs> {
        MCU_EN_W::new(self, 1)
    }
}
#[doc = "SYSCFG compensation cell enable set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cmpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cmpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_CMPENCLRRrs;
impl crate::RegisterSpec for SYSCFG_CMPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cmpenclrr::R`](R) reader structure"]
impl crate::Readable for SYSCFG_CMPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cmpenclrr::W`](W) writer structure"]
impl crate::Writable for SYSCFG_CMPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_CMPENCLRR to value 0"]
impl crate::Resettable for SYSCFG_CMPENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
