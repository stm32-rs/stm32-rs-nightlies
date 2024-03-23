#[doc = "Register `PCR` reader"]
pub type R = crate::R<PCRrs>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PCRrs>;
#[doc = "Field `ETTXE` reader - ETTXE"]
pub type ETTXE_R = crate::BitReader;
#[doc = "Field `ETTXE` writer - ETTXE"]
pub type ETTXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRXE` reader - ETRXE"]
pub type ETRXE_R = crate::BitReader;
#[doc = "Field `ETRXE` writer - ETRXE"]
pub type ETRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTAE` reader - BTAE"]
pub type BTAE_R = crate::BitReader;
#[doc = "Field `BTAE` writer - BTAE"]
pub type BTAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCRXE` reader - ECCRXE"]
pub type ECCRXE_R = crate::BitReader;
#[doc = "Field `ECCRXE` writer - ECCRXE"]
pub type ECCRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRXE` reader - CRCRXE"]
pub type CRCRXE_R = crate::BitReader;
#[doc = "Field `CRCRXE` writer - CRCRXE"]
pub type CRCRXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    #[must_use]
    pub fn ettxe(&mut self) -> ETTXE_W<PCRrs> {
        ETTXE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    #[must_use]
    pub fn etrxe(&mut self) -> ETRXE_W<PCRrs> {
        ETRXE_W::new(self, 1)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    #[must_use]
    pub fn btae(&mut self) -> BTAE_W<PCRrs> {
        BTAE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    #[must_use]
    pub fn eccrxe(&mut self) -> ECCRXE_W<PCRrs> {
        ECCRXE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    #[must_use]
    pub fn crcrxe(&mut self) -> CRCRXE_W<PCRrs> {
        CRCRXE_W::new(self, 4)
    }
}
#[doc = "DSI Host protocol configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PCRrs {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0;
}
