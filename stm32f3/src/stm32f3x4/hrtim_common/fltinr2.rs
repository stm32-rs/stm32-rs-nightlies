///Register `FLTINR2` reader
pub type R = crate::R<FLTINR2rs>;
///Register `FLTINR2` writer
pub type W = crate::W<FLTINR2rs>;
/**FLT%sE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5E {
    ///0: Fault input disabled
    Disabled = 0,
    ///1: Fault input enabled
    Enabled = 1,
}
impl From<FLT5E> for bool {
    #[inline(always)]
    fn from(variant: FLT5E) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTE(5-5)` reader - FLT%sE
pub type FLTE_R = crate::BitReader<FLT5E>;
impl FLTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT5E {
        match self.bits {
            false => FLT5E::Disabled,
            true => FLT5E::Enabled,
        }
    }
    ///Fault input disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5E::Disabled
    }
    ///Fault input enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT5E::Enabled
    }
}
///Field `FLTE(5-5)` writer - FLT%sE
pub type FLTE_W<'a, REG> = crate::BitWriter<'a, REG, FLT5E>;
impl<'a, REG> FLTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5E::Disabled)
    }
    ///Fault input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5E::Enabled)
    }
}
/**FLT%sP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5P {
    ///0: Fault input is active low
    ActiveLow = 0,
    ///1: Fault input is active high
    ActiveHigh = 1,
}
impl From<FLT5P> for bool {
    #[inline(always)]
    fn from(variant: FLT5P) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTP(5-5)` reader - FLT%sP
pub type FLTP_R = crate::BitReader<FLT5P>;
impl FLTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT5P {
        match self.bits {
            false => FLT5P::ActiveLow,
            true => FLT5P::ActiveHigh,
        }
    }
    ///Fault input is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FLT5P::ActiveLow
    }
    ///Fault input is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FLT5P::ActiveHigh
    }
}
///Field `FLTP(5-5)` writer - FLT%sP
pub type FLTP_W<'a, REG> = crate::BitWriter<'a, REG, FLT5P>;
impl<'a, REG> FLTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault input is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5P::ActiveLow)
    }
    ///Fault input is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5P::ActiveHigh)
    }
}
/**Fault %s source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5SRC {
    ///0: Fault input is FLTx input pin
    Input = 0,
    ///1: Fault input is FLTn_Int signal
    Internal = 1,
}
impl From<FLT5SRC> for bool {
    #[inline(always)]
    fn from(variant: FLT5SRC) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTSRC(5-5)` reader - Fault %s source
pub type FLTSRC_R = crate::BitReader<FLT5SRC>;
impl FLTSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT5SRC {
        match self.bits {
            false => FLT5SRC::Input,
            true => FLT5SRC::Internal,
        }
    }
    ///Fault input is FLTx input pin
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FLT5SRC::Input
    }
    ///Fault input is FLTn_Int signal
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == FLT5SRC::Internal
    }
}
///Field `FLTSRC(5-5)` writer - Fault %s source
pub type FLTSRC_W<'a, REG> = crate::BitWriter<'a, REG, FLT5SRC>;
impl<'a, REG> FLTSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault input is FLTx input pin
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5SRC::Input)
    }
    ///Fault input is FLTn_Int signal
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5SRC::Internal)
    }
}
/**FLT%sF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT5F {
    ///0: No filter, FLTx acts asynchronously
    Disabled = 0,
    ///1: f_SAMPLING=f_HRTIM, N=2
    Div1N2 = 1,
    ///2: f_SAMPLING=f_HRTIM, N=4
    Div1N4 = 2,
    ///3: f_SAMPLING=f_HRTIM, N=8
    Div1N8 = 3,
    ///4: f_SAMPLING=f_HRTIM/2, N=6
    Div2N6 = 4,
    ///5: f_SAMPLING=f_HRTIM/2, N=8
    Div2N8 = 5,
    ///6: f_SAMPLING=f_HRTIM/4, N=6
    Div4N6 = 6,
    ///7: f_SAMPLING=f_HRTIM/4, N=8
    Div4N8 = 7,
    ///8: f_SAMPLING=f_HRTIM/8, N=6
    Div8N6 = 8,
    ///9: f_SAMPLING=f_HRTIM/8, N=8
    Div8N8 = 9,
    ///10: f_SAMPLING=f_HRTIM/16, N=5
    Div16N5 = 10,
    ///11: f_SAMPLING=f_HRTIM/16, N=6
    Div16N6 = 11,
    ///12: f_SAMPLING=f_HRTIM/16, N=8
    Div16N8 = 12,
    ///13: f_SAMPLING=f_HRTIM/32, N=5
    Div32N5 = 13,
    ///14: f_SAMPLING=f_HRTIM/32, N=6
    Div32N6 = 14,
    ///15: f_SAMPLING=f_HRTIM/32, N=8
    Div32N8 = 15,
}
impl From<FLT5F> for u8 {
    #[inline(always)]
    fn from(variant: FLT5F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLT5F {
    type Ux = u8;
}
impl crate::IsEnum for FLT5F {}
///Field `FLTF(5-5)` reader - FLT%sF
pub type FLTF_R = crate::FieldReader<FLT5F>;
impl FLTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT5F {
        match self.bits {
            0 => FLT5F::Disabled,
            1 => FLT5F::Div1N2,
            2 => FLT5F::Div1N4,
            3 => FLT5F::Div1N8,
            4 => FLT5F::Div2N6,
            5 => FLT5F::Div2N8,
            6 => FLT5F::Div4N6,
            7 => FLT5F::Div4N8,
            8 => FLT5F::Div8N6,
            9 => FLT5F::Div8N8,
            10 => FLT5F::Div16N5,
            11 => FLT5F::Div16N6,
            12 => FLT5F::Div16N8,
            13 => FLT5F::Div32N5,
            14 => FLT5F::Div32N6,
            15 => FLT5F::Div32N8,
            _ => unreachable!(),
        }
    }
    ///No filter, FLTx acts asynchronously
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5F::Disabled
    }
    ///f_SAMPLING=f_HRTIM, N=2
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == FLT5F::Div1N2
    }
    ///f_SAMPLING=f_HRTIM, N=4
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == FLT5F::Div1N4
    }
    ///f_SAMPLING=f_HRTIM, N=8
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == FLT5F::Div1N8
    }
    ///f_SAMPLING=f_HRTIM/2, N=6
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == FLT5F::Div2N6
    }
    ///f_SAMPLING=f_HRTIM/2, N=8
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == FLT5F::Div2N8
    }
    ///f_SAMPLING=f_HRTIM/4, N=6
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == FLT5F::Div4N6
    }
    ///f_SAMPLING=f_HRTIM/4, N=8
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == FLT5F::Div4N8
    }
    ///f_SAMPLING=f_HRTIM/8, N=6
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == FLT5F::Div8N6
    }
    ///f_SAMPLING=f_HRTIM/8, N=8
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == FLT5F::Div8N8
    }
    ///f_SAMPLING=f_HRTIM/16, N=5
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == FLT5F::Div16N5
    }
    ///f_SAMPLING=f_HRTIM/16, N=6
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == FLT5F::Div16N6
    }
    ///f_SAMPLING=f_HRTIM/16, N=8
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == FLT5F::Div16N8
    }
    ///f_SAMPLING=f_HRTIM/32, N=5
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == FLT5F::Div32N5
    }
    ///f_SAMPLING=f_HRTIM/32, N=6
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == FLT5F::Div32N6
    }
    ///f_SAMPLING=f_HRTIM/32, N=8
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == FLT5F::Div32N8
    }
}
///Field `FLTF(5-5)` writer - FLT%sF
pub type FLTF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, FLT5F, crate::Safe>;
impl<'a, REG> FLTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, FLTx acts asynchronously
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Disabled)
    }
    ///f_SAMPLING=f_HRTIM, N=2
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div1N2)
    }
    ///f_SAMPLING=f_HRTIM, N=4
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div1N4)
    }
    ///f_SAMPLING=f_HRTIM, N=8
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div1N8)
    }
    ///f_SAMPLING=f_HRTIM/2, N=6
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div2N6)
    }
    ///f_SAMPLING=f_HRTIM/2, N=8
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div2N8)
    }
    ///f_SAMPLING=f_HRTIM/4, N=6
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div4N6)
    }
    ///f_SAMPLING=f_HRTIM/4, N=8
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div4N8)
    }
    ///f_SAMPLING=f_HRTIM/8, N=6
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div8N6)
    }
    ///f_SAMPLING=f_HRTIM/8, N=8
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div8N8)
    }
    ///f_SAMPLING=f_HRTIM/16, N=5
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div16N5)
    }
    ///f_SAMPLING=f_HRTIM/16, N=6
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div16N6)
    }
    ///f_SAMPLING=f_HRTIM/16, N=8
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div16N8)
    }
    ///f_SAMPLING=f_HRTIM/32, N=5
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div32N5)
    }
    ///f_SAMPLING=f_HRTIM/32, N=6
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div32N6)
    }
    ///f_SAMPLING=f_HRTIM/32, N=8
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div32N8)
    }
}
/**FLT5LCK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5LCKR {
    ///0: Fault bits are read/write
    Unlocked = 0,
    ///1: Fault bits are read-only
    Locked = 1,
}
impl From<FLT5LCKR> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCKR) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT5LCK` reader - FLT5LCK
pub type FLT5LCK_R = crate::BitReader<FLT5LCKR>;
impl FLT5LCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT5LCKR {
        match self.bits {
            false => FLT5LCKR::Unlocked,
            true => FLT5LCKR::Locked,
        }
    }
    ///Fault bits are read/write
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLT5LCKR::Unlocked
    }
    ///Fault bits are read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLT5LCKR::Locked
    }
}
/**FLT5LCK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5LCKW {
    ///1: Lock corresponding fault bits
    Lock = 1,
}
impl From<FLT5LCKW> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT5LCK` writer - FLT5LCK
pub type FLT5LCK_W<'a, REG> = crate::BitWriter<'a, REG, FLT5LCKW>;
impl<'a, REG> FLT5LCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Lock corresponding fault bits
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5LCKW::Lock)
    }
}
/**FLTSD

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLTSD {
    ///0: f_FLTS=f_HRTIM
    Div1 = 0,
    ///1: f_FLTS=f_HRTIM/2
    Div2 = 1,
    ///2: f_FLTS=f_HRTIM/4
    Div4 = 2,
    ///3: f_FLTS=f_HRTIM/8
    Div8 = 3,
}
impl From<FLTSD> for u8 {
    #[inline(always)]
    fn from(variant: FLTSD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLTSD {
    type Ux = u8;
}
impl crate::IsEnum for FLTSD {}
///Field `FLTSD` reader - FLTSD
pub type FLTSD_R = crate::FieldReader<FLTSD>;
impl FLTSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLTSD {
        match self.bits {
            0 => FLTSD::Div1,
            1 => FLTSD::Div2,
            2 => FLTSD::Div4,
            3 => FLTSD::Div8,
            _ => unreachable!(),
        }
    }
    ///f_FLTS=f_HRTIM
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FLTSD::Div1
    }
    ///f_FLTS=f_HRTIM/2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FLTSD::Div2
    }
    ///f_FLTS=f_HRTIM/4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FLTSD::Div4
    }
    ///f_FLTS=f_HRTIM/8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FLTSD::Div8
    }
}
///Field `FLTSD` writer - FLTSD
pub type FLTSD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FLTSD, crate::Safe>;
impl<'a, REG> FLTSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///f_FLTS=f_HRTIM
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div1)
    }
    ///f_FLTS=f_HRTIM/2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div2)
    }
    ///f_FLTS=f_HRTIM/4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div4)
    }
    ///f_FLTS=f_HRTIM/8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div8)
    }
}
impl R {
    ///FLT(5-5)E
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5E` field.</div>
    #[inline(always)]
    pub fn flte(&self, n: u8) -> FLTE_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTE_R::new(((self.bits >> (n * 0)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(5-5)E
    #[inline(always)]
    pub fn flte_iter(&self) -> impl Iterator<Item = FLTE_R> + '_ {
        (0..1).map(move |n| FLTE_R::new(((self.bits >> (n * 0)) & 1) != 0))
    }
    ///Bit 0 - FLT5E
    #[inline(always)]
    pub fn flt5e(&self) -> FLTE_R {
        FLTE_R::new((self.bits & 1) != 0)
    }
    ///FLT(5-5)P
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5P` field.</div>
    #[inline(always)]
    pub fn fltp(&self, n: u8) -> FLTP_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(5-5)P
    #[inline(always)]
    pub fn fltp_iter(&self) -> impl Iterator<Item = FLTP_R> + '_ {
        (0..1).map(move |n| FLTP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    ///Bit 1 - FLT5P
    #[inline(always)]
    pub fn flt5p(&self) -> FLTP_R {
        FLTP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Fault (5-5) source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5SRC` field.</div>
    #[inline(always)]
    pub fn fltsrc(&self, n: u8) -> FLTSRC_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTSRC_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Fault (5-5) source
    #[inline(always)]
    pub fn fltsrc_iter(&self) -> impl Iterator<Item = FLTSRC_R> + '_ {
        (0..1).map(move |n| FLTSRC_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0))
    }
    ///Bit 2 - Fault 5 source
    #[inline(always)]
    pub fn flt5src(&self) -> FLTSRC_R {
        FLTSRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///FLT(5-5)F
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5F` field.</div>
    #[inline(always)]
    pub fn fltf(&self, n: u8) -> FLTF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTF_R::new(((self.bits >> (n * 0 + 3)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///FLT(5-5)F
    #[inline(always)]
    pub fn fltf_iter(&self) -> impl Iterator<Item = FLTF_R> + '_ {
        (0..1).map(move |n| FLTF_R::new(((self.bits >> (n * 0 + 3)) & 0x0f) as u8))
    }
    ///Bits 3:6 - FLT5F
    #[inline(always)]
    pub fn flt5f(&self) -> FLTF_R {
        FLTF_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bit 7 - FLT5LCK
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 24:25 - FLTSD
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR2")
            .field("fltsd", &self.fltsd())
            .field("flt5lck", &self.flt5lck())
            .field("flt5f", &self.flt5f())
            .field("flt5src", &self.flt5src())
            .field("flt5p", &self.flt5p())
            .field("flt5e", &self.flt5e())
            .finish()
    }
}
impl W {
    ///FLT(5-5)E
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5E` field.</div>
    #[inline(always)]
    pub fn flte(&mut self, n: u8) -> FLTE_W<'_, FLTINR2rs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTE_W::new(self, n * 0)
    }
    ///Bit 0 - FLT5E
    #[inline(always)]
    pub fn flt5e(&mut self) -> FLTE_W<'_, FLTINR2rs> {
        FLTE_W::new(self, 0)
    }
    ///FLT(5-5)P
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5P` field.</div>
    #[inline(always)]
    pub fn fltp(&mut self, n: u8) -> FLTP_W<'_, FLTINR2rs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTP_W::new(self, n * 0 + 1)
    }
    ///Bit 1 - FLT5P
    #[inline(always)]
    pub fn flt5p(&mut self) -> FLTP_W<'_, FLTINR2rs> {
        FLTP_W::new(self, 1)
    }
    ///Fault (5-5) source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5SRC` field.</div>
    #[inline(always)]
    pub fn fltsrc(&mut self, n: u8) -> FLTSRC_W<'_, FLTINR2rs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTSRC_W::new(self, n * 0 + 2)
    }
    ///Bit 2 - Fault 5 source
    #[inline(always)]
    pub fn flt5src(&mut self) -> FLTSRC_W<'_, FLTINR2rs> {
        FLTSRC_W::new(self, 2)
    }
    ///FLT(5-5)F
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT5F` field.</div>
    #[inline(always)]
    pub fn fltf(&mut self, n: u8) -> FLTF_W<'_, FLTINR2rs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        FLTF_W::new(self, n * 0 + 3)
    }
    ///Bits 3:6 - FLT5F
    #[inline(always)]
    pub fn flt5f(&mut self) -> FLTF_W<'_, FLTINR2rs> {
        FLTF_W::new(self, 3)
    }
    ///Bit 7 - FLT5LCK
    #[inline(always)]
    pub fn flt5lck(&mut self) -> FLT5LCK_W<'_, FLTINR2rs> {
        FLT5LCK_W::new(self, 7)
    }
    ///Bits 24:25 - FLTSD
    #[inline(always)]
    pub fn fltsd(&mut self) -> FLTSD_W<'_, FLTINR2rs> {
        FLTSD_W::new(self, 24)
    }
}
/**HRTIM Fault Input Register 2

You can [`read`](crate::Reg::read) this register and get [`fltinr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Common:FLTINR2)*/
pub struct FLTINR2rs;
impl crate::RegisterSpec for FLTINR2rs {
    type Ux = u32;
}
///`read()` method returns [`fltinr2::R`](R) reader structure
impl crate::Readable for FLTINR2rs {}
///`write(|w| ..)` method takes [`fltinr2::W`](W) writer structure
impl crate::Writable for FLTINR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTINR2 to value 0
impl crate::Resettable for FLTINR2rs {}
