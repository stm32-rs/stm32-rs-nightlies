#[doc = "Register `MMCTIMR` reader"]
pub type R = crate::R<MMCTIMRrs>;
#[doc = "Register `MMCTIMR` writer"]
pub type W = crate::W<MMCTIMRrs>;
#[doc = "Transmitted good frames single collision mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGFSCM {
    #[doc = "0: Transmitted-good-single-collision half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Transmitted-good-single-collision half-full interrupt disabled"]
    Masked = 1,
}
impl From<TGFSCM> for bool {
    #[inline(always)]
    fn from(variant: TGFSCM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFSCM` reader - Transmitted good frames single collision mask"]
pub type TGFSCM_R = crate::BitReader<TGFSCM>;
impl TGFSCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TGFSCM {
        match self.bits {
            false => TGFSCM::Unmasked,
            true => TGFSCM::Masked,
        }
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFSCM::Unmasked
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFSCM::Masked
    }
}
#[doc = "Field `TGFSCM` writer - Transmitted good frames single collision mask"]
pub type TGFSCM_W<'a, REG> = crate::BitWriter<'a, REG, TGFSCM>;
impl<'a, REG> TGFSCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFSCM::Unmasked)
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFSCM::Masked)
    }
}
#[doc = "Transmitted good frames more than single collision mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGFMSCM {
    #[doc = "0: Transmitted-good-multiple-collision half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Transmitted-good-multiple-collision half-full interrupt disabled"]
    Masked = 1,
}
impl From<TGFMSCM> for bool {
    #[inline(always)]
    fn from(variant: TGFMSCM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFMSCM` reader - Transmitted good frames more than single collision mask"]
pub type TGFMSCM_R = crate::BitReader<TGFMSCM>;
impl TGFMSCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TGFMSCM {
        match self.bits {
            false => TGFMSCM::Unmasked,
            true => TGFMSCM::Masked,
        }
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFMSCM::Unmasked
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFMSCM::Masked
    }
}
#[doc = "Field `TGFMSCM` writer - Transmitted good frames more than single collision mask"]
pub type TGFMSCM_W<'a, REG> = crate::BitWriter<'a, REG, TGFMSCM>;
impl<'a, REG> TGFMSCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFMSCM::Unmasked)
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFMSCM::Masked)
    }
}
#[doc = "Transmitted good frames mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TGFM {
    #[doc = "0: Transmitted-good counter half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Transmitted-good counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<TGFM> for bool {
    #[inline(always)]
    fn from(variant: TGFM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFM` reader - Transmitted good frames mask"]
pub type TGFM_R = crate::BitReader<TGFM>;
impl TGFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TGFM {
        match self.bits {
            false => TGFM::Unmasked,
            true => TGFM::Masked,
        }
    }
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFM::Unmasked
    }
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFM::Masked
    }
}
#[doc = "Field `TGFM` writer - Transmitted good frames mask"]
pub type TGFM_W<'a, REG> = crate::BitWriter<'a, REG, TGFM>;
impl<'a, REG> TGFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFM::Unmasked)
    }
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TGFM::Masked)
    }
}
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfscm(&mut self) -> TGFSCM_W<MMCTIMRrs> {
        TGFSCM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W<MMCTIMRrs> {
        TGFMSCM_W::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfm(&mut self) -> TGFM_W<MMCTIMRrs> {
        TGFM_W::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTIMRrs;
impl crate::RegisterSpec for MMCTIMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctimr::R`](R) reader structure"]
impl crate::Readable for MMCTIMRrs {}
#[doc = "`write(|w| ..)` method takes [`mmctimr::W`](W) writer structure"]
impl crate::Writable for MMCTIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCTIMR to value 0"]
impl crate::Resettable for MMCTIMRrs {
    const RESET_VALUE: u32 = 0;
}
