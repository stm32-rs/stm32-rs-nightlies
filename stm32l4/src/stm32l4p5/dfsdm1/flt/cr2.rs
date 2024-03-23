#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Injected end of conversion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE {
    #[doc = "0: Injected end of conversion interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Injected end of conversion interrupt is enabled"]
    Enabled = 1,
}
impl From<JEOCIE> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOCIE` reader - Injected end of conversion interrupt enable"]
pub type JEOCIE_R = crate::BitReader<JEOCIE>;
impl JEOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOCIE {
        match self.bits {
            false => JEOCIE::Disabled,
            true => JEOCIE::Enabled,
        }
    }
    #[doc = "Injected end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE::Disabled
    }
    #[doc = "Injected end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE::Enabled
    }
}
#[doc = "Field `JEOCIE` writer - Injected end of conversion interrupt enable"]
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOCIE>;
impl<'a, REG> JEOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Disabled)
    }
    #[doc = "Injected end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Enabled)
    }
}
#[doc = "Regular end of conversion interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REOCIE {
    #[doc = "0: Regular end of conversion interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Regular end of conversion interrupt is enabled"]
    Enabled = 1,
}
impl From<REOCIE> for bool {
    #[inline(always)]
    fn from(variant: REOCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REOCIE` reader - Regular end of conversion interrupt enable"]
pub type REOCIE_R = crate::BitReader<REOCIE>;
impl REOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REOCIE {
        match self.bits {
            false => REOCIE::Disabled,
            true => REOCIE::Enabled,
        }
    }
    #[doc = "Regular end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REOCIE::Disabled
    }
    #[doc = "Regular end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REOCIE::Enabled
    }
}
#[doc = "Field `REOCIE` writer - Regular end of conversion interrupt enable"]
pub type REOCIE_W<'a, REG> = crate::BitWriter<'a, REG, REOCIE>;
impl<'a, REG> REOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REOCIE::Disabled)
    }
    #[doc = "Regular end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REOCIE::Enabled)
    }
}
#[doc = "Injected data overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVRIE {
    #[doc = "0: Injected data overrun interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Injected data overrun interrupt is enabled"]
    Enabled = 1,
}
impl From<JOVRIE> for bool {
    #[inline(always)]
    fn from(variant: JOVRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JOVRIE` reader - Injected data overrun interrupt enable"]
pub type JOVRIE_R = crate::BitReader<JOVRIE>;
impl JOVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JOVRIE {
        match self.bits {
            false => JOVRIE::Disabled,
            true => JOVRIE::Enabled,
        }
    }
    #[doc = "Injected data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVRIE::Disabled
    }
    #[doc = "Injected data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVRIE::Enabled
    }
}
#[doc = "Field `JOVRIE` writer - Injected data overrun interrupt enable"]
pub type JOVRIE_W<'a, REG> = crate::BitWriter<'a, REG, JOVRIE>;
impl<'a, REG> JOVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVRIE::Disabled)
    }
    #[doc = "Injected data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVRIE::Enabled)
    }
}
#[doc = "Regular data overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVRIE {
    #[doc = "0: Regular data overrun interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Regular data overrun interrupt is enabled"]
    Enabled = 1,
}
impl From<ROVRIE> for bool {
    #[inline(always)]
    fn from(variant: ROVRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVRIE` reader - Regular data overrun interrupt enable"]
pub type ROVRIE_R = crate::BitReader<ROVRIE>;
impl ROVRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROVRIE {
        match self.bits {
            false => ROVRIE::Disabled,
            true => ROVRIE::Enabled,
        }
    }
    #[doc = "Regular data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVRIE::Disabled
    }
    #[doc = "Regular data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVRIE::Enabled
    }
}
#[doc = "Field `ROVRIE` writer - Regular data overrun interrupt enable"]
pub type ROVRIE_W<'a, REG> = crate::BitWriter<'a, REG, ROVRIE>;
impl<'a, REG> ROVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVRIE::Disabled)
    }
    #[doc = "Regular data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVRIE::Enabled)
    }
}
#[doc = "Analog watchdog interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE {
    #[doc = "0: Analog watchdog interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog interrupt is enabled"]
    Enabled = 1,
}
impl From<AWDIE> for bool {
    #[inline(always)]
    fn from(variant: AWDIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader<AWDIE>;
impl AWDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWDIE {
        match self.bits {
            false => AWDIE::Disabled,
            true => AWDIE::Enabled,
        }
    }
    #[doc = "Analog watchdog interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE::Disabled
    }
    #[doc = "Analog watchdog interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE::Enabled
    }
}
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG, AWDIE>;
impl<'a, REG> AWDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Disabled)
    }
    #[doc = "Analog watchdog interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Enabled)
    }
}
#[doc = "Short-circuit detector interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCDIE {
    #[doc = "0: Short-circuit detector interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Short-circuit detector interrupt is enabled"]
    Enabled = 1,
}
impl From<SCDIE> for bool {
    #[inline(always)]
    fn from(variant: SCDIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCDIE` reader - Short-circuit detector interrupt enable"]
pub type SCDIE_R = crate::BitReader<SCDIE>;
impl SCDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCDIE {
        match self.bits {
            false => SCDIE::Disabled,
            true => SCDIE::Enabled,
        }
    }
    #[doc = "Short-circuit detector interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCDIE::Disabled
    }
    #[doc = "Short-circuit detector interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCDIE::Enabled
    }
}
#[doc = "Field `SCDIE` writer - Short-circuit detector interrupt enable"]
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG, SCDIE>;
impl<'a, REG> SCDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short-circuit detector interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDIE::Disabled)
    }
    #[doc = "Short-circuit detector interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDIE::Enabled)
    }
}
#[doc = "Clock absence interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKABIE {
    #[doc = "0: Detection of channel input clock absence interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Detection of channel input clock absence interrupt is enabled"]
    Enabled = 1,
}
impl From<CKABIE> for bool {
    #[inline(always)]
    fn from(variant: CKABIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKABIE` reader - Clock absence interrupt enable"]
pub type CKABIE_R = crate::BitReader<CKABIE>;
impl CKABIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKABIE {
        match self.bits {
            false => CKABIE::Disabled,
            true => CKABIE::Enabled,
        }
    }
    #[doc = "Detection of channel input clock absence interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKABIE::Disabled
    }
    #[doc = "Detection of channel input clock absence interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKABIE::Enabled
    }
}
#[doc = "Field `CKABIE` writer - Clock absence interrupt enable"]
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG, CKABIE>;
impl<'a, REG> CKABIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection of channel input clock absence interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABIE::Disabled)
    }
    #[doc = "Detection of channel input clock absence interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABIE::Enabled)
    }
}
#[doc = "Extremes detector channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXCH {
    #[doc = "0: Extremes detector does not accept data from channel y"]
    Disabled = 0,
    #[doc = "1: Extremes detector accepts data from channel y"]
    Enabled = 1,
}
impl From<EXCH> for u8 {
    #[inline(always)]
    fn from(variant: EXCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXCH {
    type Ux = u8;
}
#[doc = "Field `EXCH` reader - Extremes detector channel selection"]
pub type EXCH_R = crate::FieldReader<EXCH>;
impl EXCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXCH> {
        match self.bits {
            0 => Some(EXCH::Disabled),
            1 => Some(EXCH::Enabled),
            _ => None,
        }
    }
    #[doc = "Extremes detector does not accept data from channel y"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXCH::Disabled
    }
    #[doc = "Extremes detector accepts data from channel y"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXCH::Enabled
    }
}
#[doc = "Field `EXCH` writer - Extremes detector channel selection"]
pub type EXCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXCH>;
impl<'a, REG> EXCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Extremes detector does not accept data from channel y"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXCH::Disabled)
    }
    #[doc = "Extremes detector accepts data from channel y"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXCH::Enabled)
    }
}
#[doc = "Analog watchdog channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWDCH {
    #[doc = "0: Analog watchdog is disabled on channel y"]
    Disabled = 0,
    #[doc = "1: Analog watchdog is enabled on channel y"]
    Enabled = 1,
}
impl From<AWDCH> for u8 {
    #[inline(always)]
    fn from(variant: AWDCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWDCH {
    type Ux = u8;
}
#[doc = "Field `AWDCH` reader - Analog watchdog channel selection"]
pub type AWDCH_R = crate::FieldReader<AWDCH>;
impl AWDCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWDCH> {
        match self.bits {
            0 => Some(AWDCH::Disabled),
            1 => Some(AWDCH::Enabled),
            _ => None,
        }
    }
    #[doc = "Analog watchdog is disabled on channel y"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDCH::Disabled
    }
    #[doc = "Analog watchdog is enabled on channel y"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDCH::Enabled
    }
}
#[doc = "Field `AWDCH` writer - Analog watchdog channel selection"]
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, AWDCH>;
impl<'a, REG> AWDCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog watchdog is disabled on channel y"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDCH::Disabled)
    }
    #[doc = "Analog watchdog is enabled on channel y"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDCH::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<CR2rs> {
        JEOCIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reocie(&mut self) -> REOCIE_W<CR2rs> {
        REOCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jovrie(&mut self) -> JOVRIE_W<CR2rs> {
        JOVRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rovrie(&mut self) -> ROVRIE_W<CR2rs> {
        ROVRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<CR2rs> {
        AWDIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn scdie(&mut self) -> SCDIE_W<CR2rs> {
        SCDIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<CR2rs> {
        CKABIE_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn exch(&mut self) -> EXCH_W<CR2rs> {
        EXCH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<CR2rs> {
        AWDCH_W::new(self, 16)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
