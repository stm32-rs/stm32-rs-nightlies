///Register `EECR1` reader
pub type R = crate::R<EECR1rs>;
///Register `EECR1` writer
pub type W = crate::W<EECR1rs>;
/**External Event 1 Source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE1SRC {
    ///0: Source 1
    Src1 = 0,
    ///1: Source 2
    Src2 = 1,
    ///2: Source 3
    Src3 = 2,
    ///3: Source 4
    Src4 = 3,
}
impl From<EE1SRC> for u8 {
    #[inline(always)]
    fn from(variant: EE1SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE1SRC {
    type Ux = u8;
}
impl crate::IsEnum for EE1SRC {}
///Field `EE1SRC` reader - External Event 1 Source
pub type EE1SRC_R = crate::FieldReader<EE1SRC>;
impl EE1SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE1SRC {
        match self.bits {
            0 => EE1SRC::Src1,
            1 => EE1SRC::Src2,
            2 => EE1SRC::Src3,
            3 => EE1SRC::Src4,
            _ => unreachable!(),
        }
    }
    ///Source 1
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        *self == EE1SRC::Src1
    }
    ///Source 2
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        *self == EE1SRC::Src2
    }
    ///Source 3
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        *self == EE1SRC::Src3
    }
    ///Source 4
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        *self == EE1SRC::Src4
    }
}
///Field `EE1SRC` writer - External Event 1 Source
pub type EE1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE1SRC, crate::Safe>;
impl<'a, REG> EE1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Source 1
    #[inline(always)]
    pub fn src1(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SRC::Src1)
    }
    ///Source 2
    #[inline(always)]
    pub fn src2(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SRC::Src2)
    }
    ///Source 3
    #[inline(always)]
    pub fn src3(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SRC::Src3)
    }
    ///Source 4
    #[inline(always)]
    pub fn src4(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SRC::Src4)
    }
}
/**External Event 1 Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE1POL {
    ///0: External event is active high
    ActiveHigh = 0,
    ///1: External event is active low
    ActiveLow = 1,
}
impl From<EE1POL> for bool {
    #[inline(always)]
    fn from(variant: EE1POL) -> Self {
        variant as u8 != 0
    }
}
///Field `EE1POL` reader - External Event 1 Polarity
pub type EE1POL_R = crate::BitReader<EE1POL>;
impl EE1POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE1POL {
        match self.bits {
            false => EE1POL::ActiveHigh,
            true => EE1POL::ActiveLow,
        }
    }
    ///External event is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == EE1POL::ActiveHigh
    }
    ///External event is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == EE1POL::ActiveLow
    }
}
///Field `EE1POL` writer - External Event 1 Polarity
pub type EE1POL_W<'a, REG> = crate::BitWriter<'a, REG, EE1POL>;
impl<'a, REG> EE1POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External event is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(EE1POL::ActiveHigh)
    }
    ///External event is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(EE1POL::ActiveLow)
    }
}
/**External Event 1 Sensitivity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE1SNS {
    ///0: On active level defined by EExPOL bit
    Active = 0,
    ///1: Rising edge
    Rising = 1,
    ///2: Falling edge
    Falling = 2,
    ///3: Both edges
    Both = 3,
}
impl From<EE1SNS> for u8 {
    #[inline(always)]
    fn from(variant: EE1SNS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE1SNS {
    type Ux = u8;
}
impl crate::IsEnum for EE1SNS {}
///Field `EE1SNS` reader - External Event 1 Sensitivity
pub type EE1SNS_R = crate::FieldReader<EE1SNS>;
impl EE1SNS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE1SNS {
        match self.bits {
            0 => EE1SNS::Active,
            1 => EE1SNS::Rising,
            2 => EE1SNS::Falling,
            3 => EE1SNS::Both,
            _ => unreachable!(),
        }
    }
    ///On active level defined by EExPOL bit
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == EE1SNS::Active
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EE1SNS::Rising
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EE1SNS::Falling
    }
    ///Both edges
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EE1SNS::Both
    }
}
///Field `EE1SNS` writer - External Event 1 Sensitivity
pub type EE1SNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE1SNS, crate::Safe>;
impl<'a, REG> EE1SNS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///On active level defined by EExPOL bit
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SNS::Active)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SNS::Rising)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SNS::Falling)
    }
    ///Both edges
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(EE1SNS::Both)
    }
}
/**External Event 1 Fast mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE1FAST {
    ///0: External event is re-synchronised by the HRTIM logic before acting on outputs
    Resynchronized = 0,
    ///1: External event is acting asynchronously on outputs (low-latency mode)
    Asynchronous = 1,
}
impl From<EE1FAST> for bool {
    #[inline(always)]
    fn from(variant: EE1FAST) -> Self {
        variant as u8 != 0
    }
}
///Field `EE1FAST` reader - External Event 1 Fast mode
pub type EE1FAST_R = crate::BitReader<EE1FAST>;
impl EE1FAST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE1FAST {
        match self.bits {
            false => EE1FAST::Resynchronized,
            true => EE1FAST::Asynchronous,
        }
    }
    ///External event is re-synchronised by the HRTIM logic before acting on outputs
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == EE1FAST::Resynchronized
    }
    ///External event is acting asynchronously on outputs (low-latency mode)
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == EE1FAST::Asynchronous
    }
}
///Field `EE1FAST` writer - External Event 1 Fast mode
pub type EE1FAST_W<'a, REG> = crate::BitWriter<'a, REG, EE1FAST>;
impl<'a, REG> EE1FAST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External event is re-synchronised by the HRTIM logic before acting on outputs
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FAST::Resynchronized)
    }
    ///External event is acting asynchronously on outputs (low-latency mode)
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FAST::Asynchronous)
    }
}
///Field `EE2FAST` reader - External Event 2 Fast mode
pub use EE1FAST_R as EE2FAST_R;
///Field `EE3FAST` reader - External Event 3 Fast mode
pub use EE1FAST_R as EE3FAST_R;
///Field `EE4FAST` reader - External Event 4 Fast mode
pub use EE1FAST_R as EE4FAST_R;
///Field `EE5FAST` reader - External Event 5 Fast mode
pub use EE1FAST_R as EE5FAST_R;
///Field `EE2FAST` writer - External Event 2 Fast mode
pub use EE1FAST_W as EE2FAST_W;
///Field `EE3FAST` writer - External Event 3 Fast mode
pub use EE1FAST_W as EE3FAST_W;
///Field `EE4FAST` writer - External Event 4 Fast mode
pub use EE1FAST_W as EE4FAST_W;
///Field `EE5FAST` writer - External Event 5 Fast mode
pub use EE1FAST_W as EE5FAST_W;
///Field `EE2POL` reader - External Event 2 Polarity
pub use EE1POL_R as EE2POL_R;
///Field `EE3POL` reader - External Event 3 Polarity
pub use EE1POL_R as EE3POL_R;
///Field `EE4POL` reader - External Event 4 Polarity
pub use EE1POL_R as EE4POL_R;
///Field `EE5POL` reader - External Event 5 Polarity
pub use EE1POL_R as EE5POL_R;
///Field `EE2POL` writer - External Event 2 Polarity
pub use EE1POL_W as EE2POL_W;
///Field `EE3POL` writer - External Event 3 Polarity
pub use EE1POL_W as EE3POL_W;
///Field `EE4POL` writer - External Event 4 Polarity
pub use EE1POL_W as EE4POL_W;
///Field `EE5POL` writer - External Event 5 Polarity
pub use EE1POL_W as EE5POL_W;
///Field `EE2SNS` reader - External Event 2 Sensitivity
pub use EE1SNS_R as EE2SNS_R;
///Field `EE3SNS` reader - External Event 3 Sensitivity
pub use EE1SNS_R as EE3SNS_R;
///Field `EE4SNS` reader - External Event 4 Sensitivity
pub use EE1SNS_R as EE4SNS_R;
///Field `EE5SNS` reader - External Event 5 Sensitivity
pub use EE1SNS_R as EE5SNS_R;
///Field `EE2SNS` writer - External Event 2 Sensitivity
pub use EE1SNS_W as EE2SNS_W;
///Field `EE3SNS` writer - External Event 3 Sensitivity
pub use EE1SNS_W as EE3SNS_W;
///Field `EE4SNS` writer - External Event 4 Sensitivity
pub use EE1SNS_W as EE4SNS_W;
///Field `EE5SNS` writer - External Event 5 Sensitivity
pub use EE1SNS_W as EE5SNS_W;
///Field `EE2SRC` reader - External Event 2 Source
pub use EE1SRC_R as EE2SRC_R;
///Field `EE3SRC` reader - External Event 3 Source
pub use EE1SRC_R as EE3SRC_R;
///Field `EE4SRC` reader - External Event 4 Source
pub use EE1SRC_R as EE4SRC_R;
///Field `EE5SRC` reader - External Event 5 Source
pub use EE1SRC_R as EE5SRC_R;
///Field `EE2SRC` writer - External Event 2 Source
pub use EE1SRC_W as EE2SRC_W;
///Field `EE3SRC` writer - External Event 3 Source
pub use EE1SRC_W as EE3SRC_W;
///Field `EE4SRC` writer - External Event 4 Source
pub use EE1SRC_W as EE4SRC_W;
///Field `EE5SRC` writer - External Event 5 Source
pub use EE1SRC_W as EE5SRC_W;
impl R {
    ///Bits 0:1 - External Event 1 Source
    #[inline(always)]
    pub fn ee1src(&self) -> EE1SRC_R {
        EE1SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - External Event 1 Polarity
    #[inline(always)]
    pub fn ee1pol(&self) -> EE1POL_R {
        EE1POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - External Event 1 Sensitivity
    #[inline(always)]
    pub fn ee1sns(&self) -> EE1SNS_R {
        EE1SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - External Event 1 Fast mode
    #[inline(always)]
    pub fn ee1fast(&self) -> EE1FAST_R {
        EE1FAST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - External Event 2 Source
    #[inline(always)]
    pub fn ee2src(&self) -> EE2SRC_R {
        EE2SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - External Event 2 Polarity
    #[inline(always)]
    pub fn ee2pol(&self) -> EE2POL_R {
        EE2POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - External Event 2 Sensitivity
    #[inline(always)]
    pub fn ee2sns(&self) -> EE2SNS_R {
        EE2SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - External Event 2 Fast mode
    #[inline(always)]
    pub fn ee2fast(&self) -> EE2FAST_R {
        EE2FAST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - External Event 3 Source
    #[inline(always)]
    pub fn ee3src(&self) -> EE3SRC_R {
        EE3SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External Event 3 Polarity
    #[inline(always)]
    pub fn ee3pol(&self) -> EE3POL_R {
        EE3POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:16 - External Event 3 Sensitivity
    #[inline(always)]
    pub fn ee3sns(&self) -> EE3SNS_R {
        EE3SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bit 17 - External Event 3 Fast mode
    #[inline(always)]
    pub fn ee3fast(&self) -> EE3FAST_R {
        EE3FAST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - External Event 4 Source
    #[inline(always)]
    pub fn ee4src(&self) -> EE4SRC_R {
        EE4SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - External Event 4 Polarity
    #[inline(always)]
    pub fn ee4pol(&self) -> EE4POL_R {
        EE4POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - External Event 4 Sensitivity
    #[inline(always)]
    pub fn ee4sns(&self) -> EE4SNS_R {
        EE4SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - External Event 4 Fast mode
    #[inline(always)]
    pub fn ee4fast(&self) -> EE4FAST_R {
        EE4FAST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - External Event 5 Source
    #[inline(always)]
    pub fn ee5src(&self) -> EE5SRC_R {
        EE5SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - External Event 5 Polarity
    #[inline(always)]
    pub fn ee5pol(&self) -> EE5POL_R {
        EE5POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - External Event 5 Sensitivity
    #[inline(always)]
    pub fn ee5sns(&self) -> EE5SNS_R {
        EE5SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 29 - External Event 5 Fast mode
    #[inline(always)]
    pub fn ee5fast(&self) -> EE5FAST_R {
        EE5FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR1")
            .field("ee1fast", &self.ee1fast())
            .field("ee5fast", &self.ee5fast())
            .field("ee1sns", &self.ee1sns())
            .field("ee5sns", &self.ee5sns())
            .field("ee1pol", &self.ee1pol())
            .field("ee5pol", &self.ee5pol())
            .field("ee1src", &self.ee1src())
            .field("ee5src", &self.ee5src())
            .field("ee4fast", &self.ee4fast())
            .field("ee4sns", &self.ee4sns())
            .field("ee4pol", &self.ee4pol())
            .field("ee4src", &self.ee4src())
            .field("ee3fast", &self.ee3fast())
            .field("ee3sns", &self.ee3sns())
            .field("ee3pol", &self.ee3pol())
            .field("ee3src", &self.ee3src())
            .field("ee2fast", &self.ee2fast())
            .field("ee2sns", &self.ee2sns())
            .field("ee2pol", &self.ee2pol())
            .field("ee2src", &self.ee2src())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - External Event 1 Source
    #[inline(always)]
    #[must_use]
    pub fn ee1src(&mut self) -> EE1SRC_W<EECR1rs> {
        EE1SRC_W::new(self, 0)
    }
    ///Bit 2 - External Event 1 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee1pol(&mut self) -> EE1POL_W<EECR1rs> {
        EE1POL_W::new(self, 2)
    }
    ///Bits 3:4 - External Event 1 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee1sns(&mut self) -> EE1SNS_W<EECR1rs> {
        EE1SNS_W::new(self, 3)
    }
    ///Bit 5 - External Event 1 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee1fast(&mut self) -> EE1FAST_W<EECR1rs> {
        EE1FAST_W::new(self, 5)
    }
    ///Bits 6:7 - External Event 2 Source
    #[inline(always)]
    #[must_use]
    pub fn ee2src(&mut self) -> EE2SRC_W<EECR1rs> {
        EE2SRC_W::new(self, 6)
    }
    ///Bit 8 - External Event 2 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee2pol(&mut self) -> EE2POL_W<EECR1rs> {
        EE2POL_W::new(self, 8)
    }
    ///Bits 9:10 - External Event 2 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee2sns(&mut self) -> EE2SNS_W<EECR1rs> {
        EE2SNS_W::new(self, 9)
    }
    ///Bit 11 - External Event 2 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee2fast(&mut self) -> EE2FAST_W<EECR1rs> {
        EE2FAST_W::new(self, 11)
    }
    ///Bits 12:13 - External Event 3 Source
    #[inline(always)]
    #[must_use]
    pub fn ee3src(&mut self) -> EE3SRC_W<EECR1rs> {
        EE3SRC_W::new(self, 12)
    }
    ///Bit 14 - External Event 3 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee3pol(&mut self) -> EE3POL_W<EECR1rs> {
        EE3POL_W::new(self, 14)
    }
    ///Bits 15:16 - External Event 3 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee3sns(&mut self) -> EE3SNS_W<EECR1rs> {
        EE3SNS_W::new(self, 15)
    }
    ///Bit 17 - External Event 3 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee3fast(&mut self) -> EE3FAST_W<EECR1rs> {
        EE3FAST_W::new(self, 17)
    }
    ///Bits 18:19 - External Event 4 Source
    #[inline(always)]
    #[must_use]
    pub fn ee4src(&mut self) -> EE4SRC_W<EECR1rs> {
        EE4SRC_W::new(self, 18)
    }
    ///Bit 20 - External Event 4 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee4pol(&mut self) -> EE4POL_W<EECR1rs> {
        EE4POL_W::new(self, 20)
    }
    ///Bits 21:22 - External Event 4 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee4sns(&mut self) -> EE4SNS_W<EECR1rs> {
        EE4SNS_W::new(self, 21)
    }
    ///Bit 23 - External Event 4 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee4fast(&mut self) -> EE4FAST_W<EECR1rs> {
        EE4FAST_W::new(self, 23)
    }
    ///Bits 24:25 - External Event 5 Source
    #[inline(always)]
    #[must_use]
    pub fn ee5src(&mut self) -> EE5SRC_W<EECR1rs> {
        EE5SRC_W::new(self, 24)
    }
    ///Bit 26 - External Event 5 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee5pol(&mut self) -> EE5POL_W<EECR1rs> {
        EE5POL_W::new(self, 26)
    }
    ///Bits 27:28 - External Event 5 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee5sns(&mut self) -> EE5SNS_W<EECR1rs> {
        EE5SNS_W::new(self, 27)
    }
    ///Bit 29 - External Event 5 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee5fast(&mut self) -> EE5FAST_W<EECR1rs> {
        EE5FAST_W::new(self, 29)
    }
}
/**Timer External Event Control Register 1

You can [`read`](crate::Reg::read) this register and get [`eecr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Common:EECR1)*/
pub struct EECR1rs;
impl crate::RegisterSpec for EECR1rs {
    type Ux = u32;
}
///`read()` method returns [`eecr1::R`](R) reader structure
impl crate::Readable for EECR1rs {}
///`write(|w| ..)` method takes [`eecr1::W`](W) writer structure
impl crate::Writable for EECR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EECR1 to value 0
impl crate::Resettable for EECR1rs {
    const RESET_VALUE: u32 = 0;
}
