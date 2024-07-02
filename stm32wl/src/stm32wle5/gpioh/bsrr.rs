///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Port x set bit y (y= 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS3W {
    ///1: Sets the corresponding ODRx bit
    Set = 1,
}
impl From<BS3W> for bool {
    #[inline(always)]
    fn from(variant: BS3W) -> Self {
        variant as u8 != 0
    }
}
///Field `BS3` writer - Port x set bit y (y= 0..15)
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG, BS3W>;
impl<'a, REG> BS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODRx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BS3W::Set)
    }
}
/**Port x reset bit y (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3W {
    ///1: Resets the corresponding ODRx bit
    Reset = 1,
}
impl From<BR3W> for bool {
    #[inline(always)]
    fn from(variant: BR3W) -> Self {
        variant as u8 != 0
    }
}
///Field `BR3` writer - Port x reset bit y (y = 0..15)
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BR3W>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the corresponding ODRx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR3W::Reset)
    }
}
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - Port x set bit y (y= 0..15)
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    ///Bit 19 - Port x reset bit y (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BSRRrs> {
        BR3_W::new(self, 19)
    }
}
/**GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#GPIOH:BSRR)*/
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
