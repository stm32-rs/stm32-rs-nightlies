///Register `DISR` reader
pub type R = crate::R<DISRrs>;
///Register `DISR` writer
pub type W = crate::W<DISRrs>;
///Field `TA1ODIS` reader - TA1ODIS
pub type TA1ODIS_R = crate::BitReader;
///Field `TA1ODIS` writer - TA1ODIS
pub type TA1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TA2ODIS` reader - TA2ODIS
pub type TA2ODIS_R = crate::BitReader;
///Field `TA2ODIS` writer - TA2ODIS
pub type TA2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB1ODIS` reader - TB1ODIS
pub type TB1ODIS_R = crate::BitReader;
///Field `TB1ODIS` writer - TB1ODIS
pub type TB1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB2ODIS` reader - TB2ODIS
pub type TB2ODIS_R = crate::BitReader;
///Field `TB2ODIS` writer - TB2ODIS
pub type TB2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC1ODIS` reader - TC1ODIS
pub type TC1ODIS_R = crate::BitReader;
///Field `TC1ODIS` writer - TC1ODIS
pub type TC1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC2ODIS` reader - TC2ODIS
pub type TC2ODIS_R = crate::BitReader;
///Field `TC2ODIS` writer - TC2ODIS
pub type TC2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD1ODIS` reader - TD1ODIS
pub type TD1ODIS_R = crate::BitReader;
///Field `TD1ODIS` writer - TD1ODIS
pub type TD1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD2ODIS` reader - TD2ODIS
pub type TD2ODIS_R = crate::BitReader;
///Field `TD2ODIS` writer - TD2ODIS
pub type TD2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE1ODIS` reader - TE1ODIS
pub type TE1ODIS_R = crate::BitReader;
///Field `TE1ODIS` writer - TE1ODIS
pub type TE1ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE2ODIS` reader - TE2ODIS
pub type TE2ODIS_R = crate::BitReader;
///Field `TE2ODIS` writer - TE2ODIS
pub type TE2ODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TA1ODIS
    #[inline(always)]
    pub fn ta1odis(&self) -> TA1ODIS_R {
        TA1ODIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TA2ODIS
    #[inline(always)]
    pub fn ta2odis(&self) -> TA2ODIS_R {
        TA2ODIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TB1ODIS
    #[inline(always)]
    pub fn tb1odis(&self) -> TB1ODIS_R {
        TB1ODIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TB2ODIS
    #[inline(always)]
    pub fn tb2odis(&self) -> TB2ODIS_R {
        TB2ODIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TC1ODIS
    #[inline(always)]
    pub fn tc1odis(&self) -> TC1ODIS_R {
        TC1ODIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TC2ODIS
    #[inline(always)]
    pub fn tc2odis(&self) -> TC2ODIS_R {
        TC2ODIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TD1ODIS
    #[inline(always)]
    pub fn td1odis(&self) -> TD1ODIS_R {
        TD1ODIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TD2ODIS
    #[inline(always)]
    pub fn td2odis(&self) -> TD2ODIS_R {
        TD2ODIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TE1ODIS
    #[inline(always)]
    pub fn te1odis(&self) -> TE1ODIS_R {
        TE1ODIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TE2ODIS
    #[inline(always)]
    pub fn te2odis(&self) -> TE2ODIS_R {
        TE2ODIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DISR")
            .field("te2odis", &self.te2odis())
            .field("te1odis", &self.te1odis())
            .field("td2odis", &self.td2odis())
            .field("td1odis", &self.td1odis())
            .field("tc2odis", &self.tc2odis())
            .field("tc1odis", &self.tc1odis())
            .field("tb2odis", &self.tb2odis())
            .field("tb1odis", &self.tb1odis())
            .field("ta2odis", &self.ta2odis())
            .field("ta1odis", &self.ta1odis())
            .finish()
    }
}
impl W {
    ///Bit 0 - TA1ODIS
    #[inline(always)]
    #[must_use]
    pub fn ta1odis(&mut self) -> TA1ODIS_W<DISRrs> {
        TA1ODIS_W::new(self, 0)
    }
    ///Bit 1 - TA2ODIS
    #[inline(always)]
    #[must_use]
    pub fn ta2odis(&mut self) -> TA2ODIS_W<DISRrs> {
        TA2ODIS_W::new(self, 1)
    }
    ///Bit 2 - TB1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tb1odis(&mut self) -> TB1ODIS_W<DISRrs> {
        TB1ODIS_W::new(self, 2)
    }
    ///Bit 3 - TB2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tb2odis(&mut self) -> TB2ODIS_W<DISRrs> {
        TB2ODIS_W::new(self, 3)
    }
    ///Bit 4 - TC1ODIS
    #[inline(always)]
    #[must_use]
    pub fn tc1odis(&mut self) -> TC1ODIS_W<DISRrs> {
        TC1ODIS_W::new(self, 4)
    }
    ///Bit 5 - TC2ODIS
    #[inline(always)]
    #[must_use]
    pub fn tc2odis(&mut self) -> TC2ODIS_W<DISRrs> {
        TC2ODIS_W::new(self, 5)
    }
    ///Bit 6 - TD1ODIS
    #[inline(always)]
    #[must_use]
    pub fn td1odis(&mut self) -> TD1ODIS_W<DISRrs> {
        TD1ODIS_W::new(self, 6)
    }
    ///Bit 7 - TD2ODIS
    #[inline(always)]
    #[must_use]
    pub fn td2odis(&mut self) -> TD2ODIS_W<DISRrs> {
        TD2ODIS_W::new(self, 7)
    }
    ///Bit 8 - TE1ODIS
    #[inline(always)]
    #[must_use]
    pub fn te1odis(&mut self) -> TE1ODIS_W<DISRrs> {
        TE1ODIS_W::new(self, 8)
    }
    ///Bit 9 - TE2ODIS
    #[inline(always)]
    #[must_use]
    pub fn te2odis(&mut self) -> TE2ODIS_W<DISRrs> {
        TE2ODIS_W::new(self, 9)
    }
}
/**DISR

You can [`read`](crate::Reg::read) this register and get [`disr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#HRTIM_Common:DISR)*/
pub struct DISRrs;
impl crate::RegisterSpec for DISRrs {
    type Ux = u32;
}
///`read()` method returns [`disr::R`](R) reader structure
impl crate::Readable for DISRrs {}
///`write(|w| ..)` method takes [`disr::W`](W) writer structure
impl crate::Writable for DISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DISR to value 0
impl crate::Resettable for DISRrs {
    const RESET_VALUE: u32 = 0;
}
