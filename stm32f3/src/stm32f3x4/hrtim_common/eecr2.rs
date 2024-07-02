///Register `EECR2` reader
pub type R = crate::R<EECR2rs>;
///Register `EECR2` writer
pub type W = crate::W<EECR2rs>;
/**External Event 6 Source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6SRC {
    ///0: Source 1
    Src1 = 0,
    ///1: Source 2
    Src2 = 1,
    ///2: Source 3
    Src3 = 2,
    ///3: Source 4
    Src4 = 3,
}
impl From<EE6SRC> for u8 {
    #[inline(always)]
    fn from(variant: EE6SRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE6SRC {
    type Ux = u8;
}
impl crate::IsEnum for EE6SRC {}
///Field `EE6SRC` reader - External Event 6 Source
pub type EE6SRC_R = crate::FieldReader<EE6SRC>;
impl EE6SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE6SRC {
        match self.bits {
            0 => EE6SRC::Src1,
            1 => EE6SRC::Src2,
            2 => EE6SRC::Src3,
            3 => EE6SRC::Src4,
            _ => unreachable!(),
        }
    }
    ///Source 1
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        *self == EE6SRC::Src1
    }
    ///Source 2
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        *self == EE6SRC::Src2
    }
    ///Source 3
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        *self == EE6SRC::Src3
    }
    ///Source 4
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        *self == EE6SRC::Src4
    }
}
///Field `EE6SRC` writer - External Event 6 Source
pub type EE6SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE6SRC, crate::Safe>;
impl<'a, REG> EE6SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Source 1
    #[inline(always)]
    pub fn src1(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SRC::Src1)
    }
    ///Source 2
    #[inline(always)]
    pub fn src2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SRC::Src2)
    }
    ///Source 3
    #[inline(always)]
    pub fn src3(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SRC::Src3)
    }
    ///Source 4
    #[inline(always)]
    pub fn src4(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SRC::Src4)
    }
}
/**External Event 6 Polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE6POL {
    ///0: External event is active high
    ActiveHigh = 0,
    ///1: External event is active low
    ActiveLow = 1,
}
impl From<EE6POL> for bool {
    #[inline(always)]
    fn from(variant: EE6POL) -> Self {
        variant as u8 != 0
    }
}
///Field `EE6POL` reader - External Event 6 Polarity
pub type EE6POL_R = crate::BitReader<EE6POL>;
impl EE6POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE6POL {
        match self.bits {
            false => EE6POL::ActiveHigh,
            true => EE6POL::ActiveLow,
        }
    }
    ///External event is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == EE6POL::ActiveHigh
    }
    ///External event is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == EE6POL::ActiveLow
    }
}
///Field `EE6POL` writer - External Event 6 Polarity
pub type EE6POL_W<'a, REG> = crate::BitWriter<'a, REG, EE6POL>;
impl<'a, REG> EE6POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External event is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(EE6POL::ActiveHigh)
    }
    ///External event is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(EE6POL::ActiveLow)
    }
}
/**External Event 6 Sensitivity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6SNS {
    ///0: On active level defined by EExPOL bit
    Active = 0,
    ///1: Rising edge
    Rising = 1,
    ///2: Falling edge
    Falling = 2,
    ///3: Both edges
    Both = 3,
}
impl From<EE6SNS> for u8 {
    #[inline(always)]
    fn from(variant: EE6SNS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE6SNS {
    type Ux = u8;
}
impl crate::IsEnum for EE6SNS {}
///Field `EE6SNS` reader - External Event 6 Sensitivity
pub type EE6SNS_R = crate::FieldReader<EE6SNS>;
impl EE6SNS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE6SNS {
        match self.bits {
            0 => EE6SNS::Active,
            1 => EE6SNS::Rising,
            2 => EE6SNS::Falling,
            3 => EE6SNS::Both,
            _ => unreachable!(),
        }
    }
    ///On active level defined by EExPOL bit
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == EE6SNS::Active
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EE6SNS::Rising
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EE6SNS::Falling
    }
    ///Both edges
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EE6SNS::Both
    }
}
///Field `EE6SNS` writer - External Event 6 Sensitivity
pub type EE6SNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE6SNS, crate::Safe>;
impl<'a, REG> EE6SNS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///On active level defined by EExPOL bit
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SNS::Active)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SNS::Rising)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SNS::Falling)
    }
    ///Both edges
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(EE6SNS::Both)
    }
}
///Field `EE7POL` reader - External Event 7 Polarity
pub use EE6POL_R as EE7POL_R;
///Field `EE8POL` reader - External Event 8 Polarity
pub use EE6POL_R as EE8POL_R;
///Field `EE9POL` reader - External Event 9 Polarity
pub use EE6POL_R as EE9POL_R;
///Field `EE10POL` reader - External Event 10 Polarity
pub use EE6POL_R as EE10POL_R;
///Field `EE7POL` writer - External Event 7 Polarity
pub use EE6POL_W as EE7POL_W;
///Field `EE8POL` writer - External Event 8 Polarity
pub use EE6POL_W as EE8POL_W;
///Field `EE9POL` writer - External Event 9 Polarity
pub use EE6POL_W as EE9POL_W;
///Field `EE10POL` writer - External Event 10 Polarity
pub use EE6POL_W as EE10POL_W;
///Field `EE7SNS` reader - External Event 7 Sensitivity
pub use EE6SNS_R as EE7SNS_R;
///Field `EE8SNS` reader - External Event 8 Sensitivity
pub use EE6SNS_R as EE8SNS_R;
///Field `EE9SNS` reader - External Event 9 Sensitivity
pub use EE6SNS_R as EE9SNS_R;
///Field `EE10SNS` reader - External Event 10 Sensitivity
pub use EE6SNS_R as EE10SNS_R;
///Field `EE7SNS` writer - External Event 7 Sensitivity
pub use EE6SNS_W as EE7SNS_W;
///Field `EE8SNS` writer - External Event 8 Sensitivity
pub use EE6SNS_W as EE8SNS_W;
///Field `EE9SNS` writer - External Event 9 Sensitivity
pub use EE6SNS_W as EE9SNS_W;
///Field `EE10SNS` writer - External Event 10 Sensitivity
pub use EE6SNS_W as EE10SNS_W;
///Field `EE7SRC` reader - External Event 7 Source
pub use EE6SRC_R as EE7SRC_R;
///Field `EE8SRC` reader - External Event 8 Source
pub use EE6SRC_R as EE8SRC_R;
///Field `EE9SRC` reader - External Event 9 Source
pub use EE6SRC_R as EE9SRC_R;
///Field `EE10SRC` reader - External Event 10 Source
pub use EE6SRC_R as EE10SRC_R;
///Field `EE7SRC` writer - External Event 7 Source
pub use EE6SRC_W as EE7SRC_W;
///Field `EE8SRC` writer - External Event 8 Source
pub use EE6SRC_W as EE8SRC_W;
///Field `EE9SRC` writer - External Event 9 Source
pub use EE6SRC_W as EE9SRC_W;
///Field `EE10SRC` writer - External Event 10 Source
pub use EE6SRC_W as EE10SRC_W;
impl R {
    ///Bits 0:1 - External Event 6 Source
    #[inline(always)]
    pub fn ee6src(&self) -> EE6SRC_R {
        EE6SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - External Event 6 Polarity
    #[inline(always)]
    pub fn ee6pol(&self) -> EE6POL_R {
        EE6POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - External Event 6 Sensitivity
    #[inline(always)]
    pub fn ee6sns(&self) -> EE6SNS_R {
        EE6SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - External Event 7 Source
    #[inline(always)]
    pub fn ee7src(&self) -> EE7SRC_R {
        EE7SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - External Event 7 Polarity
    #[inline(always)]
    pub fn ee7pol(&self) -> EE7POL_R {
        EE7POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - External Event 7 Sensitivity
    #[inline(always)]
    pub fn ee7sns(&self) -> EE7SNS_R {
        EE7SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 12:13 - External Event 8 Source
    #[inline(always)]
    pub fn ee8src(&self) -> EE8SRC_R {
        EE8SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External Event 8 Polarity
    #[inline(always)]
    pub fn ee8pol(&self) -> EE8POL_R {
        EE8POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:16 - External Event 8 Sensitivity
    #[inline(always)]
    pub fn ee8sns(&self) -> EE8SNS_R {
        EE8SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 18:19 - External Event 9 Source
    #[inline(always)]
    pub fn ee9src(&self) -> EE9SRC_R {
        EE9SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - External Event 9 Polarity
    #[inline(always)]
    pub fn ee9pol(&self) -> EE9POL_R {
        EE9POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - External Event 9 Sensitivity
    #[inline(always)]
    pub fn ee9sns(&self) -> EE9SNS_R {
        EE9SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 24:25 - External Event 10 Source
    #[inline(always)]
    pub fn ee10src(&self) -> EE10SRC_R {
        EE10SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - External Event 10 Polarity
    #[inline(always)]
    pub fn ee10pol(&self) -> EE10POL_R {
        EE10POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - External Event 10 Sensitivity
    #[inline(always)]
    pub fn ee10sns(&self) -> EE10SNS_R {
        EE10SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR2")
            .field("ee6sns", &self.ee6sns())
            .field("ee10sns", &self.ee10sns())
            .field("ee6pol", &self.ee6pol())
            .field("ee10pol", &self.ee10pol())
            .field("ee6src", &self.ee6src())
            .field("ee10src", &self.ee10src())
            .field("ee9sns", &self.ee9sns())
            .field("ee9pol", &self.ee9pol())
            .field("ee9src", &self.ee9src())
            .field("ee8sns", &self.ee8sns())
            .field("ee8pol", &self.ee8pol())
            .field("ee8src", &self.ee8src())
            .field("ee7sns", &self.ee7sns())
            .field("ee7pol", &self.ee7pol())
            .field("ee7src", &self.ee7src())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - External Event 6 Source
    #[inline(always)]
    #[must_use]
    pub fn ee6src(&mut self) -> EE6SRC_W<EECR2rs> {
        EE6SRC_W::new(self, 0)
    }
    ///Bit 2 - External Event 6 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee6pol(&mut self) -> EE6POL_W<EECR2rs> {
        EE6POL_W::new(self, 2)
    }
    ///Bits 3:4 - External Event 6 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee6sns(&mut self) -> EE6SNS_W<EECR2rs> {
        EE6SNS_W::new(self, 3)
    }
    ///Bits 6:7 - External Event 7 Source
    #[inline(always)]
    #[must_use]
    pub fn ee7src(&mut self) -> EE7SRC_W<EECR2rs> {
        EE7SRC_W::new(self, 6)
    }
    ///Bit 8 - External Event 7 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee7pol(&mut self) -> EE7POL_W<EECR2rs> {
        EE7POL_W::new(self, 8)
    }
    ///Bits 9:10 - External Event 7 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee7sns(&mut self) -> EE7SNS_W<EECR2rs> {
        EE7SNS_W::new(self, 9)
    }
    ///Bits 12:13 - External Event 8 Source
    #[inline(always)]
    #[must_use]
    pub fn ee8src(&mut self) -> EE8SRC_W<EECR2rs> {
        EE8SRC_W::new(self, 12)
    }
    ///Bit 14 - External Event 8 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee8pol(&mut self) -> EE8POL_W<EECR2rs> {
        EE8POL_W::new(self, 14)
    }
    ///Bits 15:16 - External Event 8 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee8sns(&mut self) -> EE8SNS_W<EECR2rs> {
        EE8SNS_W::new(self, 15)
    }
    ///Bits 18:19 - External Event 9 Source
    #[inline(always)]
    #[must_use]
    pub fn ee9src(&mut self) -> EE9SRC_W<EECR2rs> {
        EE9SRC_W::new(self, 18)
    }
    ///Bit 20 - External Event 9 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee9pol(&mut self) -> EE9POL_W<EECR2rs> {
        EE9POL_W::new(self, 20)
    }
    ///Bits 21:22 - External Event 9 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee9sns(&mut self) -> EE9SNS_W<EECR2rs> {
        EE9SNS_W::new(self, 21)
    }
    ///Bits 24:25 - External Event 10 Source
    #[inline(always)]
    #[must_use]
    pub fn ee10src(&mut self) -> EE10SRC_W<EECR2rs> {
        EE10SRC_W::new(self, 24)
    }
    ///Bit 26 - External Event 10 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee10pol(&mut self) -> EE10POL_W<EECR2rs> {
        EE10POL_W::new(self, 26)
    }
    ///Bits 27:28 - External Event 10 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee10sns(&mut self) -> EE10SNS_W<EECR2rs> {
        EE10SNS_W::new(self, 27)
    }
}
/**Timer External Event Control Register 2

You can [`read`](crate::Reg::read) this register and get [`eecr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Common:EECR2)*/
pub struct EECR2rs;
impl crate::RegisterSpec for EECR2rs {
    type Ux = u32;
}
///`read()` method returns [`eecr2::R`](R) reader structure
impl crate::Readable for EECR2rs {}
///`write(|w| ..)` method takes [`eecr2::W`](W) writer structure
impl crate::Writable for EECR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EECR2 to value 0
impl crate::Resettable for EECR2rs {
    const RESET_VALUE: u32 = 0;
}
