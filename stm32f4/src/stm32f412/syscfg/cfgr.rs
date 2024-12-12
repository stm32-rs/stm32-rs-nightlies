///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `FMPI2C1_SCL` reader - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SCL_R = crate::BitReader;
///Field `FMPI2C1_SCL` writer - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMPI2C1_SDA` reader - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SDA_R = crate::BitReader;
///Field `FMPI2C1_SDA` writer - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SDA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Forces FM+ drive capability on I2CFMP1_SCL pin
    #[inline(always)]
    pub fn fmpi2c1_scl(&self) -> FMPI2C1_SCL_R {
        FMPI2C1_SCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Forces FM+ drive capability on I2CFMP1_SCL pin
    #[inline(always)]
    pub fn fmpi2c1_sda(&self) -> FMPI2C1_SDA_R {
        FMPI2C1_SDA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("fmpi2c1_scl", &self.fmpi2c1_scl())
            .field("fmpi2c1_sda", &self.fmpi2c1_sda())
            .finish()
    }
}
impl W {
    ///Bit 0 - Forces FM+ drive capability on I2CFMP1_SCL pin
    #[inline(always)]
    pub fn fmpi2c1_scl(&mut self) -> FMPI2C1_SCL_W<CFGRrs> {
        FMPI2C1_SCL_W::new(self, 0)
    }
    ///Bit 1 - Forces FM+ drive capability on I2CFMP1_SCL pin
    #[inline(always)]
    pub fn fmpi2c1_sda(&mut self) -> FMPI2C1_SDA_W<CFGRrs> {
        FMPI2C1_SDA_W::new(self, 1)
    }
}
/**Configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#SYSCFG:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
