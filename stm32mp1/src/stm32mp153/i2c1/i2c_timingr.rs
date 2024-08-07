///Register `I2C_TIMINGR` reader
pub type R = crate::R<I2C_TIMINGRrs>;
///Register `I2C_TIMINGR` writer
pub type W = crate::W<I2C_TIMINGRrs>;
///Field `SCLL` reader - SCLL
pub type SCLL_R = crate::FieldReader;
///Field `SCLL` writer - SCLL
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLH` reader - SCLH
pub type SCLH_R = crate::FieldReader;
///Field `SCLH` writer - SCLH
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SDADEL` reader - SDADEL
pub type SDADEL_R = crate::FieldReader;
///Field `SDADEL` writer - SDADEL
pub type SDADEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SCLDEL` reader - SCLDEL
pub type SCLDEL_R = crate::FieldReader;
///Field `SCLDEL` writer - SCLDEL
pub type SCLDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PRESC` reader - PRESC
pub type PRESC_R = crate::FieldReader;
///Field `PRESC` writer - PRESC
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - SCLL
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - SCLH
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - SDADEL
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - SCLDEL
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 28:31 - PRESC
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TIMINGR")
            .field("scll", &self.scll())
            .field("sclh", &self.sclh())
            .field("sdadel", &self.sdadel())
            .field("scldel", &self.scldel())
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - SCLL
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<I2C_TIMINGRrs> {
        SCLL_W::new(self, 0)
    }
    ///Bits 8:15 - SCLH
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<I2C_TIMINGRrs> {
        SCLH_W::new(self, 8)
    }
    ///Bits 16:19 - SDADEL
    #[inline(always)]
    #[must_use]
    pub fn sdadel(&mut self) -> SDADEL_W<I2C_TIMINGRrs> {
        SDADEL_W::new(self, 16)
    }
    ///Bits 20:23 - SCLDEL
    #[inline(always)]
    #[must_use]
    pub fn scldel(&mut self) -> SCLDEL_W<I2C_TIMINGRrs> {
        SCLDEL_W::new(self, 20)
    }
    ///Bits 28:31 - PRESC
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<I2C_TIMINGRrs> {
        PRESC_W::new(self, 28)
    }
}
/**Access: No wait states

You can [`read`](crate::Reg::read) this register and get [`i2c_timingr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_timingr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_TIMINGR)*/
pub struct I2C_TIMINGRrs;
impl crate::RegisterSpec for I2C_TIMINGRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c_timingr::R`](R) reader structure
impl crate::Readable for I2C_TIMINGRrs {}
///`write(|w| ..)` method takes [`i2c_timingr::W`](W) writer structure
impl crate::Writable for I2C_TIMINGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C_TIMINGR to value 0
impl crate::Resettable for I2C_TIMINGRrs {
    const RESET_VALUE: u32 = 0;
}
