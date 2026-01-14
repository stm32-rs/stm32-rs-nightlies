///Register `DIVLPENR` reader
pub type R = crate::R<DIVLPENRrs>;
///Register `DIVLPENR` writer
pub type W = crate::W<DIVLPENRrs>;
///Field `IC1LPEN` reader - IC1 sleep enable
pub type IC1LPEN_R = crate::BitReader;
///Field `IC1LPEN` writer - IC1 sleep enable
pub type IC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2LPEN` reader - IC2 sleep enable
pub type IC2LPEN_R = crate::BitReader;
///Field `IC2LPEN` writer - IC2 sleep enable
pub type IC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3LPEN` reader - IC3 sleep enable
pub type IC3LPEN_R = crate::BitReader;
///Field `IC3LPEN` writer - IC3 sleep enable
pub type IC3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4LPEN` reader - IC4 sleep enable
pub type IC4LPEN_R = crate::BitReader;
///Field `IC4LPEN` writer - IC4 sleep enable
pub type IC4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5LPEN` reader - IC5 sleep enable
pub type IC5LPEN_R = crate::BitReader;
///Field `IC5LPEN` writer - IC5 sleep enable
pub type IC5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6LPEN` reader - IC6 sleep enable
pub type IC6LPEN_R = crate::BitReader;
///Field `IC6LPEN` writer - IC6 sleep enable
pub type IC6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7LPEN` reader - IC7 sleep enable
pub type IC7LPEN_R = crate::BitReader;
///Field `IC7LPEN` writer - IC7 sleep enable
pub type IC7LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8LPEN` reader - IC8 sleep enable
pub type IC8LPEN_R = crate::BitReader;
///Field `IC8LPEN` writer - IC8 sleep enable
pub type IC8LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9LPEN` reader - IC9 sleep enable
pub type IC9LPEN_R = crate::BitReader;
///Field `IC9LPEN` writer - IC9 sleep enable
pub type IC9LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10LPEN` reader - IC10 sleep enable
pub type IC10LPEN_R = crate::BitReader;
///Field `IC10LPEN` writer - IC10 sleep enable
pub type IC10LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11LPEN` reader - IC11 sleep enable
pub type IC11LPEN_R = crate::BitReader;
///Field `IC11LPEN` writer - IC11 sleep enable
pub type IC11LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12LPEN` reader - IC12 sleep enable
pub type IC12LPEN_R = crate::BitReader;
///Field `IC12LPEN` writer - IC12 sleep enable
pub type IC12LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13LPEN` reader - IC13 sleep enable
pub type IC13LPEN_R = crate::BitReader;
///Field `IC13LPEN` writer - IC13 sleep enable
pub type IC13LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14LPEN` reader - IC14 sleep enable
pub type IC14LPEN_R = crate::BitReader;
///Field `IC14LPEN` writer - IC14 sleep enable
pub type IC14LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15LPEN` reader - IC15 sleep enable
pub type IC15LPEN_R = crate::BitReader;
///Field `IC15LPEN` writer - IC15 sleep enable
pub type IC15LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16LPEN` reader - IC16 sleep enable
pub type IC16LPEN_R = crate::BitReader;
///Field `IC16LPEN` writer - IC16 sleep enable
pub type IC16LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17LPEN` reader - IC17 sleep enable
pub type IC17LPEN_R = crate::BitReader;
///Field `IC17LPEN` writer - IC17 sleep enable
pub type IC17LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18LPEN` reader - IC18 sleep enable
pub type IC18LPEN_R = crate::BitReader;
///Field `IC18LPEN` writer - IC18 sleep enable
pub type IC18LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19LPEN` reader - IC19 sleep enable
pub type IC19LPEN_R = crate::BitReader;
///Field `IC19LPEN` writer - IC19 sleep enable
pub type IC19LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20LPEN` reader - IC20 sleep enable
pub type IC20LPEN_R = crate::BitReader;
///Field `IC20LPEN` writer - IC20 sleep enable
pub type IC20LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IC1 sleep enable
    #[inline(always)]
    pub fn ic1lpen(&self) -> IC1LPEN_R {
        IC1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IC2 sleep enable
    #[inline(always)]
    pub fn ic2lpen(&self) -> IC2LPEN_R {
        IC2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IC3 sleep enable
    #[inline(always)]
    pub fn ic3lpen(&self) -> IC3LPEN_R {
        IC3LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IC4 sleep enable
    #[inline(always)]
    pub fn ic4lpen(&self) -> IC4LPEN_R {
        IC4LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IC5 sleep enable
    #[inline(always)]
    pub fn ic5lpen(&self) -> IC5LPEN_R {
        IC5LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IC6 sleep enable
    #[inline(always)]
    pub fn ic6lpen(&self) -> IC6LPEN_R {
        IC6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IC7 sleep enable
    #[inline(always)]
    pub fn ic7lpen(&self) -> IC7LPEN_R {
        IC7LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IC8 sleep enable
    #[inline(always)]
    pub fn ic8lpen(&self) -> IC8LPEN_R {
        IC8LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IC9 sleep enable
    #[inline(always)]
    pub fn ic9lpen(&self) -> IC9LPEN_R {
        IC9LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IC10 sleep enable
    #[inline(always)]
    pub fn ic10lpen(&self) -> IC10LPEN_R {
        IC10LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IC11 sleep enable
    #[inline(always)]
    pub fn ic11lpen(&self) -> IC11LPEN_R {
        IC11LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - IC12 sleep enable
    #[inline(always)]
    pub fn ic12lpen(&self) -> IC12LPEN_R {
        IC12LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IC13 sleep enable
    #[inline(always)]
    pub fn ic13lpen(&self) -> IC13LPEN_R {
        IC13LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IC14 sleep enable
    #[inline(always)]
    pub fn ic14lpen(&self) -> IC14LPEN_R {
        IC14LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - IC15 sleep enable
    #[inline(always)]
    pub fn ic15lpen(&self) -> IC15LPEN_R {
        IC15LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IC16 sleep enable
    #[inline(always)]
    pub fn ic16lpen(&self) -> IC16LPEN_R {
        IC16LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IC17 sleep enable
    #[inline(always)]
    pub fn ic17lpen(&self) -> IC17LPEN_R {
        IC17LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - IC18 sleep enable
    #[inline(always)]
    pub fn ic18lpen(&self) -> IC18LPEN_R {
        IC18LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IC19 sleep enable
    #[inline(always)]
    pub fn ic19lpen(&self) -> IC19LPEN_R {
        IC19LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - IC20 sleep enable
    #[inline(always)]
    pub fn ic20lpen(&self) -> IC20LPEN_R {
        IC20LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVLPENR")
            .field("ic1lpen", &self.ic1lpen())
            .field("ic2lpen", &self.ic2lpen())
            .field("ic3lpen", &self.ic3lpen())
            .field("ic4lpen", &self.ic4lpen())
            .field("ic5lpen", &self.ic5lpen())
            .field("ic6lpen", &self.ic6lpen())
            .field("ic7lpen", &self.ic7lpen())
            .field("ic8lpen", &self.ic8lpen())
            .field("ic9lpen", &self.ic9lpen())
            .field("ic10lpen", &self.ic10lpen())
            .field("ic11lpen", &self.ic11lpen())
            .field("ic12lpen", &self.ic12lpen())
            .field("ic13lpen", &self.ic13lpen())
            .field("ic14lpen", &self.ic14lpen())
            .field("ic15lpen", &self.ic15lpen())
            .field("ic16lpen", &self.ic16lpen())
            .field("ic17lpen", &self.ic17lpen())
            .field("ic18lpen", &self.ic18lpen())
            .field("ic19lpen", &self.ic19lpen())
            .field("ic20lpen", &self.ic20lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IC1 sleep enable
    #[inline(always)]
    pub fn ic1lpen(&mut self) -> IC1LPEN_W<'_, DIVLPENRrs> {
        IC1LPEN_W::new(self, 0)
    }
    ///Bit 1 - IC2 sleep enable
    #[inline(always)]
    pub fn ic2lpen(&mut self) -> IC2LPEN_W<'_, DIVLPENRrs> {
        IC2LPEN_W::new(self, 1)
    }
    ///Bit 2 - IC3 sleep enable
    #[inline(always)]
    pub fn ic3lpen(&mut self) -> IC3LPEN_W<'_, DIVLPENRrs> {
        IC3LPEN_W::new(self, 2)
    }
    ///Bit 3 - IC4 sleep enable
    #[inline(always)]
    pub fn ic4lpen(&mut self) -> IC4LPEN_W<'_, DIVLPENRrs> {
        IC4LPEN_W::new(self, 3)
    }
    ///Bit 4 - IC5 sleep enable
    #[inline(always)]
    pub fn ic5lpen(&mut self) -> IC5LPEN_W<'_, DIVLPENRrs> {
        IC5LPEN_W::new(self, 4)
    }
    ///Bit 5 - IC6 sleep enable
    #[inline(always)]
    pub fn ic6lpen(&mut self) -> IC6LPEN_W<'_, DIVLPENRrs> {
        IC6LPEN_W::new(self, 5)
    }
    ///Bit 6 - IC7 sleep enable
    #[inline(always)]
    pub fn ic7lpen(&mut self) -> IC7LPEN_W<'_, DIVLPENRrs> {
        IC7LPEN_W::new(self, 6)
    }
    ///Bit 7 - IC8 sleep enable
    #[inline(always)]
    pub fn ic8lpen(&mut self) -> IC8LPEN_W<'_, DIVLPENRrs> {
        IC8LPEN_W::new(self, 7)
    }
    ///Bit 8 - IC9 sleep enable
    #[inline(always)]
    pub fn ic9lpen(&mut self) -> IC9LPEN_W<'_, DIVLPENRrs> {
        IC9LPEN_W::new(self, 8)
    }
    ///Bit 9 - IC10 sleep enable
    #[inline(always)]
    pub fn ic10lpen(&mut self) -> IC10LPEN_W<'_, DIVLPENRrs> {
        IC10LPEN_W::new(self, 9)
    }
    ///Bit 10 - IC11 sleep enable
    #[inline(always)]
    pub fn ic11lpen(&mut self) -> IC11LPEN_W<'_, DIVLPENRrs> {
        IC11LPEN_W::new(self, 10)
    }
    ///Bit 11 - IC12 sleep enable
    #[inline(always)]
    pub fn ic12lpen(&mut self) -> IC12LPEN_W<'_, DIVLPENRrs> {
        IC12LPEN_W::new(self, 11)
    }
    ///Bit 12 - IC13 sleep enable
    #[inline(always)]
    pub fn ic13lpen(&mut self) -> IC13LPEN_W<'_, DIVLPENRrs> {
        IC13LPEN_W::new(self, 12)
    }
    ///Bit 13 - IC14 sleep enable
    #[inline(always)]
    pub fn ic14lpen(&mut self) -> IC14LPEN_W<'_, DIVLPENRrs> {
        IC14LPEN_W::new(self, 13)
    }
    ///Bit 14 - IC15 sleep enable
    #[inline(always)]
    pub fn ic15lpen(&mut self) -> IC15LPEN_W<'_, DIVLPENRrs> {
        IC15LPEN_W::new(self, 14)
    }
    ///Bit 15 - IC16 sleep enable
    #[inline(always)]
    pub fn ic16lpen(&mut self) -> IC16LPEN_W<'_, DIVLPENRrs> {
        IC16LPEN_W::new(self, 15)
    }
    ///Bit 16 - IC17 sleep enable
    #[inline(always)]
    pub fn ic17lpen(&mut self) -> IC17LPEN_W<'_, DIVLPENRrs> {
        IC17LPEN_W::new(self, 16)
    }
    ///Bit 17 - IC18 sleep enable
    #[inline(always)]
    pub fn ic18lpen(&mut self) -> IC18LPEN_W<'_, DIVLPENRrs> {
        IC18LPEN_W::new(self, 17)
    }
    ///Bit 18 - IC19 sleep enable
    #[inline(always)]
    pub fn ic19lpen(&mut self) -> IC19LPEN_W<'_, DIVLPENRrs> {
        IC19LPEN_W::new(self, 18)
    }
    ///Bit 19 - IC20 sleep enable
    #[inline(always)]
    pub fn ic20lpen(&mut self) -> IC20LPEN_W<'_, DIVLPENRrs> {
        IC20LPEN_W::new(self, 19)
    }
}
/**RCC dividers Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`divlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:DIVLPENR)*/
pub struct DIVLPENRrs;
impl crate::RegisterSpec for DIVLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`divlpenr::R`](R) reader structure
impl crate::Readable for DIVLPENRrs {}
///`write(|w| ..)` method takes [`divlpenr::W`](W) writer structure
impl crate::Writable for DIVLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIVLPENR to value 0
impl crate::Resettable for DIVLPENRrs {}
