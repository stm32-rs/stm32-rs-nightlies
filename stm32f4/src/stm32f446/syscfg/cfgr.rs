///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**Forces FM+ drive capability on I2CFMP1_SCL pin

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPI2C1_SCL {
    ///0: `0`
    Clear = 0,
    ///1: forces FM+ drive capability on FMPI2C1_SCL pin
    Forced = 1,
}
impl From<FMPI2C1_SCL> for bool {
    #[inline(always)]
    fn from(variant: FMPI2C1_SCL) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPI2C1_SCL` reader - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SCL_R = crate::BitReader<FMPI2C1_SCL>;
impl FMPI2C1_SCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMPI2C1_SCL {
        match self.bits {
            false => FMPI2C1_SCL::Clear,
            true => FMPI2C1_SCL::Forced,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FMPI2C1_SCL::Clear
    }
    ///forces FM+ drive capability on FMPI2C1_SCL pin
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == FMPI2C1_SCL::Forced
    }
}
///Field `FMPI2C1_SCL` writer - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SCL_W<'a, REG> = crate::BitWriter<'a, REG, FMPI2C1_SCL>;
impl<'a, REG> FMPI2C1_SCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`0`
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1_SCL::Clear)
    }
    ///forces FM+ drive capability on FMPI2C1_SCL pin
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1_SCL::Forced)
    }
}
/**Forces FM+ drive capability on I2CFMP1_SCL pin

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPI2C1_SDA {
    ///0: `0`
    Clear = 0,
    ///1: forces FM+ drive capability on FMPI2C1_SDA pin
    Forced = 1,
}
impl From<FMPI2C1_SDA> for bool {
    #[inline(always)]
    fn from(variant: FMPI2C1_SDA) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPI2C1_SDA` reader - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SDA_R = crate::BitReader<FMPI2C1_SDA>;
impl FMPI2C1_SDA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMPI2C1_SDA {
        match self.bits {
            false => FMPI2C1_SDA::Clear,
            true => FMPI2C1_SDA::Forced,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FMPI2C1_SDA::Clear
    }
    ///forces FM+ drive capability on FMPI2C1_SDA pin
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == FMPI2C1_SDA::Forced
    }
}
///Field `FMPI2C1_SDA` writer - Forces FM+ drive capability on I2CFMP1_SCL pin
pub type FMPI2C1_SDA_W<'a, REG> = crate::BitWriter<'a, REG, FMPI2C1_SDA>;
impl<'a, REG> FMPI2C1_SDA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`0`
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1_SDA::Clear)
    }
    ///forces FM+ drive capability on FMPI2C1_SDA pin
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C1_SDA::Forced)
    }
}
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
    pub fn fmpi2c1_scl(&mut self) -> FMPI2C1_SCL_W<'_, CFGRrs> {
        FMPI2C1_SCL_W::new(self, 0)
    }
    ///Bit 1 - Forces FM+ drive capability on I2CFMP1_SCL pin
    #[inline(always)]
    pub fn fmpi2c1_sda(&mut self) -> FMPI2C1_SDA_W<'_, CFGRrs> {
        FMPI2C1_SDA_W::new(self, 1)
    }
}
/**Configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#SYSCFG:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
