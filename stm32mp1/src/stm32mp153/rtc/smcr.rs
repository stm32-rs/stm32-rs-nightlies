#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCRrs>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCRrs>;
#[doc = "Field `ALRADPROT` reader - ALRADPROT"]
pub type ALRADPROT_R = crate::BitReader;
#[doc = "Field `ALRADPROT` writer - ALRADPROT"]
pub type ALRADPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBDPROT` reader - ALRBDPROT"]
pub type ALRBDPROT_R = crate::BitReader;
#[doc = "Field `ALRBDPROT` writer - ALRBDPROT"]
pub type ALRBDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTDPROT` reader - WUTDPROT"]
pub type WUTDPROT_R = crate::BitReader;
#[doc = "Field `WUTDPROT` writer - WUTDPROT"]
pub type WUTDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSDPROT` reader - TSDPROT"]
pub type TSDPROT_R = crate::BitReader;
#[doc = "Field `TSDPROT` writer - TSDPROT"]
pub type TSDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALDPROT` reader - CALDPROT"]
pub type CALDPROT_R = crate::BitReader;
#[doc = "Field `CALDPROT` writer - CALDPROT"]
pub type CALDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITDPROT` reader - INITDPROT"]
pub type INITDPROT_R = crate::BitReader;
#[doc = "Field `INITDPROT` writer - INITDPROT"]
pub type INITDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECPROT` reader - DECPROT"]
pub type DECPROT_R = crate::BitReader;
#[doc = "Field `DECPROT` writer - DECPROT"]
pub type DECPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    pub fn alradprot(&self) -> ALRADPROT_R {
        ALRADPROT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    pub fn alrbdprot(&self) -> ALRBDPROT_R {
        ALRBDPROT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    pub fn wutdprot(&self) -> WUTDPROT_R {
        WUTDPROT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    pub fn tsdprot(&self) -> TSDPROT_R {
        TSDPROT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    pub fn caldprot(&self) -> CALDPROT_R {
        CALDPROT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    pub fn initdprot(&self) -> INITDPROT_R {
        INITDPROT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    pub fn decprot(&self) -> DECPROT_R {
        DECPROT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    #[must_use]
    pub fn alradprot(&mut self) -> ALRADPROT_W<SMCRrs> {
        ALRADPROT_W::new(self, 0)
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn alrbdprot(&mut self) -> ALRBDPROT_W<SMCRrs> {
        ALRBDPROT_W::new(self, 1)
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn wutdprot(&mut self) -> WUTDPROT_W<SMCRrs> {
        WUTDPROT_W::new(self, 2)
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn tsdprot(&mut self) -> TSDPROT_W<SMCRrs> {
        TSDPROT_W::new(self, 3)
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn caldprot(&mut self) -> CALDPROT_W<SMCRrs> {
        CALDPROT_W::new(self, 13)
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    #[must_use]
    pub fn initdprot(&mut self) -> INITDPROT_W<SMCRrs> {
        INITDPROT_W::new(self, 14)
    }
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    #[must_use]
    pub fn decprot(&mut self) -> DECPROT_W<SMCRrs> {
        DECPROT_W::new(self, 15)
    }
}
#[doc = "This register can be written only when the APB access is secure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCRrs {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0xe00f"]
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0xe00f;
}
