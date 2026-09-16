///Register `EVCR` reader
pub type R = crate::R<EVCRrs>;
///Register `EVCR` writer
pub type W = crate::W<EVCRrs>;
///Field `EV1EN` reader - event 1 enable
pub type EV1EN_R = crate::BitReader;
///Field `EV1EN` writer - event 1 enable
pub type EV1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EV2EN` reader - event 2 enable
pub type EV2EN_R = crate::BitReader;
///Field `EV2EN` writer - event 2 enable
pub type EV2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EV3EN` reader - event 3 enable
pub type EV3EN_R = crate::BitReader;
///Field `EV3EN` writer - event 3 enable
pub type EV3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EV4EN` reader - event 4 enable
pub type EV4EN_R = crate::BitReader;
///Field `EV4EN` writer - event 4 enable
pub type EV4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - event 1 enable
    #[inline(always)]
    pub fn ev1en(&self) -> EV1EN_R {
        EV1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - event 2 enable
    #[inline(always)]
    pub fn ev2en(&self) -> EV2EN_R {
        EV2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - event 3 enable
    #[inline(always)]
    pub fn ev3en(&self) -> EV3EN_R {
        EV3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - event 4 enable
    #[inline(always)]
    pub fn ev4en(&self) -> EV4EN_R {
        EV4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVCR")
            .field("ev1en", &self.ev1en())
            .field("ev2en", &self.ev2en())
            .field("ev3en", &self.ev3en())
            .field("ev4en", &self.ev4en())
            .finish()
    }
}
impl W {
    ///Bit 0 - event 1 enable
    #[inline(always)]
    pub fn ev1en(&mut self) -> EV1EN_W<'_, EVCRrs> {
        EV1EN_W::new(self, 0)
    }
    ///Bit 1 - event 2 enable
    #[inline(always)]
    pub fn ev2en(&mut self) -> EV2EN_W<'_, EVCRrs> {
        EV2EN_W::new(self, 1)
    }
    ///Bit 2 - event 3 enable
    #[inline(always)]
    pub fn ev3en(&mut self) -> EV3EN_W<'_, EVCRrs> {
        EV3EN_W::new(self, 2)
    }
    ///Bit 3 - event 4 enable
    #[inline(always)]
    pub fn ev4en(&mut self) -> EV4EN_W<'_, EVCRrs> {
        EV4EN_W::new(self, 3)
    }
}
/**GFXTIM events control register

You can [`read`](crate::Reg::read) this register and get [`evcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:EVCR)*/
pub struct EVCRrs;
impl crate::RegisterSpec for EVCRrs {
    type Ux = u32;
}
///`read()` method returns [`evcr::R`](R) reader structure
impl crate::Readable for EVCRrs {}
///`write(|w| ..)` method takes [`evcr::W`](W) writer structure
impl crate::Writable for EVCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EVCR to value 0
impl crate::Resettable for EVCRrs {}
