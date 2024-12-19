///Register `FIR0` reader
pub type R = crate::R<FIR0rs>;
///Register `FIR0` writer
pub type W = crate::W<FIR0rs>;
///Field `FAE(0-15)` reader - Force acknowledge error %s
pub type FAE_R = crate::BitReader;
///Field `FAE(0-15)` writer - Force acknowledge error %s
pub type FAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPE(0-4)` reader - Force PHY error %s
pub type FPE_R = crate::BitReader;
///Field `FPE(0-4)` writer - Force PHY error %s
pub type FPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Force acknowledge error (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FAE0` field.</div>
    #[inline(always)]
    pub fn fae(&self, n: u8) -> FAE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        FAE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Force acknowledge error (0-15)
    #[inline(always)]
    pub fn fae_iter(&self) -> impl Iterator<Item = FAE_R> + '_ {
        (0..16).map(move |n| FAE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Force acknowledge error 0
    #[inline(always)]
    pub fn fae0(&self) -> FAE_R {
        FAE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Force acknowledge error 1
    #[inline(always)]
    pub fn fae1(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Force acknowledge error 2
    #[inline(always)]
    pub fn fae2(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Force acknowledge error 3
    #[inline(always)]
    pub fn fae3(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Force acknowledge error 4
    #[inline(always)]
    pub fn fae4(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Force acknowledge error 5
    #[inline(always)]
    pub fn fae5(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Force acknowledge error 6
    #[inline(always)]
    pub fn fae6(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Force acknowledge error 7
    #[inline(always)]
    pub fn fae7(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Force acknowledge error 8
    #[inline(always)]
    pub fn fae8(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Force acknowledge error 9
    #[inline(always)]
    pub fn fae9(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Force acknowledge error 10
    #[inline(always)]
    pub fn fae10(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Force acknowledge error 11
    #[inline(always)]
    pub fn fae11(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Force acknowledge error 12
    #[inline(always)]
    pub fn fae12(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Force acknowledge error 13
    #[inline(always)]
    pub fn fae13(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Force acknowledge error 14
    #[inline(always)]
    pub fn fae14(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Force acknowledge error 15
    #[inline(always)]
    pub fn fae15(&self) -> FAE_R {
        FAE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Force PHY error (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FPE0` field.</div>
    #[inline(always)]
    pub fn fpe(&self, n: u8) -> FPE_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FPE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Force PHY error (0-4)
    #[inline(always)]
    pub fn fpe_iter(&self) -> impl Iterator<Item = FPE_R> + '_ {
        (0..5).map(move |n| FPE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - Force PHY error 0
    #[inline(always)]
    pub fn fpe0(&self) -> FPE_R {
        FPE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Force PHY error 1
    #[inline(always)]
    pub fn fpe1(&self) -> FPE_R {
        FPE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Force PHY error 2
    #[inline(always)]
    pub fn fpe2(&self) -> FPE_R {
        FPE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Force PHY error 3
    #[inline(always)]
    pub fn fpe3(&self) -> FPE_R {
        FPE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Force PHY error 4
    #[inline(always)]
    pub fn fpe4(&self) -> FPE_R {
        FPE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIR0")
            .field("fpe0", &self.fpe0())
            .field("fpe1", &self.fpe1())
            .field("fpe2", &self.fpe2())
            .field("fpe3", &self.fpe3())
            .field("fpe4", &self.fpe4())
            .field("fae0", &self.fae0())
            .field("fae1", &self.fae1())
            .field("fae2", &self.fae2())
            .field("fae3", &self.fae3())
            .field("fae4", &self.fae4())
            .field("fae5", &self.fae5())
            .field("fae6", &self.fae6())
            .field("fae7", &self.fae7())
            .field("fae8", &self.fae8())
            .field("fae9", &self.fae9())
            .field("fae10", &self.fae10())
            .field("fae11", &self.fae11())
            .field("fae12", &self.fae12())
            .field("fae13", &self.fae13())
            .field("fae14", &self.fae14())
            .field("fae15", &self.fae15())
            .finish()
    }
}
impl W {
    ///Force acknowledge error (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FAE0` field.</div>
    #[inline(always)]
    pub fn fae(&mut self, n: u8) -> FAE_W<FIR0rs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        FAE_W::new(self, n)
    }
    ///Bit 0 - Force acknowledge error 0
    #[inline(always)]
    pub fn fae0(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 0)
    }
    ///Bit 1 - Force acknowledge error 1
    #[inline(always)]
    pub fn fae1(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 1)
    }
    ///Bit 2 - Force acknowledge error 2
    #[inline(always)]
    pub fn fae2(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 2)
    }
    ///Bit 3 - Force acknowledge error 3
    #[inline(always)]
    pub fn fae3(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 3)
    }
    ///Bit 4 - Force acknowledge error 4
    #[inline(always)]
    pub fn fae4(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 4)
    }
    ///Bit 5 - Force acknowledge error 5
    #[inline(always)]
    pub fn fae5(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 5)
    }
    ///Bit 6 - Force acknowledge error 6
    #[inline(always)]
    pub fn fae6(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 6)
    }
    ///Bit 7 - Force acknowledge error 7
    #[inline(always)]
    pub fn fae7(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 7)
    }
    ///Bit 8 - Force acknowledge error 8
    #[inline(always)]
    pub fn fae8(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 8)
    }
    ///Bit 9 - Force acknowledge error 9
    #[inline(always)]
    pub fn fae9(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 9)
    }
    ///Bit 10 - Force acknowledge error 10
    #[inline(always)]
    pub fn fae10(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 10)
    }
    ///Bit 11 - Force acknowledge error 11
    #[inline(always)]
    pub fn fae11(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 11)
    }
    ///Bit 12 - Force acknowledge error 12
    #[inline(always)]
    pub fn fae12(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 12)
    }
    ///Bit 13 - Force acknowledge error 13
    #[inline(always)]
    pub fn fae13(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 13)
    }
    ///Bit 14 - Force acknowledge error 14
    #[inline(always)]
    pub fn fae14(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 14)
    }
    ///Bit 15 - Force acknowledge error 15
    #[inline(always)]
    pub fn fae15(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 15)
    }
    ///Force PHY error (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FPE0` field.</div>
    #[inline(always)]
    pub fn fpe(&mut self, n: u8) -> FPE_W<FIR0rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FPE_W::new(self, n + 16)
    }
    ///Bit 16 - Force PHY error 0
    #[inline(always)]
    pub fn fpe0(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 16)
    }
    ///Bit 17 - Force PHY error 1
    #[inline(always)]
    pub fn fpe1(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 17)
    }
    ///Bit 18 - Force PHY error 2
    #[inline(always)]
    pub fn fpe2(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 18)
    }
    ///Bit 19 - Force PHY error 3
    #[inline(always)]
    pub fn fpe3(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 19)
    }
    ///Bit 20 - Force PHY error 4
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 20)
    }
}
/**DSI Host force interrupt register 0

You can [`read`](crate::Reg::read) this register and get [`fir0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:FIR0)*/
pub struct FIR0rs;
impl crate::RegisterSpec for FIR0rs {
    type Ux = u32;
}
///`read()` method returns [`fir0::R`](R) reader structure
impl crate::Readable for FIR0rs {}
///`write(|w| ..)` method takes [`fir0::W`](W) writer structure
impl crate::Writable for FIR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIR0 to value 0
impl crate::Resettable for FIR0rs {
    const RESET_VALUE: u32 = 0;
}
