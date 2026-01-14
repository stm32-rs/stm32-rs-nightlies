///Register `MPCWM1ACFGR` reader
pub type R = crate::R<MPCWM1ACFGRrs>;
///Register `MPCWM1ACFGR` writer
pub type W = crate::W<MPCWM1ACFGRrs>;
///Field `SREN` reader - subregion A enable
pub type SREN_R = crate::BitReader;
///Field `SREN` writer - subregion A enable
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRLOCK` reader - subregion A lock
pub type SRLOCK_R = crate::BitReader;
///Field `SRLOCK` writer - subregion A lock
pub type SRLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC` reader - Secure subregion A of base region x
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - Secure subregion A of base region x
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - Privileged subregion A of base region x
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - Privileged subregion A of base region x
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - subregion A enable
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - subregion A lock
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Secure subregion A of base region x
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Privileged subregion A of base region x
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM1ACFGR")
            .field("sren", &self.sren())
            .field("srlock", &self.srlock())
            .field("sec", &self.sec())
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bit 0 - subregion A enable
    #[inline(always)]
    pub fn sren(&mut self) -> SREN_W<'_, MPCWM1ACFGRrs> {
        SREN_W::new(self, 0)
    }
    ///Bit 1 - subregion A lock
    #[inline(always)]
    pub fn srlock(&mut self) -> SRLOCK_W<'_, MPCWM1ACFGRrs> {
        SRLOCK_W::new(self, 1)
    }
    ///Bit 8 - Secure subregion A of base region x
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<'_, MPCWM1ACFGRrs> {
        SEC_W::new(self, 8)
    }
    ///Bit 9 - Privileged subregion A of base region x
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, MPCWM1ACFGRrs> {
        PRIV_W::new(self, 9)
    }
}
/**GTZC1 TZSC memory 1 subregion A watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`mpcwm1acfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1acfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#GTZC1_TZSC:MPCWM1ACFGR)*/
pub struct MPCWM1ACFGRrs;
impl crate::RegisterSpec for MPCWM1ACFGRrs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm1acfgr::R`](R) reader structure
impl crate::Readable for MPCWM1ACFGRrs {}
///`write(|w| ..)` method takes [`mpcwm1acfgr::W`](W) writer structure
impl crate::Writable for MPCWM1ACFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM1ACFGR to value 0
impl crate::Resettable for MPCWM1ACFGRrs {}
