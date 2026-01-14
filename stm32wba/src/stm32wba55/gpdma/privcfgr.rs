///Register `PRIVCFGR` reader
pub type R = crate::R<PRIVCFGRrs>;
///Register `PRIVCFGR` writer
pub type W = crate::W<PRIVCFGRrs>;
///Field `PRIV(0-7)` reader - privileged state of channel x
pub type PRIV_R = crate::BitReader;
///Field `PRIV(0-7)` writer - privileged state of channel x
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///privileged state of channel x
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PRIV0` field.</div>
    #[inline(always)]
    pub fn priv_(&self, n: u8) -> PRIV_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PRIV_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///privileged state of channel x
    #[inline(always)]
    pub fn priv__iter(&self) -> impl Iterator<Item = PRIV_R> + '_ {
        (0..8).map(move |n| PRIV_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - privileged state of channel x
    #[inline(always)]
    pub fn priv0(&self) -> PRIV_R {
        PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged state of channel x
    #[inline(always)]
    pub fn priv1(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - privileged state of channel x
    #[inline(always)]
    pub fn priv2(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - privileged state of channel x
    #[inline(always)]
    pub fn priv3(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged state of channel x
    #[inline(always)]
    pub fn priv4(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - privileged state of channel x
    #[inline(always)]
    pub fn priv5(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged state of channel x
    #[inline(always)]
    pub fn priv6(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - privileged state of channel x
    #[inline(always)]
    pub fn priv7(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 7) & 1) != 0)
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
            .finish()
    }
}
impl W {
    ///privileged state of channel x
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PRIV0` field.</div>
    #[inline(always)]
    pub fn priv_(&mut self, n: u8) -> PRIV_W<'_, PRIVCFGRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PRIV_W::new(self, n)
    }
    ///Bit 0 - privileged state of channel x
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged state of channel x
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 1)
    }
    ///Bit 2 - privileged state of channel x
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 2)
    }
    ///Bit 3 - privileged state of channel x
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 3)
    }
    ///Bit 4 - privileged state of channel x
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 4)
    }
    ///Bit 5 - privileged state of channel x
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 5)
    }
    ///Bit 6 - privileged state of channel x
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 6)
    }
    ///Bit 7 - privileged state of channel x
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV_W<'_, PRIVCFGRrs> {
        PRIV_W::new(self, 7)
    }
}
/**GPDMA privileged configuration register

You can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPDMA:PRIVCFGR)*/
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
