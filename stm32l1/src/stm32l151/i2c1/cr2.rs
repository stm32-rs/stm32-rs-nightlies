#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `FREQ` reader - Peripheral clock frequency"]
pub type FREQ_R = crate::FieldReader;
#[doc = "Field `FREQ` writer - Peripheral clock frequency"]
pub type FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITERREN {
    #[doc = "0: Error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt enabled"]
    Enabled = 1,
}
impl From<ITERREN> for bool {
    #[inline(always)]
    fn from(variant: ITERREN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITERREN` reader - Error interrupt enable"]
pub type ITERREN_R = crate::BitReader<ITERREN>;
impl ITERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITERREN {
        match self.bits {
            false => ITERREN::Disabled,
            true => ITERREN::Enabled,
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITERREN::Disabled
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITERREN::Enabled
    }
}
#[doc = "Field `ITERREN` writer - Error interrupt enable"]
pub type ITERREN_W<'a, REG> = crate::BitWriter<'a, REG, ITERREN>;
impl<'a, REG> ITERREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITERREN::Disabled)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITERREN::Enabled)
    }
}
#[doc = "Event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITEVTEN {
    #[doc = "0: Event interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Event interrupt enabled"]
    Enabled = 1,
}
impl From<ITEVTEN> for bool {
    #[inline(always)]
    fn from(variant: ITEVTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITEVTEN` reader - Event interrupt enable"]
pub type ITEVTEN_R = crate::BitReader<ITEVTEN>;
impl ITEVTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEVTEN {
        match self.bits {
            false => ITEVTEN::Disabled,
            true => ITEVTEN::Enabled,
        }
    }
    #[doc = "Event interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITEVTEN::Disabled
    }
    #[doc = "Event interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITEVTEN::Enabled
    }
}
#[doc = "Field `ITEVTEN` writer - Event interrupt enable"]
pub type ITEVTEN_W<'a, REG> = crate::BitWriter<'a, REG, ITEVTEN>;
impl<'a, REG> ITEVTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITEVTEN::Disabled)
    }
    #[doc = "Event interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITEVTEN::Enabled)
    }
}
#[doc = "Buffer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITBUFEN {
    #[doc = "0: TxE=1 or RxNE=1 does not generate any interrupt"]
    Disabled = 0,
    #[doc = "1: TxE=1 or RxNE=1 generates Event interrupt"]
    Enabled = 1,
}
impl From<ITBUFEN> for bool {
    #[inline(always)]
    fn from(variant: ITBUFEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITBUFEN` reader - Buffer interrupt enable"]
pub type ITBUFEN_R = crate::BitReader<ITBUFEN>;
impl ITBUFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITBUFEN {
        match self.bits {
            false => ITBUFEN::Disabled,
            true => ITBUFEN::Enabled,
        }
    }
    #[doc = "TxE=1 or RxNE=1 does not generate any interrupt"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITBUFEN::Disabled
    }
    #[doc = "TxE=1 or RxNE=1 generates Event interrupt"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITBUFEN::Enabled
    }
}
#[doc = "Field `ITBUFEN` writer - Buffer interrupt enable"]
pub type ITBUFEN_W<'a, REG> = crate::BitWriter<'a, REG, ITBUFEN>;
impl<'a, REG> ITBUFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TxE=1 or RxNE=1 does not generate any interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITBUFEN::Disabled)
    }
    #[doc = "TxE=1 or RxNE=1 generates Event interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITBUFEN::Enabled)
    }
}
#[doc = "DMA requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    #[doc = "0: DMA requests disabled"]
    Disabled = 0,
    #[doc = "1: DMA request enabled when TxE=1 or RxNE=1"]
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA requests enable"]
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    #[doc = "DMA requests disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    #[doc = "DMA request enabled when TxE=1 or RxNE=1"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA requests enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA requests disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    #[doc = "DMA request enabled when TxE=1 or RxNE=1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
#[doc = "DMA last transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAST {
    #[doc = "0: Next DMA EOT is not the last transfer"]
    NotLast = 0,
    #[doc = "1: Next DMA EOT is the last transfer"]
    Last = 1,
}
impl From<LAST> for bool {
    #[inline(always)]
    fn from(variant: LAST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAST` reader - DMA last transfer"]
pub type LAST_R = crate::BitReader<LAST>;
impl LAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LAST {
        match self.bits {
            false => LAST::NotLast,
            true => LAST::Last,
        }
    }
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == LAST::NotLast
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == LAST::Last
    }
}
#[doc = "Field `LAST` writer - DMA last transfer"]
pub type LAST_W<'a, REG> = crate::BitWriter<'a, REG, LAST>;
impl<'a, REG> LAST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn not_last(self) -> &'a mut crate::W<REG> {
        self.variant(LAST::NotLast)
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(LAST::Last)
    }
}
impl R {
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn iterren(&self) -> ITERREN_R {
        ITERREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn itevten(&self) -> ITEVTEN_R {
        ITEVTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn itbufen(&self) -> ITBUFEN_R {
        ITBUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    pub fn last(&self) -> LAST_R {
        LAST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<CR2rs> {
        FREQ_W::new(self, 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iterren(&mut self) -> ITERREN_W<CR2rs> {
        ITERREN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itevten(&mut self) -> ITEVTEN_W<CR2rs> {
        ITEVTEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itbufen(&mut self) -> ITBUFEN_W<CR2rs> {
        ITBUFEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CR2rs> {
        DMAEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    #[must_use]
    pub fn last(&mut self) -> LAST_W<CR2rs> {
        LAST_W::new(self, 12)
    }
}
#[doc = "CR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
