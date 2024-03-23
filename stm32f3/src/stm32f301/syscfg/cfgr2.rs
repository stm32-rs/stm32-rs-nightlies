#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "PVD lock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVD_LOCK {
    #[doc = "0: PVD interrupt disconnected from TIM15/16/17 Break input"]
    Disconnected = 0,
    #[doc = "1: PVD interrupt connected to TIM15/16/17 Break input"]
    Connected = 1,
}
impl From<PVD_LOCK> for bool {
    #[inline(always)]
    fn from(variant: PVD_LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PVD_LOCK_R = crate::BitReader<PVD_LOCK>;
impl PVD_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVD_LOCK {
        match self.bits {
            false => PVD_LOCK::Disconnected,
            true => PVD_LOCK::Connected,
        }
    }
    #[doc = "PVD interrupt disconnected from TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PVD_LOCK::Disconnected
    }
    #[doc = "PVD interrupt connected to TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == PVD_LOCK::Connected
    }
}
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PVD_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, PVD_LOCK>;
impl<'a, REG> PVD_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVD interrupt disconnected from TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(PVD_LOCK::Disconnected)
    }
    #[doc = "PVD interrupt connected to TIM15/16/17 Break input"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(PVD_LOCK::Connected)
    }
}
impl R {
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<CFGR2rs> {
        PVD_LOCK_W::new(self, 2)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
