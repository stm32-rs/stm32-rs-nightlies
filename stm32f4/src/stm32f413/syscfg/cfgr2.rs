///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**core lockup lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLL {
    ///0: Cortex-M4 LOCKUP output not connected to TIM1/8 Break input
    NotConnected = 0,
    ///1: Cortex-M4 LOCKUP output connected to TIM1/8 Break input
    Connected = 1,
}
impl From<CLL> for bool {
    #[inline(always)]
    fn from(variant: CLL) -> Self {
        variant as u8 != 0
    }
}
///Field `CLL` reader - core lockup lock
pub type CLL_R = crate::BitReader<CLL>;
impl CLL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CLL {
        match self.bits {
            false => CLL::NotConnected,
            true => CLL::Connected,
        }
    }
    ///Cortex-M4 LOCKUP output not connected to TIM1/8 Break input
    #[inline(always)]
    pub fn is_not_connected(&self) -> bool {
        *self == CLL::NotConnected
    }
    ///Cortex-M4 LOCKUP output connected to TIM1/8 Break input
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CLL::Connected
    }
}
///Field `CLL` writer - core lockup lock
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLL>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cortex-M4 LOCKUP output not connected to TIM1/8 Break input
    #[inline(always)]
    pub fn not_connected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::NotConnected)
    }
    ///Cortex-M4 LOCKUP output connected to TIM1/8 Break input
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(CLL::Connected)
    }
}
/**PVD lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDL {
    ///0: PVD interrupt not connected to TIM1/8 Break input. PVDE and PVDS\[2:0\] can be read and modified
    NotConnected = 0,
    ///1: PVD interrupt connected to TIM1/8 Break input. PVDE and PVDS\[2:0\] are read-only
    Connected = 1,
}
impl From<PVDL> for bool {
    #[inline(always)]
    fn from(variant: PVDL) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDL` reader - PVD lock
pub type PVDL_R = crate::BitReader<PVDL>;
impl PVDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDL {
        match self.bits {
            false => PVDL::NotConnected,
            true => PVDL::Connected,
        }
    }
    ///PVD interrupt not connected to TIM1/8 Break input. PVDE and PVDS\[2:0\] can be read and modified
    #[inline(always)]
    pub fn is_not_connected(&self) -> bool {
        *self == PVDL::NotConnected
    }
    ///PVD interrupt connected to TIM1/8 Break input. PVDE and PVDS\[2:0\] are read-only
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVDL::Connected
    }
}
///Field `PVDL` writer - PVD lock
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG, PVDL>;
impl<'a, REG> PVDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD interrupt not connected to TIM1/8 Break input. PVDE and PVDS\[2:0\] can be read and modified
    #[inline(always)]
    pub fn not_connected(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL::NotConnected)
    }
    ///PVD interrupt connected to TIM1/8 Break input. PVDE and PVDS\[2:0\] are read-only
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL::Connected)
    }
}
impl R {
    ///Bit 0 - core lockup lock
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PVD lock
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("pvdl", &self.pvdl())
            .field("cll", &self.cll())
            .finish()
    }
}
impl W {
    ///Bit 0 - core lockup lock
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W<'_, CFGR2rs> {
        CLL_W::new(self, 0)
    }
    ///Bit 2 - PVD lock
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGR2rs> {
        PVDL_W::new(self, 2)
    }
}
/**ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
