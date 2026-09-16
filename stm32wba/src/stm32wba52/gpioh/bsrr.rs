///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Port H set I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3.

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
///Field `BS3` writer - Port H set I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3.
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG, BIT_SET>;
impl<'a, REG> BS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BIT_SET::Set)
    }
}
/**Port H reset I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3. Note: If both BS3 and BR3 are set, BS3 has priority.

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
///Field `BR3` writer - Port H reset I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3. Note: If both BS3 and BR3 are set, BS3 has priority.
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BIT_RESET>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the corresponding ODx bit
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
    ///Bit 3 - Port H set I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W<'_, BSRRrs> {
        BS3_W::new(self, 3)
    }
    ///Bit 19 - Port H reset I/O pin 3 This bit is write-only. A read to this bit returns the value 0. Access can be protected by GPIOH SEC3. Note: If both BS3 and BR3 are set, BS3 has priority.
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<'_, BSRRrs> {
        BR3_W::new(self, 19)
    }
}
/**GPIO port H bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOH:BSRR)*/
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
