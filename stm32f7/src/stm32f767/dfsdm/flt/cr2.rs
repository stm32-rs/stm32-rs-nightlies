///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Injected end of conversion interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCIE {
    ///0: Injected end of conversion interrupt is disabled
    Disabled = 0,
    ///1: Injected end of conversion interrupt is enabled
    Enabled = 1,
}
impl From<JEOCIE> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOCIE` reader - Injected end of conversion interrupt enable
pub type JEOCIE_R = crate::BitReader<JEOCIE>;
impl JEOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOCIE {
        match self.bits {
            false => JEOCIE::Disabled,
            true => JEOCIE::Enabled,
        }
    }
    ///Injected end of conversion interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEOCIE::Disabled
    }
    ///Injected end of conversion interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEOCIE::Enabled
    }
}
///Field `JEOCIE` writer - Injected end of conversion interrupt enable
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG, JEOCIE>;
impl<'a, REG> JEOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected end of conversion interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Disabled)
    }
    ///Injected end of conversion interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCIE::Enabled)
    }
}
/**Regular end of conversion interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REOCIE {
    ///0: Regular end of conversion interrupt is disabled
    Disabled = 0,
    ///1: Regular end of conversion interrupt is enabled
    Enabled = 1,
}
impl From<REOCIE> for bool {
    #[inline(always)]
    fn from(variant: REOCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `REOCIE` reader - Regular end of conversion interrupt enable
pub type REOCIE_R = crate::BitReader<REOCIE>;
impl REOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REOCIE {
        match self.bits {
            false => REOCIE::Disabled,
            true => REOCIE::Enabled,
        }
    }
    ///Regular end of conversion interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REOCIE::Disabled
    }
    ///Regular end of conversion interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REOCIE::Enabled
    }
}
///Field `REOCIE` writer - Regular end of conversion interrupt enable
pub type REOCIE_W<'a, REG> = crate::BitWriter<'a, REG, REOCIE>;
impl<'a, REG> REOCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular end of conversion interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REOCIE::Disabled)
    }
    ///Regular end of conversion interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REOCIE::Enabled)
    }
}
/**Injected data overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVRIE {
    ///0: Injected data overrun interrupt is disabled
    Disabled = 0,
    ///1: Injected data overrun interrupt is enabled
    Enabled = 1,
}
impl From<JOVRIE> for bool {
    #[inline(always)]
    fn from(variant: JOVRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `JOVRIE` reader - Injected data overrun interrupt enable
pub type JOVRIE_R = crate::BitReader<JOVRIE>;
impl JOVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JOVRIE {
        match self.bits {
            false => JOVRIE::Disabled,
            true => JOVRIE::Enabled,
        }
    }
    ///Injected data overrun interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVRIE::Disabled
    }
    ///Injected data overrun interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVRIE::Enabled
    }
}
///Field `JOVRIE` writer - Injected data overrun interrupt enable
pub type JOVRIE_W<'a, REG> = crate::BitWriter<'a, REG, JOVRIE>;
impl<'a, REG> JOVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected data overrun interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVRIE::Disabled)
    }
    ///Injected data overrun interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVRIE::Enabled)
    }
}
/**Regular data overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVRIE {
    ///0: Regular data overrun interrupt is disabled
    Disabled = 0,
    ///1: Regular data overrun interrupt is enabled
    Enabled = 1,
}
impl From<ROVRIE> for bool {
    #[inline(always)]
    fn from(variant: ROVRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVRIE` reader - Regular data overrun interrupt enable
pub type ROVRIE_R = crate::BitReader<ROVRIE>;
impl ROVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVRIE {
        match self.bits {
            false => ROVRIE::Disabled,
            true => ROVRIE::Enabled,
        }
    }
    ///Regular data overrun interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVRIE::Disabled
    }
    ///Regular data overrun interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVRIE::Enabled
    }
}
///Field `ROVRIE` writer - Regular data overrun interrupt enable
pub type ROVRIE_W<'a, REG> = crate::BitWriter<'a, REG, ROVRIE>;
impl<'a, REG> ROVRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular data overrun interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVRIE::Disabled)
    }
    ///Regular data overrun interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVRIE::Enabled)
    }
}
/**Analog watchdog interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE {
    ///0: Analog watchdog interrupt is disabled
    Disabled = 0,
    ///1: Analog watchdog interrupt is enabled
    Enabled = 1,
}
impl From<AWDIE> for bool {
    #[inline(always)]
    fn from(variant: AWDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub type AWDIE_R = crate::BitReader<AWDIE>;
impl AWDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDIE {
        match self.bits {
            false => AWDIE::Disabled,
            true => AWDIE::Enabled,
        }
    }
    ///Analog watchdog interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE::Disabled
    }
    ///Analog watchdog interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE::Enabled
    }
}
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG, AWDIE>;
impl<'a, REG> AWDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Disabled)
    }
    ///Analog watchdog interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDIE::Enabled)
    }
}
/**Short-circuit detector interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCDIE {
    ///0: Short-circuit detector interrupt is disabled
    Disabled = 0,
    ///1: Short-circuit detector interrupt is enabled
    Enabled = 1,
}
impl From<SCDIE> for bool {
    #[inline(always)]
    fn from(variant: SCDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SCDIE` reader - Short-circuit detector interrupt enable
pub type SCDIE_R = crate::BitReader<SCDIE>;
impl SCDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCDIE {
        match self.bits {
            false => SCDIE::Disabled,
            true => SCDIE::Enabled,
        }
    }
    ///Short-circuit detector interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCDIE::Disabled
    }
    ///Short-circuit detector interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCDIE::Enabled
    }
}
///Field `SCDIE` writer - Short-circuit detector interrupt enable
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG, SCDIE>;
impl<'a, REG> SCDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Short-circuit detector interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDIE::Disabled)
    }
    ///Short-circuit detector interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDIE::Enabled)
    }
}
/**Clock absence interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKABIE {
    ///0: Detection of channel input clock absence interrupt is disabled
    Disabled = 0,
    ///1: Detection of channel input clock absence interrupt is enabled
    Enabled = 1,
}
impl From<CKABIE> for bool {
    #[inline(always)]
    fn from(variant: CKABIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CKABIE` reader - Clock absence interrupt enable
pub type CKABIE_R = crate::BitReader<CKABIE>;
impl CKABIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKABIE {
        match self.bits {
            false => CKABIE::Disabled,
            true => CKABIE::Enabled,
        }
    }
    ///Detection of channel input clock absence interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKABIE::Disabled
    }
    ///Detection of channel input clock absence interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKABIE::Enabled
    }
}
///Field `CKABIE` writer - Clock absence interrupt enable
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG, CKABIE>;
impl<'a, REG> CKABIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Detection of channel input clock absence interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABIE::Disabled)
    }
    ///Detection of channel input clock absence interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABIE::Enabled)
    }
}
/**Extremes detector channel selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXCH {
    ///0: Extremes detector does not accept data from channel y
    Disabled = 0,
    ///1: Extremes detector accepts data from channel y
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
impl crate::IsEnum for EXCH {}
///Field `EXCH` reader - Extremes detector channel selection
pub type EXCH_R = crate::FieldReader<EXCH>;
impl EXCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXCH> {
        match self.bits {
            0 => Some(EXCH::Disabled),
            1 => Some(EXCH::Enabled),
            _ => None,
        }
    }
    ///Extremes detector does not accept data from channel y
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXCH::Disabled
    }
    ///Extremes detector accepts data from channel y
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXCH::Enabled
    }
}
///Field `EXCH` writer - Extremes detector channel selection
pub type EXCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXCH>;
impl<'a, REG> EXCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Extremes detector does not accept data from channel y
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXCH::Disabled)
    }
    ///Extremes detector accepts data from channel y
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXCH::Enabled)
    }
}
/**Analog watchdog channel selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWDCH {
    ///0: Analog watchdog is disabled on channel y
    Disabled = 0,
    ///1: Analog watchdog is enabled on channel y
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
impl crate::IsEnum for AWDCH {}
///Field `AWDCH` reader - Analog watchdog channel selection
pub type AWDCH_R = crate::FieldReader<AWDCH>;
impl AWDCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWDCH> {
        match self.bits {
            0 => Some(AWDCH::Disabled),
            1 => Some(AWDCH::Enabled),
            _ => None,
        }
    }
    ///Analog watchdog is disabled on channel y
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDCH::Disabled
    }
    ///Analog watchdog is enabled on channel y
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDCH::Enabled
    }
}
///Field `AWDCH` writer - Analog watchdog channel selection
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, AWDCH>;
impl<'a, REG> AWDCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Analog watchdog is disabled on channel y
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDCH::Disabled)
    }
    ///Analog watchdog is enabled on channel y
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDCH::Enabled)
    }
}
impl R {
    ///Bit 0 - Injected end of conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular end of conversion interrupt enable
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Regular data overrun interrupt enable
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Short-circuit detector interrupt enable
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clock absence interrupt enable
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:15 - Extremes detector channel selection
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("jeocie", &self.jeocie())
            .field("reocie", &self.reocie())
            .field("jovrie", &self.jovrie())
            .field("rovrie", &self.rovrie())
            .field("awdie", &self.awdie())
            .field("scdie", &self.scdie())
            .field("ckabie", &self.ckabie())
            .field("exch", &self.exch())
            .field("awdch", &self.awdch())
            .finish()
    }
}
impl W {
    ///Bit 0 - Injected end of conversion interrupt enable
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<'_, CR2rs> {
        JEOCIE_W::new(self, 0)
    }
    ///Bit 1 - Regular end of conversion interrupt enable
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W<'_, CR2rs> {
        REOCIE_W::new(self, 1)
    }
    ///Bit 2 - Injected data overrun interrupt enable
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W<'_, CR2rs> {
        JOVRIE_W::new(self, 2)
    }
    ///Bit 3 - Regular data overrun interrupt enable
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W<'_, CR2rs> {
        ROVRIE_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W<'_, CR2rs> {
        AWDIE_W::new(self, 4)
    }
    ///Bit 5 - Short-circuit detector interrupt enable
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W<'_, CR2rs> {
        SCDIE_W::new(self, 5)
    }
    ///Bit 6 - Clock absence interrupt enable
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W<'_, CR2rs> {
        CKABIE_W::new(self, 6)
    }
    ///Bits 8:15 - Extremes detector channel selection
    #[inline(always)]
    pub fn exch(&mut self) -> EXCH_W<'_, CR2rs> {
        EXCH_W::new(self, 8)
    }
    ///Bits 16:23 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<'_, CR2rs> {
        AWDCH_W::new(self, 16)
    }
}
/**DFSDM control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
