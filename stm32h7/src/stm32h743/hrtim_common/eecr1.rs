///Register `EECR1` reader
pub type R = crate::R<EECR1rs>;
///Register `EECR1` writer
pub type W = crate::W<EECR1rs>;
/**External Event %s Source

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
///Field `EESRC(1-5)` reader - External Event %s Source
pub type EESRC_R = crate::FieldReader<EE1SRC>;
impl EESRC_R {
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
///Field `EESRC(1-5)` writer - External Event %s Source
pub type EESRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE1SRC, crate::Safe>;
impl<'a, REG> EESRC_W<'a, REG>
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
/**External Event %s Polarity

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
///Field `EEPOL(1-5)` reader - External Event %s Polarity
pub type EEPOL_R = crate::BitReader<EE1POL>;
impl EEPOL_R {
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
///Field `EEPOL(1-5)` writer - External Event %s Polarity
pub type EEPOL_W<'a, REG> = crate::BitWriter<'a, REG, EE1POL>;
impl<'a, REG> EEPOL_W<'a, REG>
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
/**External Event %s Sensitivity

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
///Field `EESNS(1-5)` reader - External Event %s Sensitivity
pub type EESNS_R = crate::FieldReader<EE1SNS>;
impl EESNS_R {
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
///Field `EESNS(1-5)` writer - External Event %s Sensitivity
pub type EESNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE1SNS, crate::Safe>;
impl<'a, REG> EESNS_W<'a, REG>
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
/**External Event %s Fast mode

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
///Field `EEFAST(1-5)` reader - External Event %s Fast mode
pub type EEFAST_R = crate::BitReader<EE1FAST>;
impl EEFAST_R {
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
///Field `EEFAST(1-5)` writer - External Event %s Fast mode
pub type EEFAST_W<'a, REG> = crate::BitWriter<'a, REG, EE1FAST>;
impl<'a, REG> EEFAST_W<'a, REG>
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
impl R {
    ///External Event (1-5) Source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1SRC` field.</div>
    #[inline(always)]
    pub fn eesrc(&self, n: u8) -> EESRC_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESRC_R::new(((self.bits >> (n * 6)) & 3) as u8)
    }
    ///Iterator for array of:
    ///External Event (1-5) Source
    #[inline(always)]
    pub fn eesrc_iter(&self) -> impl Iterator<Item = EESRC_R> + '_ {
        (0..5).map(move |n| EESRC_R::new(((self.bits >> (n * 6)) & 3) as u8))
    }
    ///Bits 0:1 - External Event 1 Source
    #[inline(always)]
    pub fn ee1src(&self) -> EESRC_R {
        EESRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 6:7 - External Event 2 Source
    #[inline(always)]
    pub fn ee2src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 12:13 - External Event 3 Source
    #[inline(always)]
    pub fn ee3src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 18:19 - External Event 4 Source
    #[inline(always)]
    pub fn ee4src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 24:25 - External Event 5 Source
    #[inline(always)]
    pub fn ee5src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///External Event (1-5) Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1POL` field.</div>
    #[inline(always)]
    pub fn eepol(&self, n: u8) -> EEPOL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEPOL_R::new(((self.bits >> (n * 6 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///External Event (1-5) Polarity
    #[inline(always)]
    pub fn eepol_iter(&self) -> impl Iterator<Item = EEPOL_R> + '_ {
        (0..5).map(move |n| EEPOL_R::new(((self.bits >> (n * 6 + 2)) & 1) != 0))
    }
    ///Bit 2 - External Event 1 Polarity
    #[inline(always)]
    pub fn ee1pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - External Event 2 Polarity
    #[inline(always)]
    pub fn ee2pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 14 - External Event 3 Polarity
    #[inline(always)]
    pub fn ee3pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 20 - External Event 4 Polarity
    #[inline(always)]
    pub fn ee4pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 26 - External Event 5 Polarity
    #[inline(always)]
    pub fn ee5pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///External Event (1-5) Sensitivity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1SNS` field.</div>
    #[inline(always)]
    pub fn eesns(&self, n: u8) -> EESNS_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESNS_R::new(((self.bits >> (n * 6 + 3)) & 3) as u8)
    }
    ///Iterator for array of:
    ///External Event (1-5) Sensitivity
    #[inline(always)]
    pub fn eesns_iter(&self) -> impl Iterator<Item = EESNS_R> + '_ {
        (0..5).map(move |n| EESNS_R::new(((self.bits >> (n * 6 + 3)) & 3) as u8))
    }
    ///Bits 3:4 - External Event 1 Sensitivity
    #[inline(always)]
    pub fn ee1sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 9:10 - External Event 2 Sensitivity
    #[inline(always)]
    pub fn ee2sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 15:16 - External Event 3 Sensitivity
    #[inline(always)]
    pub fn ee3sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 21:22 - External Event 4 Sensitivity
    #[inline(always)]
    pub fn ee4sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 27:28 - External Event 5 Sensitivity
    #[inline(always)]
    pub fn ee5sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///External Event (1-5) Fast mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1FAST` field.</div>
    #[inline(always)]
    pub fn eefast(&self, n: u8) -> EEFAST_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEFAST_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0)
    }
    ///Iterator for array of:
    ///External Event (1-5) Fast mode
    #[inline(always)]
    pub fn eefast_iter(&self) -> impl Iterator<Item = EEFAST_R> + '_ {
        (0..5).map(move |n| EEFAST_R::new(((self.bits >> (n * 6 + 5)) & 1) != 0))
    }
    ///Bit 5 - External Event 1 Fast mode
    #[inline(always)]
    pub fn ee1fast(&self) -> EEFAST_R {
        EEFAST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - External Event 2 Fast mode
    #[inline(always)]
    pub fn ee2fast(&self) -> EEFAST_R {
        EEFAST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - External Event 3 Fast mode
    #[inline(always)]
    pub fn ee3fast(&self) -> EEFAST_R {
        EEFAST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 23 - External Event 4 Fast mode
    #[inline(always)]
    pub fn ee4fast(&self) -> EEFAST_R {
        EEFAST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 29 - External Event 5 Fast mode
    #[inline(always)]
    pub fn ee5fast(&self) -> EEFAST_R {
        EEFAST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR1")
            .field("ee1fast", &self.ee1fast())
            .field("ee2fast", &self.ee2fast())
            .field("ee3fast", &self.ee3fast())
            .field("ee4fast", &self.ee4fast())
            .field("ee5fast", &self.ee5fast())
            .field("ee1sns", &self.ee1sns())
            .field("ee2sns", &self.ee2sns())
            .field("ee3sns", &self.ee3sns())
            .field("ee4sns", &self.ee4sns())
            .field("ee5sns", &self.ee5sns())
            .field("ee1pol", &self.ee1pol())
            .field("ee2pol", &self.ee2pol())
            .field("ee3pol", &self.ee3pol())
            .field("ee4pol", &self.ee4pol())
            .field("ee5pol", &self.ee5pol())
            .field("ee1src", &self.ee1src())
            .field("ee2src", &self.ee2src())
            .field("ee3src", &self.ee3src())
            .field("ee4src", &self.ee4src())
            .field("ee5src", &self.ee5src())
            .finish()
    }
}
impl W {
    ///External Event (1-5) Source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1SRC` field.</div>
    #[inline(always)]
    pub fn eesrc(&mut self, n: u8) -> EESRC_W<'_, EECR1rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESRC_W::new(self, n * 6)
    }
    ///Bits 0:1 - External Event 1 Source
    #[inline(always)]
    pub fn ee1src(&mut self) -> EESRC_W<'_, EECR1rs> {
        EESRC_W::new(self, 0)
    }
    ///Bits 6:7 - External Event 2 Source
    #[inline(always)]
    pub fn ee2src(&mut self) -> EESRC_W<'_, EECR1rs> {
        EESRC_W::new(self, 6)
    }
    ///Bits 12:13 - External Event 3 Source
    #[inline(always)]
    pub fn ee3src(&mut self) -> EESRC_W<'_, EECR1rs> {
        EESRC_W::new(self, 12)
    }
    ///Bits 18:19 - External Event 4 Source
    #[inline(always)]
    pub fn ee4src(&mut self) -> EESRC_W<'_, EECR1rs> {
        EESRC_W::new(self, 18)
    }
    ///Bits 24:25 - External Event 5 Source
    #[inline(always)]
    pub fn ee5src(&mut self) -> EESRC_W<'_, EECR1rs> {
        EESRC_W::new(self, 24)
    }
    ///External Event (1-5) Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1POL` field.</div>
    #[inline(always)]
    pub fn eepol(&mut self, n: u8) -> EEPOL_W<'_, EECR1rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEPOL_W::new(self, n * 6 + 2)
    }
    ///Bit 2 - External Event 1 Polarity
    #[inline(always)]
    pub fn ee1pol(&mut self) -> EEPOL_W<'_, EECR1rs> {
        EEPOL_W::new(self, 2)
    }
    ///Bit 8 - External Event 2 Polarity
    #[inline(always)]
    pub fn ee2pol(&mut self) -> EEPOL_W<'_, EECR1rs> {
        EEPOL_W::new(self, 8)
    }
    ///Bit 14 - External Event 3 Polarity
    #[inline(always)]
    pub fn ee3pol(&mut self) -> EEPOL_W<'_, EECR1rs> {
        EEPOL_W::new(self, 14)
    }
    ///Bit 20 - External Event 4 Polarity
    #[inline(always)]
    pub fn ee4pol(&mut self) -> EEPOL_W<'_, EECR1rs> {
        EEPOL_W::new(self, 20)
    }
    ///Bit 26 - External Event 5 Polarity
    #[inline(always)]
    pub fn ee5pol(&mut self) -> EEPOL_W<'_, EECR1rs> {
        EEPOL_W::new(self, 26)
    }
    ///External Event (1-5) Sensitivity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1SNS` field.</div>
    #[inline(always)]
    pub fn eesns(&mut self, n: u8) -> EESNS_W<'_, EECR1rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESNS_W::new(self, n * 6 + 3)
    }
    ///Bits 3:4 - External Event 1 Sensitivity
    #[inline(always)]
    pub fn ee1sns(&mut self) -> EESNS_W<'_, EECR1rs> {
        EESNS_W::new(self, 3)
    }
    ///Bits 9:10 - External Event 2 Sensitivity
    #[inline(always)]
    pub fn ee2sns(&mut self) -> EESNS_W<'_, EECR1rs> {
        EESNS_W::new(self, 9)
    }
    ///Bits 15:16 - External Event 3 Sensitivity
    #[inline(always)]
    pub fn ee3sns(&mut self) -> EESNS_W<'_, EECR1rs> {
        EESNS_W::new(self, 15)
    }
    ///Bits 21:22 - External Event 4 Sensitivity
    #[inline(always)]
    pub fn ee4sns(&mut self) -> EESNS_W<'_, EECR1rs> {
        EESNS_W::new(self, 21)
    }
    ///Bits 27:28 - External Event 5 Sensitivity
    #[inline(always)]
    pub fn ee5sns(&mut self) -> EESNS_W<'_, EECR1rs> {
        EESNS_W::new(self, 27)
    }
    ///External Event (1-5) Fast mode
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1FAST` field.</div>
    #[inline(always)]
    pub fn eefast(&mut self, n: u8) -> EEFAST_W<'_, EECR1rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEFAST_W::new(self, n * 6 + 5)
    }
    ///Bit 5 - External Event 1 Fast mode
    #[inline(always)]
    pub fn ee1fast(&mut self) -> EEFAST_W<'_, EECR1rs> {
        EEFAST_W::new(self, 5)
    }
    ///Bit 11 - External Event 2 Fast mode
    #[inline(always)]
    pub fn ee2fast(&mut self) -> EEFAST_W<'_, EECR1rs> {
        EEFAST_W::new(self, 11)
    }
    ///Bit 17 - External Event 3 Fast mode
    #[inline(always)]
    pub fn ee3fast(&mut self) -> EEFAST_W<'_, EECR1rs> {
        EEFAST_W::new(self, 17)
    }
    ///Bit 23 - External Event 4 Fast mode
    #[inline(always)]
    pub fn ee4fast(&mut self) -> EEFAST_W<'_, EECR1rs> {
        EEFAST_W::new(self, 23)
    }
    ///Bit 29 - External Event 5 Fast mode
    #[inline(always)]
    pub fn ee5fast(&mut self) -> EEFAST_W<'_, EECR1rs> {
        EEFAST_W::new(self, 29)
    }
}
/**Timer External Event Control Register 1

You can [`read`](crate::Reg::read) this register and get [`eecr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_Common:EECR1)*/
pub struct EECR1rs;
impl crate::RegisterSpec for EECR1rs {
    type Ux = u32;
}
///`read()` method returns [`eecr1::R`](R) reader structure
impl crate::Readable for EECR1rs {}
///`write(|w| ..)` method takes [`eecr1::W`](W) writer structure
impl crate::Writable for EECR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EECR1 to value 0
impl crate::Resettable for EECR1rs {}
