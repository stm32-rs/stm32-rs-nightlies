///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Port C set I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_SET {
    ///1: Sets the corresponding ODx bit
    Set = 1,
}
impl From<BIT_SET> for bool {
    #[inline(always)]
    fn from(variant: BIT_SET) -> Self {
        variant as u8 != 0
    }
}
///Field `BS13` writer - Port C set I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub type BS13_W<'a, REG> = crate::BitWriter<'a, REG, BIT_SET>;
impl<'a, REG> BS13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_SET::Set)
    }
}
///Field `BS14` writer - Port C set I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub use BS13_W as BS14_W;
///Field `BS15` writer - Port C set I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
pub use BS13_W as BS15_W;
/**Port C reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy. Note: If both BSy and BRy are set, BSy has priority.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_RESET {
    ///1: Resets the corresponding ODx bit
    Reset = 1,
}
impl From<BIT_RESET> for bool {
    #[inline(always)]
    fn from(variant: BIT_RESET) -> Self {
        variant as u8 != 0
    }
}
///Field `BR13` writer - Port C reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy. Note: If both BSy and BRy are set, BSy has priority.
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the corresponding ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_RESET::Reset)
    }
}
///Field `BR14` writer - Port C reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy. Note: If both BSy and BRy are set, BSy has priority.
pub use BR13_W as BR14_W;
///Field `BR15` writer - Port C reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy. Note: If both BSy and BRy are set, BSy has priority.
pub use BR13_W as BR15_W;
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 13 - Port C set I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn bs13(&mut self) -> BS13_W<'_, BSRRrs> {
        BS13_W::new(self, 13)
    }
    ///Bit 14 - Port C set I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn bs14(&mut self) -> BS14_W<'_, BSRRrs> {
        BS14_W::new(self, 14)
    }
    ///Bit 15 - Port C set I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn bs15(&mut self) -> BS15_W<'_, BSRRrs> {
        BS15_W::new(self, 15)
    }
    ///Bit 29 - Port C reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy. Note: If both BSy and BRy are set, BSy has priority.
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<'_, BSRRrs> {
        BR13_W::new(self, 29)
    }
    ///Bit 30 - Port C reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy. Note: If both BSy and BRy are set, BSy has priority.
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<'_, BSRRrs> {
        BR14_W::new(self, 30)
    }
    ///Bit 31 - Port C reset I/O pin y These bits are write-only. A read to these bits returns the value 0. Access can be protected by GPIOC SECy. Note: If both BSy and BRy are set, BSy has priority.
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<'_, BSRRrs> {
        BR15_W::new(self, 31)
    }
}
/**GPIO port C bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPIOC:BSRR)*/
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
