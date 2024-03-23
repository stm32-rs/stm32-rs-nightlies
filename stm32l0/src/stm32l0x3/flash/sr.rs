#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Write/erase operations in progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY {
    #[doc = "0: No write/erase operation is in progress"]
    Inactive = 0,
    #[doc = "1: No write/erase operation is in progress"]
    Active = 1,
}
impl From<BSY> for bool {
    #[inline(always)]
    fn from(variant: BSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Write/erase operations in progress"]
pub type BSY_R = crate::BitReader<BSY>;
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSY {
        match self.bits {
            false => BSY::Inactive,
            true => BSY::Active,
        }
    }
    #[doc = "No write/erase operation is in progress"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSY::Inactive
    }
    #[doc = "No write/erase operation is in progress"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSY::Active
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOP {
    #[doc = "0: No EOP operation occurred"]
    NoEvent = 0,
    #[doc = "1: An EOP event occurred"]
    Event = 1,
}
impl From<EOP> for bool {
    #[inline(always)]
    fn from(variant: EOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<EOP>;
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOP {
        match self.bits {
            false => EOP::NoEvent,
            true => EOP::Event,
        }
    }
    #[doc = "No EOP operation occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOP::NoEvent
    }
    #[doc = "An EOP event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOP::Event
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG, EOP>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No EOP operation occurred"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(EOP::NoEvent)
    }
    #[doc = "An EOP event occurred"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(EOP::Event)
    }
}
#[doc = "End of high voltage\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDHV {
    #[doc = "0: High voltage is executing a write/erase operation in the NVM"]
    Active = 0,
    #[doc = "1: High voltage is off, no write/erase operation is ongoing"]
    Inactive = 1,
}
impl From<ENDHV> for bool {
    #[inline(always)]
    fn from(variant: ENDHV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDHV` reader - End of high voltage"]
pub type ENDHV_R = crate::BitReader<ENDHV>;
impl ENDHV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENDHV {
        match self.bits {
            false => ENDHV::Active,
            true => ENDHV::Inactive,
        }
    }
    #[doc = "High voltage is executing a write/erase operation in the NVM"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ENDHV::Active
    }
    #[doc = "High voltage is off, no write/erase operation is ongoing"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ENDHV::Inactive
    }
}
#[doc = "Flash memory module ready after low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY {
    #[doc = "0: The NVM is not ready"]
    NotReady = 0,
    #[doc = "1: The NVM is ready"]
    Ready = 1,
}
impl From<READY> for bool {
    #[inline(always)]
    fn from(variant: READY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Flash memory module ready after low power mode"]
pub type READY_R = crate::BitReader<READY>;
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> READY {
        match self.bits {
            false => READY::NotReady,
            true => READY::Ready,
        }
    }
    #[doc = "The NVM is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == READY::NotReady
    }
    #[doc = "The NVM is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY::Ready
    }
}
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRR {
    #[doc = "0: No protection error happened"]
    NoError = 0,
    #[doc = "1: One protection error happened"]
    Error = 1,
}
impl From<WRPERRR> for bool {
    #[inline(always)]
    fn from(variant: WRPERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` reader - Write protected error"]
pub type WRPERR_R = crate::BitReader<WRPERRR>;
impl WRPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRPERRR {
        match self.bits {
            false => WRPERRR::NoError,
            true => WRPERRR::Error,
        }
    }
    #[doc = "No protection error happened"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPERRR::NoError
    }
    #[doc = "One protection error happened"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPERRR::Error
    }
}
#[doc = "Write protected error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<WRPERRW> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` writer - Write protected error"]
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG, WRPERRW>;
impl<'a, REG> WRPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WRPERRW::Clear)
    }
}
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRR {
    #[doc = "0: No alignment error happened"]
    NoError = 0,
    #[doc = "1: One alignment error happened"]
    Error = 1,
}
impl From<PGAERRR> for bool {
    #[inline(always)]
    fn from(variant: PGAERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PGAERR_R = crate::BitReader<PGAERRR>;
impl PGAERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGAERRR {
        match self.bits {
            false => PGAERRR::NoError,
            true => PGAERRR::Error,
        }
    }
    #[doc = "No alignment error happened"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGAERRR::NoError
    }
    #[doc = "One alignment error happened"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGAERRR::Error
    }
}
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<PGAERRW> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG, PGAERRW>;
impl<'a, REG> PGAERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGAERRW::Clear)
    }
}
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRR {
    #[doc = "0: No size error happened"]
    NoError = 0,
    #[doc = "1: One size error happened"]
    Error = 1,
}
impl From<SIZERRR> for bool {
    #[inline(always)]
    fn from(variant: SIZERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZERR` reader - Size error"]
pub type SIZERR_R = crate::BitReader<SIZERRR>;
impl SIZERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIZERRR {
        match self.bits {
            false => SIZERRR::NoError,
            true => SIZERRR::Error,
        }
    }
    #[doc = "No size error happened"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SIZERRR::NoError
    }
    #[doc = "One size error happened"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SIZERRR::Error
    }
}
#[doc = "Size error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<SIZERRW> for bool {
    #[inline(always)]
    fn from(variant: SIZERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZERR` writer - Size error"]
pub type SIZERR_W<'a, REG> = crate::BitWriter<'a, REG, SIZERRW>;
impl<'a, REG> SIZERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SIZERRW::Clear)
    }
}
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRR {
    #[doc = "0: No error happened during the Option bytes loading"]
    NoError = 0,
    #[doc = "1: One or more errors happened during the Option bytes loading"]
    Error = 1,
}
impl From<OPTVERRR> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTVERR` reader - Option validity error"]
pub type OPTVERR_R = crate::BitReader<OPTVERRR>;
impl OPTVERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTVERRR {
        match self.bits {
            false => OPTVERRR::NoError,
            true => OPTVERRR::Error,
        }
    }
    #[doc = "No error happened during the Option bytes loading"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPTVERRR::NoError
    }
    #[doc = "One or more errors happened during the Option bytes loading"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPTVERRR::Error
    }
}
#[doc = "Option validity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<OPTVERRW> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTVERR` writer - Option validity error"]
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG, OPTVERRW>;
impl<'a, REG> OPTVERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OPTVERRW::Clear)
    }
}
#[doc = "RDERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRR {
    #[doc = "0: No read protection error happened."]
    NoError = 0,
    #[doc = "1: One read protection error happened"]
    Error = 1,
}
impl From<RDERRR> for bool {
    #[inline(always)]
    fn from(variant: RDERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` reader - RDERR"]
pub type RDERR_R = crate::BitReader<RDERRR>;
impl RDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDERRR {
        match self.bits {
            false => RDERRR::NoError,
            true => RDERRR::Error,
        }
    }
    #[doc = "No read protection error happened."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERRR::NoError
    }
    #[doc = "One read protection error happened"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERRR::Error
    }
}
#[doc = "RDERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<RDERRW> for bool {
    #[inline(always)]
    fn from(variant: RDERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERR` writer - RDERR"]
pub type RDERR_W<'a, REG> = crate::BitWriter<'a, REG, RDERRW>;
impl<'a, REG> RDERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRW::Clear)
    }
}
#[doc = "NOTZEROERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTZEROERRR {
    #[doc = "0: The write operation is done in an erased region or the memory interface can apply an erase before a write"]
    NoEvent = 0,
    #[doc = "1: The write operation is attempting to write to a not-erased region and the memory interface cannot apply an erase before a write"]
    Event = 1,
}
impl From<NOTZEROERRR> for bool {
    #[inline(always)]
    fn from(variant: NOTZEROERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTZEROERR` reader - NOTZEROERR"]
pub type NOTZEROERR_R = crate::BitReader<NOTZEROERRR>;
impl NOTZEROERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOTZEROERRR {
        match self.bits {
            false => NOTZEROERRR::NoEvent,
            true => NOTZEROERRR::Event,
        }
    }
    #[doc = "The write operation is done in an erased region or the memory interface can apply an erase before a write"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == NOTZEROERRR::NoEvent
    }
    #[doc = "The write operation is attempting to write to a not-erased region and the memory interface cannot apply an erase before a write"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == NOTZEROERRR::Event
    }
}
#[doc = "NOTZEROERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTZEROERRW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<NOTZEROERRW> for bool {
    #[inline(always)]
    fn from(variant: NOTZEROERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTZEROERR` writer - NOTZEROERR"]
pub type NOTZEROERR_W<'a, REG> = crate::BitWriter<'a, REG, NOTZEROERRW>;
impl<'a, REG> NOTZEROERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NOTZEROERRW::Clear)
    }
}
#[doc = "FWWERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWWERRR {
    #[doc = "0: No write/erase operation aborted to perform a fetch"]
    NoError = 0,
    #[doc = "1: A write/erase operation aborted to perform a fetch"]
    Error = 1,
}
impl From<FWWERRR> for bool {
    #[inline(always)]
    fn from(variant: FWWERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWWERR` reader - FWWERR"]
pub type FWWERR_R = crate::BitReader<FWWERRR>;
impl FWWERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FWWERRR {
        match self.bits {
            false => FWWERRR::NoError,
            true => FWWERRR::Error,
        }
    }
    #[doc = "No write/erase operation aborted to perform a fetch"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FWWERRR::NoError
    }
    #[doc = "A write/erase operation aborted to perform a fetch"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FWWERRR::Error
    }
}
#[doc = "FWWERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWWERRW {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<FWWERRW> for bool {
    #[inline(always)]
    fn from(variant: FWWERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWWERR` writer - FWWERR"]
pub type FWWERR_W<'a, REG> = crate::BitWriter<'a, REG, FWWERRW>;
impl<'a, REG> FWWERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FWWERRW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write/erase operations in progress"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of high voltage"]
    #[inline(always)]
    pub fn endhv(&self) -> ENDHV_R {
        ENDHV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash memory module ready after low power mode"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - RDERR"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - NOTZEROERR"]
    #[inline(always)]
    pub fn notzeroerr(&self) -> NOTZEROERR_R {
        NOTZEROERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FWWERR"]
    #[inline(always)]
    pub fn fwwerr(&self) -> FWWERR_R {
        FWWERR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<SRrs> {
        EOP_W::new(self, 1)
    }
    #[doc = "Bit 8 - Write protected error"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<SRrs> {
        WRPERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Programming alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<SRrs> {
        PGAERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Size error"]
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<SRrs> {
        SIZERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Option validity error"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<SRrs> {
        OPTVERR_W::new(self, 11)
    }
    #[doc = "Bit 14 - RDERR"]
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RDERR_W<SRrs> {
        RDERR_W::new(self, 14)
    }
    #[doc = "Bit 16 - NOTZEROERR"]
    #[inline(always)]
    #[must_use]
    pub fn notzeroerr(&mut self) -> NOTZEROERR_W<SRrs> {
        NOTZEROERR_W::new(self, 16)
    }
    #[doc = "Bit 17 - FWWERR"]
    #[inline(always)]
    #[must_use]
    pub fn fwwerr(&mut self) -> FWWERR_W<SRrs> {
        FWWERR_W::new(self, 17)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x04"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x04;
}
