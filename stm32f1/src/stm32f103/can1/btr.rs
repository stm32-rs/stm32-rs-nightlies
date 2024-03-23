#[doc = "Register `BTR` reader"]
pub type R = crate::R<BTRrs>;
#[doc = "Register `BTR` writer"]
pub type W = crate::W<BTRrs>;
#[doc = "Field `BRP` reader - BRP"]
pub type BRP_R = crate::FieldReader<u16>;
#[doc = "Field `BRP` writer - BRP"]
pub type BRP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TS1` reader - TS1"]
pub type TS1_R = crate::FieldReader;
#[doc = "Field `TS1` writer - TS1"]
pub type TS1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS2` reader - TS2"]
pub type TS2_R = crate::FieldReader;
#[doc = "Field `TS2` writer - TS2"]
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SJW` reader - SJW"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - SJW"]
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "LBKM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKM {
    #[doc = "0: Loop Back Mode disabled"]
    Disabled = 0,
    #[doc = "1: Loop Back Mode enabled"]
    Enabled = 1,
}
impl From<LBKM> for bool {
    #[inline(always)]
    fn from(variant: LBKM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKM` reader - LBKM"]
pub type LBKM_R = crate::BitReader<LBKM>;
impl LBKM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBKM {
        match self.bits {
            false => LBKM::Disabled,
            true => LBKM::Enabled,
        }
    }
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBKM::Disabled
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBKM::Enabled
    }
}
#[doc = "Field `LBKM` writer - LBKM"]
pub type LBKM_W<'a, REG> = crate::BitWriter<'a, REG, LBKM>;
impl<'a, REG> LBKM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop Back Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBKM::Disabled)
    }
    #[doc = "Loop Back Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBKM::Enabled)
    }
}
#[doc = "SILM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SILM {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Silent Mode"]
    Silent = 1,
}
impl From<SILM> for bool {
    #[inline(always)]
    fn from(variant: SILM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SILM` reader - SILM"]
pub type SILM_R = crate::BitReader<SILM>;
impl SILM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SILM {
        match self.bits {
            false => SILM::Normal,
            true => SILM::Silent,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SILM::Normal
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == SILM::Silent
    }
}
#[doc = "Field `SILM` writer - SILM"]
pub type SILM_W<'a, REG> = crate::BitWriter<'a, REG, SILM>;
impl<'a, REG> SILM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SILM::Normal)
    }
    #[doc = "Silent Mode"]
    #[inline(always)]
    pub fn silent(self) -> &'a mut crate::W<REG> {
        self.variant(SILM::Silent)
    }
}
impl R {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<BTRrs> {
        BRP_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<BTRrs> {
        TS1_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<BTRrs> {
        TS2_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BTRrs> {
        SJW_W::new(self, 24)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    #[must_use]
    pub fn lbkm(&mut self) -> LBKM_W<BTRrs> {
        LBKM_W::new(self, 30)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    #[must_use]
    pub fn silm(&mut self) -> SILM_W<BTRrs> {
        SILM_W::new(self, 31)
    }
}
#[doc = "CAN_BTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTRrs;
impl crate::RegisterSpec for BTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr::R`](R) reader structure"]
impl crate::Readable for BTRrs {}
#[doc = "`write(|w| ..)` method takes [`btr::W`](W) writer structure"]
impl crate::Writable for BTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR to value 0"]
impl crate::Resettable for BTRrs {
    const RESET_VALUE: u32 = 0;
}
