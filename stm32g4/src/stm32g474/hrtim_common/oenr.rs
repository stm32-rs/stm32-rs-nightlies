///Register `OENR` reader
pub type R = crate::R<OENRrs>;
///Register `OENR` writer
pub type W = crate::W<OENRrs>;
/**Timer %s Output 1 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOENR {
    ///0: Output disabled
    Disabled = 0,
    ///1: Output enabled
    Enabled = 1,
}
impl From<TOENR> for bool {
    #[inline(always)]
    fn from(variant: TOENR) -> Self {
        variant as u8 != 0
    }
}
///Field `T1OEN(A,B,C,D,E,F)` reader - Timer %s Output 1 Enable
pub type T1OEN_R = crate::BitReader<TOENR>;
impl T1OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOENR {
        match self.bits {
            false => TOENR::Disabled,
            true => TOENR::Enabled,
        }
    }
    ///Output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TOENR::Disabled
    }
    ///Output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TOENR::Enabled
    }
}
/**Timer %s Output 1 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TORNW {
    ///1: Enable output
    Enable = 1,
}
impl From<TORNW> for bool {
    #[inline(always)]
    fn from(variant: TORNW) -> Self {
        variant as u8 != 0
    }
}
///Field `T1OEN(A,B,C,D,E,F)` writer - Timer %s Output 1 Enable
pub type T1OEN_W<'a, REG> = crate::BitWriter1S<'a, REG, TORNW>;
impl<'a, REG> T1OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable output
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TORNW::Enable)
    }
}
///Field `T2OEN(A,B,C,D,E,F)` reader - Timer %s Output 2 Enable
pub use T1OEN_R as T2OEN_R;
///Field `T2OEN(A,B,C,D,E,F)` writer - Timer %s Output 2 Enable
pub use T1OEN_W as T2OEN_W;
impl R {
    ///Timer (A,B,C,D,E,F) Output 1 Enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA1OEN` field.</div>
    #[inline(always)]
    pub fn t1oen(&self, n: u8) -> T1OEN_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        T1OEN_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E,F) Output 1 Enable
    #[inline(always)]
    pub fn t1oen_iter(&self) -> impl Iterator<Item = T1OEN_R> + '_ {
        (0..6).map(move |n| T1OEN_R::new(((self.bits >> (n * 2)) & 1) != 0))
    }
    ///Bit 0 - Timer A Output 1 Enable
    #[inline(always)]
    pub fn ta1oen(&self) -> T1OEN_R {
        T1OEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Timer B Output 1 Enable
    #[inline(always)]
    pub fn tb1oen(&self) -> T1OEN_R {
        T1OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Timer C Output 1 Enable
    #[inline(always)]
    pub fn tc1oen(&self) -> T1OEN_R {
        T1OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Timer D Output 1 Enable
    #[inline(always)]
    pub fn td1oen(&self) -> T1OEN_R {
        T1OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Timer E Output 1 Enable
    #[inline(always)]
    pub fn te1oen(&self) -> T1OEN_R {
        T1OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Timer F Output 1 Enable
    #[inline(always)]
    pub fn tf1oen(&self) -> T1OEN_R {
        T1OEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Timer (A,B,C,D,E,F) Output 2 Enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA2OEN` field.</div>
    #[inline(always)]
    pub fn t2oen(&self, n: u8) -> T2OEN_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        T2OEN_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E,F) Output 2 Enable
    #[inline(always)]
    pub fn t2oen_iter(&self) -> impl Iterator<Item = T2OEN_R> + '_ {
        (0..6).map(move |n| T2OEN_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0))
    }
    ///Bit 1 - Timer A Output 2 Enable
    #[inline(always)]
    pub fn ta2oen(&self) -> T2OEN_R {
        T2OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Timer B Output 2 Enable
    #[inline(always)]
    pub fn tb2oen(&self) -> T2OEN_R {
        T2OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Timer C Output 2 Enable
    #[inline(always)]
    pub fn tc2oen(&self) -> T2OEN_R {
        T2OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Timer D Output 2 Enable
    #[inline(always)]
    pub fn td2oen(&self) -> T2OEN_R {
        T2OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Timer E Output 2 Enable
    #[inline(always)]
    pub fn te2oen(&self) -> T2OEN_R {
        T2OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Timer F Output 2 Enable
    #[inline(always)]
    pub fn tf2oen(&self) -> T2OEN_R {
        T2OEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OENR")
            .field("ta1oen", &self.ta1oen())
            .field("tb1oen", &self.tb1oen())
            .field("tc1oen", &self.tc1oen())
            .field("td1oen", &self.td1oen())
            .field("te1oen", &self.te1oen())
            .field("tf1oen", &self.tf1oen())
            .field("ta2oen", &self.ta2oen())
            .field("tb2oen", &self.tb2oen())
            .field("tc2oen", &self.tc2oen())
            .field("td2oen", &self.td2oen())
            .field("te2oen", &self.te2oen())
            .field("tf2oen", &self.tf2oen())
            .finish()
    }
}
impl W {
    ///Timer (A,B,C,D,E,F) Output 1 Enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA1OEN` field.</div>
    #[inline(always)]
    pub fn t1oen(&mut self, n: u8) -> T1OEN_W<'_, OENRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        T1OEN_W::new(self, n * 2)
    }
    ///Bit 0 - Timer A Output 1 Enable
    #[inline(always)]
    pub fn ta1oen(&mut self) -> T1OEN_W<'_, OENRrs> {
        T1OEN_W::new(self, 0)
    }
    ///Bit 2 - Timer B Output 1 Enable
    #[inline(always)]
    pub fn tb1oen(&mut self) -> T1OEN_W<'_, OENRrs> {
        T1OEN_W::new(self, 2)
    }
    ///Bit 4 - Timer C Output 1 Enable
    #[inline(always)]
    pub fn tc1oen(&mut self) -> T1OEN_W<'_, OENRrs> {
        T1OEN_W::new(self, 4)
    }
    ///Bit 6 - Timer D Output 1 Enable
    #[inline(always)]
    pub fn td1oen(&mut self) -> T1OEN_W<'_, OENRrs> {
        T1OEN_W::new(self, 6)
    }
    ///Bit 8 - Timer E Output 1 Enable
    #[inline(always)]
    pub fn te1oen(&mut self) -> T1OEN_W<'_, OENRrs> {
        T1OEN_W::new(self, 8)
    }
    ///Bit 10 - Timer F Output 1 Enable
    #[inline(always)]
    pub fn tf1oen(&mut self) -> T1OEN_W<'_, OENRrs> {
        T1OEN_W::new(self, 10)
    }
    ///Timer (A,B,C,D,E,F) Output 2 Enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TA2OEN` field.</div>
    #[inline(always)]
    pub fn t2oen(&mut self, n: u8) -> T2OEN_W<'_, OENRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        T2OEN_W::new(self, n * 2 + 1)
    }
    ///Bit 1 - Timer A Output 2 Enable
    #[inline(always)]
    pub fn ta2oen(&mut self) -> T2OEN_W<'_, OENRrs> {
        T2OEN_W::new(self, 1)
    }
    ///Bit 3 - Timer B Output 2 Enable
    #[inline(always)]
    pub fn tb2oen(&mut self) -> T2OEN_W<'_, OENRrs> {
        T2OEN_W::new(self, 3)
    }
    ///Bit 5 - Timer C Output 2 Enable
    #[inline(always)]
    pub fn tc2oen(&mut self) -> T2OEN_W<'_, OENRrs> {
        T2OEN_W::new(self, 5)
    }
    ///Bit 7 - Timer D Output 2 Enable
    #[inline(always)]
    pub fn td2oen(&mut self) -> T2OEN_W<'_, OENRrs> {
        T2OEN_W::new(self, 7)
    }
    ///Bit 9 - Timer E Output 2 Enable
    #[inline(always)]
    pub fn te2oen(&mut self) -> T2OEN_W<'_, OENRrs> {
        T2OEN_W::new(self, 9)
    }
    ///Bit 11 - Timer F Output 2 Enable
    #[inline(always)]
    pub fn tf2oen(&mut self) -> T2OEN_W<'_, OENRrs> {
        T2OEN_W::new(self, 11)
    }
}
/**Output Enable Register

You can [`read`](crate::Reg::read) this register and get [`oenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#HRTIM_Common:OENR)*/
pub struct OENRrs;
impl crate::RegisterSpec for OENRrs {
    type Ux = u32;
}
///`read()` method returns [`oenr::R`](R) reader structure
impl crate::Readable for OENRrs {}
///`write(|w| ..)` method takes [`oenr::W`](W) writer structure
impl crate::Writable for OENRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fff;
}
///`reset()` method sets OENR to value 0
impl crate::Resettable for OENRrs {}
