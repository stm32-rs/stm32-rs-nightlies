///Register `FIR0` writer
pub type W = crate::W<FIR0rs>;
///Field `FAE(0-15)` writer - Force acknowledge error %s
pub type FAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPE(0-4)` writer - Force PHY error %s
pub type FPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FIR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Force acknowledge error (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FAE0` field.</div>
    #[inline(always)]
    pub fn fae(&mut self, n: u8) -> FAE_W<'_, FIR0rs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        FAE_W::new(self, n)
    }
    ///Bit 0 - Force acknowledge error 0
    #[inline(always)]
    pub fn fae0(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 0)
    }
    ///Bit 1 - Force acknowledge error 1
    #[inline(always)]
    pub fn fae1(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 1)
    }
    ///Bit 2 - Force acknowledge error 2
    #[inline(always)]
    pub fn fae2(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 2)
    }
    ///Bit 3 - Force acknowledge error 3
    #[inline(always)]
    pub fn fae3(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 3)
    }
    ///Bit 4 - Force acknowledge error 4
    #[inline(always)]
    pub fn fae4(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 4)
    }
    ///Bit 5 - Force acknowledge error 5
    #[inline(always)]
    pub fn fae5(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 5)
    }
    ///Bit 6 - Force acknowledge error 6
    #[inline(always)]
    pub fn fae6(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 6)
    }
    ///Bit 7 - Force acknowledge error 7
    #[inline(always)]
    pub fn fae7(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 7)
    }
    ///Bit 8 - Force acknowledge error 8
    #[inline(always)]
    pub fn fae8(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 8)
    }
    ///Bit 9 - Force acknowledge error 9
    #[inline(always)]
    pub fn fae9(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 9)
    }
    ///Bit 10 - Force acknowledge error 10
    #[inline(always)]
    pub fn fae10(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 10)
    }
    ///Bit 11 - Force acknowledge error 11
    #[inline(always)]
    pub fn fae11(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 11)
    }
    ///Bit 12 - Force acknowledge error 12
    #[inline(always)]
    pub fn fae12(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 12)
    }
    ///Bit 13 - Force acknowledge error 13
    #[inline(always)]
    pub fn fae13(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 13)
    }
    ///Bit 14 - Force acknowledge error 14
    #[inline(always)]
    pub fn fae14(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 14)
    }
    ///Bit 15 - Force acknowledge error 15
    #[inline(always)]
    pub fn fae15(&mut self) -> FAE_W<'_, FIR0rs> {
        FAE_W::new(self, 15)
    }
    ///Force PHY error (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FPE0` field.</div>
    #[inline(always)]
    pub fn fpe(&mut self, n: u8) -> FPE_W<'_, FIR0rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FPE_W::new(self, n + 16)
    }
    ///Bit 16 - Force PHY error 0
    #[inline(always)]
    pub fn fpe0(&mut self) -> FPE_W<'_, FIR0rs> {
        FPE_W::new(self, 16)
    }
    ///Bit 17 - Force PHY error 1
    #[inline(always)]
    pub fn fpe1(&mut self) -> FPE_W<'_, FIR0rs> {
        FPE_W::new(self, 17)
    }
    ///Bit 18 - Force PHY error 2
    #[inline(always)]
    pub fn fpe2(&mut self) -> FPE_W<'_, FIR0rs> {
        FPE_W::new(self, 18)
    }
    ///Bit 19 - Force PHY error 3
    #[inline(always)]
    pub fn fpe3(&mut self) -> FPE_W<'_, FIR0rs> {
        FPE_W::new(self, 19)
    }
    ///Bit 20 - Force PHY error 4
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE_W<'_, FIR0rs> {
        FPE_W::new(self, 20)
    }
}
/**DSI Host Force Interrupt Register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#DSI:FIR0)*/
pub struct FIR0rs;
impl crate::RegisterSpec for FIR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fir0::W`](W) writer structure
impl crate::Writable for FIR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIR0 to value 0
impl crate::Resettable for FIR0rs {}
