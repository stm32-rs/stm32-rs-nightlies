///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Reset bit %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RESET {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
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
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::Reset)
    }
}
impl core::fmt::Debug for crate::generic::Reg<BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Reset bit (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<'_, BRRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        BR_W::new(self, n)
    }
    ///Bit 0 - Reset bit 0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 0)
    }
    ///Bit 1 - Reset bit 1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 1)
    }
    ///Bit 2 - Reset bit 2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 2)
    }
    ///Bit 3 - Reset bit 3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 3)
    }
    ///Bit 4 - Reset bit 4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 4)
    }
    ///Bit 5 - Reset bit 5
    #[inline(always)]
    pub fn br5(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 5)
    }
    ///Bit 6 - Reset bit 6
    #[inline(always)]
    pub fn br6(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 6)
    }
    ///Bit 7 - Reset bit 7
    #[inline(always)]
    pub fn br7(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 7)
    }
    ///Bit 8 - Reset bit 8
    #[inline(always)]
    pub fn br8(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 8)
    }
    ///Bit 9 - Reset bit 9
    #[inline(always)]
    pub fn br9(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 9)
    }
    ///Bit 10 - Reset bit 10
    #[inline(always)]
    pub fn br10(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 10)
    }
    ///Bit 11 - Reset bit 11
    #[inline(always)]
    pub fn br11(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 11)
    }
    ///Bit 12 - Reset bit 12
    #[inline(always)]
    pub fn br12(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 12)
    }
    ///Bit 13 - Reset bit 13
    #[inline(always)]
    pub fn br13(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 13)
    }
    ///Bit 14 - Reset bit 14
    #[inline(always)]
    pub fn br14(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 14)
    }
    ///Bit 15 - Reset bit 15
    #[inline(always)]
    pub fn br15(&mut self) -> BR_W<'_, BRRrs> {
        BR_W::new(self, 15)
    }
}
/**Port bit reset register (GPIOn_BRR)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#GPIOA:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {}
