///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC0` reader - SEC0
pub type SEC0_R = crate::BitReader;
///Field `SEC0` writer - SEC0
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC1` reader - SEC1
pub type SEC1_R = crate::BitReader;
///Field `SEC1` writer - SEC1
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC2` reader - SEC2
pub type SEC2_R = crate::BitReader;
///Field `SEC2` writer - SEC2
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC3` reader - SEC3
pub type SEC3_R = crate::BitReader;
///Field `SEC3` writer - SEC3
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SEC0
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("sec0", &self.sec0())
            .field("sec1", &self.sec1())
            .field("sec2", &self.sec2())
            .field("sec3", &self.sec3())
            .finish()
    }
}
impl W {
    ///Bit 0 - SEC0
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
}
/**LPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#LPDMA1:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
