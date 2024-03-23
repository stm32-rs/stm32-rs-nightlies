#[doc = "Register `RCC_MP_AHB3ENCLRR` reader"]
pub type R = crate::R<RCC_MP_AHB3ENCLRRrs>;
#[doc = "Register `RCC_MP_AHB3ENCLRR` writer"]
pub type W = crate::W<RCC_MP_AHB3ENCLRRrs>;
#[doc = "Field `DCMIEN` reader - DCMIEN"]
pub type DCMIEN_R = crate::BitReader;
#[doc = "Field `DCMIEN` writer - DCMIEN"]
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYP2EN` reader - CRYP2EN"]
pub type CRYP2EN_R = crate::BitReader;
#[doc = "Field `CRYP2EN` writer - CRYP2EN"]
pub type CRYP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH2EN` reader - HASH2EN"]
pub type HASH2EN_R = crate::BitReader;
#[doc = "Field `HASH2EN` writer - HASH2EN"]
pub type HASH2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG2EN` reader - RNG2EN"]
pub type RNG2EN_R = crate::BitReader;
#[doc = "Field `RNG2EN` writer - RNG2EN"]
pub type RNG2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC2EN` reader - CRC2EN"]
pub type CRC2EN_R = crate::BitReader;
#[doc = "Field `CRC2EN` writer - CRC2EN"]
pub type CRC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEMEN` reader - HSEMEN"]
pub type HSEMEN_R = crate::BitReader;
#[doc = "Field `HSEMEN` writer - HSEMEN"]
pub type HSEMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCCEN` reader - IPCCEN"]
pub type IPCCEN_R = crate::BitReader;
#[doc = "Field `IPCCEN` writer - IPCCEN"]
pub type IPCCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCMIEN"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP2EN"]
    #[inline(always)]
    pub fn cryp2en(&self) -> CRYP2EN_R {
        CRYP2EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH2EN"]
    #[inline(always)]
    pub fn hash2en(&self) -> HASH2EN_R {
        HASH2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG2EN"]
    #[inline(always)]
    pub fn rng2en(&self) -> RNG2EN_R {
        RNG2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC2EN"]
    #[inline(always)]
    pub fn crc2en(&self) -> CRC2EN_R {
        CRC2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMIEN"]
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<RCC_MP_AHB3ENCLRRrs> {
        DCMIEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYP2EN"]
    #[inline(always)]
    #[must_use]
    pub fn cryp2en(&mut self) -> CRYP2EN_W<RCC_MP_AHB3ENCLRRrs> {
        CRYP2EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH2EN"]
    #[inline(always)]
    #[must_use]
    pub fn hash2en(&mut self) -> HASH2EN_W<RCC_MP_AHB3ENCLRRrs> {
        HASH2EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG2EN"]
    #[inline(always)]
    #[must_use]
    pub fn rng2en(&mut self) -> RNG2EN_W<RCC_MP_AHB3ENCLRRrs> {
        RNG2EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC2EN"]
    #[inline(always)]
    #[must_use]
    pub fn crc2en(&mut self) -> CRC2EN_W<RCC_MP_AHB3ENCLRRrs> {
        CRC2EN_W::new(self, 7)
    }
    #[doc = "Bit 11 - HSEMEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<RCC_MP_AHB3ENCLRRrs> {
        HSEMEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - IPCCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<RCC_MP_AHB3ENCLRRrs> {
        IPCCEN_W::new(self, 12)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit of the corresponding peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb3enclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb3enclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_AHB3ENCLRRrs;
impl crate::RegisterSpec for RCC_MP_AHB3ENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_ahb3enclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_AHB3ENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_ahb3enclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_AHB3ENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB3ENCLRR to value 0"]
impl crate::Resettable for RCC_MP_AHB3ENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
