///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**I2C4SCL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2CFMP1_SCL {
    ///0: `0`
    Clear = 0,
    ///1: forces FM+ drive capability on I2CFMP1_SCL pin
    Forced = 1,
}
impl From<I2CFMP1_SCL> for bool {
    #[inline(always)]
    fn from(variant: I2CFMP1_SCL) -> Self {
        variant as u8 != 0
    }
}
///Field `I2CFMP1_SCL` reader - I2C4SCL
pub type I2CFMP1_SCL_R = crate::BitReader<I2CFMP1_SCL>;
impl I2CFMP1_SCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2CFMP1_SCL {
        match self.bits {
            false => I2CFMP1_SCL::Clear,
            true => I2CFMP1_SCL::Forced,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == I2CFMP1_SCL::Clear
    }
    ///forces FM+ drive capability on I2CFMP1_SCL pin
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == I2CFMP1_SCL::Forced
    }
}
///Field `I2CFMP1_SCL` writer - I2C4SCL
pub type I2CFMP1_SCL_W<'a, REG> = crate::BitWriter<'a, REG, I2CFMP1_SCL>;
impl<'a, REG> I2CFMP1_SCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`0`
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(I2CFMP1_SCL::Clear)
    }
    ///forces FM+ drive capability on I2CFMP1_SCL pin
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(I2CFMP1_SCL::Forced)
    }
}
/**I2C4SDA

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2CFMP1_SDA {
    ///0: `0`
    Clear = 0,
    ///1: forces FM+ drive capability on I2CFMP1_SDA pin
    Forced = 1,
}
impl From<I2CFMP1_SDA> for bool {
    #[inline(always)]
    fn from(variant: I2CFMP1_SDA) -> Self {
        variant as u8 != 0
    }
}
///Field `I2CFMP1_SDA` reader - I2C4SDA
pub type I2CFMP1_SDA_R = crate::BitReader<I2CFMP1_SDA>;
impl I2CFMP1_SDA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2CFMP1_SDA {
        match self.bits {
            false => I2CFMP1_SDA::Clear,
            true => I2CFMP1_SDA::Forced,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == I2CFMP1_SDA::Clear
    }
    ///forces FM+ drive capability on I2CFMP1_SDA pin
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == I2CFMP1_SDA::Forced
    }
}
///Field `I2CFMP1_SDA` writer - I2C4SDA
pub type I2CFMP1_SDA_W<'a, REG> = crate::BitWriter<'a, REG, I2CFMP1_SDA>;
impl<'a, REG> I2CFMP1_SDA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`0`
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(I2CFMP1_SDA::Clear)
    }
    ///forces FM+ drive capability on I2CFMP1_SDA pin
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(I2CFMP1_SDA::Forced)
    }
}
impl R {
    ///Bit 0 - I2C4SCL
    #[inline(always)]
    pub fn i2cfmp1_scl(&self) -> I2CFMP1_SCL_R {
        I2CFMP1_SCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4SDA
    #[inline(always)]
    pub fn i2cfmp1_sda(&self) -> I2CFMP1_SDA_R {
        I2CFMP1_SDA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("i2cfmp1_scl", &self.i2cfmp1_scl())
            .field("i2cfmp1_sda", &self.i2cfmp1_sda())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2C4SCL
    #[inline(always)]
    pub fn i2cfmp1_scl(&mut self) -> I2CFMP1_SCL_W<'_, CFGRrs> {
        I2CFMP1_SCL_W::new(self, 0)
    }
    ///Bit 1 - I2C4SDA
    #[inline(always)]
    pub fn i2cfmp1_sda(&mut self) -> I2CFMP1_SDA_W<'_, CFGRrs> {
        I2CFMP1_SDA_W::new(self, 1)
    }
}
/**Configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#SYSCFG:CFGR)*/
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
