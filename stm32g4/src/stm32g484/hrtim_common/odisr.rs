///Register `ODISR` writer
pub type W = crate::W<ODISRrs>;
///Field `TA1ODIS` writer - TA1ODIS
pub type TA1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TA2ODIS` writer - TA2ODIS
pub type TA2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB1ODIS` writer - TB1ODIS
pub type TB1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB2ODIS` writer - TB2ODIS
pub type TB2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC1ODIS` writer - TC1ODIS
pub type TC1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC2ODIS` writer - TC2ODIS
pub type TC2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD1ODIS` writer - TD1ODIS
pub type TD1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD2ODIS` writer - TD2ODIS
pub type TD2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE1ODIS` writer - TE1ODIS
pub type TE1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE2ODIS` writer - TE2ODIS
pub type TE2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TF1ODIS` writer - TF1ODIS
pub type TF1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TF2ODIS` writer - TF2ODIS
pub type TF2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ODISRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TA1ODIS
    #[inline(always)]
    #[must_use]
    pub fn ta1odis(&mut self) -> TA1ODIS_W<ODISRrs> {
        TA1ODIS_W::new(self, 0)
    }
    ///Bit 1 - TA2ODIS
    #[inline(always)]
    #[must_use]
    pub fn ta2odis(&mut self) -> TA2ODIS_W<ODISRrs> {
        TA2ODIS_W::new(self, 1)
    }
    ///Bit 2 - TB1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tb1odis(&mut self) -> TB1ODIS_W<ODISRrs> {
        TB1ODIS_W::new(self, 2)
    }
    ///Bit 3 - TB2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tb2odis(&mut self) -> TB2ODIS_W<ODISRrs> {
        TB2ODIS_W::new(self, 3)
    }
    ///Bit 4 - TC1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tc1odis(&mut self) -> TC1ODIS_W<ODISRrs> {
        TC1ODIS_W::new(self, 4)
    }
    ///Bit 5 - TC2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tc2odis(&mut self) -> TC2ODIS_W<ODISRrs> {
        TC2ODIS_W::new(self, 5)
    }
    ///Bit 6 - TD1ODIS
    #[inline(always)]
    #[must_use]
    pub fn td1odis(&mut self) -> TD1ODIS_W<ODISRrs> {
        TD1ODIS_W::new(self, 6)
    }
    ///Bit 7 - TD2ODIS
    #[inline(always)]
    #[must_use]
    pub fn td2odis(&mut self) -> TD2ODIS_W<ODISRrs> {
        TD2ODIS_W::new(self, 7)
    }
    ///Bit 8 - TE1ODIS
    #[inline(always)]
    #[must_use]
    pub fn te1odis(&mut self) -> TE1ODIS_W<ODISRrs> {
        TE1ODIS_W::new(self, 8)
    }
    ///Bit 9 - TE2ODIS
    #[inline(always)]
    #[must_use]
    pub fn te2odis(&mut self) -> TE2ODIS_W<ODISRrs> {
        TE2ODIS_W::new(self, 9)
    }
    ///Bit 10 - TF1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tf1odis(&mut self) -> TF1ODIS_W<ODISRrs> {
        TF1ODIS_W::new(self, 10)
    }
    ///Bit 11 - TF2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tf2odis(&mut self) -> TF2ODIS_W<ODISRrs> {
        TF2ODIS_W::new(self, 11)
    }
}
/**ODISR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odisr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#HRTIM_Common:ODISR)*/
pub struct ODISRrs;
impl crate::RegisterSpec for ODISRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`odisr::W`](W) writer structure
impl crate::Writable for ODISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ODISR to value 0
impl crate::Resettable for ODISRrs {
    const RESET_VALUE: u32 = 0;
}
