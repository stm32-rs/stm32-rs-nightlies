///Register `ODISR` writer
pub type W = crate::W<ODISRrs>;
/**T%s1ODIS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TODIS {
    ///1: Disable output
    Disable = 1,
}
impl From<TODIS> for bool {
    #[inline(always)]
    fn from(variant: TODIS) -> Self {
        variant as u8 != 0
    }
}
///Field `T1ODIS(A,B,C,D,E,F)` writer - T%s1ODIS
pub type T1ODIS_W<'a, REG> = crate::BitWriter1S<'a, REG, TODIS>;
impl<'a, REG> T1ODIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable output
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TODIS::Disable)
    }
}
///Field `T2ODIS(A,B,C,D,E,F)` writer - T%s2ODIS
pub use T1ODIS_W as T2ODIS_W;
impl core::fmt::Debug for crate::generic::Reg<ODISRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///T(A,B,C,D,E,F)1ODIS
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA1ODIS` field.</div>
    #[inline(always)]
    pub fn t1odis(&mut self, n: u8) -> T1ODIS_W<'_, ODISRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        T1ODIS_W::new(self, n * 2)
    }
    ///Bit 0 - TA1ODIS
    #[inline(always)]
    pub fn ta1odis(&mut self) -> T1ODIS_W<'_, ODISRrs> {
        T1ODIS_W::new(self, 0)
    }
    ///Bit 2 - TB1ODIS
    #[inline(always)]
    pub fn tb1odis(&mut self) -> T1ODIS_W<'_, ODISRrs> {
        T1ODIS_W::new(self, 2)
    }
    ///Bit 4 - TC1ODIS
    #[inline(always)]
    pub fn tc1odis(&mut self) -> T1ODIS_W<'_, ODISRrs> {
        T1ODIS_W::new(self, 4)
    }
    ///Bit 6 - TD1ODIS
    #[inline(always)]
    pub fn td1odis(&mut self) -> T1ODIS_W<'_, ODISRrs> {
        T1ODIS_W::new(self, 6)
    }
    ///Bit 8 - TE1ODIS
    #[inline(always)]
    pub fn te1odis(&mut self) -> T1ODIS_W<'_, ODISRrs> {
        T1ODIS_W::new(self, 8)
    }
    ///Bit 10 - TF1ODIS
    #[inline(always)]
    pub fn tf1odis(&mut self) -> T1ODIS_W<'_, ODISRrs> {
        T1ODIS_W::new(self, 10)
    }
    ///T(A,B,C,D,E,F)2ODIS
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA2ODIS` field.</div>
    #[inline(always)]
    pub fn t2odis(&mut self, n: u8) -> T2ODIS_W<'_, ODISRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        T2ODIS_W::new(self, n * 2 + 1)
    }
    ///Bit 1 - TA2ODIS
    #[inline(always)]
    pub fn ta2odis(&mut self) -> T2ODIS_W<'_, ODISRrs> {
        T2ODIS_W::new(self, 1)
    }
    ///Bit 3 - TB2ODIS
    #[inline(always)]
    pub fn tb2odis(&mut self) -> T2ODIS_W<'_, ODISRrs> {
        T2ODIS_W::new(self, 3)
    }
    ///Bit 5 - TC2ODIS
    #[inline(always)]
    pub fn tc2odis(&mut self) -> T2ODIS_W<'_, ODISRrs> {
        T2ODIS_W::new(self, 5)
    }
    ///Bit 7 - TD2ODIS
    #[inline(always)]
    pub fn td2odis(&mut self) -> T2ODIS_W<'_, ODISRrs> {
        T2ODIS_W::new(self, 7)
    }
    ///Bit 9 - TE2ODIS
    #[inline(always)]
    pub fn te2odis(&mut self) -> T2ODIS_W<'_, ODISRrs> {
        T2ODIS_W::new(self, 9)
    }
    ///Bit 11 - TF2ODIS
    #[inline(always)]
    pub fn tf2odis(&mut self) -> T2ODIS_W<'_, ODISRrs> {
        T2ODIS_W::new(self, 11)
    }
}
/**ODISR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odisr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:ODISR)*/
pub struct ODISRrs;
impl crate::RegisterSpec for ODISRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`odisr::W`](W) writer structure
impl crate::Writable for ODISRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fff;
}
///`reset()` method sets ODISR to value 0
impl crate::Resettable for ODISRrs {}
