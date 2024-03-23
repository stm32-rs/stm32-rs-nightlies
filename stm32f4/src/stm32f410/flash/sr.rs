#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<EOPW> for bool {
    #[inline(always)]
    fn from(variant: EOPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<EOPW>;
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EOPW> {
        match self.bits {
            true => Some(EOPW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EOPW::Clear
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, REG> = crate::BitWriter1C<'a, REG, EOPW>;
impl<'a, REG> EOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOPW::Clear)
    }
}
#[doc = "Operation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRW {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<OPERRW> for bool {
    #[inline(always)]
    fn from(variant: OPERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPERR` reader - Operation error"]
pub type OPERR_R = crate::BitReader<OPERRW>;
impl OPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPERRW> {
        match self.bits {
            true => Some(OPERRW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == OPERRW::Clear
    }
}
#[doc = "Field `OPERR` writer - Operation error"]
pub type OPERR_W<'a, REG> = crate::BitWriter1C<'a, REG, OPERRW>;
impl<'a, REG> OPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OPERRW::Clear)
    }
}
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<WRPERRW> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPERR` reader - Write protection error"]
pub type WRPERR_R = crate::BitReader<WRPERRW>;
impl WRPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WRPERRW> {
        match self.bits {
            true => Some(WRPERRW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WRPERRW::Clear
    }
}
#[doc = "Field `WRPERR` writer - Write protection error"]
pub type WRPERR_W<'a, REG> = crate::BitWriter1C<'a, REG, WRPERRW>;
impl<'a, REG> WRPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WRPERRW::Clear)
    }
}
#[doc = "Programming alignment error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<PGAERRW> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PGAERR_R = crate::BitReader<PGAERRW>;
impl PGAERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PGAERRW> {
        match self.bits {
            true => Some(PGAERRW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PGAERRW::Clear
    }
}
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PGAERR_W<'a, REG> = crate::BitWriter1C<'a, REG, PGAERRW>;
impl<'a, REG> PGAERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGAERRW::Clear)
    }
}
#[doc = "Programming parallelism error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGPERRW {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<PGPERRW> for bool {
    #[inline(always)]
    fn from(variant: PGPERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGPERR` reader - Programming parallelism error"]
pub type PGPERR_R = crate::BitReader<PGPERRW>;
impl PGPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PGPERRW> {
        match self.bits {
            true => Some(PGPERRW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PGPERRW::Clear
    }
}
#[doc = "Field `PGPERR` writer - Programming parallelism error"]
pub type PGPERR_W<'a, REG> = crate::BitWriter1C<'a, REG, PGPERRW>;
impl<'a, REG> PGPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGPERRW::Clear)
    }
}
#[doc = "Programming sequence error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRW {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<PGSERRW> for bool {
    #[inline(always)]
    fn from(variant: PGSERRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PGSERR_R = crate::BitReader<PGSERRW>;
impl PGSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PGSERRW> {
        match self.bits {
            true => Some(PGSERRW::Clear),
            _ => None,
        }
    }
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PGSERRW::Clear
    }
}
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PGSERR_W<'a, REG> = crate::BitWriter1C<'a, REG, PGSERRW>;
impl<'a, REG> PGSERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PGSERRW::Clear)
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR {
    #[doc = "0: no Flash memory operation ongoing"]
    NotBusy = 0,
    #[doc = "1: Flash memory operation ongoing"]
    Busy = 1,
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
            false => BSYR::NotBusy,
            true => BSYR::Busy,
        }
    }
    #[doc = "no Flash memory operation ongoing"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR::NotBusy
    }
    #[doc = "Flash memory operation ongoing"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR::Busy
    }
}
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Programming parallelism error"]
    #[inline(always)]
    pub fn pgperr(&self) -> PGPERR_R {
        PGPERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<SRrs> {
        EOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<SRrs> {
        OPERR_W::new(self, 1)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<SRrs> {
        WRPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<SRrs> {
        PGAERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Programming parallelism error"]
    #[inline(always)]
    #[must_use]
    pub fn pgperr(&mut self) -> PGPERR_W<SRrs> {
        PGPERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<SRrs> {
        PGSERR_W::new(self, 7)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf3;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
