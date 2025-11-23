///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**FMPI2C1_SCL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPI2C4_SCL {
    ///0: `0`
    Clear = 0,
    ///1: forces FM+ drive capability on FMPI2C4_SCL pin
    Forced = 1,
}
impl From<FMPI2C4_SCL> for bool {
    #[inline(always)]
    fn from(variant: FMPI2C4_SCL) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPI2C4_SCL` reader - FMPI2C1_SCL
pub type FMPI2C4_SCL_R = crate::BitReader<FMPI2C4_SCL>;
impl FMPI2C4_SCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMPI2C4_SCL {
        match self.bits {
            false => FMPI2C4_SCL::Clear,
            true => FMPI2C4_SCL::Forced,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FMPI2C4_SCL::Clear
    }
    ///forces FM+ drive capability on FMPI2C4_SCL pin
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == FMPI2C4_SCL::Forced
    }
}
///Field `FMPI2C4_SCL` writer - FMPI2C1_SCL
pub type FMPI2C4_SCL_W<'a, REG> = crate::BitWriter<'a, REG, FMPI2C4_SCL>;
impl<'a, REG> FMPI2C4_SCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`0`
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C4_SCL::Clear)
    }
    ///forces FM+ drive capability on FMPI2C4_SCL pin
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C4_SCL::Forced)
    }
}
/**FMPI2C1_SDA

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPI2C4_SDA {
    ///0: `0`
    Clear = 0,
    ///1: forces FM+ drive capability on FMPI2C4_SDA pin
    Forced = 1,
}
impl From<FMPI2C4_SDA> for bool {
    #[inline(always)]
    fn from(variant: FMPI2C4_SDA) -> Self {
        variant as u8 != 0
    }
}
///Field `FMPI2C4_SDA` reader - FMPI2C1_SDA
pub type FMPI2C4_SDA_R = crate::BitReader<FMPI2C4_SDA>;
impl FMPI2C4_SDA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMPI2C4_SDA {
        match self.bits {
            false => FMPI2C4_SDA::Clear,
            true => FMPI2C4_SDA::Forced,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FMPI2C4_SDA::Clear
    }
    ///forces FM+ drive capability on FMPI2C4_SDA pin
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == FMPI2C4_SDA::Forced
    }
}
///Field `FMPI2C4_SDA` writer - FMPI2C1_SDA
pub type FMPI2C4_SDA_W<'a, REG> = crate::BitWriter<'a, REG, FMPI2C4_SDA>;
impl<'a, REG> FMPI2C4_SDA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///`0`
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C4_SDA::Clear)
    }
    ///forces FM+ drive capability on FMPI2C4_SDA pin
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(FMPI2C4_SDA::Forced)
    }
}
impl R {
    ///Bit 0 - FMPI2C1_SCL
    #[inline(always)]
    pub fn fmpi2c4_scl(&self) -> FMPI2C4_SCL_R {
        FMPI2C4_SCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FMPI2C1_SDA
    #[inline(always)]
    pub fn fmpi2c4_sda(&self) -> FMPI2C4_SDA_R {
        FMPI2C4_SDA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("fmpi2c4_scl", &self.fmpi2c4_scl())
            .field("fmpi2c4_sda", &self.fmpi2c4_sda())
            .finish()
    }
}
impl W {
    ///Bit 0 - FMPI2C1_SCL
    #[inline(always)]
    pub fn fmpi2c4_scl(&mut self) -> FMPI2C4_SCL_W<'_, CFGRrs> {
        FMPI2C4_SCL_W::new(self, 0)
    }
    ///Bit 1 - FMPI2C1_SDA
    #[inline(always)]
    pub fn fmpi2c4_sda(&mut self) -> FMPI2C4_SDA_W<'_, CFGRrs> {
        FMPI2C4_SDA_W::new(self, 1)
    }
}
/**Configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#SYSCFG:CFGR)*/
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
