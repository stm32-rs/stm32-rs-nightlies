///Register `SECCFGR2` reader
pub type R = crate::R<SECCFGR2rs>;
///Register `SECCFGR2` writer
pub type W = crate::W<SECCFGR2rs>;
///Field `IC1SEC` reader - Defines the secure protection of the IC1 divider configuration bits.
pub type IC1SEC_R = crate::BitReader;
///Field `IC1SEC` writer - Defines the secure protection of the IC1 divider configuration bits.
pub type IC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2SEC` reader - Defines the secure protection of the IC2 divider configuration bits.
pub type IC2SEC_R = crate::BitReader;
///Field `IC2SEC` writer - Defines the secure protection of the IC2 divider configuration bits.
pub type IC2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3SEC` reader - Defines the secure protection of the IC3 divider configuration bits.
pub type IC3SEC_R = crate::BitReader;
///Field `IC3SEC` writer - Defines the secure protection of the IC3 divider configuration bits.
pub type IC3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4SEC` reader - Defines the secure protection of the IC4 divider configuration bits.
pub type IC4SEC_R = crate::BitReader;
///Field `IC4SEC` writer - Defines the secure protection of the IC4 divider configuration bits.
pub type IC4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5SEC` reader - Defines the secure protection of the IC5 divider configuration bits.
pub type IC5SEC_R = crate::BitReader;
///Field `IC5SEC` writer - Defines the secure protection of the IC5 divider configuration bits.
pub type IC5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6SEC` reader - Defines the secure protection of the IC6 divider configuration bits.
pub type IC6SEC_R = crate::BitReader;
///Field `IC6SEC` writer - Defines the secure protection of the IC6 divider configuration bits.
pub type IC6SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7SEC` reader - Defines the secure protection of the IC7 divider configuration bits.
pub type IC7SEC_R = crate::BitReader;
///Field `IC7SEC` writer - Defines the secure protection of the IC7 divider configuration bits.
pub type IC7SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8SEC` reader - Defines the secure protection of the IC8 divider configuration bits.
pub type IC8SEC_R = crate::BitReader;
///Field `IC8SEC` writer - Defines the secure protection of the IC8 divider configuration bits.
pub type IC8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9SEC` reader - Defines the secure protection of the IC9 divider configuration bits.
pub type IC9SEC_R = crate::BitReader;
///Field `IC9SEC` writer - Defines the secure protection of the IC9 divider configuration bits.
pub type IC9SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10SEC` reader - Defines the secure protection of the IC10 divider configuration bits.
pub type IC10SEC_R = crate::BitReader;
///Field `IC10SEC` writer - Defines the secure protection of the IC10 divider configuration bits.
pub type IC10SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11SEC` reader - Defines the secure protection of the IC11 divider configuration bits.
pub type IC11SEC_R = crate::BitReader;
///Field `IC11SEC` writer - Defines the secure protection of the IC11 divider configuration bits.
pub type IC11SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12SEC` reader - Defines the secure protection of the IC12 divider configuration bits.
pub type IC12SEC_R = crate::BitReader;
///Field `IC12SEC` writer - Defines the secure protection of the IC12 divider configuration bits.
pub type IC12SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13SEC` reader - Defines the secure protection of the IC13 divider configuration bits.
pub type IC13SEC_R = crate::BitReader;
///Field `IC13SEC` writer - Defines the secure protection of the IC13 divider configuration bits.
pub type IC13SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14SEC` reader - Defines the secure protection of the IC14 divider configuration bits.
pub type IC14SEC_R = crate::BitReader;
///Field `IC14SEC` writer - Defines the secure protection of the IC14 divider configuration bits.
pub type IC14SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15SEC` reader - Defines the secure protection of the IC15 divider configuration bits.
pub type IC15SEC_R = crate::BitReader;
///Field `IC15SEC` writer - Defines the secure protection of the IC15 divider configuration bits.
pub type IC15SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16SEC` reader - Defines the secure protection of the IC16 divider configuration bits.
pub type IC16SEC_R = crate::BitReader;
///Field `IC16SEC` writer - Defines the secure protection of the IC16 divider configuration bits.
pub type IC16SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17SEC` reader - Defines the secure protection of the IC17 divider configuration bits.
pub type IC17SEC_R = crate::BitReader;
///Field `IC17SEC` writer - Defines the secure protection of the IC17 divider configuration bits.
pub type IC17SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18SEC` reader - Defines the secure protection of the IC18 divider configuration bits.
pub type IC18SEC_R = crate::BitReader;
///Field `IC18SEC` writer - Defines the secure protection of the IC18 divider configuration bits.
pub type IC18SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19SEC` reader - Defines the secure protection of the IC19 divider configuration bits.
pub type IC19SEC_R = crate::BitReader;
///Field `IC19SEC` writer - Defines the secure protection of the IC19 divider configuration bits.
pub type IC19SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20SEC` reader - Defines the secure protection of the IC20 divider configuration bits.
pub type IC20SEC_R = crate::BitReader;
///Field `IC20SEC` writer - Defines the secure protection of the IC20 divider configuration bits.
pub type IC20SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the secure protection of the IC1 divider configuration bits.
    #[inline(always)]
    pub fn ic1sec(&self) -> IC1SEC_R {
        IC1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the secure protection of the IC2 divider configuration bits.
    #[inline(always)]
    pub fn ic2sec(&self) -> IC2SEC_R {
        IC2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the secure protection of the IC3 divider configuration bits.
    #[inline(always)]
    pub fn ic3sec(&self) -> IC3SEC_R {
        IC3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the secure protection of the IC4 divider configuration bits.
    #[inline(always)]
    pub fn ic4sec(&self) -> IC4SEC_R {
        IC4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the secure protection of the IC5 divider configuration bits.
    #[inline(always)]
    pub fn ic5sec(&self) -> IC5SEC_R {
        IC5SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the secure protection of the IC6 divider configuration bits.
    #[inline(always)]
    pub fn ic6sec(&self) -> IC6SEC_R {
        IC6SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the secure protection of the IC7 divider configuration bits.
    #[inline(always)]
    pub fn ic7sec(&self) -> IC7SEC_R {
        IC7SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Defines the secure protection of the IC8 divider configuration bits.
    #[inline(always)]
    pub fn ic8sec(&self) -> IC8SEC_R {
        IC8SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Defines the secure protection of the IC9 divider configuration bits.
    #[inline(always)]
    pub fn ic9sec(&self) -> IC9SEC_R {
        IC9SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Defines the secure protection of the IC10 divider configuration bits.
    #[inline(always)]
    pub fn ic10sec(&self) -> IC10SEC_R {
        IC10SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Defines the secure protection of the IC11 divider configuration bits.
    #[inline(always)]
    pub fn ic11sec(&self) -> IC11SEC_R {
        IC11SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Defines the secure protection of the IC12 divider configuration bits.
    #[inline(always)]
    pub fn ic12sec(&self) -> IC12SEC_R {
        IC12SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Defines the secure protection of the IC13 divider configuration bits.
    #[inline(always)]
    pub fn ic13sec(&self) -> IC13SEC_R {
        IC13SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Defines the secure protection of the IC14 divider configuration bits.
    #[inline(always)]
    pub fn ic14sec(&self) -> IC14SEC_R {
        IC14SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Defines the secure protection of the IC15 divider configuration bits.
    #[inline(always)]
    pub fn ic15sec(&self) -> IC15SEC_R {
        IC15SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Defines the secure protection of the IC16 divider configuration bits.
    #[inline(always)]
    pub fn ic16sec(&self) -> IC16SEC_R {
        IC16SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Defines the secure protection of the IC17 divider configuration bits.
    #[inline(always)]
    pub fn ic17sec(&self) -> IC17SEC_R {
        IC17SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Defines the secure protection of the IC18 divider configuration bits.
    #[inline(always)]
    pub fn ic18sec(&self) -> IC18SEC_R {
        IC18SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Defines the secure protection of the IC19 divider configuration bits.
    #[inline(always)]
    pub fn ic19sec(&self) -> IC19SEC_R {
        IC19SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Defines the secure protection of the IC20 divider configuration bits.
    #[inline(always)]
    pub fn ic20sec(&self) -> IC20SEC_R {
        IC20SEC_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR2")
            .field("ic1sec", &self.ic1sec())
            .field("ic2sec", &self.ic2sec())
            .field("ic3sec", &self.ic3sec())
            .field("ic4sec", &self.ic4sec())
            .field("ic5sec", &self.ic5sec())
            .field("ic6sec", &self.ic6sec())
            .field("ic7sec", &self.ic7sec())
            .field("ic8sec", &self.ic8sec())
            .field("ic9sec", &self.ic9sec())
            .field("ic10sec", &self.ic10sec())
            .field("ic11sec", &self.ic11sec())
            .field("ic12sec", &self.ic12sec())
            .field("ic13sec", &self.ic13sec())
            .field("ic14sec", &self.ic14sec())
            .field("ic15sec", &self.ic15sec())
            .field("ic16sec", &self.ic16sec())
            .field("ic17sec", &self.ic17sec())
            .field("ic18sec", &self.ic18sec())
            .field("ic19sec", &self.ic19sec())
            .field("ic20sec", &self.ic20sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the secure protection of the IC1 divider configuration bits.
    #[inline(always)]
    pub fn ic1sec(&mut self) -> IC1SEC_W<'_, SECCFGR2rs> {
        IC1SEC_W::new(self, 0)
    }
    ///Bit 1 - Defines the secure protection of the IC2 divider configuration bits.
    #[inline(always)]
    pub fn ic2sec(&mut self) -> IC2SEC_W<'_, SECCFGR2rs> {
        IC2SEC_W::new(self, 1)
    }
    ///Bit 2 - Defines the secure protection of the IC3 divider configuration bits.
    #[inline(always)]
    pub fn ic3sec(&mut self) -> IC3SEC_W<'_, SECCFGR2rs> {
        IC3SEC_W::new(self, 2)
    }
    ///Bit 3 - Defines the secure protection of the IC4 divider configuration bits.
    #[inline(always)]
    pub fn ic4sec(&mut self) -> IC4SEC_W<'_, SECCFGR2rs> {
        IC4SEC_W::new(self, 3)
    }
    ///Bit 4 - Defines the secure protection of the IC5 divider configuration bits.
    #[inline(always)]
    pub fn ic5sec(&mut self) -> IC5SEC_W<'_, SECCFGR2rs> {
        IC5SEC_W::new(self, 4)
    }
    ///Bit 5 - Defines the secure protection of the IC6 divider configuration bits.
    #[inline(always)]
    pub fn ic6sec(&mut self) -> IC6SEC_W<'_, SECCFGR2rs> {
        IC6SEC_W::new(self, 5)
    }
    ///Bit 6 - Defines the secure protection of the IC7 divider configuration bits.
    #[inline(always)]
    pub fn ic7sec(&mut self) -> IC7SEC_W<'_, SECCFGR2rs> {
        IC7SEC_W::new(self, 6)
    }
    ///Bit 7 - Defines the secure protection of the IC8 divider configuration bits.
    #[inline(always)]
    pub fn ic8sec(&mut self) -> IC8SEC_W<'_, SECCFGR2rs> {
        IC8SEC_W::new(self, 7)
    }
    ///Bit 8 - Defines the secure protection of the IC9 divider configuration bits.
    #[inline(always)]
    pub fn ic9sec(&mut self) -> IC9SEC_W<'_, SECCFGR2rs> {
        IC9SEC_W::new(self, 8)
    }
    ///Bit 9 - Defines the secure protection of the IC10 divider configuration bits.
    #[inline(always)]
    pub fn ic10sec(&mut self) -> IC10SEC_W<'_, SECCFGR2rs> {
        IC10SEC_W::new(self, 9)
    }
    ///Bit 10 - Defines the secure protection of the IC11 divider configuration bits.
    #[inline(always)]
    pub fn ic11sec(&mut self) -> IC11SEC_W<'_, SECCFGR2rs> {
        IC11SEC_W::new(self, 10)
    }
    ///Bit 11 - Defines the secure protection of the IC12 divider configuration bits.
    #[inline(always)]
    pub fn ic12sec(&mut self) -> IC12SEC_W<'_, SECCFGR2rs> {
        IC12SEC_W::new(self, 11)
    }
    ///Bit 12 - Defines the secure protection of the IC13 divider configuration bits.
    #[inline(always)]
    pub fn ic13sec(&mut self) -> IC13SEC_W<'_, SECCFGR2rs> {
        IC13SEC_W::new(self, 12)
    }
    ///Bit 13 - Defines the secure protection of the IC14 divider configuration bits.
    #[inline(always)]
    pub fn ic14sec(&mut self) -> IC14SEC_W<'_, SECCFGR2rs> {
        IC14SEC_W::new(self, 13)
    }
    ///Bit 14 - Defines the secure protection of the IC15 divider configuration bits.
    #[inline(always)]
    pub fn ic15sec(&mut self) -> IC15SEC_W<'_, SECCFGR2rs> {
        IC15SEC_W::new(self, 14)
    }
    ///Bit 15 - Defines the secure protection of the IC16 divider configuration bits.
    #[inline(always)]
    pub fn ic16sec(&mut self) -> IC16SEC_W<'_, SECCFGR2rs> {
        IC16SEC_W::new(self, 15)
    }
    ///Bit 16 - Defines the secure protection of the IC17 divider configuration bits.
    #[inline(always)]
    pub fn ic17sec(&mut self) -> IC17SEC_W<'_, SECCFGR2rs> {
        IC17SEC_W::new(self, 16)
    }
    ///Bit 17 - Defines the secure protection of the IC18 divider configuration bits.
    #[inline(always)]
    pub fn ic18sec(&mut self) -> IC18SEC_W<'_, SECCFGR2rs> {
        IC18SEC_W::new(self, 17)
    }
    ///Bit 18 - Defines the secure protection of the IC19 divider configuration bits.
    #[inline(always)]
    pub fn ic19sec(&mut self) -> IC19SEC_W<'_, SECCFGR2rs> {
        IC19SEC_W::new(self, 18)
    }
    ///Bit 19 - Defines the secure protection of the IC20 divider configuration bits.
    #[inline(always)]
    pub fn ic20sec(&mut self) -> IC20SEC_W<'_, SECCFGR2rs> {
        IC20SEC_W::new(self, 19)
    }
}
/**RCC divider secure configuration register2

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:SECCFGR2)*/
pub struct SECCFGR2rs;
impl crate::RegisterSpec for SECCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr2::R`](R) reader structure
impl crate::Readable for SECCFGR2rs {}
///`write(|w| ..)` method takes [`seccfgr2::W`](W) writer structure
impl crate::Writable for SECCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR2 to value 0
impl crate::Resettable for SECCFGR2rs {}
