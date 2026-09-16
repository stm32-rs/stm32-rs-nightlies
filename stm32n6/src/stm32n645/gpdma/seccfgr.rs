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
///Field `SEC8` reader - secure state of channel x
pub type SEC8_R = crate::BitReader;
///Field `SEC8` writer - secure state of channel x
pub type SEC8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC9` reader - secure state of channel x
pub type SEC9_R = crate::BitReader;
///Field `SEC9` writer - secure state of channel x
pub type SEC9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC10` reader - secure state of channel x
pub type SEC10_R = crate::BitReader;
///Field `SEC10` writer - secure state of channel x
pub type SEC10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC11` reader - secure state of channel x
pub type SEC11_R = crate::BitReader;
///Field `SEC11` writer - secure state of channel x
pub type SEC11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC12` reader - secure state of channel x
pub type SEC12_R = crate::BitReader;
///Field `SEC12` writer - secure state of channel x
pub type SEC12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC13` reader - secure state of channel x
pub type SEC13_R = crate::BitReader;
///Field `SEC13` writer - secure state of channel x
pub type SEC13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC14` reader - secure state of channel x
pub type SEC14_R = crate::BitReader;
///Field `SEC14` writer - secure state of channel x
pub type SEC14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC15` reader - secure state of channel x
pub type SEC15_R = crate::BitReader;
///Field `SEC15` writer - secure state of channel x
pub type SEC15_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - secure state of channel x
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - secure state of channel x
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - secure state of channel x
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - secure state of channel x
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - secure state of channel x
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - secure state of channel x
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - secure state of channel x
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - secure state of channel x
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 1) != 0)
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
            .field("sec8", &self.sec8())
            .field("sec9", &self.sec9())
            .field("sec10", &self.sec10())
            .field("sec11", &self.sec11())
            .field("sec12", &self.sec12())
            .field("sec13", &self.sec13())
            .field("sec14", &self.sec14())
            .field("sec15", &self.sec15())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure state of channel x
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W<'_, SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    ///Bit 1 - secure state of channel x
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W<'_, SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    ///Bit 2 - secure state of channel x
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W<'_, SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    ///Bit 3 - secure state of channel x
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W<'_, SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    ///Bit 4 - secure state of channel x
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W<'_, SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    ///Bit 5 - secure state of channel x
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W<'_, SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    ///Bit 6 - secure state of channel x
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W<'_, SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    ///Bit 7 - secure state of channel x
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W<'_, SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
    ///Bit 8 - secure state of channel x
    #[inline(always)]
    pub fn sec8(&mut self) -> SEC8_W<'_, SECCFGRrs> {
        SEC8_W::new(self, 8)
    }
    ///Bit 9 - secure state of channel x
    #[inline(always)]
    pub fn sec9(&mut self) -> SEC9_W<'_, SECCFGRrs> {
        SEC9_W::new(self, 9)
    }
    ///Bit 10 - secure state of channel x
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W<'_, SECCFGRrs> {
        SEC10_W::new(self, 10)
    }
    ///Bit 11 - secure state of channel x
    #[inline(always)]
    pub fn sec11(&mut self) -> SEC11_W<'_, SECCFGRrs> {
        SEC11_W::new(self, 11)
    }
    ///Bit 12 - secure state of channel x
    #[inline(always)]
    pub fn sec12(&mut self) -> SEC12_W<'_, SECCFGRrs> {
        SEC12_W::new(self, 12)
    }
    ///Bit 13 - secure state of channel x
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W<'_, SECCFGRrs> {
        SEC13_W::new(self, 13)
    }
    ///Bit 14 - secure state of channel x
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W<'_, SECCFGRrs> {
        SEC14_W::new(self, 14)
    }
    ///Bit 15 - secure state of channel x
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W<'_, SECCFGRrs> {
        SEC15_W::new(self, 15)
    }
}
/**GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPDMA:SECCFGR)*/
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
