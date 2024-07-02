///Register `OENR` reader
pub type R = crate::R<OENRrs>;
///Register `OENR` writer
pub type W = crate::W<OENRrs>;
///Field `TA1OEN` reader - Timer A Output 1 Enable
pub type TA1OEN_R = crate::BitReader;
///Field `TA1OEN` writer - Timer A Output 1 Enable
pub type TA1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TA2OEN` reader - Timer A Output 2 Enable
pub type TA2OEN_R = crate::BitReader;
///Field `TA2OEN` writer - Timer A Output 2 Enable
pub type TA2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB1OEN` reader - Timer B Output 1 Enable
pub type TB1OEN_R = crate::BitReader;
///Field `TB1OEN` writer - Timer B Output 1 Enable
pub type TB1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TB2OEN` reader - Timer B Output 2 Enable
pub type TB2OEN_R = crate::BitReader;
///Field `TB2OEN` writer - Timer B Output 2 Enable
pub type TB2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC1OEN` reader - Timer C Output 1 Enable
pub type TC1OEN_R = crate::BitReader;
///Field `TC1OEN` writer - Timer C Output 1 Enable
pub type TC1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TC2OEN` reader - Timer C Output 2 Enable
pub type TC2OEN_R = crate::BitReader;
///Field `TC2OEN` writer - Timer C Output 2 Enable
pub type TC2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD1OEN` reader - Timer D Output 1 Enable
pub type TD1OEN_R = crate::BitReader;
///Field `TD1OEN` writer - Timer D Output 1 Enable
pub type TD1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TD2OEN` reader - Timer D Output 2 Enable
pub type TD2OEN_R = crate::BitReader;
///Field `TD2OEN` writer - Timer D Output 2 Enable
pub type TD2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE1OEN` reader - Timer E Output 1 Enable
pub type TE1OEN_R = crate::BitReader;
///Field `TE1OEN` writer - Timer E Output 1 Enable
pub type TE1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE2OEN` reader - Timer E Output 2 Enable
pub type TE2OEN_R = crate::BitReader;
///Field `TE2OEN` writer - Timer E Output 2 Enable
pub type TE2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Bit 10 - Timer F Output 1 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF1OENR {
    ///0: Output disabled
    Disabled = 0,
    ///1: Output enabled
    Enabled = 1,
}
impl From<TF1OENR> for bool {
    #[inline(always)]
    fn from(variant: TF1OENR) -> Self {
        variant as u8 != 0
    }
}
///Field `TF1OEN` reader - Bit 10 - Timer F Output 1 Enable
pub type TF1OEN_R = crate::BitReader<TF1OENR>;
impl TF1OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TF1OENR {
        match self.bits {
            false => TF1OENR::Disabled,
            true => TF1OENR::Enabled,
        }
    }
    ///Output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TF1OENR::Disabled
    }
    ///Output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TF1OENR::Enabled
    }
}
/**Bit 10 - Timer F Output 1 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF1OENW {
    ///1: Enable output
    Enable = 1,
}
impl From<TF1OENW> for bool {
    #[inline(always)]
    fn from(variant: TF1OENW) -> Self {
        variant as u8 != 0
    }
}
///Field `TF1OEN` writer - Bit 10 - Timer F Output 1 Enable
pub type TF1OEN_W<'a, REG> = crate::BitWriter<'a, REG, TF1OENW>;
impl<'a, REG> TF1OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable output
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TF1OENW::Enable)
    }
}
/**Bit 11 - Timer F Output 2 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF2OENR {
    ///0: Output disabled
    Disabled = 0,
    ///1: Output enabled
    Enabled = 1,
}
impl From<TF2OENR> for bool {
    #[inline(always)]
    fn from(variant: TF2OENR) -> Self {
        variant as u8 != 0
    }
}
///Field `TF2OEN` reader - Bit 11 - Timer F Output 2 Enable
pub type TF2OEN_R = crate::BitReader<TF2OENR>;
impl TF2OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TF2OENR {
        match self.bits {
            false => TF2OENR::Disabled,
            true => TF2OENR::Enabled,
        }
    }
    ///Output disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TF2OENR::Disabled
    }
    ///Output enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TF2OENR::Enabled
    }
}
/**Bit 11 - Timer F Output 2 Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF2OENW {
    ///1: Enable output
    Enable = 1,
}
impl From<TF2OENW> for bool {
    #[inline(always)]
    fn from(variant: TF2OENW) -> Self {
        variant as u8 != 0
    }
}
///Field `TF2OEN` writer - Bit 11 - Timer F Output 2 Enable
pub type TF2OEN_W<'a, REG> = crate::BitWriter<'a, REG, TF2OENW>;
impl<'a, REG> TF2OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable output
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TF2OENW::Enable)
    }
}
impl R {
    ///Bit 0 - Timer A Output 1 Enable
    #[inline(always)]
    pub fn ta1oen(&self) -> TA1OEN_R {
        TA1OEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A Output 2 Enable
    #[inline(always)]
    pub fn ta2oen(&self) -> TA2OEN_R {
        TA2OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer B Output 1 Enable
    #[inline(always)]
    pub fn tb1oen(&self) -> TB1OEN_R {
        TB1OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer B Output 2 Enable
    #[inline(always)]
    pub fn tb2oen(&self) -> TB2OEN_R {
        TB2OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer C Output 1 Enable
    #[inline(always)]
    pub fn tc1oen(&self) -> TC1OEN_R {
        TC1OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer C Output 2 Enable
    #[inline(always)]
    pub fn tc2oen(&self) -> TC2OEN_R {
        TC2OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer D Output 1 Enable
    #[inline(always)]
    pub fn td1oen(&self) -> TD1OEN_R {
        TD1OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer D Output 2 Enable
    #[inline(always)]
    pub fn td2oen(&self) -> TD2OEN_R {
        TD2OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Timer E Output 1 Enable
    #[inline(always)]
    pub fn te1oen(&self) -> TE1OEN_R {
        TE1OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Timer E Output 2 Enable
    #[inline(always)]
    pub fn te2oen(&self) -> TE2OEN_R {
        TE2OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Bit 10 - Timer F Output 1 Enable
    #[inline(always)]
    pub fn tf1oen(&self) -> TF1OEN_R {
        TF1OEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Bit 11 - Timer F Output 2 Enable
    #[inline(always)]
    pub fn tf2oen(&self) -> TF2OEN_R {
        TF2OEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OENR")
            .field("tf2oen", &self.tf2oen())
            .field("tf1oen", &self.tf1oen())
            .field("te2oen", &self.te2oen())
            .field("te1oen", &self.te1oen())
            .field("td2oen", &self.td2oen())
            .field("td1oen", &self.td1oen())
            .field("tc2oen", &self.tc2oen())
            .field("tc1oen", &self.tc1oen())
            .field("tb2oen", &self.tb2oen())
            .field("tb1oen", &self.tb1oen())
            .field("ta2oen", &self.ta2oen())
            .field("ta1oen", &self.ta1oen())
            .finish()
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
    ///Bit 10 - Bit 10 - Timer F Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn tf1oen(&mut self) -> TF1OEN_W<OENRrs> {
        TF1OEN_W::new(self, 10)
    }
    ///Bit 11 - Bit 11 - Timer F Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn tf2oen(&mut self) -> TF2OEN_W<OENRrs> {
        TF2OEN_W::new(self, 11)
    }
}
/**Output Enable Register

You can [`read`](crate::Reg::read) this register and get [`oenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_Common:OENR)*/
pub struct OENRrs;
impl crate::RegisterSpec for OENRrs {
    type Ux = u32;
}
///`read()` method returns [`oenr::R`](R) reader structure
impl crate::Readable for OENRrs {}
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
