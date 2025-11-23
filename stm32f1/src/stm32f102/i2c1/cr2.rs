///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `FREQ` reader - Peripheral clock frequency
pub type FREQ_R = crate::FieldReader;
///Field `FREQ` writer - Peripheral clock frequency
pub type FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITERREN {
    ///0: Error interrupt disabled
    Disabled = 0,
    ///1: Error interrupt enabled
    Enabled = 1,
}
impl From<ITERREN> for bool {
    #[inline(always)]
    fn from(variant: ITERREN) -> Self {
        variant as u8 != 0
    }
}
///Field `ITERREN` reader - Error interrupt enable
pub type ITERREN_R = crate::BitReader<ITERREN>;
impl ITERREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITERREN {
        match self.bits {
            false => ITERREN::Disabled,
            true => ITERREN::Enabled,
        }
    }
    ///Error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITERREN::Disabled
    }
    ///Error interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITERREN::Enabled
    }
}
///Field `ITERREN` writer - Error interrupt enable
pub type ITERREN_W<'a, REG> = crate::BitWriter<'a, REG, ITERREN>;
impl<'a, REG> ITERREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITERREN::Disabled)
    }
    ///Error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITERREN::Enabled)
    }
}
/**Event interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITEVTEN {
    ///0: Event interrupt disabled
    Disabled = 0,
    ///1: Event interrupt enabled
    Enabled = 1,
}
impl From<ITEVTEN> for bool {
    #[inline(always)]
    fn from(variant: ITEVTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ITEVTEN` reader - Event interrupt enable
pub type ITEVTEN_R = crate::BitReader<ITEVTEN>;
impl ITEVTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITEVTEN {
        match self.bits {
            false => ITEVTEN::Disabled,
            true => ITEVTEN::Enabled,
        }
    }
    ///Event interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITEVTEN::Disabled
    }
    ///Event interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITEVTEN::Enabled
    }
}
///Field `ITEVTEN` writer - Event interrupt enable
pub type ITEVTEN_W<'a, REG> = crate::BitWriter<'a, REG, ITEVTEN>;
impl<'a, REG> ITEVTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITEVTEN::Disabled)
    }
    ///Event interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITEVTEN::Enabled)
    }
}
/**Buffer interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITBUFEN {
    ///0: TxE=1 or RxNE=1 does not generate any interrupt
    Disabled = 0,
    ///1: TxE=1 or RxNE=1 generates Event interrupt
    Enabled = 1,
}
impl From<ITBUFEN> for bool {
    #[inline(always)]
    fn from(variant: ITBUFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ITBUFEN` reader - Buffer interrupt enable
pub type ITBUFEN_R = crate::BitReader<ITBUFEN>;
impl ITBUFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITBUFEN {
        match self.bits {
            false => ITBUFEN::Disabled,
            true => ITBUFEN::Enabled,
        }
    }
    ///TxE=1 or RxNE=1 does not generate any interrupt
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITBUFEN::Disabled
    }
    ///TxE=1 or RxNE=1 generates Event interrupt
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITBUFEN::Enabled
    }
}
///Field `ITBUFEN` writer - Buffer interrupt enable
pub type ITBUFEN_W<'a, REG> = crate::BitWriter<'a, REG, ITBUFEN>;
impl<'a, REG> ITBUFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TxE=1 or RxNE=1 does not generate any interrupt
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITBUFEN::Disabled)
    }
    ///TxE=1 or RxNE=1 generates Event interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITBUFEN::Enabled)
    }
}
/**DMA requests enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: DMA requests disabled
    Disabled = 0,
    ///1: DMA request enabled when TxE=1 or RxNE=1
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA requests enable
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    ///DMA requests disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///DMA request enabled when TxE=1 or RxNE=1
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - DMA requests enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA requests disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///DMA request enabled when TxE=1 or RxNE=1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
/**DMA last transfer

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAST {
    ///0: Next DMA EOT is not the last transfer
    NotLast = 0,
    ///1: Next DMA EOT is the last transfer
    Last = 1,
}
impl From<LAST> for bool {
    #[inline(always)]
    fn from(variant: LAST) -> Self {
        variant as u8 != 0
    }
}
///Field `LAST` reader - DMA last transfer
pub type LAST_R = crate::BitReader<LAST>;
impl LAST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LAST {
        match self.bits {
            false => LAST::NotLast,
            true => LAST::Last,
        }
    }
    ///Next DMA EOT is not the last transfer
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == LAST::NotLast
    }
    ///Next DMA EOT is the last transfer
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == LAST::Last
    }
}
///Field `LAST` writer - DMA last transfer
pub type LAST_W<'a, REG> = crate::BitWriter<'a, REG, LAST>;
impl<'a, REG> LAST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Next DMA EOT is not the last transfer
    #[inline(always)]
    pub fn not_last(self) -> &'a mut crate::W<REG> {
        self.variant(LAST::NotLast)
    }
    ///Next DMA EOT is the last transfer
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(LAST::Last)
    }
}
impl R {
    ///Bits 0:5 - Peripheral clock frequency
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 8 - Error interrupt enable
    #[inline(always)]
    pub fn iterren(&self) -> ITERREN_R {
        ITERREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event interrupt enable
    #[inline(always)]
    pub fn itevten(&self) -> ITEVTEN_R {
        ITEVTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Buffer interrupt enable
    #[inline(always)]
    pub fn itbufen(&self) -> ITBUFEN_R {
        ITBUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA last transfer
    #[inline(always)]
    pub fn last(&self) -> LAST_R {
        LAST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("last", &self.last())
            .field("dmaen", &self.dmaen())
            .field("itbufen", &self.itbufen())
            .field("itevten", &self.itevten())
            .field("iterren", &self.iterren())
            .field("freq", &self.freq())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Peripheral clock frequency
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W<'_, CR2rs> {
        FREQ_W::new(self, 0)
    }
    ///Bit 8 - Error interrupt enable
    #[inline(always)]
    pub fn iterren(&mut self) -> ITERREN_W<'_, CR2rs> {
        ITERREN_W::new(self, 8)
    }
    ///Bit 9 - Event interrupt enable
    #[inline(always)]
    pub fn itevten(&mut self) -> ITEVTEN_W<'_, CR2rs> {
        ITEVTEN_W::new(self, 9)
    }
    ///Bit 10 - Buffer interrupt enable
    #[inline(always)]
    pub fn itbufen(&mut self) -> ITBUFEN_W<'_, CR2rs> {
        ITBUFEN_W::new(self, 10)
    }
    ///Bit 11 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CR2rs> {
        DMAEN_W::new(self, 11)
    }
    ///Bit 12 - DMA last transfer
    #[inline(always)]
    pub fn last(&mut self) -> LAST_W<'_, CR2rs> {
        LAST_W::new(self, 12)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#I2C1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
