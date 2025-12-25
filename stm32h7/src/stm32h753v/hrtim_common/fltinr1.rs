///Register `FLTINR1` reader
pub type R = crate::R<FLTINR1rs>;
///Register `FLTINR1` writer
pub type W = crate::W<FLTINR1rs>;
/**FLT%sE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1E {
    ///0: Fault input disabled
    Disabled = 0,
    ///1: Fault input enabled
    Enabled = 1,
}
impl From<FLT1E> for bool {
    #[inline(always)]
    fn from(variant: FLT1E) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTE(1-4)` reader - FLT%sE
pub type FLTE_R = crate::BitReader<FLT1E>;
impl FLTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1E {
        match self.bits {
            false => FLT1E::Disabled,
            true => FLT1E::Enabled,
        }
    }
    ///Fault input disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1E::Disabled
    }
    ///Fault input enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT1E::Enabled
    }
}
///Field `FLTE(1-4)` writer - FLT%sE
pub type FLTE_W<'a, REG> = crate::BitWriter<'a, REG, FLT1E>;
impl<'a, REG> FLTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1E::Disabled)
    }
    ///Fault input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1E::Enabled)
    }
}
/**FLT%sP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1P {
    ///0: Fault input is active low
    ActiveLow = 0,
    ///1: Fault input is active high
    ActiveHigh = 1,
}
impl From<FLT1P> for bool {
    #[inline(always)]
    fn from(variant: FLT1P) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTP(1-4)` reader - FLT%sP
pub type FLTP_R = crate::BitReader<FLT1P>;
impl FLTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1P {
        match self.bits {
            false => FLT1P::ActiveLow,
            true => FLT1P::ActiveHigh,
        }
    }
    ///Fault input is active low
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FLT1P::ActiveLow
    }
    ///Fault input is active high
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FLT1P::ActiveHigh
    }
}
///Field `FLTP(1-4)` writer - FLT%sP
pub type FLTP_W<'a, REG> = crate::BitWriter<'a, REG, FLT1P>;
impl<'a, REG> FLTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault input is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1P::ActiveLow)
    }
    ///Fault input is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1P::ActiveHigh)
    }
}
/**Fault %s source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1SRC {
    ///0: Fault input is FLTx input pin
    Input = 0,
    ///1: Fault input is FLTn_Int signal
    Internal = 1,
}
impl From<FLT1SRC> for bool {
    #[inline(always)]
    fn from(variant: FLT1SRC) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTSRC(1-4)` reader - Fault %s source
pub type FLTSRC_R = crate::BitReader<FLT1SRC>;
impl FLTSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1SRC {
        match self.bits {
            false => FLT1SRC::Input,
            true => FLT1SRC::Internal,
        }
    }
    ///Fault input is FLTx input pin
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FLT1SRC::Input
    }
    ///Fault input is FLTn_Int signal
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == FLT1SRC::Internal
    }
}
///Field `FLTSRC(1-4)` writer - Fault %s source
pub type FLTSRC_W<'a, REG> = crate::BitWriter<'a, REG, FLT1SRC>;
impl<'a, REG> FLTSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault input is FLTx input pin
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1SRC::Input)
    }
    ///Fault input is FLTn_Int signal
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1SRC::Internal)
    }
}
/**FLT%sF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT1F {
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
impl From<FLT1F> for u8 {
    #[inline(always)]
    fn from(variant: FLT1F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLT1F {
    type Ux = u8;
}
impl crate::IsEnum for FLT1F {}
///Field `FLTF(1-4)` reader - FLT%sF
pub type FLTF_R = crate::FieldReader<FLT1F>;
impl FLTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1F {
        match self.bits {
            0 => FLT1F::Disabled,
            1 => FLT1F::Div1N2,
            2 => FLT1F::Div1N4,
            3 => FLT1F::Div1N8,
            4 => FLT1F::Div2N6,
            5 => FLT1F::Div2N8,
            6 => FLT1F::Div4N6,
            7 => FLT1F::Div4N8,
            8 => FLT1F::Div8N6,
            9 => FLT1F::Div8N8,
            10 => FLT1F::Div16N5,
            11 => FLT1F::Div16N6,
            12 => FLT1F::Div16N8,
            13 => FLT1F::Div32N5,
            14 => FLT1F::Div32N6,
            15 => FLT1F::Div32N8,
            _ => unreachable!(),
        }
    }
    ///No filter, FLTx acts asynchronously
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1F::Disabled
    }
    ///f_SAMPLING=f_HRTIM, N=2
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == FLT1F::Div1N2
    }
    ///f_SAMPLING=f_HRTIM, N=4
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == FLT1F::Div1N4
    }
    ///f_SAMPLING=f_HRTIM, N=8
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == FLT1F::Div1N8
    }
    ///f_SAMPLING=f_HRTIM/2, N=6
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == FLT1F::Div2N6
    }
    ///f_SAMPLING=f_HRTIM/2, N=8
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == FLT1F::Div2N8
    }
    ///f_SAMPLING=f_HRTIM/4, N=6
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == FLT1F::Div4N6
    }
    ///f_SAMPLING=f_HRTIM/4, N=8
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == FLT1F::Div4N8
    }
    ///f_SAMPLING=f_HRTIM/8, N=6
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == FLT1F::Div8N6
    }
    ///f_SAMPLING=f_HRTIM/8, N=8
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == FLT1F::Div8N8
    }
    ///f_SAMPLING=f_HRTIM/16, N=5
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == FLT1F::Div16N5
    }
    ///f_SAMPLING=f_HRTIM/16, N=6
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == FLT1F::Div16N6
    }
    ///f_SAMPLING=f_HRTIM/16, N=8
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == FLT1F::Div16N8
    }
    ///f_SAMPLING=f_HRTIM/32, N=5
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == FLT1F::Div32N5
    }
    ///f_SAMPLING=f_HRTIM/32, N=6
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == FLT1F::Div32N6
    }
    ///f_SAMPLING=f_HRTIM/32, N=8
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == FLT1F::Div32N8
    }
}
///Field `FLTF(1-4)` writer - FLT%sF
pub type FLTF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, FLT1F, crate::Safe>;
impl<'a, REG> FLTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, FLTx acts asynchronously
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Disabled)
    }
    ///f_SAMPLING=f_HRTIM, N=2
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div1N2)
    }
    ///f_SAMPLING=f_HRTIM, N=4
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div1N4)
    }
    ///f_SAMPLING=f_HRTIM, N=8
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div1N8)
    }
    ///f_SAMPLING=f_HRTIM/2, N=6
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div2N6)
    }
    ///f_SAMPLING=f_HRTIM/2, N=8
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div2N8)
    }
    ///f_SAMPLING=f_HRTIM/4, N=6
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div4N6)
    }
    ///f_SAMPLING=f_HRTIM/4, N=8
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div4N8)
    }
    ///f_SAMPLING=f_HRTIM/8, N=6
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div8N6)
    }
    ///f_SAMPLING=f_HRTIM/8, N=8
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div8N8)
    }
    ///f_SAMPLING=f_HRTIM/16, N=5
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div16N5)
    }
    ///f_SAMPLING=f_HRTIM/16, N=6
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div16N6)
    }
    ///f_SAMPLING=f_HRTIM/16, N=8
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div16N8)
    }
    ///f_SAMPLING=f_HRTIM/32, N=5
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div32N5)
    }
    ///f_SAMPLING=f_HRTIM/32, N=6
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div32N6)
    }
    ///f_SAMPLING=f_HRTIM/32, N=8
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div32N8)
    }
}
/**FLT1LCK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1LCKR {
    ///0: Fault bits are read/write
    Unlocked = 0,
    ///1: Fault bits are read-only
    Locked = 1,
}
impl From<FLT1LCKR> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCKR) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT1LCK` reader - FLT1LCK
pub type FLT1LCK_R = crate::BitReader<FLT1LCKR>;
impl FLT1LCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1LCKR {
        match self.bits {
            false => FLT1LCKR::Unlocked,
            true => FLT1LCKR::Locked,
        }
    }
    ///Fault bits are read/write
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLT1LCKR::Unlocked
    }
    ///Fault bits are read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLT1LCKR::Locked
    }
}
/**FLT1LCK

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1LCKW {
    ///1: Lock corresponding fault bits
    Lock = 1,
}
impl From<FLT1LCKW> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT1LCK` writer - FLT1LCK
pub type FLT1LCK_W<'a, REG> = crate::BitWriter<'a, REG, FLT1LCKW>;
impl<'a, REG> FLT1LCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Lock corresponding fault bits
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1LCKW::Lock)
    }
}
///Field `FLT2LCK` reader - FLT2LCK
pub use FLT1LCK_R as FLT2LCK_R;
///Field `FLT3LCK` reader - FLT3LCK
pub use FLT1LCK_R as FLT3LCK_R;
///Field `FLT4LCK` reader - FLT4LCK
pub use FLT1LCK_R as FLT4LCK_R;
///Field `FLT2LCK` writer - FLT2LCK
pub use FLT1LCK_W as FLT2LCK_W;
///Field `FLT3LCK` writer - FLT3LCK
pub use FLT1LCK_W as FLT3LCK_W;
///Field `FLT4LCK` writer - FLT4LCK
pub use FLT1LCK_W as FLT4LCK_W;
impl R {
    ///FLT(1-4)E
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1E` field.</div>
    #[inline(always)]
    pub fn flte(&self, n: u8) -> FLTE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTE_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(1-4)E
    #[inline(always)]
    pub fn flte_iter(&self) -> impl Iterator<Item = FLTE_R> + '_ {
        (0..4).map(move |n| FLTE_R::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    ///Bit 0 - FLT1E
    #[inline(always)]
    pub fn flt1e(&self) -> FLTE_R {
        FLTE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FLT2E
    #[inline(always)]
    pub fn flt2e(&self) -> FLTE_R {
        FLTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - FLT3E
    #[inline(always)]
    pub fn flt3e(&self) -> FLTE_R {
        FLTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - FLT4E
    #[inline(always)]
    pub fn flt4e(&self) -> FLTE_R {
        FLTE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///FLT(1-4)P
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1P` field.</div>
    #[inline(always)]
    pub fn fltp(&self, n: u8) -> FLTP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTP_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///FLT(1-4)P
    #[inline(always)]
    pub fn fltp_iter(&self) -> impl Iterator<Item = FLTP_R> + '_ {
        (0..4).map(move |n| FLTP_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0))
    }
    ///Bit 1 - FLT1P
    #[inline(always)]
    pub fn flt1p(&self) -> FLTP_R {
        FLTP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - FLT2P
    #[inline(always)]
    pub fn flt2p(&self) -> FLTP_R {
        FLTP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - FLT3P
    #[inline(always)]
    pub fn flt3p(&self) -> FLTP_R {
        FLTP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 25 - FLT4P
    #[inline(always)]
    pub fn flt4p(&self) -> FLTP_R {
        FLTP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Fault (1-4) source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1SRC` field.</div>
    #[inline(always)]
    pub fn fltsrc(&self, n: u8) -> FLTSRC_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTSRC_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Fault (1-4) source
    #[inline(always)]
    pub fn fltsrc_iter(&self) -> impl Iterator<Item = FLTSRC_R> + '_ {
        (0..4).map(move |n| FLTSRC_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    ///Bit 2 - Fault 1 source
    #[inline(always)]
    pub fn flt1src(&self) -> FLTSRC_R {
        FLTSRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - Fault 2 source
    #[inline(always)]
    pub fn flt2src(&self) -> FLTSRC_R {
        FLTSRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 18 - Fault 3 source
    #[inline(always)]
    pub fn flt3src(&self) -> FLTSRC_R {
        FLTSRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 26 - Fault 4 source
    #[inline(always)]
    pub fn flt4src(&self) -> FLTSRC_R {
        FLTSRC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///FLT(1-4)F
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1F` field.</div>
    #[inline(always)]
    pub fn fltf(&self, n: u8) -> FLTF_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTF_R::new(((self.bits >> (n * 8 + 3)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///FLT(1-4)F
    #[inline(always)]
    pub fn fltf_iter(&self) -> impl Iterator<Item = FLTF_R> + '_ {
        (0..4).map(move |n| FLTF_R::new(((self.bits >> (n * 8 + 3)) & 0x0f) as u8))
    }
    ///Bits 3:6 - FLT1F
    #[inline(always)]
    pub fn flt1f(&self) -> FLTF_R {
        FLTF_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bits 11:14 - FLT2F
    #[inline(always)]
    pub fn flt2f(&self) -> FLTF_R {
        FLTF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bits 19:22 - FLT3F
    #[inline(always)]
    pub fn flt3f(&self) -> FLTF_R {
        FLTF_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bits 27:30 - FLT4F
    #[inline(always)]
    pub fn flt4f(&self) -> FLTF_R {
        FLTF_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    ///Bit 7 - FLT1LCK
    #[inline(always)]
    pub fn flt1lck(&self) -> FLT1LCK_R {
        FLT1LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - FLT2LCK
    #[inline(always)]
    pub fn flt2lck(&self) -> FLT2LCK_R {
        FLT2LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 23 - FLT3LCK
    #[inline(always)]
    pub fn flt3lck(&self) -> FLT3LCK_R {
        FLT3LCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - FLT4LCK
    #[inline(always)]
    pub fn flt4lck(&self) -> FLT4LCK_R {
        FLT4LCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTINR1")
            .field("flt1lck", &self.flt1lck())
            .field("flt4lck", &self.flt4lck())
            .field("flt1f", &self.flt1f())
            .field("flt2f", &self.flt2f())
            .field("flt3f", &self.flt3f())
            .field("flt4f", &self.flt4f())
            .field("flt1src", &self.flt1src())
            .field("flt2src", &self.flt2src())
            .field("flt3src", &self.flt3src())
            .field("flt4src", &self.flt4src())
            .field("flt1p", &self.flt1p())
            .field("flt2p", &self.flt2p())
            .field("flt3p", &self.flt3p())
            .field("flt4p", &self.flt4p())
            .field("flt1e", &self.flt1e())
            .field("flt2e", &self.flt2e())
            .field("flt3e", &self.flt3e())
            .field("flt4e", &self.flt4e())
            .field("flt3lck", &self.flt3lck())
            .field("flt2lck", &self.flt2lck())
            .finish()
    }
}
impl W {
    ///FLT(1-4)E
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1E` field.</div>
    #[inline(always)]
    pub fn flte(&mut self, n: u8) -> FLTE_W<'_, FLTINR1rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTE_W::new(self, n * 8)
    }
    ///Bit 0 - FLT1E
    #[inline(always)]
    pub fn flt1e(&mut self) -> FLTE_W<'_, FLTINR1rs> {
        FLTE_W::new(self, 0)
    }
    ///Bit 8 - FLT2E
    #[inline(always)]
    pub fn flt2e(&mut self) -> FLTE_W<'_, FLTINR1rs> {
        FLTE_W::new(self, 8)
    }
    ///Bit 16 - FLT3E
    #[inline(always)]
    pub fn flt3e(&mut self) -> FLTE_W<'_, FLTINR1rs> {
        FLTE_W::new(self, 16)
    }
    ///Bit 24 - FLT4E
    #[inline(always)]
    pub fn flt4e(&mut self) -> FLTE_W<'_, FLTINR1rs> {
        FLTE_W::new(self, 24)
    }
    ///FLT(1-4)P
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1P` field.</div>
    #[inline(always)]
    pub fn fltp(&mut self, n: u8) -> FLTP_W<'_, FLTINR1rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTP_W::new(self, n * 8 + 1)
    }
    ///Bit 1 - FLT1P
    #[inline(always)]
    pub fn flt1p(&mut self) -> FLTP_W<'_, FLTINR1rs> {
        FLTP_W::new(self, 1)
    }
    ///Bit 9 - FLT2P
    #[inline(always)]
    pub fn flt2p(&mut self) -> FLTP_W<'_, FLTINR1rs> {
        FLTP_W::new(self, 9)
    }
    ///Bit 17 - FLT3P
    #[inline(always)]
    pub fn flt3p(&mut self) -> FLTP_W<'_, FLTINR1rs> {
        FLTP_W::new(self, 17)
    }
    ///Bit 25 - FLT4P
    #[inline(always)]
    pub fn flt4p(&mut self) -> FLTP_W<'_, FLTINR1rs> {
        FLTP_W::new(self, 25)
    }
    ///Fault (1-4) source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1SRC` field.</div>
    #[inline(always)]
    pub fn fltsrc(&mut self, n: u8) -> FLTSRC_W<'_, FLTINR1rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTSRC_W::new(self, n * 8 + 2)
    }
    ///Bit 2 - Fault 1 source
    #[inline(always)]
    pub fn flt1src(&mut self) -> FLTSRC_W<'_, FLTINR1rs> {
        FLTSRC_W::new(self, 2)
    }
    ///Bit 10 - Fault 2 source
    #[inline(always)]
    pub fn flt2src(&mut self) -> FLTSRC_W<'_, FLTINR1rs> {
        FLTSRC_W::new(self, 10)
    }
    ///Bit 18 - Fault 3 source
    #[inline(always)]
    pub fn flt3src(&mut self) -> FLTSRC_W<'_, FLTINR1rs> {
        FLTSRC_W::new(self, 18)
    }
    ///Bit 26 - Fault 4 source
    #[inline(always)]
    pub fn flt4src(&mut self) -> FLTSRC_W<'_, FLTINR1rs> {
        FLTSRC_W::new(self, 26)
    }
    ///FLT(1-4)F
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1F` field.</div>
    #[inline(always)]
    pub fn fltf(&mut self, n: u8) -> FLTF_W<'_, FLTINR1rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FLTF_W::new(self, n * 8 + 3)
    }
    ///Bits 3:6 - FLT1F
    #[inline(always)]
    pub fn flt1f(&mut self) -> FLTF_W<'_, FLTINR1rs> {
        FLTF_W::new(self, 3)
    }
    ///Bits 11:14 - FLT2F
    #[inline(always)]
    pub fn flt2f(&mut self) -> FLTF_W<'_, FLTINR1rs> {
        FLTF_W::new(self, 11)
    }
    ///Bits 19:22 - FLT3F
    #[inline(always)]
    pub fn flt3f(&mut self) -> FLTF_W<'_, FLTINR1rs> {
        FLTF_W::new(self, 19)
    }
    ///Bits 27:30 - FLT4F
    #[inline(always)]
    pub fn flt4f(&mut self) -> FLTF_W<'_, FLTINR1rs> {
        FLTF_W::new(self, 27)
    }
    ///Bit 7 - FLT1LCK
    #[inline(always)]
    pub fn flt1lck(&mut self) -> FLT1LCK_W<'_, FLTINR1rs> {
        FLT1LCK_W::new(self, 7)
    }
    ///Bit 15 - FLT2LCK
    #[inline(always)]
    pub fn flt2lck(&mut self) -> FLT2LCK_W<'_, FLTINR1rs> {
        FLT2LCK_W::new(self, 15)
    }
    ///Bit 23 - FLT3LCK
    #[inline(always)]
    pub fn flt3lck(&mut self) -> FLT3LCK_W<'_, FLTINR1rs> {
        FLT3LCK_W::new(self, 23)
    }
    ///Bit 31 - FLT4LCK
    #[inline(always)]
    pub fn flt4lck(&mut self) -> FLT4LCK_W<'_, FLTINR1rs> {
        FLT4LCK_W::new(self, 31)
    }
}
/**HRTIM Fault Input Register 1

You can [`read`](crate::Reg::read) this register and get [`fltinr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_Common:FLTINR1)*/
pub struct FLTINR1rs;
impl crate::RegisterSpec for FLTINR1rs {
    type Ux = u32;
}
///`read()` method returns [`fltinr1::R`](R) reader structure
impl crate::Readable for FLTINR1rs {}
///`write(|w| ..)` method takes [`fltinr1::W`](W) writer structure
impl crate::Writable for FLTINR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTINR1 to value 0
impl crate::Resettable for FLTINR1rs {}
