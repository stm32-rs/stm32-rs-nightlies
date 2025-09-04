///Register `CMD` reader
pub type R = crate::R<CMDrs>;
///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `CMDINDEX` reader - Command index
pub type CMDINDEX_R = crate::FieldReader;
///Field `CMDINDEX` writer - Command index
pub type CMDINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
/**Wait for response bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAITRESP {
    ///0: No response
    NoResponse = 0,
    ///1: Short response
    ShortResponse = 1,
    ///2: No reponse
    NoResponse2 = 2,
    ///3: Long reponse
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
impl crate::IsEnum for WAITRESP {}
///Field `WAITRESP` reader - Wait for response bits
pub type WAITRESP_R = crate::FieldReader<WAITRESP>;
impl WAITRESP_R {
    ///Get enumerated values variant
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
    ///No response
    #[inline(always)]
    pub fn is_no_response(&self) -> bool {
        *self == WAITRESP::NoResponse
    }
    ///Short response
    #[inline(always)]
    pub fn is_short_response(&self) -> bool {
        *self == WAITRESP::ShortResponse
    }
    ///No reponse
    #[inline(always)]
    pub fn is_no_response2(&self) -> bool {
        *self == WAITRESP::NoResponse2
    }
    ///Long reponse
    #[inline(always)]
    pub fn is_long_response(&self) -> bool {
        *self == WAITRESP::LongResponse
    }
}
///Field `WAITRESP` writer - Wait for response bits
pub type WAITRESP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WAITRESP, crate::Safe>;
impl<'a, REG> WAITRESP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No response
    #[inline(always)]
    pub fn no_response(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::NoResponse)
    }
    ///Short response
    #[inline(always)]
    pub fn short_response(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::ShortResponse)
    }
    ///No reponse
    #[inline(always)]
    pub fn no_response2(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::NoResponse2)
    }
    ///Long reponse
    #[inline(always)]
    pub fn long_response(self) -> &'a mut crate::W<REG> {
        self.variant(WAITRESP::LongResponse)
    }
}
/**CPSM waits for interrupt request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITINT {
    ///0: Don't wait for interrupt request
    Disabled = 0,
    ///1: Wait for interrupt request
    Enabled = 1,
}
impl From<WAITINT> for bool {
    #[inline(always)]
    fn from(variant: WAITINT) -> Self {
        variant as u8 != 0
    }
}
///Field `WAITINT` reader - CPSM waits for interrupt request
pub type WAITINT_R = crate::BitReader<WAITINT>;
impl WAITINT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAITINT {
        match self.bits {
            false => WAITINT::Disabled,
            true => WAITINT::Enabled,
        }
    }
    ///Don't wait for interrupt request
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITINT::Disabled
    }
    ///Wait for interrupt request
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITINT::Enabled
    }
}
///Field `WAITINT` writer - CPSM waits for interrupt request
pub type WAITINT_W<'a, REG> = crate::BitWriter<'a, REG, WAITINT>;
impl<'a, REG> WAITINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Don't wait for interrupt request
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITINT::Disabled)
    }
    ///Wait for interrupt request
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITINT::Enabled)
    }
}
/**CPSM Waits for ends of data transfer (CmdPend internal signal)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITPEND {
    ///0: Don't wait for data end
    Disabled = 0,
    ///1: Wait for end of data transfer signal before sending command
    Enabled = 1,
}
impl From<WAITPEND> for bool {
    #[inline(always)]
    fn from(variant: WAITPEND) -> Self {
        variant as u8 != 0
    }
}
///Field `WAITPEND` reader - CPSM Waits for ends of data transfer (CmdPend internal signal)
pub type WAITPEND_R = crate::BitReader<WAITPEND>;
impl WAITPEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAITPEND {
        match self.bits {
            false => WAITPEND::Disabled,
            true => WAITPEND::Enabled,
        }
    }
    ///Don't wait for data end
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITPEND::Disabled
    }
    ///Wait for end of data transfer signal before sending command
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITPEND::Enabled
    }
}
///Field `WAITPEND` writer - CPSM Waits for ends of data transfer (CmdPend internal signal)
pub type WAITPEND_W<'a, REG> = crate::BitWriter<'a, REG, WAITPEND>;
impl<'a, REG> WAITPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Don't wait for data end
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPEND::Disabled)
    }
    ///Wait for end of data transfer signal before sending command
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAITPEND::Enabled)
    }
}
/**Command path state machine (CPSM) Enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPSMEN {
    ///0: Command path state machine disabled
    Disabled = 0,
    ///1: Command path state machine enabled
    Enabled = 1,
}
impl From<CPSMEN> for bool {
    #[inline(always)]
    fn from(variant: CPSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit
pub type CPSMEN_R = crate::BitReader<CPSMEN>;
impl CPSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPSMEN {
        match self.bits {
            false => CPSMEN::Disabled,
            true => CPSMEN::Enabled,
        }
    }
    ///Command path state machine disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPSMEN::Disabled
    }
    ///Command path state machine enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPSMEN::Enabled
    }
}
///Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit
pub type CPSMEN_W<'a, REG> = crate::BitWriter<'a, REG, CPSMEN>;
impl<'a, REG> CPSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command path state machine disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPSMEN::Disabled)
    }
    ///Command path state machine enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPSMEN::Enabled)
    }
}
/**SD I/O suspend command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIOSUSPEND {
    ///0: Next command is not a SDIO suspend command
    Disabled = 0,
    ///1: Next command send is a SDIO suspend command
    Enabled = 1,
}
impl From<SDIOSUSPEND> for bool {
    #[inline(always)]
    fn from(variant: SDIOSUSPEND) -> Self {
        variant as u8 != 0
    }
}
///Field `SDIOSuspend` reader - SD I/O suspend command
pub type SDIOSUSPEND_R = crate::BitReader<SDIOSUSPEND>;
impl SDIOSUSPEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDIOSUSPEND {
        match self.bits {
            false => SDIOSUSPEND::Disabled,
            true => SDIOSUSPEND::Enabled,
        }
    }
    ///Next command is not a SDIO suspend command
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOSUSPEND::Disabled
    }
    ///Next command send is a SDIO suspend command
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOSUSPEND::Enabled
    }
}
///Field `SDIOSuspend` writer - SD I/O suspend command
pub type SDIOSUSPEND_W<'a, REG> = crate::BitWriter<'a, REG, SDIOSUSPEND>;
impl<'a, REG> SDIOSUSPEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Next command is not a SDIO suspend command
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSUSPEND::Disabled)
    }
    ///Next command send is a SDIO suspend command
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDIOSUSPEND::Enabled)
    }
}
/**Enable CMD completion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENCMDCOMPL {
    ///0: Command complete signal disabled
    Disabled = 0,
    ///1: Command complete signal enabled
    Enabled = 1,
}
impl From<ENCMDCOMPL> for bool {
    #[inline(always)]
    fn from(variant: ENCMDCOMPL) -> Self {
        variant as u8 != 0
    }
}
///Field `ENCMDcompl` reader - Enable CMD completion
pub type ENCMDCOMPL_R = crate::BitReader<ENCMDCOMPL>;
impl ENCMDCOMPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENCMDCOMPL {
        match self.bits {
            false => ENCMDCOMPL::Disabled,
            true => ENCMDCOMPL::Enabled,
        }
    }
    ///Command complete signal disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENCMDCOMPL::Disabled
    }
    ///Command complete signal enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENCMDCOMPL::Enabled
    }
}
///Field `ENCMDcompl` writer - Enable CMD completion
pub type ENCMDCOMPL_W<'a, REG> = crate::BitWriter<'a, REG, ENCMDCOMPL>;
impl<'a, REG> ENCMDCOMPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Command complete signal disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENCMDCOMPL::Disabled)
    }
    ///Command complete signal enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENCMDCOMPL::Enabled)
    }
}
/**not Interrupt Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_IEN {
    ///0: Interrupts to the CE-ATA not disabled
    Disabled = 0,
    ///1: Interrupt to the CE-ATA are disabled
    Enabled = 1,
}
impl From<N_IEN> for bool {
    #[inline(always)]
    fn from(variant: N_IEN) -> Self {
        variant as u8 != 0
    }
}
///Field `nIEN` reader - not Interrupt Enable
pub type N_IEN_R = crate::BitReader<N_IEN>;
impl N_IEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_IEN {
        match self.bits {
            false => N_IEN::Disabled,
            true => N_IEN::Enabled,
        }
    }
    ///Interrupts to the CE-ATA not disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_IEN::Disabled
    }
    ///Interrupt to the CE-ATA are disabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_IEN::Enabled
    }
}
///Field `nIEN` writer - not Interrupt Enable
pub type N_IEN_W<'a, REG> = crate::BitWriter<'a, REG, N_IEN>;
impl<'a, REG> N_IEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts to the CE-ATA not disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_IEN::Disabled)
    }
    ///Interrupt to the CE-ATA are disabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(N_IEN::Enabled)
    }
}
/**CE-ATA command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CE_ATACMD {
    ///0: CE-ATA command disabled
    Disabled = 0,
    ///1: CE-ATA command enabled
    Enabled = 1,
}
impl From<CE_ATACMD> for bool {
    #[inline(always)]
    fn from(variant: CE_ATACMD) -> Self {
        variant as u8 != 0
    }
}
///Field `CE_ATACMD` reader - CE-ATA command
pub type CE_ATACMD_R = crate::BitReader<CE_ATACMD>;
impl CE_ATACMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CE_ATACMD {
        match self.bits {
            false => CE_ATACMD::Disabled,
            true => CE_ATACMD::Enabled,
        }
    }
    ///CE-ATA command disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CE_ATACMD::Disabled
    }
    ///CE-ATA command enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CE_ATACMD::Enabled
    }
}
///Field `CE_ATACMD` writer - CE-ATA command
pub type CE_ATACMD_W<'a, REG> = crate::BitWriter<'a, REG, CE_ATACMD>;
impl<'a, REG> CE_ATACMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CE-ATA command disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CE_ATACMD::Disabled)
    }
    ///CE-ATA command enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CE_ATACMD::Enabled)
    }
}
impl R {
    ///Bits 0:5 - Command index
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Wait for response bits
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - CPSM waits for interrupt request
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Command path state machine (CPSM) Enable bit
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SD I/O suspend command
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable CMD completion
    #[inline(always)]
    pub fn encmdcompl(&self) -> ENCMDCOMPL_R {
        ENCMDCOMPL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - not Interrupt Enable
    #[inline(always)]
    pub fn n_ien(&self) -> N_IEN_R {
        N_IEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CE-ATA command
    #[inline(always)]
    pub fn ce_atacmd(&self) -> CE_ATACMD_R {
        CE_ATACMD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("ce_atacmd", &self.ce_atacmd())
            .field("n_ien", &self.n_ien())
            .field("encmdcompl", &self.encmdcompl())
            .field("sdiosuspend", &self.sdiosuspend())
            .field("cpsmen", &self.cpsmen())
            .field("waitpend", &self.waitpend())
            .field("waitint", &self.waitint())
            .field("waitresp", &self.waitresp())
            .field("cmdindex", &self.cmdindex())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Command index
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<CMDrs> {
        CMDINDEX_W::new(self, 0)
    }
    ///Bits 6:7 - Wait for response bits
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W<CMDrs> {
        WAITRESP_W::new(self, 6)
    }
    ///Bit 8 - CPSM waits for interrupt request
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W<CMDrs> {
        WAITINT_W::new(self, 8)
    }
    ///Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W<CMDrs> {
        WAITPEND_W::new(self, 9)
    }
    ///Bit 10 - Command path state machine (CPSM) Enable bit
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W<CMDrs> {
        CPSMEN_W::new(self, 10)
    }
    ///Bit 11 - SD I/O suspend command
    #[inline(always)]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<CMDrs> {
        SDIOSUSPEND_W::new(self, 11)
    }
    ///Bit 12 - Enable CMD completion
    #[inline(always)]
    pub fn encmdcompl(&mut self) -> ENCMDCOMPL_W<CMDrs> {
        ENCMDCOMPL_W::new(self, 12)
    }
    ///Bit 13 - not Interrupt Enable
    #[inline(always)]
    pub fn n_ien(&mut self) -> N_IEN_W<CMDrs> {
        N_IEN_W::new(self, 13)
    }
    ///Bit 14 - CE-ATA command
    #[inline(always)]
    pub fn ce_atacmd(&mut self) -> CE_ATACMD_W<CMDrs> {
        CE_ATACMD_W::new(self, 14)
    }
}
/**command register

You can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#SDMMC1:CMD)*/
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
///`read()` method returns [`cmd::R`](R) reader structure
impl crate::Readable for CMDrs {}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMDrs {}
