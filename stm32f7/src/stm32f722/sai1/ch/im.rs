///Register `IM` reader
pub type R = crate::R<IMrs>;
///Register `IM` writer
pub type W = crate::W<IMrs>;
/**Overrun/underrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRUDRIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<OVRUDRIE> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRUDRIE` reader - Overrun/underrun interrupt enable
pub type OVRUDRIE_R = crate::BitReader<OVRUDRIE>;
impl OVRUDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRUDRIE {
        match self.bits {
            false => OVRUDRIE::Disabled,
            true => OVRUDRIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRUDRIE::Disabled
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRUDRIE::Enabled
    }
}
///Field `OVRUDRIE` writer - Overrun/underrun interrupt enable
pub type OVRUDRIE_W<'a, REG> = crate::BitWriter<'a, REG, OVRUDRIE>;
impl<'a, REG> OVRUDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRUDRIE::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRUDRIE::Enabled)
    }
}
/**Mute detection interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEDETIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<MUTEDETIE> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETIE) -> Self {
        variant as u8 != 0
    }
}
///Field `MUTEDETIE` reader - Mute detection interrupt enable
pub type MUTEDETIE_R = crate::BitReader<MUTEDETIE>;
impl MUTEDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MUTEDETIE {
        match self.bits {
            false => MUTEDETIE::Disabled,
            true => MUTEDETIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTEDETIE::Disabled
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTEDETIE::Enabled
    }
}
///Field `MUTEDETIE` writer - Mute detection interrupt enable
pub type MUTEDETIE_W<'a, REG> = crate::BitWriter<'a, REG, MUTEDETIE>;
impl<'a, REG> MUTEDETIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUTEDETIE::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUTEDETIE::Enabled)
    }
}
/**Wrong clock configuration interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCKCFGIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<WCKCFGIE> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGIE) -> Self {
        variant as u8 != 0
    }
}
///Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable
pub type WCKCFGIE_R = crate::BitReader<WCKCFGIE>;
impl WCKCFGIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WCKCFGIE {
        match self.bits {
            false => WCKCFGIE::Disabled,
            true => WCKCFGIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WCKCFGIE::Disabled
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WCKCFGIE::Enabled
    }
}
///Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable
pub type WCKCFGIE_W<'a, REG> = crate::BitWriter<'a, REG, WCKCFGIE>;
impl<'a, REG> WCKCFGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WCKCFGIE::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WCKCFGIE::Enabled)
    }
}
/**FIFO request interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<FREQIE> for bool {
    #[inline(always)]
    fn from(variant: FREQIE) -> Self {
        variant as u8 != 0
    }
}
///Field `FREQIE` reader - FIFO request interrupt enable
pub type FREQIE_R = crate::BitReader<FREQIE>;
impl FREQIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FREQIE {
        match self.bits {
            false => FREQIE::Disabled,
            true => FREQIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FREQIE::Disabled
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FREQIE::Enabled
    }
}
///Field `FREQIE` writer - FIFO request interrupt enable
pub type FREQIE_W<'a, REG> = crate::BitWriter<'a, REG, FREQIE>;
impl<'a, REG> FREQIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FREQIE::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FREQIE::Enabled)
    }
}
/**Codec not ready interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNRDYIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<CNRDYIE> for bool {
    #[inline(always)]
    fn from(variant: CNRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CNRDYIE` reader - Codec not ready interrupt enable
pub type CNRDYIE_R = crate::BitReader<CNRDYIE>;
impl CNRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CNRDYIE {
        match self.bits {
            false => CNRDYIE::Disabled,
            true => CNRDYIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CNRDYIE::Disabled
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CNRDYIE::Enabled
    }
}
///Field `CNRDYIE` writer - Codec not ready interrupt enable
pub type CNRDYIE_W<'a, REG> = crate::BitWriter<'a, REG, CNRDYIE>;
impl<'a, REG> CNRDYIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CNRDYIE::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CNRDYIE::Enabled)
    }
}
/**Anticipated frame synchronization detection interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFSDETIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<AFSDETIE> for bool {
    #[inline(always)]
    fn from(variant: AFSDETIE) -> Self {
        variant as u8 != 0
    }
}
///Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable
pub type AFSDETIE_R = crate::BitReader<AFSDETIE>;
impl AFSDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSDETIE {
        match self.bits {
            false => AFSDETIE::Disabled,
            true => AFSDETIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFSDETIE::Disabled
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFSDETIE::Enabled
    }
}
///Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable
pub type AFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG, AFSDETIE>;
impl<'a, REG> AFSDETIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFSDETIE::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AFSDETIE::Enabled)
    }
}
/**Late frame synchronization detection interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSDETIE {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<LFSDETIE> for bool {
    #[inline(always)]
    fn from(variant: LFSDETIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable
pub type LFSDETIE_R = crate::BitReader<LFSDETIE>;
impl LFSDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LFSDETIE {
        match self.bits {
            false => LFSDETIE::Disabled,
            true => LFSDETIE::Enabled,
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFSDETIE::Disabled
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFSDETIE::Enabled
    }
}
///Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable
pub type LFSDETIE_W<'a, REG> = crate::BitWriter<'a, REG, LFSDETIE>;
impl<'a, REG> LFSDETIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFSDETIE::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFSDETIE::Enabled)
    }
}
impl R {
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IM")
            .field("lfsdetie", &self.lfsdetie())
            .field("afsdetie", &self.afsdetie())
            .field("cnrdyie", &self.cnrdyie())
            .field("freqie", &self.freqie())
            .field("wckcfgie", &self.wckcfgie())
            .field("mutedetie", &self.mutedetie())
            .field("ovrudrie", &self.ovrudrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<IMrs> {
        OVRUDRIE_W::new(self, 0)
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<IMrs> {
        MUTEDETIE_W::new(self, 1)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<IMrs> {
        WCKCFGIE_W::new(self, 2)
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    pub fn freqie(&mut self) -> FREQIE_W<IMrs> {
        FREQIE_W::new(self, 3)
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<IMrs> {
        CNRDYIE_W::new(self, 4)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<IMrs> {
        AFSDETIE_W::new(self, 5)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<IMrs> {
        LFSDETIE_W::new(self, 6)
    }
}
/**AInterrupt mask register2

You can [`read`](crate::Reg::read) this register and get [`im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IMrs;
impl crate::RegisterSpec for IMrs {
    type Ux = u32;
}
///`read()` method returns [`im::R`](R) reader structure
impl crate::Readable for IMrs {}
///`write(|w| ..)` method takes [`im::W`](W) writer structure
impl crate::Writable for IMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IM to value 0
impl crate::Resettable for IMrs {}
