///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Port x set pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0W {
    ///1: Sets the corresponding ODRx bit
    Set = 1,
}
impl From<BS0W> for bool {
    #[inline(always)]
    fn from(variant: BS0W) -> Self {
        variant as u8 != 0
    }
}
///Field `BS(0-4)` writer - Port x set pin %s
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG, BS0W>;
impl<'a, REG> BS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BS0W::Set)
    }
}
/**Port x reset pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W {
    ///1: Resets the corresponding ODRx bit
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
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::Reset)
    }
}
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Port x set pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BS0` field.</div>
    #[inline(always)]
    pub fn bs(&mut self, n: u8) -> BS_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        BS_W::new(self, n)
    }
    ///Bit 0 - Port x set pin 0
    #[inline(always)]
    pub fn bs0(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 0)
    }
    ///Bit 1 - Port x set pin 1
    #[inline(always)]
    pub fn bs1(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 1)
    }
    ///Bit 2 - Port x set pin 2
    #[inline(always)]
    pub fn bs2(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 2)
    }
    ///Bit 3 - Port x set pin 3
    #[inline(always)]
    pub fn bs3(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 3)
    }
    ///Bit 4 - Port x set pin 4
    #[inline(always)]
    pub fn bs4(&mut self) -> BS_W<BSRRrs> {
        BS_W::new(self, 4)
    }
    ///Port x reset pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BR0` field.</div>
    #[inline(always)]
    pub fn br(&mut self, n: u8) -> BR_W<BSRRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        BR_W::new(self, n + 16)
    }
    ///Bit 16 - Port x reset pin 0
    #[inline(always)]
    pub fn br0(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 16)
    }
    ///Bit 17 - Port x reset pin 1
    #[inline(always)]
    pub fn br1(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 17)
    }
    ///Bit 18 - Port x reset pin 2
    #[inline(always)]
    pub fn br2(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 18)
    }
    ///Bit 19 - Port x reset pin 3
    #[inline(always)]
    pub fn br3(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 19)
    }
    ///Bit 20 - Port x reset pin 4
    #[inline(always)]
    pub fn br4(&mut self) -> BR_W<BSRRrs> {
        BR_W::new(self, 20)
    }
}
/**GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:BSRR)*/
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bsrr::W`](W) writer structure
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRRrs {
    const RESET_VALUE: u32 = 0;
}
