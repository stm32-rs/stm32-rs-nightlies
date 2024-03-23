#[doc = "Register `AF2` reader"]
pub type R = crate::R<AF2rs>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<AF2rs>;
#[doc = "BRK2 BKIN input enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INE {
    #[doc = "0: BKIN input disabled"]
    Disabled = 0,
    #[doc = "1: BKIN input enabled"]
    Enabled = 1,
}
impl From<BK2INE> for bool {
    #[inline(always)]
    fn from(variant: BK2INE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable"]
pub type BK2INE_R = crate::BitReader<BK2INE>;
impl BK2INE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2INE {
        match self.bits {
            false => BK2INE::Disabled,
            true => BK2INE::Enabled,
        }
    }
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2INE::Disabled
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2INE::Enabled
    }
}
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable"]
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG, BK2INE>;
impl<'a, REG> BK2INE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::Disabled)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::Enabled)
    }
}
#[doc = "BRK2 COMP1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1E {
    #[doc = "0: COMP1 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP1 input enabled"]
    Enabled = 1,
}
impl From<BK2CMP1E> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type BK2CMP1E_R = crate::BitReader<BK2CMP1E>;
impl BK2CMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1E {
        match self.bits {
            false => BK2CMP1E::Disabled,
            true => BK2CMP1E::Enabled,
        }
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2CMP1E::Disabled
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2CMP1E::Enabled
    }
}
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1E>;
impl<'a, REG> BK2CMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E::Disabled)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E::Enabled)
    }
}
#[doc = "BRK2 COMP2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2E {
    #[doc = "0: COMP2 input disabled"]
    Disabled = 0,
    #[doc = "1: COMP2 input enabled"]
    Enabled = 1,
}
impl From<BK2CMP2E> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type BK2CMP2E_R = crate::BitReader<BK2CMP2E>;
impl BK2CMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2E {
        match self.bits {
            false => BK2CMP2E::Disabled,
            true => BK2CMP2E::Enabled,
        }
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2CMP2E::Disabled
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2CMP2E::Enabled
    }
}
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type BK2CMP2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2E>;
impl<'a, REG> BK2CMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E::Disabled)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E::Enabled)
    }
}
#[doc = "BRK2 BKIN2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INP {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BK2INP> for bool {
    #[inline(always)]
    fn from(variant: BK2INP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity"]
pub type BK2INP_R = crate::BitReader<BK2INP>;
impl BK2INP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2INP {
        match self.bits {
            false => BK2INP::NotInverted,
            true => BK2INP::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2INP::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2INP::Inverted
    }
}
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity"]
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG, BK2INP>;
impl<'a, REG> BK2INP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::Inverted)
    }
}
#[doc = "BRK2 COMP1 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1P {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BK2CMP1P> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub type BK2CMP1P_R = crate::BitReader<BK2CMP1P>;
impl BK2CMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1P {
        match self.bits {
            false => BK2CMP1P::NotInverted,
            true => BK2CMP1P::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2CMP1P::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2CMP1P::Inverted
    }
}
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
pub type BK2CMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1P>;
impl<'a, REG> BK2CMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P::Inverted)
    }
}
#[doc = "BRK2 COMP2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2P {
    #[doc = "0: Input polarity not inverted"]
    NotInverted = 0,
    #[doc = "1: Input polarity inverted"]
    Inverted = 1,
}
impl From<BK2CMP2P> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_R = crate::BitReader<BK2CMP2P>;
impl BK2CMP2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2P {
        match self.bits {
            false => BK2CMP2P::NotInverted,
            true => BK2CMP2P::Inverted,
        }
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2CMP2P::NotInverted
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2CMP2P::Inverted
    }
}
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2P>;
impl<'a, REG> BK2CMP2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P::NotInverted)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P::Inverted)
    }
}
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<AF2rs> {
        BK2INE_W::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<AF2rs> {
        BK2CMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<AF2rs> {
        BK2CMP2E_W::new(self, 2)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<AF2rs> {
        BK2INP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<AF2rs> {
        BK2CMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<AF2rs> {
        BK2CMP2P_W::new(self, 11)
    }
}
#[doc = "Alternate function register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for AF2rs {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF2 to value 0x01"]
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
