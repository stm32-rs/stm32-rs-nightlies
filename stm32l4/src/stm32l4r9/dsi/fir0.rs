#[doc = "Register `FIR0` writer"]
pub type W = crate::W<FIR0rs>;
#[doc = "Field `FAE(0-15)` writer - Force Acknowledge Error %s"]
pub type FAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPE(0-4)` writer - Force PHY Error %s"]
pub type FPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Force Acknowledge Error (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FAE0` field"]
    #[inline(always)]
    #[must_use]
    pub fn fae(&mut self, n: u8) -> FAE_W<FIR0rs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        FAE_W::new(self, n)
    }
    #[doc = "Bit 0 - Force Acknowledge Error 0"]
    #[inline(always)]
    #[must_use]
    pub fn fae0(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force Acknowledge Error 1"]
    #[inline(always)]
    #[must_use]
    pub fn fae1(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Force Acknowledge Error 2"]
    #[inline(always)]
    #[must_use]
    pub fn fae2(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Force Acknowledge Error 3"]
    #[inline(always)]
    #[must_use]
    pub fn fae3(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Force Acknowledge Error 4"]
    #[inline(always)]
    #[must_use]
    pub fn fae4(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Force Acknowledge Error 5"]
    #[inline(always)]
    #[must_use]
    pub fn fae5(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force Acknowledge Error 6"]
    #[inline(always)]
    #[must_use]
    pub fn fae6(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Acknowledge Error 7"]
    #[inline(always)]
    #[must_use]
    pub fn fae7(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Force Acknowledge Error 8"]
    #[inline(always)]
    #[must_use]
    pub fn fae8(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Force Acknowledge Error 9"]
    #[inline(always)]
    #[must_use]
    pub fn fae9(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Force Acknowledge Error 10"]
    #[inline(always)]
    #[must_use]
    pub fn fae10(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Force Acknowledge Error 11"]
    #[inline(always)]
    #[must_use]
    pub fn fae11(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Force Acknowledge Error 12"]
    #[inline(always)]
    #[must_use]
    pub fn fae12(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Force Acknowledge Error 13"]
    #[inline(always)]
    #[must_use]
    pub fn fae13(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Force Acknowledge Error 14"]
    #[inline(always)]
    #[must_use]
    pub fn fae14(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Force Acknowledge Error 15"]
    #[inline(always)]
    #[must_use]
    pub fn fae15(&mut self) -> FAE_W<FIR0rs> {
        FAE_W::new(self, 15)
    }
    #[doc = "Force PHY Error (0-4)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `FPE0` field"]
    #[inline(always)]
    #[must_use]
    pub fn fpe(&mut self, n: u8) -> FPE_W<FIR0rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FPE_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Force PHY Error 0"]
    #[inline(always)]
    #[must_use]
    pub fn fpe0(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Force PHY Error 1"]
    #[inline(always)]
    #[must_use]
    pub fn fpe1(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Force PHY Error 2"]
    #[inline(always)]
    #[must_use]
    pub fn fpe2(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Force PHY Error 3"]
    #[inline(always)]
    #[must_use]
    pub fn fpe3(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Force PHY Error 4"]
    #[inline(always)]
    #[must_use]
    pub fn fpe4(&mut self) -> FPE_W<FIR0rs> {
        FPE_W::new(self, 20)
    }
}
#[doc = "DSI Host Force Interrupt Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fir0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIR0rs;
impl crate::RegisterSpec for FIR0rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fir0::W`](W) writer structure"]
impl crate::Writable for FIR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIR0 to value 0"]
impl crate::Resettable for FIR0rs {
    const RESET_VALUE: u32 = 0;
}
