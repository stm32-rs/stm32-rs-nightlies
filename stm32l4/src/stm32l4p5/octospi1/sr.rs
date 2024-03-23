#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Transfer error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEF {
    #[doc = "0: This bit is cleared by writing 1 to CTEF"]
    Cleared = 0,
    #[doc = "1: This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode"]
    InvalidAddressAccessed = 1,
}
impl From<TEF> for bool {
    #[inline(always)]
    fn from(variant: TEF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEF` reader - Transfer error flag"]
pub type TEF_R = crate::BitReader<TEF>;
impl TEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEF {
        match self.bits {
            false => TEF::Cleared,
            true => TEF::InvalidAddressAccessed,
        }
    }
    #[doc = "This bit is cleared by writing 1 to CTEF"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TEF::Cleared
    }
    #[doc = "This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode"]
    #[inline(always)]
    pub fn is_invalid_address_accessed(&self) -> bool {
        *self == TEF::InvalidAddressAccessed
    }
}
#[doc = "Field `TEF` writer - Transfer error flag"]
pub type TEF_W<'a, REG> = crate::BitWriter<'a, REG, TEF>;
impl<'a, REG> TEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is cleared by writing 1 to CTEF"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(TEF::Cleared)
    }
    #[doc = "This bit is set in Indirect mode when an invalid address is being accessed in Indirect mode"]
    #[inline(always)]
    pub fn invalid_address_accessed(self) -> &'a mut crate::W<REG> {
        self.variant(TEF::InvalidAddressAccessed)
    }
}
#[doc = "Transfer complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF {
    #[doc = "0: This bit is cleared by writing 1 to CTCF"]
    Cleared = 0,
    #[doc = "1: This bit is set when the programmed number of data has been transferred"]
    TransferCompleted = 1,
}
impl From<TCF> for bool {
    #[inline(always)]
    fn from(variant: TCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transfer complete flag"]
pub type TCF_R = crate::BitReader<TCF>;
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCF {
        match self.bits {
            false => TCF::Cleared,
            true => TCF::TransferCompleted,
        }
    }
    #[doc = "This bit is cleared by writing 1 to CTCF"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TCF::Cleared
    }
    #[doc = "This bit is set when the programmed number of data has been transferred"]
    #[inline(always)]
    pub fn is_transfer_completed(&self) -> bool {
        *self == TCF::TransferCompleted
    }
}
#[doc = "Field `TCF` writer - Transfer complete flag"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG, TCF>;
impl<'a, REG> TCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is cleared by writing 1 to CTCF"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(TCF::Cleared)
    }
    #[doc = "This bit is set when the programmed number of data has been transferred"]
    #[inline(always)]
    pub fn transfer_completed(self) -> &'a mut crate::W<REG> {
        self.variant(TCF::TransferCompleted)
    }
}
#[doc = "FIFO threshold flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF {
    #[doc = "0: It is cleared automatically as soon as the threshold condition is no longer true"]
    Cleared = 0,
    #[doc = "1: This bit is set when the FIFO threshold has been reached"]
    ThresholdReached = 1,
}
impl From<FTF> for bool {
    #[inline(always)]
    fn from(variant: FTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTF` reader - FIFO threshold flag"]
pub type FTF_R = crate::BitReader<FTF>;
impl FTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FTF {
        match self.bits {
            false => FTF::Cleared,
            true => FTF::ThresholdReached,
        }
    }
    #[doc = "It is cleared automatically as soon as the threshold condition is no longer true"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == FTF::Cleared
    }
    #[doc = "This bit is set when the FIFO threshold has been reached"]
    #[inline(always)]
    pub fn is_threshold_reached(&self) -> bool {
        *self == FTF::ThresholdReached
    }
}
#[doc = "Field `FTF` writer - FIFO threshold flag"]
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG, FTF>;
impl<'a, REG> FTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "It is cleared automatically as soon as the threshold condition is no longer true"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(FTF::Cleared)
    }
    #[doc = "This bit is set when the FIFO threshold has been reached"]
    #[inline(always)]
    pub fn threshold_reached(self) -> &'a mut crate::W<REG> {
        self.variant(FTF::ThresholdReached)
    }
}
#[doc = "Status match flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMF {
    #[doc = "0: It is cleared by writing 1 to CSMF"]
    Cleared = 0,
    #[doc = "1: This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)"]
    Matched = 1,
}
impl From<SMF> for bool {
    #[inline(always)]
    fn from(variant: SMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMF` reader - Status match flag"]
pub type SMF_R = crate::BitReader<SMF>;
impl SMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMF {
        match self.bits {
            false => SMF::Cleared,
            true => SMF::Matched,
        }
    }
    #[doc = "It is cleared by writing 1 to CSMF"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SMF::Cleared
    }
    #[doc = "This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)"]
    #[inline(always)]
    pub fn is_matched(&self) -> bool {
        *self == SMF::Matched
    }
}
#[doc = "Field `SMF` writer - Status match flag"]
pub type SMF_W<'a, REG> = crate::BitWriter<'a, REG, SMF>;
impl<'a, REG> SMF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "It is cleared by writing 1 to CSMF"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(SMF::Cleared)
    }
    #[doc = "This bit is set in Automatic status-polling mode when the unmasked received data matches the corresponding bits in the match register (OCTOSPI_PSMAR)"]
    #[inline(always)]
    pub fn matched(self) -> &'a mut crate::W<REG> {
        self.variant(SMF::Matched)
    }
}
#[doc = "Timeout flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF {
    #[doc = "0: This bit is cleared by writing 1 to CTOF"]
    Cleared = 0,
    #[doc = "1: This bit is set when timeout occurs"]
    Timeout = 1,
}
impl From<TOF> for bool {
    #[inline(always)]
    fn from(variant: TOF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Timeout flag"]
pub type TOF_R = crate::BitReader<TOF>;
impl TOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOF {
        match self.bits {
            false => TOF::Cleared,
            true => TOF::Timeout,
        }
    }
    #[doc = "This bit is cleared by writing 1 to CTOF"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == TOF::Cleared
    }
    #[doc = "This bit is set when timeout occurs"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TOF::Timeout
    }
}
#[doc = "Field `TOF` writer - Timeout flag"]
pub type TOF_W<'a, REG> = crate::BitWriter<'a, REG, TOF>;
impl<'a, REG> TOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is cleared by writing 1 to CTOF"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(TOF::Cleared)
    }
    #[doc = "This bit is set when timeout occurs"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut crate::W<REG> {
        self.variant(TOF::Timeout)
    }
}
#[doc = "BUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    #[doc = "0: This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty"]
    Cleared = 0,
    #[doc = "1: This bit is set when an operation is ongoing"]
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::Cleared,
            true => BUSY::Busy,
        }
    }
    #[doc = "This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == BUSY::Cleared
    }
    #[doc = "This bit is set when an operation is ongoing"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG, BUSY>;
impl<'a, REG> BUSY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is cleared automatically when the operation with the external device is finished and the FIFO is empty"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(BUSY::Cleared)
    }
    #[doc = "This bit is set when an operation is ongoing"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(BUSY::Busy)
    }
}
#[doc = "Field `FLEVEL` reader - FIFO level"]
pub type FLEVEL_R = crate::FieldReader;
#[doc = "Field `FLEVEL` writer - FIFO level"]
pub type FLEVEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - FIFO level"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tef(&mut self) -> TEF_W<SRrs> {
        TEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<SRrs> {
        TCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<SRrs> {
        FTF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    #[must_use]
    pub fn smf(&mut self) -> SMF_W<SRrs> {
        SMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<SRrs> {
        TOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<SRrs> {
        BUSY_W::new(self, 5)
    }
    #[doc = "Bits 8:13 - FIFO level"]
    #[inline(always)]
    #[must_use]
    pub fn flevel(&mut self) -> FLEVEL_W<SRrs> {
        FLEVEL_W::new(self, 8)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
