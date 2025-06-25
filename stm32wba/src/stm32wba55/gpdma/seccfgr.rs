///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC0` reader - secure state of channel x
pub type SEC0_R = crate::BitReader;
///Field `SEC0` writer - secure state of channel x
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC1` reader - secure state of channel x
pub type SEC1_R = crate::BitReader;
///Field `SEC1` writer - secure state of channel x
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC2` reader - secure state of channel x
pub type SEC2_R = crate::BitReader;
///Field `SEC2` writer - secure state of channel x
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC3` reader - secure state of channel x
pub type SEC3_R = crate::BitReader;
///Field `SEC3` writer - secure state of channel x
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC4` reader - secure state of channel x
pub type SEC4_R = crate::BitReader;
///Field `SEC4` writer - secure state of channel x
pub type SEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC5` reader - secure state of channel x
pub type SEC5_R = crate::BitReader;
///Field `SEC5` writer - secure state of channel x
pub type SEC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC6` reader - secure state of channel x
pub type SEC6_R = crate::BitReader;
///Field `SEC6` writer - secure state of channel x
pub type SEC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC7` reader - secure state of channel x
pub type SEC7_R = crate::BitReader;
///Field `SEC7` writer - secure state of channel x
pub type SEC7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure state of channel x
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure state of channel x
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure state of channel x
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure state of channel x
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure state of channel x
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure state of channel x
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure state of channel x
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure state of channel x
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
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
            .finish()
    }
}
impl W {
    ///Bit 0 - secure state of channel x
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W<SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    ///Bit 1 - secure state of channel x
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W<SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    ///Bit 2 - secure state of channel x
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W<SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    ///Bit 3 - secure state of channel x
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W<SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    ///Bit 4 - secure state of channel x
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W<SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    ///Bit 5 - secure state of channel x
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W<SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    ///Bit 6 - secure state of channel x
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W<SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    ///Bit 7 - secure state of channel x
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W<SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
}
/**GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPDMA:SECCFGR)*/
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
