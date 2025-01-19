///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Port x reset pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
    Reset = 1,
}
impl From<BR0W> for bool {
    #[inline(always)]
    fn from(variant: BR0W) -> Self {
        variant as u8 != 0
    }
}
///Field `BR(0-4)` writer - Port x reset pin %s
pub type BR_W<'a, REG> = crate::BitWriter<'a, REG, BR0W>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::Reset)
    }
}
impl core::fmt::Debug for crate::generic::Reg<BRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Port x reset pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<BRRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        BR_W::new(self, n)
    }
    ///Bit 0 - Port x reset pin 0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 0)
    }
    ///Bit 1 - Port x reset pin 1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 1)
    }
    ///Bit 2 - Port x reset pin 2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 2)
    }
    ///Bit 3 - Port x reset pin 3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 3)
    }
    ///Bit 4 - Port x reset pin 4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<BRRrs> {
        BR_W::new(self, 4)
    }
}
/**port bit reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
