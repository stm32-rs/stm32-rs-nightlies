///Register `DIVENR` reader
pub type R = crate::R<DIVENRrs>;
///Register `DIVENR` writer
pub type W = crate::W<DIVENRrs>;
///Field `IC1EN` reader - IC1 enable
pub type IC1EN_R = crate::BitReader;
///Field `IC1EN` writer - IC1 enable
pub type IC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2EN` reader - IC2 enable
pub type IC2EN_R = crate::BitReader;
///Field `IC2EN` writer - IC2 enable
pub type IC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3EN` reader - IC3 enable
pub type IC3EN_R = crate::BitReader;
///Field `IC3EN` writer - IC3 enable
pub type IC3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4EN` reader - IC4 enable
pub type IC4EN_R = crate::BitReader;
///Field `IC4EN` writer - IC4 enable
pub type IC4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5EN` reader - IC5 enable
pub type IC5EN_R = crate::BitReader;
///Field `IC5EN` writer - IC5 enable
pub type IC5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6EN` reader - IC6 enable
pub type IC6EN_R = crate::BitReader;
///Field `IC6EN` writer - IC6 enable
pub type IC6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7EN` reader - IC7 enable
pub type IC7EN_R = crate::BitReader;
///Field `IC7EN` writer - IC7 enable
pub type IC7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8EN` reader - IC8 enable
pub type IC8EN_R = crate::BitReader;
///Field `IC8EN` writer - IC8 enable
pub type IC8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9EN` reader - IC9 enable
pub type IC9EN_R = crate::BitReader;
///Field `IC9EN` writer - IC9 enable
pub type IC9EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10EN` reader - IC10 enable
pub type IC10EN_R = crate::BitReader;
///Field `IC10EN` writer - IC10 enable
pub type IC10EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11EN` reader - IC11 enable
pub type IC11EN_R = crate::BitReader;
///Field `IC11EN` writer - IC11 enable
pub type IC11EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12EN` reader - IC12 enable
pub type IC12EN_R = crate::BitReader;
///Field `IC12EN` writer - IC12 enable
pub type IC12EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13EN` reader - IC13 enable
pub type IC13EN_R = crate::BitReader;
///Field `IC13EN` writer - IC13 enable
pub type IC13EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14EN` reader - IC14 enable
pub type IC14EN_R = crate::BitReader;
///Field `IC14EN` writer - IC14 enable
pub type IC14EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15EN` reader - IC15 enable
pub type IC15EN_R = crate::BitReader;
///Field `IC15EN` writer - IC15 enable
pub type IC15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16EN` reader - IC16 enable
pub type IC16EN_R = crate::BitReader;
///Field `IC16EN` writer - IC16 enable
pub type IC16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17EN` reader - IC17 enable
pub type IC17EN_R = crate::BitReader;
///Field `IC17EN` writer - IC17 enable
pub type IC17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18EN` reader - IC18 enable
pub type IC18EN_R = crate::BitReader;
///Field `IC18EN` writer - IC18 enable
pub type IC18EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19EN` reader - IC19 enable
pub type IC19EN_R = crate::BitReader;
///Field `IC19EN` writer - IC19 enable
pub type IC19EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20EN` reader - IC20 enable
pub type IC20EN_R = crate::BitReader;
///Field `IC20EN` writer - IC20 enable
pub type IC20EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IC1 enable
    #[inline(always)]
    pub fn ic1en(&self) -> IC1EN_R {
        IC1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IC2 enable
    #[inline(always)]
    pub fn ic2en(&self) -> IC2EN_R {
        IC2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IC3 enable
    #[inline(always)]
    pub fn ic3en(&self) -> IC3EN_R {
        IC3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IC4 enable
    #[inline(always)]
    pub fn ic4en(&self) -> IC4EN_R {
        IC4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IC5 enable
    #[inline(always)]
    pub fn ic5en(&self) -> IC5EN_R {
        IC5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IC6 enable
    #[inline(always)]
    pub fn ic6en(&self) -> IC6EN_R {
        IC6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IC7 enable
    #[inline(always)]
    pub fn ic7en(&self) -> IC7EN_R {
        IC7EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IC8 enable
    #[inline(always)]
    pub fn ic8en(&self) -> IC8EN_R {
        IC8EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IC9 enable
    #[inline(always)]
    pub fn ic9en(&self) -> IC9EN_R {
        IC9EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IC10 enable
    #[inline(always)]
    pub fn ic10en(&self) -> IC10EN_R {
        IC10EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IC11 enable
    #[inline(always)]
    pub fn ic11en(&self) -> IC11EN_R {
        IC11EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - IC12 enable
    #[inline(always)]
    pub fn ic12en(&self) -> IC12EN_R {
        IC12EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IC13 enable
    #[inline(always)]
    pub fn ic13en(&self) -> IC13EN_R {
        IC13EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IC14 enable
    #[inline(always)]
    pub fn ic14en(&self) -> IC14EN_R {
        IC14EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - IC15 enable
    #[inline(always)]
    pub fn ic15en(&self) -> IC15EN_R {
        IC15EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IC16 enable
    #[inline(always)]
    pub fn ic16en(&self) -> IC16EN_R {
        IC16EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IC17 enable
    #[inline(always)]
    pub fn ic17en(&self) -> IC17EN_R {
        IC17EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - IC18 enable
    #[inline(always)]
    pub fn ic18en(&self) -> IC18EN_R {
        IC18EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IC19 enable
    #[inline(always)]
    pub fn ic19en(&self) -> IC19EN_R {
        IC19EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - IC20 enable
    #[inline(always)]
    pub fn ic20en(&self) -> IC20EN_R {
        IC20EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVENR")
            .field("ic1en", &self.ic1en())
            .field("ic2en", &self.ic2en())
            .field("ic3en", &self.ic3en())
            .field("ic4en", &self.ic4en())
            .field("ic5en", &self.ic5en())
            .field("ic6en", &self.ic6en())
            .field("ic7en", &self.ic7en())
            .field("ic8en", &self.ic8en())
            .field("ic9en", &self.ic9en())
            .field("ic10en", &self.ic10en())
            .field("ic11en", &self.ic11en())
            .field("ic12en", &self.ic12en())
            .field("ic13en", &self.ic13en())
            .field("ic14en", &self.ic14en())
            .field("ic15en", &self.ic15en())
            .field("ic16en", &self.ic16en())
            .field("ic17en", &self.ic17en())
            .field("ic18en", &self.ic18en())
            .field("ic19en", &self.ic19en())
            .field("ic20en", &self.ic20en())
            .finish()
    }
}
impl W {
    ///Bit 0 - IC1 enable
    #[inline(always)]
    pub fn ic1en(&mut self) -> IC1EN_W<'_, DIVENRrs> {
        IC1EN_W::new(self, 0)
    }
    ///Bit 1 - IC2 enable
    #[inline(always)]
    pub fn ic2en(&mut self) -> IC2EN_W<'_, DIVENRrs> {
        IC2EN_W::new(self, 1)
    }
    ///Bit 2 - IC3 enable
    #[inline(always)]
    pub fn ic3en(&mut self) -> IC3EN_W<'_, DIVENRrs> {
        IC3EN_W::new(self, 2)
    }
    ///Bit 3 - IC4 enable
    #[inline(always)]
    pub fn ic4en(&mut self) -> IC4EN_W<'_, DIVENRrs> {
        IC4EN_W::new(self, 3)
    }
    ///Bit 4 - IC5 enable
    #[inline(always)]
    pub fn ic5en(&mut self) -> IC5EN_W<'_, DIVENRrs> {
        IC5EN_W::new(self, 4)
    }
    ///Bit 5 - IC6 enable
    #[inline(always)]
    pub fn ic6en(&mut self) -> IC6EN_W<'_, DIVENRrs> {
        IC6EN_W::new(self, 5)
    }
    ///Bit 6 - IC7 enable
    #[inline(always)]
    pub fn ic7en(&mut self) -> IC7EN_W<'_, DIVENRrs> {
        IC7EN_W::new(self, 6)
    }
    ///Bit 7 - IC8 enable
    #[inline(always)]
    pub fn ic8en(&mut self) -> IC8EN_W<'_, DIVENRrs> {
        IC8EN_W::new(self, 7)
    }
    ///Bit 8 - IC9 enable
    #[inline(always)]
    pub fn ic9en(&mut self) -> IC9EN_W<'_, DIVENRrs> {
        IC9EN_W::new(self, 8)
    }
    ///Bit 9 - IC10 enable
    #[inline(always)]
    pub fn ic10en(&mut self) -> IC10EN_W<'_, DIVENRrs> {
        IC10EN_W::new(self, 9)
    }
    ///Bit 10 - IC11 enable
    #[inline(always)]
    pub fn ic11en(&mut self) -> IC11EN_W<'_, DIVENRrs> {
        IC11EN_W::new(self, 10)
    }
    ///Bit 11 - IC12 enable
    #[inline(always)]
    pub fn ic12en(&mut self) -> IC12EN_W<'_, DIVENRrs> {
        IC12EN_W::new(self, 11)
    }
    ///Bit 12 - IC13 enable
    #[inline(always)]
    pub fn ic13en(&mut self) -> IC13EN_W<'_, DIVENRrs> {
        IC13EN_W::new(self, 12)
    }
    ///Bit 13 - IC14 enable
    #[inline(always)]
    pub fn ic14en(&mut self) -> IC14EN_W<'_, DIVENRrs> {
        IC14EN_W::new(self, 13)
    }
    ///Bit 14 - IC15 enable
    #[inline(always)]
    pub fn ic15en(&mut self) -> IC15EN_W<'_, DIVENRrs> {
        IC15EN_W::new(self, 14)
    }
    ///Bit 15 - IC16 enable
    #[inline(always)]
    pub fn ic16en(&mut self) -> IC16EN_W<'_, DIVENRrs> {
        IC16EN_W::new(self, 15)
    }
    ///Bit 16 - IC17 enable
    #[inline(always)]
    pub fn ic17en(&mut self) -> IC17EN_W<'_, DIVENRrs> {
        IC17EN_W::new(self, 16)
    }
    ///Bit 17 - IC18 enable
    #[inline(always)]
    pub fn ic18en(&mut self) -> IC18EN_W<'_, DIVENRrs> {
        IC18EN_W::new(self, 17)
    }
    ///Bit 18 - IC19 enable
    #[inline(always)]
    pub fn ic19en(&mut self) -> IC19EN_W<'_, DIVENRrs> {
        IC19EN_W::new(self, 18)
    }
    ///Bit 19 - IC20 enable
    #[inline(always)]
    pub fn ic20en(&mut self) -> IC20EN_W<'_, DIVENRrs> {
        IC20EN_W::new(self, 19)
    }
}
/**RCC IC dividers enable register

You can [`read`](crate::Reg::read) this register and get [`divenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:DIVENR)*/
pub struct DIVENRrs;
impl crate::RegisterSpec for DIVENRrs {
    type Ux = u32;
}
///`read()` method returns [`divenr::R`](R) reader structure
impl crate::Readable for DIVENRrs {}
///`write(|w| ..)` method takes [`divenr::W`](W) writer structure
impl crate::Writable for DIVENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIVENR to value 0
impl crate::Resettable for DIVENRrs {}
