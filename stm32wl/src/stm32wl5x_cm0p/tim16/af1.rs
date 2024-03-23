#[doc = "Register `AF1` reader"]
pub type R = crate::R<AF1rs>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<AF1rs>;
#[doc = "BRK BKIN input enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINE {
    #[doc = "0: BKIN input disabled"]
    Disabled = 0,
    #[doc = "1: BKIN input enabled"]
    Enabled = 1,
}
impl From<BKINE> for bool {
    #[inline(always)]
    fn from(variant: BKINE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BKINE_R = crate::BitReader<BKINE>;
impl BKINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKINE {
        match self.bits {
            false => BKINE::Disabled,
            true => BKINE::Enabled,
        }
    }
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKINE::Disabled
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKINE::Enabled
    }
}
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG, BKINE>;
impl<'a, REG> BKINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE::Disabled)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKINE::Enabled)
    }
}
#[doc = "BRK COMP1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1E {
    #[doc = "0: COMP1 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP1 input enabled"]
    Enabled = 1,
}
impl From<BKCMP1E> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type BKCMP1E_R = crate::BitReader<BKCMP1E>;
impl BKCMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP1E {
        match self.bits {
            false => BKCMP1E::Disabled,
            true => BKCMP1E::Enabled,
        }
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKCMP1E::Disabled
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKCMP1E::Enabled
    }
}
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP1E>;
impl<'a, REG> BKCMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1E::Disabled)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1E::Enabled)
    }
}
#[doc = "BRK COMP2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP2E {
    #[doc = "0: COMP2 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP2 input enabled"]
    Enabled = 1,
}
impl From<BKCMP2E> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type BKCMP2E_R = crate::BitReader<BKCMP2E>;
impl BKCMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP2E {
        match self.bits {
            false => BKCMP2E::Disabled,
            true => BKCMP2E::Enabled,
        }
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKCMP2E::Disabled
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKCMP2E::Enabled
    }
}
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP2E>;
impl<'a, REG> BKCMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2E::Disabled)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2E::Enabled)
    }
}
#[doc = "BRK BKIN input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKINP {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKINP> for bool {
    #[inline(always)]
    fn from(variant: BKINP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BKINP_R = crate::BitReader<BKINP>;
impl BKINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKINP {
        match self.bits {
            false => BKINP::NotInverted,
            true => BKINP::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKINP::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKINP::Inverted
    }
}
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG, BKINP>;
impl<'a, REG> BKINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKINP::Inverted)
    }
}
#[doc = "BRK COMP1 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP1P {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKCMP1P> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type BKCMP1P_R = crate::BitReader<BKCMP1P>;
impl BKCMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP1P {
        match self.bits {
            false => BKCMP1P::NotInverted,
            true => BKCMP1P::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKCMP1P::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKCMP1P::Inverted
    }
}
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP1P>;
impl<'a, REG> BKCMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1P::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP1P::Inverted)
    }
}
#[doc = "BRK COMP2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKCMP2P {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BKCMP2P> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity"]
pub type BKCMP2P_R = crate::BitReader<BKCMP2P>;
impl BKCMP2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKCMP2P {
        match self.bits {
            false => BKCMP2P::NotInverted,
            true => BKCMP2P::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BKCMP2P::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BKCMP2P::Inverted
    }
}
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity"]
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG, BKCMP2P>;
impl<'a, REG> BKCMP2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2P::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BKCMP2P::Inverted)
    }
}
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<AF1rs> {
        BKINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<AF1rs> {
        BKCMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<AF1rs> {
        BKCMP2E_W::new(self, 2)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<AF1rs> {
        BKINP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<AF1rs> {
        BKCMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<AF1rs> {
        BKCMP2P_W::new(self, 11)
    }
}
#[doc = "TIM16 alternate function register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for AF1rs {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF1 to value 0x01"]
impl crate::Resettable for AF1rs {
    const RESET_VALUE: u32 = 0x01;
}
