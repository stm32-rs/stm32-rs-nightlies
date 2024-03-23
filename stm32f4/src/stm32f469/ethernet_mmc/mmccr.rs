#[doc = "Register `MMCCR` reader"]
pub type R = crate::R<MMCCRrs>;
#[doc = "Register `MMCCR` writer"]
pub type W = crate::W<MMCCRrs>;
#[doc = "Counter reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CR {
    #[doc = "1: Reset all counters. Cleared automatically"]
    Reset = 1,
}
impl From<CR> for bool {
    #[inline(always)]
    fn from(variant: CR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CR` reader - Counter reset"]
pub type CR_R = crate::BitReader<CR>;
impl CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CR> {
        match self.bits {
            true => Some(CR::Reset),
            _ => None,
        }
    }
    #[doc = "Reset all counters. Cleared automatically"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CR::Reset
    }
}
#[doc = "Field `CR` writer - Counter reset"]
pub type CR_W<'a, REG> = crate::BitWriter<'a, REG, CR>;
impl<'a, REG> CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset all counters. Cleared automatically"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CR::Reset)
    }
}
#[doc = "Counter stop rollover\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSR {
    #[doc = "0: Counters roll over to zero after reaching the maximum value"]
    Disabled = 0,
    #[doc = "1: Counters do not roll over to zero after reaching the maximum value"]
    Enabled = 1,
}
impl From<CSR> for bool {
    #[inline(always)]
    fn from(variant: CSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSR` reader - Counter stop rollover"]
pub type CSR_R = crate::BitReader<CSR>;
impl CSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSR {
        match self.bits {
            false => CSR::Disabled,
            true => CSR::Enabled,
        }
    }
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSR::Disabled
    }
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSR::Enabled
    }
}
#[doc = "Field `CSR` writer - Counter stop rollover"]
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG, CSR>;
impl<'a, REG> CSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSR::Disabled)
    }
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSR::Enabled)
    }
}
#[doc = "Reset on read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROR {
    #[doc = "0: MMC counters do not reset on read"]
    Disabled = 0,
    #[doc = "1: MMC counters reset to zero after read"]
    Enabled = 1,
}
impl From<ROR> for bool {
    #[inline(always)]
    fn from(variant: ROR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROR` reader - Reset on read"]
pub type ROR_R = crate::BitReader<ROR>;
impl ROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROR {
        match self.bits {
            false => ROR::Disabled,
            true => ROR::Enabled,
        }
    }
    #[doc = "MMC counters do not reset on read"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROR::Disabled
    }
    #[doc = "MMC counters reset to zero after read"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROR::Enabled
    }
}
#[doc = "Field `ROR` writer - Reset on read"]
pub type ROR_W<'a, REG> = crate::BitWriter<'a, REG, ROR>;
impl<'a, REG> ROR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MMC counters do not reset on read"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROR::Disabled)
    }
    #[doc = "MMC counters reset to zero after read"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROR::Enabled)
    }
}
#[doc = "MMC counter freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCF {
    #[doc = "0: All MMC counters update normally"]
    Unfrozen = 0,
    #[doc = "1: All MMC counters frozen to their current value"]
    Frozen = 1,
}
impl From<MCF> for bool {
    #[inline(always)]
    fn from(variant: MCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCF` reader - MMC counter freeze"]
pub type MCF_R = crate::BitReader<MCF>;
impl MCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCF {
        match self.bits {
            false => MCF::Unfrozen,
            true => MCF::Frozen,
        }
    }
    #[doc = "All MMC counters update normally"]
    #[inline(always)]
    pub fn is_unfrozen(&self) -> bool {
        *self == MCF::Unfrozen
    }
    #[doc = "All MMC counters frozen to their current value"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == MCF::Frozen
    }
}
#[doc = "Field `MCF` writer - MMC counter freeze"]
pub type MCF_W<'a, REG> = crate::BitWriter<'a, REG, MCF>;
impl<'a, REG> MCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All MMC counters update normally"]
    #[inline(always)]
    pub fn unfrozen(self) -> &'a mut crate::W<REG> {
        self.variant(MCF::Unfrozen)
    }
    #[doc = "All MMC counters frozen to their current value"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(MCF::Frozen)
    }
}
#[doc = "MMC counter preset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCP {
    #[doc = "1: MMC counters will be preset to almost full or almost half. Cleared automatically"]
    Preset = 1,
}
impl From<MCP> for bool {
    #[inline(always)]
    fn from(variant: MCP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCP` reader - MMC counter preset"]
pub type MCP_R = crate::BitReader<MCP>;
impl MCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCP> {
        match self.bits {
            true => Some(MCP::Preset),
            _ => None,
        }
    }
    #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
    #[inline(always)]
    pub fn is_preset(&self) -> bool {
        *self == MCP::Preset
    }
}
#[doc = "Field `MCP` writer - MMC counter preset"]
pub type MCP_W<'a, REG> = crate::BitWriter<'a, REG, MCP>;
impl<'a, REG> MCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
    #[inline(always)]
    pub fn preset(self) -> &'a mut crate::W<REG> {
        self.variant(MCP::Preset)
    }
}
#[doc = "MMC counter Full-Half preset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCFHP {
    #[doc = "0: When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    AlmostHalf = 0,
    #[doc = "1: When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    AlmostFull = 1,
}
impl From<MCFHP> for bool {
    #[inline(always)]
    fn from(variant: MCFHP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCFHP` reader - MMC counter Full-Half preset"]
pub type MCFHP_R = crate::BitReader<MCFHP>;
impl MCFHP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCFHP {
        match self.bits {
            false => MCFHP::AlmostHalf,
            true => MCFHP::AlmostFull,
        }
    }
    #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    #[inline(always)]
    pub fn is_almost_half(&self) -> bool {
        *self == MCFHP::AlmostHalf
    }
    #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    #[inline(always)]
    pub fn is_almost_full(&self) -> bool {
        *self == MCFHP::AlmostFull
    }
}
#[doc = "Field `MCFHP` writer - MMC counter Full-Half preset"]
pub type MCFHP_W<'a, REG> = crate::BitWriter<'a, REG, MCFHP>;
impl<'a, REG> MCFHP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    #[inline(always)]
    pub fn almost_half(self) -> &'a mut crate::W<REG> {
        self.variant(MCFHP::AlmostHalf)
    }
    #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    #[inline(always)]
    pub fn almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(MCFHP::AlmostFull)
    }
}
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC counter preset"]
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC counter Full-Half preset"]
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MMCCRrs> {
        CR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<MMCCRrs> {
        CSR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> ROR_W<MMCCRrs> {
        ROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - MMC counter freeze"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<MMCCRrs> {
        MCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - MMC counter preset"]
    #[inline(always)]
    #[must_use]
    pub fn mcp(&mut self) -> MCP_W<MMCCRrs> {
        MCP_W::new(self, 4)
    }
    #[doc = "Bit 5 - MMC counter Full-Half preset"]
    #[inline(always)]
    #[must_use]
    pub fn mcfhp(&mut self) -> MCFHP_W<MMCCRrs> {
        MCFHP_W::new(self, 5)
    }
}
#[doc = "Ethernet MMC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCCRrs;
impl crate::RegisterSpec for MMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmccr::R`](R) reader structure"]
impl crate::Readable for MMCCRrs {}
#[doc = "`write(|w| ..)` method takes [`mmccr::W`](W) writer structure"]
impl crate::Writable for MMCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCCR to value 0"]
impl crate::Resettable for MMCCRrs {
    const RESET_VALUE: u32 = 0;
}
