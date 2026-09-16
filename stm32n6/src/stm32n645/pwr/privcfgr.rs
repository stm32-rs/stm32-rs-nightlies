///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `PRIV0` reader - System supply configuration privileged protection
pub type PRIV0_R = crate::BitReader;
///Field `PRIV0` writer - System supply configuration privileged protection
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV1` reader - Programmable voltage detector privileged protection
pub type PRIV1_R = crate::BitReader;
///Field `PRIV1` writer - Programmable voltage detector privileged protection
pub type PRIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV2` reader - V less than sub>DDCORE less than /sub> monitor privileged protection
pub type PRIV2_R = crate::BitReader;
///Field `PRIV2` writer - V less than sub>DDCORE less than /sub> monitor privileged protection
pub type PRIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV3` reader - I-TCM, D-TCM, and I-TCM FLEX MEM low power control privileged protection
pub type PRIV3_R = crate::BitReader;
///Field `PRIV3` writer - I-TCM, D-TCM, and I-TCM FLEX MEM low power control privileged protection
pub type PRIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV4` reader - Voltage scaling selection privileged protection
pub type PRIV4_R = crate::BitReader;
///Field `PRIV4` writer - Voltage scaling selection privileged protection
pub type PRIV4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV5` reader - Backup domain privileged protection
pub type PRIV5_R = crate::BitReader;
///Field `PRIV5` writer - Backup domain privileged protection
pub type PRIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV6` reader - CPU power control privileged protection
pub type PRIV6_R = crate::BitReader;
///Field `PRIV6` writer - CPU power control privileged protection
pub type PRIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV7` reader - Peripheral voltage monitor privileged protection
pub type PRIV7_R = crate::BitReader;
///Field `PRIV7` writer - Peripheral voltage monitor privileged protection
pub type PRIV7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPPRIV1` reader - WKUP1 pin privileged protection
pub type WKUPPRIV1_R = crate::BitReader;
///Field `WKUPPRIV1` writer - WKUP1 pin privileged protection
pub type WKUPPRIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPPRIV2` reader - WKUP2 pin privileged protection
pub type WKUPPRIV2_R = crate::BitReader;
///Field `WKUPPRIV2` writer - WKUP2 pin privileged protection
pub type WKUPPRIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPPRIV3` reader - WKUP3 pin privileged protection
pub type WKUPPRIV3_R = crate::BitReader;
///Field `WKUPPRIV3` writer - WKUP3 pin privileged protection
pub type WKUPPRIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPPRIV4` reader - WKUP4 pin privileged protection
pub type WKUPPRIV4_R = crate::BitReader;
///Field `WKUPPRIV4` writer - WKUP4 pin privileged protection
pub type WKUPPRIV4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - System supply configuration privileged protection
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Programmable voltage detector privileged protection
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - V less than sub>DDCORE less than /sub> monitor privileged protection
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I-TCM, D-TCM, and I-TCM FLEX MEM low power control privileged protection
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Voltage scaling selection privileged protection
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Backup domain privileged protection
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU power control privileged protection
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Peripheral voltage monitor privileged protection
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - WKUP1 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv1(&self) -> WKUPPRIV1_R {
        WKUPPRIV1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - WKUP2 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv2(&self) -> WKUPPRIV2_R {
        WKUPPRIV2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - WKUP3 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv3(&self) -> WKUPPRIV3_R {
        WKUPPRIV3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - WKUP4 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv4(&self) -> WKUPPRIV4_R {
        WKUPPRIV4_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR")
            .field("priv0", &self.priv0())
            .field("priv1", &self.priv1())
            .field("priv2", &self.priv2())
            .field("priv3", &self.priv3())
            .field("priv4", &self.priv4())
            .field("priv5", &self.priv5())
            .field("priv6", &self.priv6())
            .field("priv7", &self.priv7())
            .field("wkuppriv1", &self.wkuppriv1())
            .field("wkuppriv2", &self.wkuppriv2())
            .field("wkuppriv3", &self.wkuppriv3())
            .field("wkuppriv4", &self.wkuppriv4())
            .finish()
    }
}
impl W {
    ///Bit 0 - System supply configuration privileged protection
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV0_W<'_, PRIVCFGRrs> {
        PRIV0_W::new(self, 0)
    }
    ///Bit 1 - Programmable voltage detector privileged protection
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV1_W<'_, PRIVCFGRrs> {
        PRIV1_W::new(self, 1)
    }
    ///Bit 2 - V less than sub>DDCORE less than /sub> monitor privileged protection
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV2_W<'_, PRIVCFGRrs> {
        PRIV2_W::new(self, 2)
    }
    ///Bit 3 - I-TCM, D-TCM, and I-TCM FLEX MEM low power control privileged protection
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV3_W<'_, PRIVCFGRrs> {
        PRIV3_W::new(self, 3)
    }
    ///Bit 4 - Voltage scaling selection privileged protection
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV4_W<'_, PRIVCFGRrs> {
        PRIV4_W::new(self, 4)
    }
    ///Bit 5 - Backup domain privileged protection
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV5_W<'_, PRIVCFGRrs> {
        PRIV5_W::new(self, 5)
    }
    ///Bit 6 - CPU power control privileged protection
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV6_W<'_, PRIVCFGRrs> {
        PRIV6_W::new(self, 6)
    }
    ///Bit 7 - Peripheral voltage monitor privileged protection
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV7_W<'_, PRIVCFGRrs> {
        PRIV7_W::new(self, 7)
    }
    ///Bit 16 - WKUP1 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv1(&mut self) -> WKUPPRIV1_W<'_, PRIVCFGRrs> {
        WKUPPRIV1_W::new(self, 16)
    }
    ///Bit 17 - WKUP2 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv2(&mut self) -> WKUPPRIV2_W<'_, PRIVCFGRrs> {
        WKUPPRIV2_W::new(self, 17)
    }
    ///Bit 18 - WKUP3 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv3(&mut self) -> WKUPPRIV3_W<'_, PRIVCFGRrs> {
        WKUPPRIV3_W::new(self, 18)
    }
    ///Bit 19 - WKUP4 pin privileged protection
    #[inline(always)]
    pub fn wkuppriv4(&mut self) -> WKUPPRIV4_W<'_, PRIVCFGRrs> {
        WKUPPRIV4_W::new(self, 19)
    }
}
/**PWR privilege configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:PRIVCFGR)*/
pub struct PRIVCFGRrs;
impl crate::RegisterSpec for PRIVCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr::R`](R) reader structure
impl crate::Readable for PRIVCFGRrs {}
///`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure
impl crate::Writable for PRIVCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGRrs {}
