#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR {
    #[doc = "0: No write/erase operation is in progress"]
    Inactive = 0,
    #[doc = "1: No write/erase operation is in progress"]
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
    #[doc = "No write/erase operation is in progress"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSYR::Active
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERRR {
    #[doc = "0: No programming error occurred"]
    NoError = 0,
    #[doc = "1: A programming error occurred"]
    Error = 1,
}
impl From<PGERRR> for bool {
    #[inline(always)]
    fn from(variant: PGERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGERR` reader - Programming error"]
pub type PGERR_R = crate::BitReader<PGERRR>;
impl PGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGERRR {
        match self.bits {
            false => PGERRR::NoError,
            true => PGERRR::Error,
        }
    }
    #[doc = "No programming error occurred"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERRR::NoError
    }
    #[doc = "A programming error occurred"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERRR::Error
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGERRW {
    #[doc = "1: Reset programming error"]
    Reset = 1,
}
impl From<PGERRW> for bool {
    #[inline(always)]
    fn from(variant: PGERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGERR` writer - Programming error"]
pub type PGERR_W<'a, REG> = crate::BitWriter<'a, REG, PGERRW>;
impl<'a, REG> PGERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset programming error"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PGERRW::Reset)
    }
}
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPRTERRR {
    #[doc = "0: No write protection error occurred"]
    NoError = 0,
    #[doc = "1: A write protection error occurred"]
    Error = 1,
}
impl From<WRPRTERRR> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPRTERR` reader - Write protection error"]
pub type WRPRTERR_R = crate::BitReader<WRPRTERRR>;
impl WRPRTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRPRTERRR {
        match self.bits {
            false => WRPRTERRR::NoError,
            true => WRPRTERRR::Error,
        }
    }
    #[doc = "No write protection error occurred"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPRTERRR::NoError
    }
    #[doc = "A write protection error occurred"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPRTERRR::Error
    }
}
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPRTERRW {
    #[doc = "1: Reset write protection error"]
    Reset = 1,
}
impl From<WRPRTERRW> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPRTERR` writer - Write protection error"]
pub type WRPRTERR_W<'a, REG> = crate::BitWriter<'a, REG, WRPRTERRW>;
impl<'a, REG> WRPRTERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset write protection error"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WRPRTERRW::Reset)
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPR {
    #[doc = "0: No EOP event occurred"]
    NoEvent = 0,
    #[doc = "1: An EOP event occurred"]
    Event = 1,
}
impl From<EOPR> for bool {
    #[inline(always)]
    fn from(variant: EOPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<EOPR>;
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOPR {
        match self.bits {
            false => EOPR::NoEvent,
            true => EOPR::Event,
        }
    }
    #[doc = "No EOP event occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOPR::NoEvent
    }
    #[doc = "An EOP event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOPR::Event
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW {
    #[doc = "1: Reset EOP event"]
    Reset = 1,
}
impl From<EOPW> for bool {
    #[inline(always)]
    fn from(variant: EOPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG, EOPW>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset EOP event"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(EOPW::Reset)
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
    pub fn wrprterr(&self) -> WRPRTERR_R {
        WRPRTERR_R::new(((self.bits >> 4) & 1) != 0)
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
    pub fn wrprterr(&mut self) -> WRPRTERR_W<SRrs> {
        WRPRTERR_W::new(self, 4)
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
