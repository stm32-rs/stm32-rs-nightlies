#[doc = "Register `FLTINR2` reader"]
pub type R = crate::R<FLTINR2rs>;
#[doc = "Register `FLTINR2` writer"]
pub type W = crate::W<FLTINR2rs>;
#[doc = "FLT5E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5E {
    #[doc = "0: Fault input disabled"]
    Disabled = 0,
    #[doc = "1: Fault input enabled"]
    Enabled = 1,
}
impl From<FLT5E> for bool {
    #[inline(always)]
    fn from(variant: FLT5E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT5E` reader - FLT5E"]
pub type FLT5E_R = crate::BitReader<FLT5E>;
impl FLT5E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT5E {
        match self.bits {
            false => FLT5E::Disabled,
            true => FLT5E::Enabled,
        }
    }
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5E::Disabled
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLT5E::Enabled
    }
}
#[doc = "Field `FLT5E` writer - FLT5E"]
pub type FLT5E_W<'a, REG> = crate::BitWriter<'a, REG, FLT5E>;
impl<'a, REG> FLT5E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5E::Disabled)
    }
    #[doc = "Fault input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5E::Enabled)
    }
}
#[doc = "FLT5P\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5P {
    #[doc = "0: Fault input is active low"]
    ActiveLow = 0,
    #[doc = "1: Fault input is active high"]
    ActiveHigh = 1,
}
impl From<FLT5P> for bool {
    #[inline(always)]
    fn from(variant: FLT5P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT5P` reader - FLT5P"]
pub type FLT5P_R = crate::BitReader<FLT5P>;
impl FLT5P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT5P {
        match self.bits {
            false => FLT5P::ActiveLow,
            true => FLT5P::ActiveHigh,
        }
    }
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FLT5P::ActiveLow
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FLT5P::ActiveHigh
    }
}
#[doc = "Field `FLT5P` writer - FLT5P"]
pub type FLT5P_W<'a, REG> = crate::BitWriter<'a, REG, FLT5P>;
impl<'a, REG> FLT5P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5P::ActiveLow)
    }
    #[doc = "Fault input is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5P::ActiveHigh)
    }
}
#[doc = "FLT5SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5SRC {
    #[doc = "0: Fault input is FLTx input pin"]
    Input = 0,
    #[doc = "1: Fault input is FLTn_Int signal"]
    Internal = 1,
}
impl From<FLT5SRC> for bool {
    #[inline(always)]
    fn from(variant: FLT5SRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT5SRC` reader - FLT5SRC"]
pub type FLT5SRC_R = crate::BitReader<FLT5SRC>;
impl FLT5SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT5SRC {
        match self.bits {
            false => FLT5SRC::Input,
            true => FLT5SRC::Internal,
        }
    }
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FLT5SRC::Input
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == FLT5SRC::Internal
    }
}
#[doc = "Field `FLT5SRC` writer - FLT5SRC"]
pub type FLT5SRC_W<'a, REG> = crate::BitWriter<'a, REG, FLT5SRC>;
impl<'a, REG> FLT5SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input is FLTx input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5SRC::Input)
    }
    #[doc = "Fault input is FLTn_Int signal"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5SRC::Internal)
    }
}
#[doc = "FLT5F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLT5F {
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
impl From<FLT5F> for u8 {
    #[inline(always)]
    fn from(variant: FLT5F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLT5F {
    type Ux = u8;
}
#[doc = "Field `FLT5F` reader - FLT5F"]
pub type FLT5F_R = crate::FieldReader<FLT5F>;
impl FLT5F_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLT5F::Disabled
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn is_div1_n2(&self) -> bool {
        *self == FLT5F::Div1N2
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn is_div1_n4(&self) -> bool {
        *self == FLT5F::Div1N4
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn is_div1_n8(&self) -> bool {
        *self == FLT5F::Div1N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn is_div2_n6(&self) -> bool {
        *self == FLT5F::Div2N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn is_div2_n8(&self) -> bool {
        *self == FLT5F::Div2N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn is_div4_n6(&self) -> bool {
        *self == FLT5F::Div4N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn is_div4_n8(&self) -> bool {
        *self == FLT5F::Div4N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn is_div8_n6(&self) -> bool {
        *self == FLT5F::Div8N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn is_div8_n8(&self) -> bool {
        *self == FLT5F::Div8N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn is_div16_n5(&self) -> bool {
        *self == FLT5F::Div16N5
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn is_div16_n6(&self) -> bool {
        *self == FLT5F::Div16N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn is_div16_n8(&self) -> bool {
        *self == FLT5F::Div16N8
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn is_div32_n5(&self) -> bool {
        *self == FLT5F::Div32N5
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn is_div32_n6(&self) -> bool {
        *self == FLT5F::Div32N6
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn is_div32_n8(&self) -> bool {
        *self == FLT5F::Div32N8
    }
}
#[doc = "Field `FLT5F` writer - FLT5F"]
pub type FLT5F_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, FLT5F>;
impl<'a, REG> FLT5F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, FLTx acts asynchronously"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Disabled)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=2"]
    #[inline(always)]
    pub fn div1_n2(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div1N2)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=4"]
    #[inline(always)]
    pub fn div1_n4(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div1N4)
    }
    #[doc = "f_SAMPLING=f_HRTIM, N=8"]
    #[inline(always)]
    pub fn div1_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div1N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=6"]
    #[inline(always)]
    pub fn div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div2N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/2, N=8"]
    #[inline(always)]
    pub fn div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div2N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=6"]
    #[inline(always)]
    pub fn div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div4N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/4, N=8"]
    #[inline(always)]
    pub fn div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div4N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=6"]
    #[inline(always)]
    pub fn div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div8N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/8, N=8"]
    #[inline(always)]
    pub fn div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div8N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=5"]
    #[inline(always)]
    pub fn div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div16N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=6"]
    #[inline(always)]
    pub fn div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div16N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/16, N=8"]
    #[inline(always)]
    pub fn div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div16N8)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=5"]
    #[inline(always)]
    pub fn div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div32N5)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=6"]
    #[inline(always)]
    pub fn div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div32N6)
    }
    #[doc = "f_SAMPLING=f_HRTIM/32, N=8"]
    #[inline(always)]
    pub fn div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5F::Div32N8)
    }
}
#[doc = "FLT5LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5LCKR {
    #[doc = "0: Fault bits are read/write"]
    Unlocked = 0,
    #[doc = "1: Fault bits are read-only"]
    Locked = 1,
}
impl From<FLT5LCKR> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT5LCK` reader - FLT5LCK"]
pub type FLT5LCK_R = crate::BitReader<FLT5LCKR>;
impl FLT5LCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT5LCKR {
        match self.bits {
            false => FLT5LCKR::Unlocked,
            true => FLT5LCKR::Locked,
        }
    }
    #[doc = "Fault bits are read/write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLT5LCKR::Unlocked
    }
    #[doc = "Fault bits are read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLT5LCKR::Locked
    }
}
#[doc = "FLT5LCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT5LCKW {
    #[doc = "1: Lock corresponding fault bits"]
    Lock = 1,
}
impl From<FLT5LCKW> for bool {
    #[inline(always)]
    fn from(variant: FLT5LCKW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT5LCK` writer - FLT5LCK"]
pub type FLT5LCK_W<'a, REG> = crate::BitWriter<'a, REG, FLT5LCKW>;
impl<'a, REG> FLT5LCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock corresponding fault bits"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(FLT5LCKW::Lock)
    }
}
#[doc = "FLTSD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLTSD {
    #[doc = "0: f_FLTS=f_HRTIM"]
    Div1 = 0,
    #[doc = "1: f_FLTS=f_HRTIM/2"]
    Div2 = 1,
    #[doc = "2: f_FLTS=f_HRTIM/4"]
    Div4 = 2,
    #[doc = "3: f_FLTS=f_HRTIM/8"]
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
#[doc = "Field `FLTSD` reader - FLTSD"]
pub type FLTSD_R = crate::FieldReader<FLTSD>;
impl FLTSD_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "f_FLTS=f_HRTIM"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FLTSD::Div1
    }
    #[doc = "f_FLTS=f_HRTIM/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FLTSD::Div2
    }
    #[doc = "f_FLTS=f_HRTIM/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FLTSD::Div4
    }
    #[doc = "f_FLTS=f_HRTIM/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FLTSD::Div8
    }
}
#[doc = "Field `FLTSD` writer - FLTSD"]
pub type FLTSD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FLTSD>;
impl<'a, REG> FLTSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "f_FLTS=f_HRTIM"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div1)
    }
    #[doc = "f_FLTS=f_HRTIM/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div2)
    }
    #[doc = "f_FLTS=f_HRTIM/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div4)
    }
    #[doc = "f_FLTS=f_HRTIM/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(FLTSD::Div8)
    }
}
impl R {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    #[must_use]
    pub fn flt5e(&mut self) -> FLT5E_W<FLTINR2rs> {
        FLT5E_W::new(self, 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    #[must_use]
    pub fn flt5p(&mut self) -> FLT5P_W<FLTINR2rs> {
        FLT5P_W::new(self, 1)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    #[must_use]
    pub fn flt5src(&mut self) -> FLT5SRC_W<FLTINR2rs> {
        FLT5SRC_W::new(self, 2)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    #[must_use]
    pub fn flt5f(&mut self) -> FLT5F_W<FLTINR2rs> {
        FLT5F_W::new(self, 3)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    #[must_use]
    pub fn flt5lck(&mut self) -> FLT5LCK_W<FLTINR2rs> {
        FLT5LCK_W::new(self, 7)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    #[must_use]
    pub fn fltsd(&mut self) -> FLTSD_W<FLTINR2rs> {
        FLTSD_W::new(self, 24)
    }
}
#[doc = "HRTIM Fault Input Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltinr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltinr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTINR2rs;
impl crate::RegisterSpec for FLTINR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr2::R`](R) reader structure"]
impl crate::Readable for FLTINR2rs {}
#[doc = "`write(|w| ..)` method takes [`fltinr2::W`](W) writer structure"]
impl crate::Writable for FLTINR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTINR2 to value 0"]
impl crate::Resettable for FLTINR2rs {
    const RESET_VALUE: u32 = 0;
}
