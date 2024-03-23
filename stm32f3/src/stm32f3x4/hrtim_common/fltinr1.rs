#[doc = "Register `FLTINR1` reader"]
pub type R = crate::R<FLTINR1rs>;
#[doc = "Register `FLTINR1` writer"]
pub type W = crate::W<FLTINR1rs>;
#[doc = "FLT1E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1E {
    #[doc = "0: Fault input disabled"]
    Disabled = 0,
    #[doc = "1: Fault input enabled"]
    Enabled = 1,
}
impl From<FLT1E> for bool {
    #[inline(always)]
    fn from(variant: FLT1E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1E` reader - FLT1E"]
pub type FLT1E_R = crate::BitReader<FLT1E>;
impl FLT1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT1E {
        match self.bits {
            false => FLT1E::Disabled,
            true => FLT1E::Enabled,
        }
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1E::Disabled
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT1E::Enabled
    }
}
#[doc = "Field `FLT1E` writer - FLT1E"]
pub type FLT1E_W<'a, REG> = crate::BitWriter<'a, REG, FLT1E>;
impl<'a, REG> FLT1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1E::Disabled)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1E::Enabled)
    }
}
#[doc = "FLT1P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1P {
    #[doc = "0: Fault input is active low"]
    ActiveLow = 0,
    #[doc = "1: Fault input is active high"]
    ActiveHigh = 1,
}
impl From<FLT1P> for bool {
    #[inline(always)]
    fn from(variant: FLT1P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1P` reader - FLT1P"]
pub type FLT1P_R = crate::BitReader<FLT1P>;
impl FLT1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT1P {
        match self.bits {
            false => FLT1P::ActiveLow,
            true => FLT1P::ActiveHigh,
        }
    }
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FLT1P::ActiveLow
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FLT1P::ActiveHigh
    }
}
#[doc = "Field `FLT1P` writer - FLT1P"]
pub type FLT1P_W<'a, REG> = crate::BitWriter<'a, REG, FLT1P>;
impl<'a, REG> FLT1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1P::ActiveLow)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1P::ActiveHigh)
    }
}
#[doc = "FLT1SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1SRC {
    #[doc = "0: Fault input is FLTx input pin"]
    Input = 0,
    #[doc = "1: Fault input is FLTn_Int signal"]
    Internal = 1,
}
impl From<FLT1SRC> for bool {
    #[inline(always)]
    fn from(variant: FLT1SRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1SRC` reader - FLT1SRC"]
pub type FLT1SRC_R = crate::BitReader<FLT1SRC>;
impl FLT1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT1SRC {
        match self.bits {
            false => FLT1SRC::Input,
            true => FLT1SRC::Internal,
        }
    }
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FLT1SRC::Input
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == FLT1SRC::Internal
    }
}
#[doc = "Field `FLT1SRC` writer - FLT1SRC"]
pub type FLT1SRC_W<'a, REG> = crate::BitWriter<'a, REG, FLT1SRC>;
impl<'a, REG> FLT1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1SRC::Input)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1SRC::Internal)
    }
}
#[doc = "FLT1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT1F {
    #[doc = "0: No filter, FLTx acts asynchronously"]
    Disabled = 0,
    #[doc = "1: f_SAMPLING=f_HRTIM, N=2"]
    Div1N2 = 1,
    #[doc = "2: f_SAMPLING=f_HRTIM, N=4"]
    Div1N4 = 2,
    #[doc = "3: f_SAMPLING=f_HRTIM, N=8"]
    Div1N8 = 3,
    #[doc = "4: f_SAMPLING=f_HRTIM/2, N=6"]
    Div2N6 = 4,
    #[doc = "5: f_SAMPLING=f_HRTIM/2, N=8"]
    Div2N8 = 5,
    #[doc = "6: f_SAMPLING=f_HRTIM/4, N=6"]
    Div4N6 = 6,
    #[doc = "7: f_SAMPLING=f_HRTIM/4, N=8"]
    Div4N8 = 7,
    #[doc = "8: f_SAMPLING=f_HRTIM/8, N=6"]
    Div8N6 = 8,
    #[doc = "9: f_SAMPLING=f_HRTIM/8, N=8"]
    Div8N8 = 9,
    #[doc = "10: f_SAMPLING=f_HRTIM/16, N=5"]
    Div16N5 = 10,
    #[doc = "11: f_SAMPLING=f_HRTIM/16, N=6"]
    Div16N6 = 11,
    #[doc = "12: f_SAMPLING=f_HRTIM/16, N=8"]
    Div16N8 = 12,
    #[doc = "13: f_SAMPLING=f_HRTIM/32, N=5"]
    Div32N5 = 13,
    #[doc = "14: f_SAMPLING=f_HRTIM/32, N=6"]
    Div32N6 = 14,
    #[doc = "15: f_SAMPLING=f_HRTIM/32, N=8"]
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
#[doc = "Field `FLT1F` reader - FLT1F"]
pub type FLT1F_R = crate::FieldReader<FLT1F>;
impl FLT1F_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT1F::Disabled
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == FLT1F::Div1N2
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == FLT1F::Div1N4
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == FLT1F::Div1N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == FLT1F::Div2N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == FLT1F::Div2N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == FLT1F::Div4N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == FLT1F::Div4N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == FLT1F::Div8N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == FLT1F::Div8N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == FLT1F::Div16N5
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == FLT1F::Div16N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == FLT1F::Div16N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == FLT1F::Div32N5
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == FLT1F::Div32N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == FLT1F::Div32N8
    }
}
#[doc = "Field `FLT1F` writer - FLT1F"]
pub type FLT1F_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, FLT1F>;
impl<'a, REG> FLT1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Disabled)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div1N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div1N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div1N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div2N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div2N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div4N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div4N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div8N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div8N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div16N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div16N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div16N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div32N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div32N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1F::Div32N8)
    }
}
#[doc = "FLT1LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1LCKR {
    #[doc = "0: Fault bits are read/write"]
    Unlocked = 0,
    #[doc = "1: Fault bits are read-only"]
    Locked = 1,
}
impl From<FLT1LCKR> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1LCK` reader - FLT1LCK"]
pub type FLT1LCK_R = crate::BitReader<FLT1LCKR>;
impl FLT1LCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT1LCKR {
        match self.bits {
            false => FLT1LCKR::Unlocked,
            true => FLT1LCKR::Locked,
        }
    }
    #[doc = "Fault bits are read/write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLT1LCKR::Unlocked
    }
    #[doc = "Fault bits are read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLT1LCKR::Locked
    }
}
#[doc = "FLT1LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1LCKW {
    #[doc = "1: Lock corresponding fault bits"]
    Lock = 1,
}
impl From<FLT1LCKW> for bool {
    #[inline(always)]
    fn from(variant: FLT1LCKW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1LCK` writer - FLT1LCK"]
pub type FLT1LCK_W<'a, REG> = crate::BitWriter<'a, REG, FLT1LCKW>;
impl<'a, REG> FLT1LCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1LCKW::Lock)
    }
}
#[doc = "Field `FLT2E` reader - FLT2E"]
pub use FLT1E_R as FLT2E_R;
#[doc = "Field `FLT3E` reader - FLT3E"]
pub use FLT1E_R as FLT3E_R;
#[doc = "Field `FLT4E` reader - FLT4E"]
pub use FLT1E_R as FLT4E_R;
#[doc = "Field `FLT2E` writer - FLT2E"]
pub use FLT1E_W as FLT2E_W;
#[doc = "Field `FLT3E` writer - FLT3E"]
pub use FLT1E_W as FLT3E_W;
#[doc = "Field `FLT4E` writer - FLT4E"]
pub use FLT1E_W as FLT4E_W;
#[doc = "Field `FLT2F` reader - FLT2F"]
pub use FLT1F_R as FLT2F_R;
#[doc = "Field `FLT3F` reader - FLT3F"]
pub use FLT1F_R as FLT3F_R;
#[doc = "Field `FLT4F` reader - FLT4F"]
pub use FLT1F_R as FLT4F_R;
#[doc = "Field `FLT2F` writer - FLT2F"]
pub use FLT1F_W as FLT2F_W;
#[doc = "Field `FLT3F` writer - FLT3F"]
pub use FLT1F_W as FLT3F_W;
#[doc = "Field `FLT4F` writer - FLT4F"]
pub use FLT1F_W as FLT4F_W;
#[doc = "Field `FLT2LCK` reader - FLT2LCK"]
pub use FLT1LCK_R as FLT2LCK_R;
#[doc = "Field `FLT3LCK` reader - FLT3LCK"]
pub use FLT1LCK_R as FLT3LCK_R;
#[doc = "Field `FLT4LCK` reader - FLT4LCK"]
pub use FLT1LCK_R as FLT4LCK_R;
#[doc = "Field `FLT2LCK` writer - FLT2LCK"]
pub use FLT1LCK_W as FLT2LCK_W;
#[doc = "Field `FLT3LCK` writer - FLT3LCK"]
pub use FLT1LCK_W as FLT3LCK_W;
#[doc = "Field `FLT4LCK` writer - FLT4LCK"]
pub use FLT1LCK_W as FLT4LCK_W;
#[doc = "Field `FLT2P` reader - FLT2P"]
pub use FLT1P_R as FLT2P_R;
#[doc = "Field `FLT3P` reader - FLT3P"]
pub use FLT1P_R as FLT3P_R;
#[doc = "Field `FLT4P` reader - FLT4P"]
pub use FLT1P_R as FLT4P_R;
#[doc = "Field `FLT2P` writer - FLT2P"]
pub use FLT1P_W as FLT2P_W;
#[doc = "Field `FLT3P` writer - FLT3P"]
pub use FLT1P_W as FLT3P_W;
#[doc = "Field `FLT4P` writer - FLT4P"]
pub use FLT1P_W as FLT4P_W;
#[doc = "Field `FLT2SRC` reader - FLT2SRC"]
pub use FLT1SRC_R as FLT2SRC_R;
#[doc = "Field `FLT3SRC` reader - FLT3SRC"]
pub use FLT1SRC_R as FLT3SRC_R;
#[doc = "Field `FLT4SRC` reader - FLT4SRC"]
pub use FLT1SRC_R as FLT4SRC_R;
#[doc = "Field `FLT2SRC` writer - FLT2SRC"]
pub use FLT1SRC_W as FLT2SRC_W;
#[doc = "Field `FLT3SRC` writer - FLT3SRC"]
pub use FLT1SRC_W as FLT3SRC_W;
#[doc = "Field `FLT4SRC` writer - FLT4SRC"]
pub use FLT1SRC_W as FLT4SRC_W;
impl R {
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    pub fn flt1e(&self) -> FLT1E_R {
        FLT1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    pub fn flt1p(&self) -> FLT1P_R {
        FLT1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    pub fn flt1src(&self) -> FLT1SRC_R {
        FLT1SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    pub fn flt1f(&self) -> FLT1F_R {
        FLT1F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    pub fn flt1lck(&self) -> FLT1LCK_R {
        FLT1LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    pub fn flt2e(&self) -> FLT2E_R {
        FLT2E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    pub fn flt2p(&self) -> FLT2P_R {
        FLT2P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    pub fn flt2src(&self) -> FLT2SRC_R {
        FLT2SRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    pub fn flt2f(&self) -> FLT2F_R {
        FLT2F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    pub fn flt2lck(&self) -> FLT2LCK_R {
        FLT2LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    pub fn flt3e(&self) -> FLT3E_R {
        FLT3E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    pub fn flt3p(&self) -> FLT3P_R {
        FLT3P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    pub fn flt3src(&self) -> FLT3SRC_R {
        FLT3SRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    pub fn flt3f(&self) -> FLT3F_R {
        FLT3F_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    pub fn flt3lck(&self) -> FLT3LCK_R {
        FLT3LCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    pub fn flt4e(&self) -> FLT4E_R {
        FLT4E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    pub fn flt4p(&self) -> FLT4P_R {
        FLT4P_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    pub fn flt4src(&self) -> FLT4SRC_R {
        FLT4SRC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    pub fn flt4f(&self) -> FLT4F_R {
        FLT4F_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    pub fn flt4lck(&self) -> FLT4LCK_R {
        FLT4LCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    #[must_use]
    pub fn flt1e(&mut self) -> FLT1E_W<FLTINR1rs> {
        FLT1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    #[must_use]
    pub fn flt1p(&mut self) -> FLT1P_W<FLTINR1rs> {
        FLT1P_W::new(self, 1)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt1src(&mut self) -> FLT1SRC_W<FLTINR1rs> {
        FLT1SRC_W::new(self, 2)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    #[must_use]
    pub fn flt1f(&mut self) -> FLT1F_W<FLTINR1rs> {
        FLT1F_W::new(self, 3)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt1lck(&mut self) -> FLT1LCK_W<FLTINR1rs> {
        FLT1LCK_W::new(self, 7)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    #[must_use]
    pub fn flt2e(&mut self) -> FLT2E_W<FLTINR1rs> {
        FLT2E_W::new(self, 8)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    #[must_use]
    pub fn flt2p(&mut self) -> FLT2P_W<FLTINR1rs> {
        FLT2P_W::new(self, 9)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt2src(&mut self) -> FLT2SRC_W<FLTINR1rs> {
        FLT2SRC_W::new(self, 10)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    #[must_use]
    pub fn flt2f(&mut self) -> FLT2F_W<FLTINR1rs> {
        FLT2F_W::new(self, 11)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt2lck(&mut self) -> FLT2LCK_W<FLTINR1rs> {
        FLT2LCK_W::new(self, 15)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    #[must_use]
    pub fn flt3e(&mut self) -> FLT3E_W<FLTINR1rs> {
        FLT3E_W::new(self, 16)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    #[must_use]
    pub fn flt3p(&mut self) -> FLT3P_W<FLTINR1rs> {
        FLT3P_W::new(self, 17)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt3src(&mut self) -> FLT3SRC_W<FLTINR1rs> {
        FLT3SRC_W::new(self, 18)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    #[must_use]
    pub fn flt3f(&mut self) -> FLT3F_W<FLTINR1rs> {
        FLT3F_W::new(self, 19)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt3lck(&mut self) -> FLT3LCK_W<FLTINR1rs> {
        FLT3LCK_W::new(self, 23)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    #[must_use]
    pub fn flt4e(&mut self) -> FLT4E_W<FLTINR1rs> {
        FLT4E_W::new(self, 24)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    #[must_use]
    pub fn flt4p(&mut self) -> FLT4P_W<FLTINR1rs> {
        FLT4P_W::new(self, 25)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt4src(&mut self) -> FLT4SRC_W<FLTINR1rs> {
        FLT4SRC_W::new(self, 26)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    #[must_use]
    pub fn flt4f(&mut self) -> FLT4F_W<FLTINR1rs> {
        FLT4F_W::new(self, 27)
    }
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt4lck(&mut self) -> FLT4LCK_W<FLTINR1rs> {
        FLT4LCK_W::new(self, 31)
    }
}
#[doc = "HRTIM Fault Input Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTINR1rs;
impl crate::RegisterSpec for FLTINR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr1::R`](R) reader structure"]
impl crate::Readable for FLTINR1rs {}
#[doc = "`write(|w| ..)` method takes [`fltinr1::W`](W) writer structure"]
impl crate::Writable for FLTINR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTINR1 to value 0"]
impl crate::Resettable for FLTINR1rs {
    const RESET_VALUE: u32 = 0;
}
