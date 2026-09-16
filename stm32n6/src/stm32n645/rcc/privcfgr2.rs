///Register `PRIVCFGR2` reader
pub type R = crate::R<PRIVCFGR2rs>;
///Register `PRIVCFGR2` writer
pub type W = crate::W<PRIVCFGR2rs>;
///Field `IC1PV` reader - Defines the privilege protection of the IC1 divider configuration bits.
pub type IC1PV_R = crate::BitReader;
///Field `IC1PV` writer - Defines the privilege protection of the IC1 divider configuration bits.
pub type IC1PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2PV` reader - Defines the privilege protection of the IC2 divider configuration bits.
pub type IC2PV_R = crate::BitReader;
///Field `IC2PV` writer - Defines the privilege protection of the IC2 divider configuration bits.
pub type IC2PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3PV` reader - Defines the privilege protection of the IC3 divider configuration bits.
pub type IC3PV_R = crate::BitReader;
///Field `IC3PV` writer - Defines the privilege protection of the IC3 divider configuration bits.
pub type IC3PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4PV` reader - Defines the privilege protection of the IC4 divider configuration bits.
pub type IC4PV_R = crate::BitReader;
///Field `IC4PV` writer - Defines the privilege protection of the IC4 divider configuration bits.
pub type IC4PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5PV` reader - Defines the privilege protection of the IC5 divider configuration bits.
pub type IC5PV_R = crate::BitReader;
///Field `IC5PV` writer - Defines the privilege protection of the IC5 divider configuration bits.
pub type IC5PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6PV` reader - Defines the privilege protection of the IC6 divider configuration bits.
pub type IC6PV_R = crate::BitReader;
///Field `IC6PV` writer - Defines the privilege protection of the IC6 divider configuration bits.
pub type IC6PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7PV` reader - Defines the privilege protection of the IC7 divider configuration bits.
pub type IC7PV_R = crate::BitReader;
///Field `IC7PV` writer - Defines the privilege protection of the IC7 divider configuration bits.
pub type IC7PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8PV` reader - Defines the privilege protection of the IC8 divider configuration bits.
pub type IC8PV_R = crate::BitReader;
///Field `IC8PV` writer - Defines the privilege protection of the IC8 divider configuration bits.
pub type IC8PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9PV` reader - Defines the privilege protection of the IC9 divider configuration bits.
pub type IC9PV_R = crate::BitReader;
///Field `IC9PV` writer - Defines the privilege protection of the IC9 divider configuration bits.
pub type IC9PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10PV` reader - Defines the privilege protection of the IC10 divider configuration bits.
pub type IC10PV_R = crate::BitReader;
///Field `IC10PV` writer - Defines the privilege protection of the IC10 divider configuration bits.
pub type IC10PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11PV` reader - Defines the privilege protection of the IC11 divider configuration bits.
pub type IC11PV_R = crate::BitReader;
///Field `IC11PV` writer - Defines the privilege protection of the IC11 divider configuration bits.
pub type IC11PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12PV` reader - Defines the privilege protection of the IC12 divider configuration bits.
pub type IC12PV_R = crate::BitReader;
///Field `IC12PV` writer - Defines the privilege protection of the IC12 divider configuration bits.
pub type IC12PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13PV` reader - Defines the privilege protection of the IC13 divider configuration bits.
pub type IC13PV_R = crate::BitReader;
///Field `IC13PV` writer - Defines the privilege protection of the IC13 divider configuration bits.
pub type IC13PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14PV` reader - Defines the privilege protection of the IC14 divider configuration bits.
pub type IC14PV_R = crate::BitReader;
///Field `IC14PV` writer - Defines the privilege protection of the IC14 divider configuration bits.
pub type IC14PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15PV` reader - Defines the privilege protection of the IC15 divider configuration bits.
pub type IC15PV_R = crate::BitReader;
///Field `IC15PV` writer - Defines the privilege protection of the IC15 divider configuration bits.
pub type IC15PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16PV` reader - Defines the privilege protection of the IC16 divider configuration bits.
pub type IC16PV_R = crate::BitReader;
///Field `IC16PV` writer - Defines the privilege protection of the IC16 divider configuration bits.
pub type IC16PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17PV` reader - Defines the privilege protection of the IC17 divider configuration bits.
pub type IC17PV_R = crate::BitReader;
///Field `IC17PV` writer - Defines the privilege protection of the IC17 divider configuration bits.
pub type IC17PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18PV` reader - Defines the privilege protection of the IC18 divider configuration bits.
pub type IC18PV_R = crate::BitReader;
///Field `IC18PV` writer - Defines the privilege protection of the IC18 divider configuration bits.
pub type IC18PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19PV` reader - Defines the privilege protection of the IC19 divider configuration bits.
pub type IC19PV_R = crate::BitReader;
///Field `IC19PV` writer - Defines the privilege protection of the IC19 divider configuration bits.
pub type IC19PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20PV` reader - Defines the privilege protection of the IC20 divider configuration bits.
pub type IC20PV_R = crate::BitReader;
///Field `IC20PV` writer - Defines the privilege protection of the IC20 divider configuration bits.
pub type IC20PV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the privilege protection of the IC1 divider configuration bits.
    #[inline(always)]
    pub fn ic1pv(&self) -> IC1PV_R {
        IC1PV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the privilege protection of the IC2 divider configuration bits.
    #[inline(always)]
    pub fn ic2pv(&self) -> IC2PV_R {
        IC2PV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the privilege protection of the IC3 divider configuration bits.
    #[inline(always)]
    pub fn ic3pv(&self) -> IC3PV_R {
        IC3PV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the privilege protection of the IC4 divider configuration bits.
    #[inline(always)]
    pub fn ic4pv(&self) -> IC4PV_R {
        IC4PV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the privilege protection of the IC5 divider configuration bits.
    #[inline(always)]
    pub fn ic5pv(&self) -> IC5PV_R {
        IC5PV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the privilege protection of the IC6 divider configuration bits.
    #[inline(always)]
    pub fn ic6pv(&self) -> IC6PV_R {
        IC6PV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the privilege protection of the IC7 divider configuration bits.
    #[inline(always)]
    pub fn ic7pv(&self) -> IC7PV_R {
        IC7PV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Defines the privilege protection of the IC8 divider configuration bits.
    #[inline(always)]
    pub fn ic8pv(&self) -> IC8PV_R {
        IC8PV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Defines the privilege protection of the IC9 divider configuration bits.
    #[inline(always)]
    pub fn ic9pv(&self) -> IC9PV_R {
        IC9PV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Defines the privilege protection of the IC10 divider configuration bits.
    #[inline(always)]
    pub fn ic10pv(&self) -> IC10PV_R {
        IC10PV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Defines the privilege protection of the IC11 divider configuration bits.
    #[inline(always)]
    pub fn ic11pv(&self) -> IC11PV_R {
        IC11PV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Defines the privilege protection of the IC12 divider configuration bits.
    #[inline(always)]
    pub fn ic12pv(&self) -> IC12PV_R {
        IC12PV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Defines the privilege protection of the IC13 divider configuration bits.
    #[inline(always)]
    pub fn ic13pv(&self) -> IC13PV_R {
        IC13PV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Defines the privilege protection of the IC14 divider configuration bits.
    #[inline(always)]
    pub fn ic14pv(&self) -> IC14PV_R {
        IC14PV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Defines the privilege protection of the IC15 divider configuration bits.
    #[inline(always)]
    pub fn ic15pv(&self) -> IC15PV_R {
        IC15PV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Defines the privilege protection of the IC16 divider configuration bits.
    #[inline(always)]
    pub fn ic16pv(&self) -> IC16PV_R {
        IC16PV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Defines the privilege protection of the IC17 divider configuration bits.
    #[inline(always)]
    pub fn ic17pv(&self) -> IC17PV_R {
        IC17PV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Defines the privilege protection of the IC18 divider configuration bits.
    #[inline(always)]
    pub fn ic18pv(&self) -> IC18PV_R {
        IC18PV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Defines the privilege protection of the IC19 divider configuration bits.
    #[inline(always)]
    pub fn ic19pv(&self) -> IC19PV_R {
        IC19PV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Defines the privilege protection of the IC20 divider configuration bits.
    #[inline(always)]
    pub fn ic20pv(&self) -> IC20PV_R {
        IC20PV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR2")
            .field("ic1pv", &self.ic1pv())
            .field("ic2pv", &self.ic2pv())
            .field("ic3pv", &self.ic3pv())
            .field("ic4pv", &self.ic4pv())
            .field("ic5pv", &self.ic5pv())
            .field("ic6pv", &self.ic6pv())
            .field("ic7pv", &self.ic7pv())
            .field("ic8pv", &self.ic8pv())
            .field("ic9pv", &self.ic9pv())
            .field("ic10pv", &self.ic10pv())
            .field("ic11pv", &self.ic11pv())
            .field("ic12pv", &self.ic12pv())
            .field("ic13pv", &self.ic13pv())
            .field("ic14pv", &self.ic14pv())
            .field("ic15pv", &self.ic15pv())
            .field("ic16pv", &self.ic16pv())
            .field("ic17pv", &self.ic17pv())
            .field("ic18pv", &self.ic18pv())
            .field("ic19pv", &self.ic19pv())
            .field("ic20pv", &self.ic20pv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the IC1 divider configuration bits.
    #[inline(always)]
    pub fn ic1pv(&mut self) -> IC1PV_W<'_, PRIVCFGR2rs> {
        IC1PV_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the IC2 divider configuration bits.
    #[inline(always)]
    pub fn ic2pv(&mut self) -> IC2PV_W<'_, PRIVCFGR2rs> {
        IC2PV_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the IC3 divider configuration bits.
    #[inline(always)]
    pub fn ic3pv(&mut self) -> IC3PV_W<'_, PRIVCFGR2rs> {
        IC3PV_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the IC4 divider configuration bits.
    #[inline(always)]
    pub fn ic4pv(&mut self) -> IC4PV_W<'_, PRIVCFGR2rs> {
        IC4PV_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the IC5 divider configuration bits.
    #[inline(always)]
    pub fn ic5pv(&mut self) -> IC5PV_W<'_, PRIVCFGR2rs> {
        IC5PV_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the IC6 divider configuration bits.
    #[inline(always)]
    pub fn ic6pv(&mut self) -> IC6PV_W<'_, PRIVCFGR2rs> {
        IC6PV_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the IC7 divider configuration bits.
    #[inline(always)]
    pub fn ic7pv(&mut self) -> IC7PV_W<'_, PRIVCFGR2rs> {
        IC7PV_W::new(self, 6)
    }
    ///Bit 7 - Defines the privilege protection of the IC8 divider configuration bits.
    #[inline(always)]
    pub fn ic8pv(&mut self) -> IC8PV_W<'_, PRIVCFGR2rs> {
        IC8PV_W::new(self, 7)
    }
    ///Bit 8 - Defines the privilege protection of the IC9 divider configuration bits.
    #[inline(always)]
    pub fn ic9pv(&mut self) -> IC9PV_W<'_, PRIVCFGR2rs> {
        IC9PV_W::new(self, 8)
    }
    ///Bit 9 - Defines the privilege protection of the IC10 divider configuration bits.
    #[inline(always)]
    pub fn ic10pv(&mut self) -> IC10PV_W<'_, PRIVCFGR2rs> {
        IC10PV_W::new(self, 9)
    }
    ///Bit 10 - Defines the privilege protection of the IC11 divider configuration bits.
    #[inline(always)]
    pub fn ic11pv(&mut self) -> IC11PV_W<'_, PRIVCFGR2rs> {
        IC11PV_W::new(self, 10)
    }
    ///Bit 11 - Defines the privilege protection of the IC12 divider configuration bits.
    #[inline(always)]
    pub fn ic12pv(&mut self) -> IC12PV_W<'_, PRIVCFGR2rs> {
        IC12PV_W::new(self, 11)
    }
    ///Bit 12 - Defines the privilege protection of the IC13 divider configuration bits.
    #[inline(always)]
    pub fn ic13pv(&mut self) -> IC13PV_W<'_, PRIVCFGR2rs> {
        IC13PV_W::new(self, 12)
    }
    ///Bit 13 - Defines the privilege protection of the IC14 divider configuration bits.
    #[inline(always)]
    pub fn ic14pv(&mut self) -> IC14PV_W<'_, PRIVCFGR2rs> {
        IC14PV_W::new(self, 13)
    }
    ///Bit 14 - Defines the privilege protection of the IC15 divider configuration bits.
    #[inline(always)]
    pub fn ic15pv(&mut self) -> IC15PV_W<'_, PRIVCFGR2rs> {
        IC15PV_W::new(self, 14)
    }
    ///Bit 15 - Defines the privilege protection of the IC16 divider configuration bits.
    #[inline(always)]
    pub fn ic16pv(&mut self) -> IC16PV_W<'_, PRIVCFGR2rs> {
        IC16PV_W::new(self, 15)
    }
    ///Bit 16 - Defines the privilege protection of the IC17 divider configuration bits.
    #[inline(always)]
    pub fn ic17pv(&mut self) -> IC17PV_W<'_, PRIVCFGR2rs> {
        IC17PV_W::new(self, 16)
    }
    ///Bit 17 - Defines the privilege protection of the IC18 divider configuration bits.
    #[inline(always)]
    pub fn ic18pv(&mut self) -> IC18PV_W<'_, PRIVCFGR2rs> {
        IC18PV_W::new(self, 17)
    }
    ///Bit 18 - Defines the privilege protection of the IC19 divider configuration bits.
    #[inline(always)]
    pub fn ic19pv(&mut self) -> IC19PV_W<'_, PRIVCFGR2rs> {
        IC19PV_W::new(self, 18)
    }
    ///Bit 19 - Defines the privilege protection of the IC20 divider configuration bits.
    #[inline(always)]
    pub fn ic20pv(&mut self) -> IC20PV_W<'_, PRIVCFGR2rs> {
        IC20PV_W::new(self, 19)
    }
}
/**RCC divider privilege configuration register2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PRIVCFGR2)*/
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr2::R`](R) reader structure
impl crate::Readable for PRIVCFGR2rs {}
///`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR2 to value 0
impl crate::Resettable for PRIVCFGR2rs {}
