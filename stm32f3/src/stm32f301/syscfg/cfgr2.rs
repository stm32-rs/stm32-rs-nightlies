///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**PVD lock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVD_LOCK {
    ///0: PVD interrupt disconnected from TIM15/16/17 Break input
    Disconnected = 0,
    ///1: PVD interrupt connected to TIM15/16/17 Break input
    Connected = 1,
}
impl From<PVD_LOCK> for bool {
    #[inline(always)]
    fn from(variant: PVD_LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `PVD_LOCK` reader - PVD lock enable bit
pub type PVD_LOCK_R = crate::BitReader<PVD_LOCK>;
impl PVD_LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVD_LOCK {
        match self.bits {
            false => PVD_LOCK::Disconnected,
            true => PVD_LOCK::Connected,
        }
    }
    ///PVD interrupt disconnected from TIM15/16/17 Break input
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PVD_LOCK::Disconnected
    }
    ///PVD interrupt connected to TIM15/16/17 Break input
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVD_LOCK::Connected
    }
}
///Field `PVD_LOCK` writer - PVD lock enable bit
pub type PVD_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, PVD_LOCK>;
impl<'a, REG> PVD_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD interrupt disconnected from TIM15/16/17 Break input
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(PVD_LOCK::Disconnected)
    }
    ///PVD interrupt connected to TIM15/16/17 Break input
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(PVD_LOCK::Connected)
    }
}
impl R {
    ///Bit 2 - PVD lock enable bit
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("pvd_lock", &self.pvd_lock())
            .finish()
    }
}
impl W {
    ///Bit 2 - PVD lock enable bit
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<'_, CFGR2rs> {
        PVD_LOCK_W::new(self, 2)
    }
}
/**configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#SYSCFG:CFGR2)*/
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
