///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Set bit %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_SET {
    ///1: Sets the corresponding ODRx bit
    Set = 1,
}
impl From<BIT_SET> for bool {
    #[inline(always)]
    fn from(variant: BIT_SET) -> Self {
        variant as u8 != 0
    }
}
///Field `BS(0-15)` writer - Set bit %s
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG, BIT_SET>;
impl<'a, REG> BS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_SET::Set)
    }
}
/**Reset bit %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RESET {
    ///1: Resets the corresponding ODRx bit
    Reset = 1,
}
impl From<BIT_RESET> for bool {
    #[inline(always)]
    fn from(variant: BIT_RESET) -> Self {
        variant as u8 != 0
    }
}
///Field `BR(0-15)` writer - Reset bit %s
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::Reset)
    }
}
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Set bit (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BS0` field.</div>
    #[inline(always)]
    pub fn bs(&mut self, n: u8) -> BS_W<'_, BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BS_W::new(self, n)
    }
    ///Bit 0 - Set bit 0
    #[inline(always)]
    pub fn bs0(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 0)
    }
    ///Bit 1 - Set bit 1
    #[inline(always)]
    pub fn bs1(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 1)
    }
    ///Bit 2 - Set bit 2
    #[inline(always)]
    pub fn bs2(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 2)
    }
    ///Bit 3 - Set bit 3
    #[inline(always)]
    pub fn bs3(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 3)
    }
    ///Bit 4 - Set bit 4
    #[inline(always)]
    pub fn bs4(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 4)
    }
    ///Bit 5 - Set bit 5
    #[inline(always)]
    pub fn bs5(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 5)
    }
    ///Bit 6 - Set bit 6
    #[inline(always)]
    pub fn bs6(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 6)
    }
    ///Bit 7 - Set bit 7
    #[inline(always)]
    pub fn bs7(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 7)
    }
    ///Bit 8 - Set bit 8
    #[inline(always)]
    pub fn bs8(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 8)
    }
    ///Bit 9 - Set bit 9
    #[inline(always)]
    pub fn bs9(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 9)
    }
    ///Bit 10 - Set bit 10
    #[inline(always)]
    pub fn bs10(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 10)
    }
    ///Bit 11 - Set bit 11
    #[inline(always)]
    pub fn bs11(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 11)
    }
    ///Bit 12 - Set bit 12
    #[inline(always)]
    pub fn bs12(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 12)
    }
    ///Bit 13 - Set bit 13
    #[inline(always)]
    pub fn bs13(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 13)
    }
    ///Bit 14 - Set bit 14
    #[inline(always)]
    pub fn bs14(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 14)
    }
    ///Bit 15 - Set bit 15
    #[inline(always)]
    pub fn bs15(&mut self) -> BS_W<'_, BSRRrs> {
        BS_W::new(self, 15)
    }
    ///Reset bit (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<'_, BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n + 16)
    }
    ///Bit 16 - Reset bit 0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 16)
    }
    ///Bit 17 - Reset bit 1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 17)
    }
    ///Bit 18 - Reset bit 2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 18)
    }
    ///Bit 19 - Reset bit 3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 19)
    }
    ///Bit 20 - Reset bit 4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 20)
    }
    ///Bit 21 - Reset bit 5
    #[inline(always)]
    pub fn br5(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 21)
    }
    ///Bit 22 - Reset bit 6
    #[inline(always)]
    pub fn br6(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 22)
    }
    ///Bit 23 - Reset bit 7
    #[inline(always)]
    pub fn br7(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 23)
    }
    ///Bit 24 - Reset bit 8
    #[inline(always)]
    pub fn br8(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 24)
    }
    ///Bit 25 - Reset bit 9
    #[inline(always)]
    pub fn br9(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 25)
    }
    ///Bit 26 - Reset bit 10
    #[inline(always)]
    pub fn br10(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 26)
    }
    ///Bit 27 - Reset bit 11
    #[inline(always)]
    pub fn br11(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 27)
    }
    ///Bit 28 - Reset bit 12
    #[inline(always)]
    pub fn br12(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 28)
    }
    ///Bit 29 - Reset bit 13
    #[inline(always)]
    pub fn br13(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 29)
    }
    ///Bit 30 - Reset bit 14
    #[inline(always)]
    pub fn br14(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 30)
    }
    ///Bit 31 - Reset bit 15
    #[inline(always)]
    pub fn br15(&mut self) -> BR_W<'_, BSRRrs> {
        BR_W::new(self, 31)
    }
}
/**Port bit set/reset register (GPIOn_BSRR)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#GPIOA:BSRR)*/
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bsrr::W`](W) writer structure
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRRrs {}
