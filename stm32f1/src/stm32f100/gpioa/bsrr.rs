#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRRrs>;
#[doc = "Set bit %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0W {
    #[doc = "1: Sets the corresponding ODRx bit"]
    Set = 1,
}
impl From<BS0W> for bool {
    #[inline(always)]
    fn from(variant: BS0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS(0-15)` writer - Set bit %s"]
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG, BS0W>;
impl<'a, REG> BS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(BS0W::Set)
    }
}
#[doc = "Reset bit %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W {
    #[doc = "1: Resets the corresponding ODRx bit"]
    Reset = 1,
}
impl From<BR0W> for bool {
    #[inline(always)]
    fn from(variant: BR0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR(0-15)` writer - Reset bit %s"]
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG, BR0W>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::Reset)
    }
}
impl W {
    #[doc = "Set bit (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `BS0` field"]
    #[inline(always)]
    #[must_use]
    pub fn bs(&mut self, n: u8) -> BS_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BS_W::new(self, n)
    }
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 15)
    }
    #[doc = "Reset bit (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `BR0` field"]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self, n: u8) -> BR_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Reset bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Reset bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Reset bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Reset bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 23)
    }
    #[doc = "Bit 24 - Reset bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Reset bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Reset bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Reset bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reset bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reset bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Reset bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reset bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 31)
    }
}
#[doc = "Port bit set/reset register (GPIOn_BSRR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRRrs {
    const RESET_VALUE: u32 = 0;
}
