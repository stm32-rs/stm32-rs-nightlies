///Register `FS1R` reader
pub type R = crate::R<FS1Rrs>;
///Register `FS1R` writer
pub type W = crate::W<FS1Rrs>;
///Field `FSC(0-13)` reader - Filter scale configuration
pub type FSC_R = crate::BitReader;
///Field `FSC(0-13)` writer - Filter scale configuration
pub type FSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Filter scale configuration
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FSC0` field.</div>
    #[inline(always)]
    pub fn fsc(&self, n: u8) -> FSC_R {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FSC_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Filter scale configuration
    #[inline(always)]
    pub fn fsc_iter(&self) -> impl Iterator<Item = FSC_R> + '_ {
        (0..14).map(move |n| FSC_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Filter scale configuration
    #[inline(always)]
    pub fn fsc0(&self) -> FSC_R {
        FSC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter scale configuration
    #[inline(always)]
    pub fn fsc1(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter scale configuration
    #[inline(always)]
    pub fn fsc2(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter scale configuration
    #[inline(always)]
    pub fn fsc3(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter scale configuration
    #[inline(always)]
    pub fn fsc4(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter scale configuration
    #[inline(always)]
    pub fn fsc5(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter scale configuration
    #[inline(always)]
    pub fn fsc6(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter scale configuration
    #[inline(always)]
    pub fn fsc7(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter scale configuration
    #[inline(always)]
    pub fn fsc8(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter scale configuration
    #[inline(always)]
    pub fn fsc9(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter scale configuration
    #[inline(always)]
    pub fn fsc10(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter scale configuration
    #[inline(always)]
    pub fn fsc11(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter scale configuration
    #[inline(always)]
    pub fn fsc12(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter scale configuration
    #[inline(always)]
    pub fn fsc13(&self) -> FSC_R {
        FSC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS1R")
            .field("fsc0", &self.fsc0())
            .field("fsc1", &self.fsc1())
            .field("fsc2", &self.fsc2())
            .field("fsc3", &self.fsc3())
            .field("fsc4", &self.fsc4())
            .field("fsc5", &self.fsc5())
            .field("fsc6", &self.fsc6())
            .field("fsc7", &self.fsc7())
            .field("fsc8", &self.fsc8())
            .field("fsc9", &self.fsc9())
            .field("fsc10", &self.fsc10())
            .field("fsc11", &self.fsc11())
            .field("fsc12", &self.fsc12())
            .field("fsc13", &self.fsc13())
            .finish()
    }
}
impl W {
    ///Filter scale configuration
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FSC0` field.</div>
    #[inline(always)]
    pub fn fsc(&mut self, n: u8) -> FSC_W<FS1Rrs> {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        FSC_W::new(self, n)
    }
    ///Bit 0 - Filter scale configuration
    #[inline(always)]
    pub fn fsc0(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 0)
    }
    ///Bit 1 - Filter scale configuration
    #[inline(always)]
    pub fn fsc1(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 1)
    }
    ///Bit 2 - Filter scale configuration
    #[inline(always)]
    pub fn fsc2(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 2)
    }
    ///Bit 3 - Filter scale configuration
    #[inline(always)]
    pub fn fsc3(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 3)
    }
    ///Bit 4 - Filter scale configuration
    #[inline(always)]
    pub fn fsc4(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 4)
    }
    ///Bit 5 - Filter scale configuration
    #[inline(always)]
    pub fn fsc5(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 5)
    }
    ///Bit 6 - Filter scale configuration
    #[inline(always)]
    pub fn fsc6(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 6)
    }
    ///Bit 7 - Filter scale configuration
    #[inline(always)]
    pub fn fsc7(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 7)
    }
    ///Bit 8 - Filter scale configuration
    #[inline(always)]
    pub fn fsc8(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 8)
    }
    ///Bit 9 - Filter scale configuration
    #[inline(always)]
    pub fn fsc9(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 9)
    }
    ///Bit 10 - Filter scale configuration
    #[inline(always)]
    pub fn fsc10(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 10)
    }
    ///Bit 11 - Filter scale configuration
    #[inline(always)]
    pub fn fsc11(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 11)
    }
    ///Bit 12 - Filter scale configuration
    #[inline(always)]
    pub fn fsc12(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 12)
    }
    ///Bit 13 - Filter scale configuration
    #[inline(always)]
    pub fn fsc13(&mut self) -> FSC_W<FS1Rrs> {
        FSC_W::new(self, 13)
    }
}
/**filter scale register

You can [`read`](crate::Reg::read) this register and get [`fs1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CAN1:FS1R)*/
pub struct FS1Rrs;
impl crate::RegisterSpec for FS1Rrs {
    type Ux = u32;
}
///`read()` method returns [`fs1r::R`](R) reader structure
impl crate::Readable for FS1Rrs {}
///`write(|w| ..)` method takes [`fs1r::W`](W) writer structure
impl crate::Writable for FS1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS1R to value 0
impl crate::Resettable for FS1Rrs {}
