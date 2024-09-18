///Register `TZSC_MPCWM2ACFGR` reader
pub type R = crate::R<TZSC_MPCWM2ACFGRrs>;
///Register `TZSC_MPCWM2ACFGR` writer
pub type W = crate::W<TZSC_MPCWM2ACFGRrs>;
///Field `SREN` reader - Sub-region enable
pub type SREN_R = crate::BitReader;
///Field `SREN` writer - Sub-region enable
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRLOCK` reader - Sub-region lock
pub type SRLOCK_R = crate::BitReader;
///Field `SRLOCK` writer - Sub-region lock
pub type SRLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC` reader - Secure sub-region
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - Secure sub-region
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - Privileged sub-region
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - Privileged sub-region
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sub-region enable
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sub-region lock
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Secure sub-region
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Privileged sub-region
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_MPCWM2ACFGR")
            .field("sren", &self.sren())
            .field("srlock", &self.srlock())
            .field("sec", &self.sec())
            .field("priv_", &self.priv_())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sub-region enable
    #[inline(always)]
    #[must_use]
    pub fn sren(&mut self) -> SREN_W<TZSC_MPCWM2ACFGRrs> {
        SREN_W::new(self, 0)
    }
    ///Bit 1 - Sub-region lock
    #[inline(always)]
    #[must_use]
    pub fn srlock(&mut self) -> SRLOCK_W<TZSC_MPCWM2ACFGRrs> {
        SRLOCK_W::new(self, 1)
    }
    ///Bit 8 - Secure sub-region
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<TZSC_MPCWM2ACFGRrs> {
        SEC_W::new(self, 8)
    }
    ///Bit 9 - Privileged sub-region
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<TZSC_MPCWM2ACFGRrs> {
        PRIV_W::new(self, 9)
    }
}
/**TZSC memory 2 sub-region A watermark configuration register

You can [`read`](crate::Reg::read) this register and get [`tzsc_mpcwm2acfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_mpcwm2acfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_TZSC:TZSC_MPCWM2ACFGR)*/
pub struct TZSC_MPCWM2ACFGRrs;
impl crate::RegisterSpec for TZSC_MPCWM2ACFGRrs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_mpcwm2acfgr::R`](R) reader structure
impl crate::Readable for TZSC_MPCWM2ACFGRrs {}
///`write(|w| ..)` method takes [`tzsc_mpcwm2acfgr::W`](W) writer structure
impl crate::Writable for TZSC_MPCWM2ACFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TZSC_MPCWM2ACFGR to value 0
impl crate::Resettable for TZSC_MPCWM2ACFGRrs {
    const RESET_VALUE: u32 = 0;
}
