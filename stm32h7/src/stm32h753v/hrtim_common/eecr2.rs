///Register `EECR2` reader
pub type R = crate::R<EECR2rs>;
///Register `EECR2` writer
pub type W = crate::W<EECR2rs>;
/**External Event %s Source

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
///Field `EESRC(6-10)` reader - External Event %s Source
pub type EESRC_R = crate::FieldReader<EE6SRC>;
impl EESRC_R {
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
///Field `EESRC(6-10)` writer - External Event %s Source
pub type EESRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE6SRC, crate::Safe>;
impl<'a, REG> EESRC_W<'a, REG>
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
/**External Event %s Polarity

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
///Field `EEPOL(6-10)` reader - External Event %s Polarity
pub type EEPOL_R = crate::BitReader<EE6POL>;
impl EEPOL_R {
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
///Field `EEPOL(6-10)` writer - External Event %s Polarity
pub type EEPOL_W<'a, REG> = crate::BitWriter<'a, REG, EE6POL>;
impl<'a, REG> EEPOL_W<'a, REG>
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
/**External Event %s Sensitivity

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
///Field `EESNS(6-10)` reader - External Event %s Sensitivity
pub type EESNS_R = crate::FieldReader<EE6SNS>;
impl EESNS_R {
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
///Field `EESNS(6-10)` writer - External Event %s Sensitivity
pub type EESNS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EE6SNS, crate::Safe>;
impl<'a, REG> EESNS_W<'a, REG>
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
impl R {
    ///External Event (6-10) Source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE6SRC` field.</div>
    #[inline(always)]
    pub fn eesrc(&self, n: u8) -> EESRC_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESRC_R::new(((self.bits >> (n * 6)) & 3) as u8)
    }
    ///Iterator for array of:
    ///External Event (6-10) Source
    #[inline(always)]
    pub fn eesrc_iter(&self) -> impl Iterator<Item = EESRC_R> + '_ {
        (0..5).map(move |n| EESRC_R::new(((self.bits >> (n * 6)) & 3) as u8))
    }
    ///Bits 0:1 - External Event 6 Source
    #[inline(always)]
    pub fn ee6src(&self) -> EESRC_R {
        EESRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 6:7 - External Event 7 Source
    #[inline(always)]
    pub fn ee7src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 12:13 - External Event 8 Source
    #[inline(always)]
    pub fn ee8src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 18:19 - External Event 9 Source
    #[inline(always)]
    pub fn ee9src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 24:25 - External Event 10 Source
    #[inline(always)]
    pub fn ee10src(&self) -> EESRC_R {
        EESRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///External Event (6-10) Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE6POL` field.</div>
    #[inline(always)]
    pub fn eepol(&self, n: u8) -> EEPOL_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEPOL_R::new(((self.bits >> (n * 6 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///External Event (6-10) Polarity
    #[inline(always)]
    pub fn eepol_iter(&self) -> impl Iterator<Item = EEPOL_R> + '_ {
        (0..5).map(move |n| EEPOL_R::new(((self.bits >> (n * 6 + 2)) & 1) != 0))
    }
    ///Bit 2 - External Event 6 Polarity
    #[inline(always)]
    pub fn ee6pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - External Event 7 Polarity
    #[inline(always)]
    pub fn ee7pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 14 - External Event 8 Polarity
    #[inline(always)]
    pub fn ee8pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 20 - External Event 9 Polarity
    #[inline(always)]
    pub fn ee9pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 26 - External Event 10 Polarity
    #[inline(always)]
    pub fn ee10pol(&self) -> EEPOL_R {
        EEPOL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///External Event (6-10) Sensitivity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE6SNS` field.</div>
    #[inline(always)]
    pub fn eesns(&self, n: u8) -> EESNS_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESNS_R::new(((self.bits >> (n * 6 + 3)) & 3) as u8)
    }
    ///Iterator for array of:
    ///External Event (6-10) Sensitivity
    #[inline(always)]
    pub fn eesns_iter(&self) -> impl Iterator<Item = EESNS_R> + '_ {
        (0..5).map(move |n| EESNS_R::new(((self.bits >> (n * 6 + 3)) & 3) as u8))
    }
    ///Bits 3:4 - External Event 6 Sensitivity
    #[inline(always)]
    pub fn ee6sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 9:10 - External Event 7 Sensitivity
    #[inline(always)]
    pub fn ee7sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 15:16 - External Event 8 Sensitivity
    #[inline(always)]
    pub fn ee8sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 21:22 - External Event 9 Sensitivity
    #[inline(always)]
    pub fn ee9sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 27:28 - External Event 10 Sensitivity
    #[inline(always)]
    pub fn ee10sns(&self) -> EESNS_R {
        EESNS_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EECR2")
            .field("ee6sns", &self.ee6sns())
            .field("ee7sns", &self.ee7sns())
            .field("ee8sns", &self.ee8sns())
            .field("ee9sns", &self.ee9sns())
            .field("ee10sns", &self.ee10sns())
            .field("ee6pol", &self.ee6pol())
            .field("ee7pol", &self.ee7pol())
            .field("ee8pol", &self.ee8pol())
            .field("ee9pol", &self.ee9pol())
            .field("ee10pol", &self.ee10pol())
            .field("ee6src", &self.ee6src())
            .field("ee7src", &self.ee7src())
            .field("ee8src", &self.ee8src())
            .field("ee9src", &self.ee9src())
            .field("ee10src", &self.ee10src())
            .finish()
    }
}
impl W {
    ///External Event (6-10) Source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE6SRC` field.</div>
    #[inline(always)]
    pub fn eesrc(&mut self, n: u8) -> EESRC_W<'_, EECR2rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESRC_W::new(self, n * 6)
    }
    ///Bits 0:1 - External Event 6 Source
    #[inline(always)]
    pub fn ee6src(&mut self) -> EESRC_W<'_, EECR2rs> {
        EESRC_W::new(self, 0)
    }
    ///Bits 6:7 - External Event 7 Source
    #[inline(always)]
    pub fn ee7src(&mut self) -> EESRC_W<'_, EECR2rs> {
        EESRC_W::new(self, 6)
    }
    ///Bits 12:13 - External Event 8 Source
    #[inline(always)]
    pub fn ee8src(&mut self) -> EESRC_W<'_, EECR2rs> {
        EESRC_W::new(self, 12)
    }
    ///Bits 18:19 - External Event 9 Source
    #[inline(always)]
    pub fn ee9src(&mut self) -> EESRC_W<'_, EECR2rs> {
        EESRC_W::new(self, 18)
    }
    ///Bits 24:25 - External Event 10 Source
    #[inline(always)]
    pub fn ee10src(&mut self) -> EESRC_W<'_, EECR2rs> {
        EESRC_W::new(self, 24)
    }
    ///External Event (6-10) Polarity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE6POL` field.</div>
    #[inline(always)]
    pub fn eepol(&mut self, n: u8) -> EEPOL_W<'_, EECR2rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEPOL_W::new(self, n * 6 + 2)
    }
    ///Bit 2 - External Event 6 Polarity
    #[inline(always)]
    pub fn ee6pol(&mut self) -> EEPOL_W<'_, EECR2rs> {
        EEPOL_W::new(self, 2)
    }
    ///Bit 8 - External Event 7 Polarity
    #[inline(always)]
    pub fn ee7pol(&mut self) -> EEPOL_W<'_, EECR2rs> {
        EEPOL_W::new(self, 8)
    }
    ///Bit 14 - External Event 8 Polarity
    #[inline(always)]
    pub fn ee8pol(&mut self) -> EEPOL_W<'_, EECR2rs> {
        EEPOL_W::new(self, 14)
    }
    ///Bit 20 - External Event 9 Polarity
    #[inline(always)]
    pub fn ee9pol(&mut self) -> EEPOL_W<'_, EECR2rs> {
        EEPOL_W::new(self, 20)
    }
    ///Bit 26 - External Event 10 Polarity
    #[inline(always)]
    pub fn ee10pol(&mut self) -> EEPOL_W<'_, EECR2rs> {
        EEPOL_W::new(self, 26)
    }
    ///External Event (6-10) Sensitivity
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE6SNS` field.</div>
    #[inline(always)]
    pub fn eesns(&mut self, n: u8) -> EESNS_W<'_, EECR2rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EESNS_W::new(self, n * 6 + 3)
    }
    ///Bits 3:4 - External Event 6 Sensitivity
    #[inline(always)]
    pub fn ee6sns(&mut self) -> EESNS_W<'_, EECR2rs> {
        EESNS_W::new(self, 3)
    }
    ///Bits 9:10 - External Event 7 Sensitivity
    #[inline(always)]
    pub fn ee7sns(&mut self) -> EESNS_W<'_, EECR2rs> {
        EESNS_W::new(self, 9)
    }
    ///Bits 15:16 - External Event 8 Sensitivity
    #[inline(always)]
    pub fn ee8sns(&mut self) -> EESNS_W<'_, EECR2rs> {
        EESNS_W::new(self, 15)
    }
    ///Bits 21:22 - External Event 9 Sensitivity
    #[inline(always)]
    pub fn ee9sns(&mut self) -> EESNS_W<'_, EECR2rs> {
        EESNS_W::new(self, 21)
    }
    ///Bits 27:28 - External Event 10 Sensitivity
    #[inline(always)]
    pub fn ee10sns(&mut self) -> EESNS_W<'_, EECR2rs> {
        EESNS_W::new(self, 27)
    }
}
/**Timer External Event Control Register 2

You can [`read`](crate::Reg::read) this register and get [`eecr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_Common:EECR2)*/
pub struct EECR2rs;
impl crate::RegisterSpec for EECR2rs {
    type Ux = u32;
}
///`read()` method returns [`eecr2::R`](R) reader structure
impl crate::Readable for EECR2rs {}
///`write(|w| ..)` method takes [`eecr2::W`](W) writer structure
impl crate::Writable for EECR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EECR2 to value 0
impl crate::Resettable for EECR2rs {}
