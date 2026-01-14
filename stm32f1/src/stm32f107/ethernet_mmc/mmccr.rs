///Register `MMCCR` reader
pub type R = crate::R<MMCCRrs>;
///Register `MMCCR` writer
pub type W = crate::W<MMCCRrs>;
/**Counter reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CR {
    ///1: Reset all counters. Cleared automatically
    Reset = 1,
}
impl From<CR> for bool {
    #[inline(always)]
    fn from(variant: CR) -> Self {
        variant as u8 != 0
    }
}
///Field `CR` reader - Counter reset
pub type CR_R = crate::BitReader<CR>;
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CR> {
        match self.bits {
            true => Some(CR::Reset),
            _ => None,
        }
    }
    ///Reset all counters. Cleared automatically
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CR::Reset
    }
}
///Field `CR` writer - Counter reset
pub type CR_W<'a, REG> = crate::BitWriter<'a, REG, CR>;
impl<'a, REG> CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset all counters. Cleared automatically
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Reset)
    }
}
/**Counter stop rollover

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSR {
    ///0: Counters roll over to zero after reaching the maximum value
    Disabled = 0,
    ///1: Counters do not roll over to zero after reaching the maximum value
    Enabled = 1,
}
impl From<CSR> for bool {
    #[inline(always)]
    fn from(variant: CSR) -> Self {
        variant as u8 != 0
    }
}
///Field `CSR` reader - Counter stop rollover
pub type CSR_R = crate::BitReader<CSR>;
impl CSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSR {
        match self.bits {
            false => CSR::Disabled,
            true => CSR::Enabled,
        }
    }
    ///Counters roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSR::Disabled
    }
    ///Counters do not roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSR::Enabled
    }
}
///Field `CSR` writer - Counter stop rollover
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG, CSR>;
impl<'a, REG> CSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Counters roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSR::Disabled)
    }
    ///Counters do not roll over to zero after reaching the maximum value
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSR::Enabled)
    }
}
/**Reset on read

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROR {
    ///0: MMC counters do not reset on read
    Disabled = 0,
    ///1: MMC counters reset to zero after read
    Enabled = 1,
}
impl From<ROR> for bool {
    #[inline(always)]
    fn from(variant: ROR) -> Self {
        variant as u8 != 0
    }
}
///Field `ROR` reader - Reset on read
pub type ROR_R = crate::BitReader<ROR>;
impl ROR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROR {
        match self.bits {
            false => ROR::Disabled,
            true => ROR::Enabled,
        }
    }
    ///MMC counters do not reset on read
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROR::Disabled
    }
    ///MMC counters reset to zero after read
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROR::Enabled
    }
}
///Field `ROR` writer - Reset on read
pub type ROR_W<'a, REG> = crate::BitWriter<'a, REG, ROR>;
impl<'a, REG> ROR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MMC counters do not reset on read
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROR::Disabled)
    }
    ///MMC counters reset to zero after read
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROR::Enabled)
    }
}
/**MMC counter freeze

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCF {
    ///0: All MMC counters update normally
    Unfrozen = 0,
    ///1: All MMC counters frozen to their current value
    Frozen = 1,
}
impl From<MCF> for bool {
    #[inline(always)]
    fn from(variant: MCF) -> Self {
        variant as u8 != 0
    }
}
///Field `MCF` reader - MMC counter freeze
pub type MCF_R = crate::BitReader<MCF>;
impl MCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCF {
        match self.bits {
            false => MCF::Unfrozen,
            true => MCF::Frozen,
        }
    }
    ///All MMC counters update normally
    #[inline(always)]
    pub fn is_unfrozen(&self) -> bool {
        *self == MCF::Unfrozen
    }
    ///All MMC counters frozen to their current value
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == MCF::Frozen
    }
}
///Field `MCF` writer - MMC counter freeze
pub type MCF_W<'a, REG> = crate::BitWriter<'a, REG, MCF>;
impl<'a, REG> MCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All MMC counters update normally
    #[inline(always)]
    pub fn unfrozen(self) -> &'a mut crate::W<REG> {
        self.variant(MCF::Unfrozen)
    }
    ///All MMC counters frozen to their current value
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(MCF::Frozen)
    }
}
impl R {
    ///Bit 0 - Counter reset
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 31 - MMC counter freeze
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCCR")
            .field("cr", &self.cr())
            .field("csr", &self.csr())
            .field("ror", &self.ror())
            .field("mcf", &self.mcf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Counter reset
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, MMCCRrs> {
        CR_W::new(self, 0)
    }
    ///Bit 1 - Counter stop rollover
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<'_, MMCCRrs> {
        CSR_W::new(self, 1)
    }
    ///Bit 2 - Reset on read
    #[inline(always)]
    pub fn ror(&mut self) -> ROR_W<'_, MMCCRrs> {
        ROR_W::new(self, 2)
    }
    ///Bit 31 - MMC counter freeze
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W<'_, MMCCRrs> {
        MCF_W::new(self, 31)
    }
}
/**Ethernet MMC control register (ETH_MMCCR)

You can [`read`](crate::Reg::read) this register and get [`mmccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#Ethernet_MMC:MMCCR)*/
pub struct MMCCRrs;
impl crate::RegisterSpec for MMCCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmccr::R`](R) reader structure
impl crate::Readable for MMCCRrs {}
///`write(|w| ..)` method takes [`mmccr::W`](W) writer structure
impl crate::Writable for MMCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMCCR to value 0
impl crate::Resettable for MMCCRrs {}
