///Register `TZSC_MPCWM4BCFGR` reader
pub type R = crate::R<TZSC_MPCWM4BCFGRrs>;
///Register `TZSC_MPCWM4BCFGR` writer
pub type W = crate::W<TZSC_MPCWM4BCFGRrs>;
///Field `SREN` reader - Sub-region z enable
pub type SREN_R = crate::BitReader;
///Field `SREN` writer - Sub-region z enable
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRLOCK` reader - Sub-region z lock This bit, once set, can be cleared only by a system reset.
pub type SRLOCK_R = crate::BitReader;
///Field `SRLOCK` writer - Sub-region z lock This bit, once set, can be cleared only by a system reset.
pub type SRLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - Privileged sub-region z This bit is taken into account only if SREN is set.
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - Privileged sub-region z This bit is taken into account only if SREN is set.
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sub-region z enable
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sub-region z lock This bit, once set, can be cleared only by a system reset.
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - Privileged sub-region z This bit is taken into account only if SREN is set.
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_MPCWM4BCFGR")
            .field("sren", &self.sren())
            .field("srlock", &self.srlock())
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sub-region z enable
    #[inline(always)]
    pub fn sren(&mut self) -> SREN_W<TZSC_MPCWM4BCFGRrs> {
        SREN_W::new(self, 0)
    }
    ///Bit 1 - Sub-region z lock This bit, once set, can be cleared only by a system reset.
    #[inline(always)]
    pub fn srlock(&mut self) -> SRLOCK_W<TZSC_MPCWM4BCFGRrs> {
        SRLOCK_W::new(self, 1)
    }
    ///Bit 9 - Privileged sub-region z This bit is taken into account only if SREN is set.
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<TZSC_MPCWM4BCFGRrs> {
        PRIV_W::new(self, 9)
    }
}
/**GTZC1 TZSC BKPSRAM sub-region B watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm4bcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm4bcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_MPCWM4BCFGR)*/
pub struct TZSC_MPCWM4BCFGRrs;
impl crate::RegisterSpec for TZSC_MPCWM4BCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_mpcwm4bcfgr::R`](R) reader structure
impl crate::Readable for TZSC_MPCWM4BCFGRrs {}
///`write(|w| ..)` method takes [`tzsc_mpcwm4bcfgr::W`](W) writer structure
impl crate::Writable for TZSC_MPCWM4BCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_MPCWM4BCFGR to value 0
impl crate::Resettable for TZSC_MPCWM4BCFGRrs {}
