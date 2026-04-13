///Register `SECCFGR3` reader
pub type R = crate::R<SECCFGR3rs>;
///Register `SECCFGR3` writer
pub type W = crate::W<SECCFGR3rs>;
///Field `SEC64` reader - Security enable on event input x
pub type SEC64_R = crate::BitReader;
///Field `SEC64` writer - Security enable on event input x
pub type SEC64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC65` reader - Security enable on event input x
pub type SEC65_R = crate::BitReader;
///Field `SEC65` writer - Security enable on event input x
pub type SEC65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC66` reader - Security enable on event input x
pub type SEC66_R = crate::BitReader;
///Field `SEC66` writer - Security enable on event input x
pub type SEC66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC68` reader - Security enable on event input x
pub type SEC68_R = crate::BitReader;
///Field `SEC68` writer - Security enable on event input x
pub type SEC68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC69` reader - Security enable on event input x
pub type SEC69_R = crate::BitReader;
///Field `SEC69` writer - Security enable on event input x
pub type SEC69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC70` reader - Security enable on event input x
pub type SEC70_R = crate::BitReader;
///Field `SEC70` writer - Security enable on event input x
pub type SEC70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC71` reader - Security enable on event input x
pub type SEC71_R = crate::BitReader;
///Field `SEC71` writer - Security enable on event input x
pub type SEC71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC72` reader - Security enable on event input x
pub type SEC72_R = crate::BitReader;
///Field `SEC72` writer - Security enable on event input x
pub type SEC72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC73` reader - Security enable on event input x
pub type SEC73_R = crate::BitReader;
///Field `SEC73` writer - Security enable on event input x
pub type SEC73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC74` reader - Security enable on event input x
pub type SEC74_R = crate::BitReader;
///Field `SEC74` writer - Security enable on event input x
pub type SEC74_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC77` reader - Security enable on event input 77
pub type SEC77_R = crate::BitReader;
///Field `SEC77` writer - Security enable on event input 77
pub type SEC77_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Security enable on event input x
    #[inline(always)]
    pub fn sec64(&self) -> SEC64_R {
        SEC64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security enable on event input x
    #[inline(always)]
    pub fn sec65(&self) -> SEC65_R {
        SEC65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security enable on event input x
    #[inline(always)]
    pub fn sec66(&self) -> SEC66_R {
        SEC66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Security enable on event input x
    #[inline(always)]
    pub fn sec68(&self) -> SEC68_R {
        SEC68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security enable on event input x
    #[inline(always)]
    pub fn sec69(&self) -> SEC69_R {
        SEC69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security enable on event input x
    #[inline(always)]
    pub fn sec70(&self) -> SEC70_R {
        SEC70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security enable on event input x
    #[inline(always)]
    pub fn sec71(&self) -> SEC71_R {
        SEC71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security enable on event input x
    #[inline(always)]
    pub fn sec72(&self) -> SEC72_R {
        SEC72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security enable on event input x
    #[inline(always)]
    pub fn sec73(&self) -> SEC73_R {
        SEC73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security enable on event input x
    #[inline(always)]
    pub fn sec74(&self) -> SEC74_R {
        SEC74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Security enable on event input 77
    #[inline(always)]
    pub fn sec77(&self) -> SEC77_R {
        SEC77_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR3")
            .field("sec64", &self.sec64())
            .field("sec65", &self.sec65())
            .field("sec66", &self.sec66())
            .field("sec68", &self.sec68())
            .field("sec69", &self.sec69())
            .field("sec70", &self.sec70())
            .field("sec71", &self.sec71())
            .field("sec72", &self.sec72())
            .field("sec73", &self.sec73())
            .field("sec74", &self.sec74())
            .field("sec77", &self.sec77())
            .finish()
    }
}
impl W {
    ///Bit 0 - Security enable on event input x
    #[inline(always)]
    pub fn sec64(&mut self) -> SEC64_W<'_, SECCFGR3rs> {
        SEC64_W::new(self, 0)
    }
    ///Bit 1 - Security enable on event input x
    #[inline(always)]
    pub fn sec65(&mut self) -> SEC65_W<'_, SECCFGR3rs> {
        SEC65_W::new(self, 1)
    }
    ///Bit 2 - Security enable on event input x
    #[inline(always)]
    pub fn sec66(&mut self) -> SEC66_W<'_, SECCFGR3rs> {
        SEC66_W::new(self, 2)
    }
    ///Bit 4 - Security enable on event input x
    #[inline(always)]
    pub fn sec68(&mut self) -> SEC68_W<'_, SECCFGR3rs> {
        SEC68_W::new(self, 4)
    }
    ///Bit 5 - Security enable on event input x
    #[inline(always)]
    pub fn sec69(&mut self) -> SEC69_W<'_, SECCFGR3rs> {
        SEC69_W::new(self, 5)
    }
    ///Bit 6 - Security enable on event input x
    #[inline(always)]
    pub fn sec70(&mut self) -> SEC70_W<'_, SECCFGR3rs> {
        SEC70_W::new(self, 6)
    }
    ///Bit 7 - Security enable on event input x
    #[inline(always)]
    pub fn sec71(&mut self) -> SEC71_W<'_, SECCFGR3rs> {
        SEC71_W::new(self, 7)
    }
    ///Bit 8 - Security enable on event input x
    #[inline(always)]
    pub fn sec72(&mut self) -> SEC72_W<'_, SECCFGR3rs> {
        SEC72_W::new(self, 8)
    }
    ///Bit 9 - Security enable on event input x
    #[inline(always)]
    pub fn sec73(&mut self) -> SEC73_W<'_, SECCFGR3rs> {
        SEC73_W::new(self, 9)
    }
    ///Bit 10 - Security enable on event input x
    #[inline(always)]
    pub fn sec74(&mut self) -> SEC74_W<'_, SECCFGR3rs> {
        SEC74_W::new(self, 10)
    }
    ///Bit 13 - Security enable on event input 77
    #[inline(always)]
    pub fn sec77(&mut self) -> SEC77_W<'_, SECCFGR3rs> {
        SEC77_W::new(self, 13)
    }
}
/**EXTI security enable register

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#EXTI:SECCFGR3)*/
pub struct SECCFGR3rs;
impl crate::RegisterSpec for SECCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr3::R`](R) reader structure
impl crate::Readable for SECCFGR3rs {}
///`write(|w| ..)` method takes [`seccfgr3::W`](W) writer structure
impl crate::Writable for SECCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR3 to value 0
impl crate::Resettable for SECCFGR3rs {}
