///Register `OENR` writer
pub type W = crate::W<OENRrs>;
///Field `TA1OEN` writer - Timer A Output 1 Enable
pub type TA1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TA2OEN` writer - Timer A Output 2 Enable
pub type TA2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB1OEN` writer - Timer B Output 1 Enable
pub type TB1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB2OEN` writer - Timer B Output 2 Enable
pub type TB2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC1OEN` writer - Timer C Output 1 Enable
pub type TC1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC2OEN` writer - Timer C Output 2 Enable
pub type TC2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD1OEN` writer - Timer D Output 1 Enable
pub type TD1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD2OEN` writer - Timer D Output 2 Enable
pub type TD2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE1OEN` writer - Timer E Output 1 Enable
pub type TE1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE2OEN` writer - Timer E Output 2 Enable
pub type TE2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OENRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Timer A Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn ta1oen(&mut self) -> TA1OEN_W<OENRrs> {
        TA1OEN_W::new(self, 0)
    }
    ///Bit 1 - Timer A Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn ta2oen(&mut self) -> TA2OEN_W<OENRrs> {
        TA2OEN_W::new(self, 1)
    }
    ///Bit 2 - Timer B Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn tb1oen(&mut self) -> TB1OEN_W<OENRrs> {
        TB1OEN_W::new(self, 2)
    }
    ///Bit 3 - Timer B Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn tb2oen(&mut self) -> TB2OEN_W<OENRrs> {
        TB2OEN_W::new(self, 3)
    }
    ///Bit 4 - Timer C Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn tc1oen(&mut self) -> TC1OEN_W<OENRrs> {
        TC1OEN_W::new(self, 4)
    }
    ///Bit 5 - Timer C Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn tc2oen(&mut self) -> TC2OEN_W<OENRrs> {
        TC2OEN_W::new(self, 5)
    }
    ///Bit 6 - Timer D Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn td1oen(&mut self) -> TD1OEN_W<OENRrs> {
        TD1OEN_W::new(self, 6)
    }
    ///Bit 7 - Timer D Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn td2oen(&mut self) -> TD2OEN_W<OENRrs> {
        TD2OEN_W::new(self, 7)
    }
    ///Bit 8 - Timer E Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn te1oen(&mut self) -> TE1OEN_W<OENRrs> {
        TE1OEN_W::new(self, 8)
    }
    ///Bit 9 - Timer E Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn te2oen(&mut self) -> TE2OEN_W<OENRrs> {
        TE2OEN_W::new(self, 9)
    }
}
/**Output Enable Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oenr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_Common:OENR)*/
pub struct OENRrs;
impl crate::RegisterSpec for OENRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`oenr::W`](W) writer structure
impl crate::Writable for OENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OENR to value 0
impl crate::Resettable for OENRrs {
    const RESET_VALUE: u32 = 0;
}
