///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC0` reader - System supply configuration secure protection
pub type SEC0_R = crate::BitReader;
///Field `SEC0` writer - System supply configuration secure protection
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC1` reader - Programmable voltage detector secure protection
pub type SEC1_R = crate::BitReader;
///Field `SEC1` writer - Programmable voltage detector secure protection
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC2` reader - Vless thansub>DDCOREless than/sub> monitor secure protection
pub type SEC2_R = crate::BitReader;
///Field `SEC2` writer - Vless thansub>DDCOREless than/sub> monitor secure protection
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC3` reader - I-TCM, D-TCM, and I-TCM FLEXMEM low power control secure protection
pub type SEC3_R = crate::BitReader;
///Field `SEC3` writer - I-TCM, D-TCM, and I-TCM FLEXMEM low power control secure protection
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC4` reader - Voltage scaling selection secure protection
pub type SEC4_R = crate::BitReader;
///Field `SEC4` writer - Voltage scaling selection secure protection
pub type SEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC5` reader - Backup domain secure protection
pub type SEC5_R = crate::BitReader;
///Field `SEC5` writer - Backup domain secure protection
pub type SEC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC6` reader - CPU power control secure protection
pub type SEC6_R = crate::BitReader;
///Field `SEC6` writer - CPU power control secure protection
pub type SEC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC7` reader - Peripheral voltage monitor secure protection
pub type SEC7_R = crate::BitReader;
///Field `SEC7` writer - Peripheral voltage monitor secure protection
pub type SEC7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPSEC1` reader - WKUP1 pin secure protection
pub type WKUPSEC1_R = crate::BitReader;
///Field `WKUPSEC1` writer - WKUP1 pin secure protection
pub type WKUPSEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPSEC2` reader - WKUP2 pin secure protection
pub type WKUPSEC2_R = crate::BitReader;
///Field `WKUPSEC2` writer - WKUP2 pin secure protection
pub type WKUPSEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPSEC3` reader - WKUP3 pin secure protection
pub type WKUPSEC3_R = crate::BitReader;
///Field `WKUPSEC3` writer - WKUP3 pin secure protection
pub type WKUPSEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPSEC4` reader - WKUP4 pin secure protection
pub type WKUPSEC4_R = crate::BitReader;
///Field `WKUPSEC4` writer - WKUP4 pin secure protection
pub type WKUPSEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - System supply configuration secure protection
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Programmable voltage detector secure protection
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Vless thansub>DDCOREless than/sub> monitor secure protection
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I-TCM, D-TCM, and I-TCM FLEXMEM low power control secure protection
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Voltage scaling selection secure protection
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Backup domain secure protection
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU power control secure protection
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Peripheral voltage monitor secure protection
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - WKUP1 pin secure protection
    #[inline(always)]
    pub fn wkupsec1(&self) -> WKUPSEC1_R {
        WKUPSEC1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - WKUP2 pin secure protection
    #[inline(always)]
    pub fn wkupsec2(&self) -> WKUPSEC2_R {
        WKUPSEC2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - WKUP3 pin secure protection
    #[inline(always)]
    pub fn wkupsec3(&self) -> WKUPSEC3_R {
        WKUPSEC3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - WKUP4 pin secure protection
    #[inline(always)]
    pub fn wkupsec4(&self) -> WKUPSEC4_R {
        WKUPSEC4_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("sec0", &self.sec0())
            .field("sec1", &self.sec1())
            .field("sec2", &self.sec2())
            .field("sec3", &self.sec3())
            .field("sec4", &self.sec4())
            .field("sec5", &self.sec5())
            .field("sec6", &self.sec6())
            .field("sec7", &self.sec7())
            .field("wkupsec1", &self.wkupsec1())
            .field("wkupsec2", &self.wkupsec2())
            .field("wkupsec3", &self.wkupsec3())
            .field("wkupsec4", &self.wkupsec4())
            .finish()
    }
}
impl W {
    ///Bit 0 - System supply configuration secure protection
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W<'_, SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    ///Bit 1 - Programmable voltage detector secure protection
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W<'_, SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    ///Bit 2 - Vless thansub>DDCOREless than/sub> monitor secure protection
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W<'_, SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    ///Bit 3 - I-TCM, D-TCM, and I-TCM FLEXMEM low power control secure protection
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W<'_, SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    ///Bit 4 - Voltage scaling selection secure protection
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W<'_, SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    ///Bit 5 - Backup domain secure protection
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W<'_, SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    ///Bit 6 - CPU power control secure protection
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W<'_, SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    ///Bit 7 - Peripheral voltage monitor secure protection
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W<'_, SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
    ///Bit 16 - WKUP1 pin secure protection
    #[inline(always)]
    pub fn wkupsec1(&mut self) -> WKUPSEC1_W<'_, SECCFGRrs> {
        WKUPSEC1_W::new(self, 16)
    }
    ///Bit 17 - WKUP2 pin secure protection
    #[inline(always)]
    pub fn wkupsec2(&mut self) -> WKUPSEC2_W<'_, SECCFGRrs> {
        WKUPSEC2_W::new(self, 17)
    }
    ///Bit 18 - WKUP3 pin secure protection
    #[inline(always)]
    pub fn wkupsec3(&mut self) -> WKUPSEC3_W<'_, SECCFGRrs> {
        WKUPSEC3_W::new(self, 18)
    }
    ///Bit 19 - WKUP4 pin secure protection
    #[inline(always)]
    pub fn wkupsec4(&mut self) -> WKUPSEC4_W<'_, SECCFGRrs> {
        WKUPSEC4_W::new(self, 19)
    }
}
/**PWR security configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#PWR:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
