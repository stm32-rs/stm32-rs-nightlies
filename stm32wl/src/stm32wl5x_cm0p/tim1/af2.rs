///Register `AF2` reader
pub type R = crate::R<AF2rs>;
///Register `AF2` writer
pub type W = crate::W<AF2rs>;
/**BRK2 BKIN input enable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INE {
    ///0: BKIN input disabled
    Disabled = 0,
    ///1: BKIN input enabled
    Enabled = 1,
}
impl From<BK2INE> for bool {
    #[inline(always)]
    fn from(variant: BK2INE) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2INE` reader - BRK2 BKIN input enable
pub type BK2INE_R = crate::BitReader<BK2INE>;
impl BK2INE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2INE {
        match self.bits {
            false => BK2INE::Disabled,
            true => BK2INE::Enabled,
        }
    }
    ///BKIN input disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2INE::Disabled
    }
    ///BKIN input enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2INE::Enabled
    }
}
///Field `BK2INE` writer - BRK2 BKIN input enable
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG, BK2INE>;
impl<'a, REG> BK2INE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BKIN input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::Disabled)
    }
    ///BKIN input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE::Enabled)
    }
}
/**BRK2 COMP1 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1E {
    ///0: COMP1 input disabled
    Disabled = 0,
    ///1: COMP1 input enabled
    Enabled = 1,
}
impl From<BK2CMP1E> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2CMP1E` reader - BRK2 COMP1 enable
pub type BK2CMP1E_R = crate::BitReader<BK2CMP1E>;
impl BK2CMP1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1E {
        match self.bits {
            false => BK2CMP1E::Disabled,
            true => BK2CMP1E::Enabled,
        }
    }
    ///COMP1 input disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2CMP1E::Disabled
    }
    ///COMP1 input enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2CMP1E::Enabled
    }
}
///Field `BK2CMP1E` writer - BRK2 COMP1 enable
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1E>;
impl<'a, REG> BK2CMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///COMP1 input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E::Disabled)
    }
    ///COMP1 input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E::Enabled)
    }
}
/**BRK2 COMP2 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2E {
    ///0: COMP2 input disabled
    Disabled = 0,
    ///1: COMP2 input enabled
    Enabled = 1,
}
impl From<BK2CMP2E> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2E) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2CMP2E` reader - BRK2 COMP2 enable
pub type BK2CMP2E_R = crate::BitReader<BK2CMP2E>;
impl BK2CMP2E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2E {
        match self.bits {
            false => BK2CMP2E::Disabled,
            true => BK2CMP2E::Enabled,
        }
    }
    ///COMP2 input disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2CMP2E::Disabled
    }
    ///COMP2 input enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2CMP2E::Enabled
    }
}
///Field `BK2CMP2E` writer - BRK2 COMP2 enable
pub type BK2CMP2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2E>;
impl<'a, REG> BK2CMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///COMP2 input disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E::Disabled)
    }
    ///COMP2 input enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E::Enabled)
    }
}
/**BRK2 BKIN2 input polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INP {
    ///0: Input polarity not inverted
    NotInverted = 0,
    ///1: Input polarity inverted
    Inverted = 1,
}
impl From<BK2INP> for bool {
    #[inline(always)]
    fn from(variant: BK2INP) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2INP` reader - BRK2 BKIN2 input polarity
pub type BK2INP_R = crate::BitReader<BK2INP>;
impl BK2INP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2INP {
        match self.bits {
            false => BK2INP::NotInverted,
            true => BK2INP::Inverted,
        }
    }
    ///Input polarity not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2INP::NotInverted
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2INP::Inverted
    }
}
///Field `BK2INP` writer - BRK2 BKIN2 input polarity
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG, BK2INP>;
impl<'a, REG> BK2INP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input polarity not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::NotInverted)
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP::Inverted)
    }
}
/**BRK2 COMP1 input polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1P {
    ///0: Input polarity not inverted
    NotInverted = 0,
    ///1: Input polarity inverted
    Inverted = 1,
}
impl From<BK2CMP1P> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2CMP1P` reader - BRK2 COMP1 input polarity
pub type BK2CMP1P_R = crate::BitReader<BK2CMP1P>;
impl BK2CMP1P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1P {
        match self.bits {
            false => BK2CMP1P::NotInverted,
            true => BK2CMP1P::Inverted,
        }
    }
    ///Input polarity not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2CMP1P::NotInverted
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2CMP1P::Inverted
    }
}
///Field `BK2CMP1P` writer - BRK2 COMP1 input polarity
pub type BK2CMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1P>;
impl<'a, REG> BK2CMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input polarity not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P::NotInverted)
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P::Inverted)
    }
}
/**BRK2 COMP2 input polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2P {
    ///0: Input polarity not inverted
    NotInverted = 0,
    ///1: Input polarity inverted
    Inverted = 1,
}
impl From<BK2CMP2P> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2P) -> Self {
        variant as u8 != 0
    }
}
///Field `BK2CMP2P` reader - BRK2 COMP2 input polarity
pub type BK2CMP2P_R = crate::BitReader<BK2CMP2P>;
impl BK2CMP2P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2P {
        match self.bits {
            false => BK2CMP2P::NotInverted,
            true => BK2CMP2P::Inverted,
        }
    }
    ///Input polarity not inverted
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == BK2CMP2P::NotInverted
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == BK2CMP2P::Inverted
    }
}
///Field `BK2CMP2P` writer - BRK2 COMP2 input polarity
pub type BK2CMP2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2P>;
impl<'a, REG> BK2CMP2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input polarity not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P::NotInverted)
    }
    ///Input polarity inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P::Inverted)
    }
}
impl R {
    ///Bit 0 - BRK2 BKIN input enable
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRK2 COMP1 enable
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRK2 COMP2 enable
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 9 - BRK2 BKIN2 input polarity
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRK2 COMP1 input polarity
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BRK2 COMP2 input polarity
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("bk2cmp2p", &self.bk2cmp2p())
            .field("bk2cmp1p", &self.bk2cmp1p())
            .field("bk2inp", &self.bk2inp())
            .field("bk2cmp2e", &self.bk2cmp2e())
            .field("bk2cmp1e", &self.bk2cmp1e())
            .field("bk2ine", &self.bk2ine())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRK2 BKIN input enable
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W<'_, AF2rs> {
        BK2INE_W::new(self, 0)
    }
    ///Bit 1 - BRK2 COMP1 enable
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<'_, AF2rs> {
        BK2CMP1E_W::new(self, 1)
    }
    ///Bit 2 - BRK2 COMP2 enable
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<'_, AF2rs> {
        BK2CMP2E_W::new(self, 2)
    }
    ///Bit 9 - BRK2 BKIN2 input polarity
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W<'_, AF2rs> {
        BK2INP_W::new(self, 9)
    }
    ///Bit 10 - BRK2 COMP1 input polarity
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<'_, AF2rs> {
        BK2CMP1P_W::new(self, 10)
    }
    ///Bit 11 - BRK2 COMP2 input polarity
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<'_, AF2rs> {
        BK2CMP2P_W::new(self, 11)
    }
}
/**Alternate function register 2

You can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TIM1:AF2)*/
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
///`read()` method returns [`af2::R`](R) reader structure
impl crate::Readable for AF2rs {}
///`write(|w| ..)` method takes [`af2::W`](W) writer structure
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF2 to value 0x01
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
