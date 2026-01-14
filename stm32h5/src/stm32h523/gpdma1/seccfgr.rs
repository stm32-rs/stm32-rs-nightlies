///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC(0-7)` reader - secure state of channel x
pub type SEC_R = crate::BitReader;
///Field `SEC(0-7)` writer - secure state of channel x
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///secure state of channel x
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&self, n: u8) -> SEC_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SEC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///secure state of channel x
    #[inline(always)]
    pub fn sec_iter(&self) -> impl Iterator<Item = SEC_R> + '_ {
        (0..8).map(move |n| SEC_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - secure state of channel x
    #[inline(always)]
    pub fn sec0(&self) -> SEC_R {
        SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure state of channel x
    #[inline(always)]
    pub fn sec1(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - secure state of channel x
    #[inline(always)]
    pub fn sec2(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - secure state of channel x
    #[inline(always)]
    pub fn sec3(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - secure state of channel x
    #[inline(always)]
    pub fn sec4(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - secure state of channel x
    #[inline(always)]
    pub fn sec5(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure state of channel x
    #[inline(always)]
    pub fn sec6(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - secure state of channel x
    #[inline(always)]
    pub fn sec7(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 7) & 1) != 0)
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
    ///secure state of channel x
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SEC0` field.</div>
    #[inline(always)]
    pub fn sec(&mut self, n: u8) -> SEC_W<'_, SECCFGRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SEC_W::new(self, n)
    }
    ///Bit 0 - secure state of channel x
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 0)
    }
    ///Bit 1 - secure state of channel x
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 1)
    }
    ///Bit 2 - secure state of channel x
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 2)
    }
    ///Bit 3 - secure state of channel x
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 3)
    }
    ///Bit 4 - secure state of channel x
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 4)
    }
    ///Bit 5 - secure state of channel x
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 5)
    }
    ///Bit 6 - secure state of channel x
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 6)
    }
    ///Bit 7 - secure state of channel x
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC_W<'_, SECCFGRrs> {
        SEC_W::new(self, 7)
    }
}
/**GPDMA secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#GPDMA1:SECCFGR)*/
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
