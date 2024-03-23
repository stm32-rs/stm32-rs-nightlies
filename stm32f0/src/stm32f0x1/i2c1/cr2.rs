#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `SADD` reader - Slave address bit 9:8 (master mode)"]
pub type SADD_R = crate::FieldReader<u16>;
#[doc = "Field `SADD` writer - Slave address bit 9:8 (master mode)"]
pub type SADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
#[doc = "Transfer direction (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_WRN {
    #[doc = "0: Master requests a write transfer"]
    Write = 0,
    #[doc = "1: Master requests a read transfer"]
    Read = 1,
}
impl From<RD_WRN> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode)"]
pub type RD_WRN_R = crate::BitReader<RD_WRN>;
impl RD_WRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RD_WRN {
        match self.bits {
            false => RD_WRN::Write,
            true => RD_WRN::Read,
        }
    }
    #[doc = "Master requests a write transfer"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == RD_WRN::Write
    }
    #[doc = "Master requests a read transfer"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == RD_WRN::Read
    }
}
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode)"]
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG, RD_WRN>;
impl<'a, REG> RD_WRN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master requests a write transfer"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN::Write)
    }
    #[doc = "Master requests a read transfer"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN::Read)
    }
}
#[doc = "10-bit addressing mode (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10 {
    #[doc = "0: The master operates in 7-bit addressing mode"]
    Bit7 = 0,
    #[doc = "1: The master operates in 10-bit addressing mode"]
    Bit10 = 1,
}
impl From<ADD10> for bool {
    #[inline(always)]
    fn from(variant: ADD10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode)"]
pub type ADD10_R = crate::BitReader<ADD10>;
impl ADD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADD10 {
        match self.bits {
            false => ADD10::Bit7,
            true => ADD10::Bit10,
        }
    }
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADD10::Bit7
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ADD10::Bit10
    }
}
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode)"]
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG, ADD10>;
impl<'a, REG> ADD10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10::Bit7)
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10::Bit10)
    }
}
#[doc = "10-bit address header only read direction (master receiver mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEAD10R {
    #[doc = "0: The master sends the complete 10 bit slave address read sequence"]
    Complete = 0,
    #[doc = "1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    Partial = 1,
}
impl From<HEAD10R> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_R = crate::BitReader<HEAD10R>;
impl HEAD10R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HEAD10R {
        match self.bits {
            false => HEAD10R::Complete,
            true => HEAD10R::Partial,
        }
    }
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == HEAD10R::Complete
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == HEAD10R::Partial
    }
}
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG, HEAD10R>;
impl<'a, REG> HEAD10R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R::Complete)
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R::Partial)
    }
}
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTR {
    #[doc = "0: No Start generation"]
    NoStart = 0,
    #[doc = "1: Restart/Start generation"]
    Start = 1,
}
impl From<STARTR> for bool {
    #[inline(always)]
    fn from(variant: STARTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader<STARTR>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STARTR {
        match self.bits {
            false => STARTR::NoStart,
            true => STARTR::Start,
        }
    }
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == STARTR::NoStart
    }
    #[doc = "Restart/Start generation"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTR::Start
    }
}
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTW {
    #[doc = "1: Restart/Start generation"]
    Start = 1,
}
impl From<STARTW> for bool {
    #[inline(always)]
    fn from(variant: STARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, REG> = crate::BitWriter1S<'a, REG, STARTW>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Restart/Start generation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STARTW::Start)
    }
}
#[doc = "Stop generation (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPR {
    #[doc = "0: No Stop generation"]
    NoStop = 0,
    #[doc = "1: Stop generation after current byte transfer"]
    Stop = 1,
}
impl From<STOPR> for bool {
    #[inline(always)]
    fn from(variant: STOPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop generation (master mode)"]
pub type STOP_R = crate::BitReader<STOPR>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPR {
        match self.bits {
            false => STOPR::NoStop,
            true => STOPR::Stop,
        }
    }
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPR::NoStop
    }
    #[doc = "Stop generation after current byte transfer"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPR::Stop
    }
}
#[doc = "Stop generation (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPW {
    #[doc = "1: Stop generation after current byte transfer"]
    Stop = 1,
}
impl From<STOPW> for bool {
    #[inline(always)]
    fn from(variant: STOPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` writer - Stop generation (master mode)"]
pub type STOP_W<'a, REG> = crate::BitWriter1S<'a, REG, STOPW>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop generation after current byte transfer"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOPW::Stop)
    }
}
#[doc = "NACK generation (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKR {
    #[doc = "0: an ACK is sent after current received byte"]
    Ack = 0,
    #[doc = "1: a NACK is sent after current received byte"]
    Nack = 1,
}
impl From<NACKR> for bool {
    #[inline(always)]
    fn from(variant: NACKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - NACK generation (slave mode)"]
pub type NACK_R = crate::BitReader<NACKR>;
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKR {
        match self.bits {
            false => NACKR::Ack,
            true => NACKR::Nack,
        }
    }
    #[doc = "an ACK is sent after current received byte"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == NACKR::Ack
    }
    #[doc = "a NACK is sent after current received byte"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKR::Nack
    }
}
#[doc = "NACK generation (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKW {
    #[doc = "1: a NACK is sent after current received byte"]
    Nack = 1,
}
impl From<NACKW> for bool {
    #[inline(always)]
    fn from(variant: NACKW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` writer - NACK generation (slave mode)"]
pub type NACK_W<'a, REG> = crate::BitWriter1S<'a, REG, NACKW>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a NACK is sent after current received byte"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(NACKW::Nack)
    }
}
#[doc = "Field `NBYTES` reader - Number of bytes"]
pub type NBYTES_R = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Number of bytes"]
pub type NBYTES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "NBYTES reload mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD {
    #[doc = "0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    Completed = 0,
    #[doc = "1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    NotCompleted = 1,
}
impl From<RELOAD> for bool {
    #[inline(always)]
    fn from(variant: RELOAD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - NBYTES reload mode"]
pub type RELOAD_R = crate::BitReader<RELOAD>;
impl RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RELOAD {
        match self.bits {
            false => RELOAD::Completed,
            true => RELOAD::NotCompleted,
        }
    }
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == RELOAD::Completed
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == RELOAD::NotCompleted
    }
}
#[doc = "Field `RELOAD` writer - NBYTES reload mode"]
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG, RELOAD>;
impl<'a, REG> RELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD::Completed)
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD::NotCompleted)
    }
}
#[doc = "Automatic end mode (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOEND {
    #[doc = "0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    Software = 0,
    #[doc = "1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    Automatic = 1,
}
impl From<AUTOEND> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode)"]
pub type AUTOEND_R = crate::BitReader<AUTOEND>;
impl AUTOEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTOEND {
        match self.bits {
            false => AUTOEND::Software,
            true => AUTOEND::Automatic,
        }
    }
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == AUTOEND::Software
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AUTOEND::Automatic
    }
}
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode)"]
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG, AUTOEND>;
impl<'a, REG> AUTOEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND::Software)
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND::Automatic)
    }
}
#[doc = "Packet error checking byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTER {
    #[doc = "0: No PEC transfer"]
    NoPec = 0,
    #[doc = "1: PEC transmission/reception is requested"]
    Pec = 1,
}
impl From<PECBYTER> for bool {
    #[inline(always)]
    fn from(variant: PECBYTER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECBYTE` reader - Packet error checking byte"]
pub type PECBYTE_R = crate::BitReader<PECBYTER>;
impl PECBYTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PECBYTER {
        match self.bits {
            false => PECBYTER::NoPec,
            true => PECBYTER::Pec,
        }
    }
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn is_no_pec(&self) -> bool {
        *self == PECBYTER::NoPec
    }
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn is_pec(&self) -> bool {
        *self == PECBYTER::Pec
    }
}
#[doc = "Packet error checking byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTEW {
    #[doc = "1: PEC transmission/reception is requested"]
    Pec = 1,
}
impl From<PECBYTEW> for bool {
    #[inline(always)]
    fn from(variant: PECBYTEW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECBYTE` writer - Packet error checking byte"]
pub type PECBYTE_W<'a, REG> = crate::BitWriter1S<'a, REG, PECBYTEW>;
impl<'a, REG> PECBYTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn pec(self) -> &'a mut crate::W<REG> {
        self.variant(PECBYTEW::Pec)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SADD_W<CR2rs> {
        SADD_W::new(self, 0)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<CR2rs> {
        ADD10_W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR2rs> {
        START_W::new(self, 13)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<CR2rs> {
        NACK_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<CR2rs> {
        NBYTES_W::new(self, 16)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<CR2rs> {
        RELOAD_W::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0400_e000;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
