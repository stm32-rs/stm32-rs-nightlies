#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMDrs>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMDrs>;
#[doc = "Field `CMDINDEX` reader - Command index"]
pub type CMDINDEX_R = crate::FieldReader;
#[doc = "Field `CMDINDEX` writer - Command index"]
pub type CMDINDEX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Wait for response bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAITRESP {
    #[doc = "0: No response"]
    NoResponse = 0,
    #[doc = "1: Short response"]
    ShortResponse = 1,
    #[doc = "2: No reponse"]
    NoResponse2 = 2,
    #[doc = "3: Long reponse"]
    LongResponse = 3,
}
impl From<WAITRESP> for u8 {
    #[inline(always)]
    fn from(variant: WAITRESP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WAITRESP {
    type Ux = u8;
}
#[doc = "Field `WAITRESP` reader - Wait for response bits"]
pub type WAITRESP_R = crate::FieldReader<WAITRESP>;
impl WAITRESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAITRESP {
        match self.bits {
            0 => WAITRESP::NoResponse,
            1 => WAITRESP::ShortResponse,
            2 => WAITRESP::NoResponse2,
            3 => WAITRESP::LongResponse,
            _ => unreachable!(),
        }
    }
    #[doc = "No response"]
    #[inline(always)]
    pub fn is_no_response(&self) -> bool {
        *self == WAITRESP::NoResponse
    }
    #[doc = "Short response"]
    #[inline(always)]
    pub fn is_short_response(&self) -> bool {
        *self == WAITRESP::ShortResponse
    }
    #[doc = "No reponse"]
    #[inline(always)]
    pub fn is_no_response2(&self) -> bool {
        *self == WAITRESP::NoResponse2
    }
    #[doc = "Long reponse"]
    #[inline(always)]
    pub fn is_long_response(&self) -> bool {
        *self == WAITRESP::LongResponse
    }
}
#[doc = "Field `WAITRESP` writer - Wait for response bits"]
pub type WAITRESP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WAITRESP>;
impl<'a, REG> WAITRESP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No response"]
    #[inline(always)]
    pub fn no_response(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::NoResponse)
    }
    #[doc = "Short response"]
    #[inline(always)]
    pub fn short_response(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::ShortResponse)
    }
    #[doc = "No reponse"]
    #[inline(always)]
    pub fn no_response2(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::NoResponse2)
    }
    #[doc = "Long reponse"]
    #[inline(always)]
    pub fn long_response(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::LongResponse)
    }
}
#[doc = "CPSM waits for interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITINT {
    #[doc = "0: Don't wait for interrupt request"]
    Disabled = 0,
    #[doc = "1: Wait for interrupt request"]
    Enabled = 1,
}
impl From<WAITINT> for bool {
    #[inline(always)]
    fn from(variant: WAITINT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITINT` reader - CPSM waits for interrupt request"]
pub type WAITINT_R = crate::BitReader<WAITINT>;
impl WAITINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAITINT {
        match self.bits {
            false => WAITINT::Disabled,
            true => WAITINT::Enabled,
        }
    }
    #[doc = "Don't wait for interrupt request"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITINT::Disabled
    }
    #[doc = "Wait for interrupt request"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITINT::Enabled
    }
}
#[doc = "Field `WAITINT` writer - CPSM waits for interrupt request"]
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG, WAITINT>;
impl<'a, REG> WAITINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't wait for interrupt request"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITINT::Disabled)
    }
    #[doc = "Wait for interrupt request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITINT::Enabled)
    }
}
#[doc = "CPSM Waits for ends of data transfer (CmdPend internal signal).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITPEND {
    #[doc = "0: Don't wait for data end"]
    Disabled = 0,
    #[doc = "1: Wait for end of data transfer signal before sending command"]
    Enabled = 1,
}
impl From<WAITPEND> for bool {
    #[inline(always)]
    fn from(variant: WAITPEND) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITPEND` reader - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
pub type WAITPEND_R = crate::BitReader<WAITPEND>;
impl WAITPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAITPEND {
        match self.bits {
            false => WAITPEND::Disabled,
            true => WAITPEND::Enabled,
        }
    }
    #[doc = "Don't wait for data end"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITPEND::Disabled
    }
    #[doc = "Wait for end of data transfer signal before sending command"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITPEND::Enabled
    }
}
#[doc = "Field `WAITPEND` writer - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG, WAITPEND>;
impl<'a, REG> WAITPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't wait for data end"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPEND::Disabled)
    }
    #[doc = "Wait for end of data transfer signal before sending command"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPEND::Enabled)
    }
}
#[doc = "Command path state machine (CPSM) Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPSMEN {
    #[doc = "0: Command path state machine disabled"]
    Disabled = 0,
    #[doc = "1: Command path state machine enabled"]
    Enabled = 1,
}
impl From<CPSMEN> for bool {
    #[inline(always)]
    fn from(variant: CPSMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_R = crate::BitReader<CPSMEN>;
impl CPSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPSMEN {
        match self.bits {
            false => CPSMEN::Disabled,
            true => CPSMEN::Enabled,
        }
    }
    #[doc = "Command path state machine disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPSMEN::Disabled
    }
    #[doc = "Command path state machine enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPSMEN::Enabled
    }
}
#[doc = "Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CPSMEN>;
impl<'a, REG> CPSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command path state machine disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPSMEN::Disabled)
    }
    #[doc = "Command path state machine enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPSMEN::Enabled)
    }
}
#[doc = "SD I/O suspend command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOSUSPEND {
    #[doc = "0: Next command is not a SDIO suspend command"]
    Disabled = 0,
    #[doc = "1: Next command send is a SDIO suspend command"]
    Enabled = 1,
}
impl From<SDIOSUSPEND> for bool {
    #[inline(always)]
    fn from(variant: SDIOSUSPEND) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOSuspend` reader - SD I/O suspend command"]
pub type SDIOSUSPEND_R = crate::BitReader<SDIOSUSPEND>;
impl SDIOSUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDIOSUSPEND {
        match self.bits {
            false => SDIOSUSPEND::Disabled,
            true => SDIOSUSPEND::Enabled,
        }
    }
    #[doc = "Next command is not a SDIO suspend command"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOSUSPEND::Disabled
    }
    #[doc = "Next command send is a SDIO suspend command"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOSUSPEND::Enabled
    }
}
#[doc = "Field `SDIOSuspend` writer - SD I/O suspend command"]
pub type SDIOSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG, SDIOSUSPEND>;
impl<'a, REG> SDIOSUSPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next command is not a SDIO suspend command"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSUSPEND::Disabled)
    }
    #[doc = "Next command send is a SDIO suspend command"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSUSPEND::Enabled)
    }
}
#[doc = "Enable CMD completion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENCMDCOMPL {
    #[doc = "0: Command complete signal disabled"]
    Disabled = 0,
    #[doc = "1: Command complete signal enabled"]
    Enabled = 1,
}
impl From<ENCMDCOMPL> for bool {
    #[inline(always)]
    fn from(variant: ENCMDCOMPL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENCMDcompl` reader - Enable CMD completion"]
pub type ENCMDCOMPL_R = crate::BitReader<ENCMDCOMPL>;
impl ENCMDCOMPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENCMDCOMPL {
        match self.bits {
            false => ENCMDCOMPL::Disabled,
            true => ENCMDCOMPL::Enabled,
        }
    }
    #[doc = "Command complete signal disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENCMDCOMPL::Disabled
    }
    #[doc = "Command complete signal enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENCMDCOMPL::Enabled
    }
}
#[doc = "Field `ENCMDcompl` writer - Enable CMD completion"]
pub type ENCMDCOMPL_W<'a, REG> = crate::BitWriter<'a, REG, ENCMDCOMPL>;
impl<'a, REG> ENCMDCOMPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command complete signal disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENCMDCOMPL::Disabled)
    }
    #[doc = "Command complete signal enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENCMDCOMPL::Enabled)
    }
}
#[doc = "not Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_IEN {
    #[doc = "0: Interrupts to the CE-ATA not disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt to the CE-ATA are disabled"]
    Enabled = 1,
}
impl From<N_IEN> for bool {
    #[inline(always)]
    fn from(variant: N_IEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nIEN` reader - not Interrupt Enable"]
pub type N_IEN_R = crate::BitReader<N_IEN>;
impl N_IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> N_IEN {
        match self.bits {
            false => N_IEN::Disabled,
            true => N_IEN::Enabled,
        }
    }
    #[doc = "Interrupts to the CE-ATA not disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_IEN::Disabled
    }
    #[doc = "Interrupt to the CE-ATA are disabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_IEN::Enabled
    }
}
#[doc = "Field `nIEN` writer - not Interrupt Enable"]
pub type N_IEN_W<'a, REG> = crate::BitWriter<'a, REG, N_IEN>;
impl<'a, REG> N_IEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts to the CE-ATA not disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_IEN::Disabled)
    }
    #[doc = "Interrupt to the CE-ATA are disabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_IEN::Enabled)
    }
}
#[doc = "CE-ATA command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CE_ATACMD {
    #[doc = "0: CE-ATA command disabled"]
    Disabled = 0,
    #[doc = "1: CE-ATA command enabled"]
    Enabled = 1,
}
impl From<CE_ATACMD> for bool {
    #[inline(always)]
    fn from(variant: CE_ATACMD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE_ATACMD` reader - CE-ATA command"]
pub type CE_ATACMD_R = crate::BitReader<CE_ATACMD>;
impl CE_ATACMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CE_ATACMD {
        match self.bits {
            false => CE_ATACMD::Disabled,
            true => CE_ATACMD::Enabled,
        }
    }
    #[doc = "CE-ATA command disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CE_ATACMD::Disabled
    }
    #[doc = "CE-ATA command enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CE_ATACMD::Enabled
    }
}
#[doc = "Field `CE_ATACMD` writer - CE-ATA command"]
pub type CE_ATACMD_W<'a, REG> = crate::BitWriter<'a, REG, CE_ATACMD>;
impl<'a, REG> CE_ATACMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CE-ATA command disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CE_ATACMD::Disabled)
    }
    #[doc = "CE-ATA command enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CE_ATACMD::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable CMD completion"]
    #[inline(always)]
    pub fn encmdcompl(&self) -> ENCMDCOMPL_R {
        ENCMDCOMPL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - not Interrupt Enable"]
    #[inline(always)]
    pub fn n_ien(&self) -> N_IEN_R {
        N_IEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CE-ATA command"]
    #[inline(always)]
    pub fn ce_atacmd(&self) -> CE_ATACMD_R {
        CE_ATACMD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<CMDrs> {
        CMDINDEX_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<CMDrs> {
        WAITRESP_W::new(self, 6)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<CMDrs> {
        WAITINT_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<CMDrs> {
        WAITPEND_W::new(self, 9)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<CMDrs> {
        CPSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    #[must_use]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<CMDrs> {
        SDIOSUSPEND_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable CMD completion"]
    #[inline(always)]
    #[must_use]
    pub fn encmdcompl(&mut self) -> ENCMDCOMPL_W<CMDrs> {
        ENCMDCOMPL_W::new(self, 12)
    }
    #[doc = "Bit 13 - not Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn n_ien(&mut self) -> N_IEN_W<CMDrs> {
        N_IEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - CE-ATA command"]
    #[inline(always)]
    #[must_use]
    pub fn ce_atacmd(&mut self) -> CE_ATACMD_W<CMDrs> {
        CE_ATACMD_W::new(self, 14)
    }
}
#[doc = "command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMDrs {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
