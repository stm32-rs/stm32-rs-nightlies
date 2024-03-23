#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR {
    #[doc = "0: No write/erase operation is in progress"]
    Inactive = 0,
    #[doc = "1: A write/erase operation is in progress"]
    Active = 1,
}
impl From<BSYR> for bool {
    #[inline(always)]
    fn from(variant: BSYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader<BSYR>;
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSYR {
        match self.bits {
            false => BSYR::Inactive,
            true => BSYR::Active,
        }
    }
    #[doc = "No write/erase operation is in progress"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSYR::Inactive
    }
    #[doc = "A write/erase operation is in progress"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSYR::Active
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERR {
    #[doc = "0: No programming error occurred"]
    NoError = 0,
    #[doc = "1: A programming error occurred"]
    Error = 1,
}
impl From<PGERR> for bool {
    #[inline(always)]
    fn from(variant: PGERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGERR` reader - Programming error"]
pub type PGERR_R = crate::BitReader<PGERR>;
impl PGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGERR {
        match self.bits {
            false => PGERR::NoError,
            true => PGERR::Error,
        }
    }
    #[doc = "No programming error occurred"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERR::NoError
    }
    #[doc = "A programming error occurred"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERR::Error
    }
}
#[doc = "Field `PGERR` writer - Programming error"]
pub type PGERR_W<'a, REG> = crate::BitWriter<'a, REG, PGERR>;
impl<'a, REG> PGERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No programming error occurred"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(PGERR::NoError)
    }
    #[doc = "A programming error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(PGERR::Error)
    }
}
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPRT {
    #[doc = "0: No write protection error occurred"]
    NoError = 0,
    #[doc = "1: A write protection error occurred"]
    Error = 1,
}
impl From<WRPRT> for bool {
    #[inline(always)]
    fn from(variant: WRPRT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPRT` reader - Write protection error"]
pub type WRPRT_R = crate::BitReader<WRPRT>;
impl WRPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRPRT {
        match self.bits {
            false => WRPRT::NoError,
            true => WRPRT::Error,
        }
    }
    #[doc = "No write protection error occurred"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPRT::NoError
    }
    #[doc = "A write protection error occurred"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPRT::Error
    }
}
#[doc = "Field `WRPRT` writer - Write protection error"]
pub type WRPRT_W<'a, REG> = crate::BitWriter<'a, REG, WRPRT>;
impl<'a, REG> WRPRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write protection error occurred"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(WRPRT::NoError)
    }
    #[doc = "A write protection error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(WRPRT::Error)
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
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprt(&self) -> WRPRT_R {
        WRPRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    #[must_use]
    pub fn pgerr(&mut self) -> PGERR_W<SRrs> {
        PGERR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    #[must_use]
    pub fn wrprt(&mut self) -> WRPRT_W<SRrs> {
        WRPRT_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<SRrs> {
        EOP_W::new(self, 5)
    }
}
#[doc = "Flash status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
