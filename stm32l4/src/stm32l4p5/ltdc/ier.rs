#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIE {
    #[doc = "0: Line interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Line interrupt enabled"]
    Enabled = 1,
}
impl From<LIE> for bool {
    #[inline(always)]
    fn from(variant: LIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIE` reader - Line Interrupt Enable"]
pub type LIE_R = crate::BitReader<LIE>;
impl LIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LIE {
        match self.bits {
            false => LIE::Disabled,
            true => LIE::Enabled,
        }
    }
    #[doc = "Line interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LIE::Disabled
    }
    #[doc = "Line interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LIE::Enabled
    }
}
#[doc = "Field `LIE` writer - Line Interrupt Enable"]
pub type LIE_W<'a, REG> = crate::BitWriter<'a, REG, LIE>;
impl<'a, REG> LIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Line interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LIE::Disabled)
    }
    #[doc = "Line interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LIE::Enabled)
    }
}
#[doc = "FIFO Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUIE {
    #[doc = "0: FIFO underrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: FIFO underrun interrupt enabled"]
    Enabled = 1,
}
impl From<FUIE> for bool {
    #[inline(always)]
    fn from(variant: FUIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUIE` reader - FIFO Underrun Interrupt Enable"]
pub type FUIE_R = crate::BitReader<FUIE>;
impl FUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FUIE {
        match self.bits {
            false => FUIE::Disabled,
            true => FUIE::Enabled,
        }
    }
    #[doc = "FIFO underrun interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FUIE::Disabled
    }
    #[doc = "FIFO underrun interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FUIE::Enabled
    }
}
#[doc = "Field `FUIE` writer - FIFO Underrun Interrupt Enable"]
pub type FUIE_W<'a, REG> = crate::BitWriter<'a, REG, FUIE>;
impl<'a, REG> FUIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO underrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FUIE::Disabled)
    }
    #[doc = "FIFO underrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FUIE::Enabled)
    }
}
#[doc = "Transfer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIE {
    #[doc = "0: Transfer error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Transfer error interrupt enabled"]
    Enabled = 1,
}
impl From<TERRIE> for bool {
    #[inline(always)]
    fn from(variant: TERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRIE` reader - Transfer Error Interrupt Enable"]
pub type TERRIE_R = crate::BitReader<TERRIE>;
impl TERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TERRIE {
        match self.bits {
            false => TERRIE::Disabled,
            true => TERRIE::Enabled,
        }
    }
    #[doc = "Transfer error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TERRIE::Disabled
    }
    #[doc = "Transfer error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TERRIE::Enabled
    }
}
#[doc = "Field `TERRIE` writer - Transfer Error Interrupt Enable"]
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG, TERRIE>;
impl<'a, REG> TERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE::Disabled)
    }
    #[doc = "Transfer error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE::Enabled)
    }
}
#[doc = "Register Reload interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIE {
    #[doc = "0: Register reload interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Register reload interrupt enabled"]
    Enabled = 1,
}
impl From<RRIE> for bool {
    #[inline(always)]
    fn from(variant: RRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRIE` reader - Register Reload interrupt enable"]
pub type RRIE_R = crate::BitReader<RRIE>;
impl RRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RRIE {
        match self.bits {
            false => RRIE::Disabled,
            true => RRIE::Enabled,
        }
    }
    #[doc = "Register reload interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRIE::Disabled
    }
    #[doc = "Register reload interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRIE::Enabled
    }
}
#[doc = "Field `RRIE` writer - Register Reload interrupt enable"]
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG, RRIE>;
impl<'a, REG> RRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Register reload interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRIE::Disabled)
    }
    #[doc = "Register reload interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lie(&mut self) -> LIE_W<IERrs> {
        LIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fuie(&mut self) -> FUIE_W<IERrs> {
        FUIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<IERrs> {
        TERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<IERrs> {
        RRIE_W::new(self, 3)
    }
}
#[doc = "LTDC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
