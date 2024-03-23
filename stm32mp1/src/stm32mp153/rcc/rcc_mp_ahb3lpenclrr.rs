#[doc = "Register `RCC_MP_AHB3LPENCLRR` reader"]
pub type R = crate::R<RCC_MP_AHB3LPENCLRRrs>;
#[doc = "Register `RCC_MP_AHB3LPENCLRR` writer"]
pub type W = crate::W<RCC_MP_AHB3LPENCLRRrs>;
#[doc = "Field `DCMILPEN` reader - DCMILPEN"]
pub type DCMILPEN_R = crate::BitReader;
#[doc = "Field `DCMILPEN` writer - DCMILPEN"]
pub type DCMILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYP2LPEN` reader - CRYP2LPEN"]
pub type CRYP2LPEN_R = crate::BitReader;
#[doc = "Field `CRYP2LPEN` writer - CRYP2LPEN"]
pub type CRYP2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASH2LPEN` reader - HASH2LPEN"]
pub type HASH2LPEN_R = crate::BitReader;
#[doc = "Field `HASH2LPEN` writer - HASH2LPEN"]
pub type HASH2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG2LPEN` reader - RNG2LPEN"]
pub type RNG2LPEN_R = crate::BitReader;
#[doc = "Field `RNG2LPEN` writer - RNG2LPEN"]
pub type RNG2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC2LPEN` reader - CRC2LPEN"]
pub type CRC2LPEN_R = crate::BitReader;
#[doc = "Field `CRC2LPEN` writer - CRC2LPEN"]
pub type CRC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEMLPEN` reader - HSEMLPEN"]
pub type HSEMLPEN_R = crate::BitReader;
#[doc = "Field `HSEMLPEN` writer - HSEMLPEN"]
pub type HSEMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCCLPEN` reader - IPCCLPEN"]
pub type IPCCLPEN_R = crate::BitReader;
#[doc = "Field `IPCCLPEN` writer - IPCCLPEN"]
pub type IPCCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    pub fn cryp2lpen(&self) -> CRYP2LPEN_R {
        CRYP2LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    pub fn hash2lpen(&self) -> HASH2LPEN_R {
        HASH2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    pub fn rng2lpen(&self) -> RNG2LPEN_R {
        RNG2LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    pub fn crc2lpen(&self) -> CRC2LPEN_R {
        CRC2LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    pub fn hsemlpen(&self) -> HSEMLPEN_R {
        HSEMLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    pub fn ipcclpen(&self) -> IPCCLPEN_R {
        IPCCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<RCC_MP_AHB3LPENCLRRrs> {
        DCMILPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cryp2lpen(&mut self) -> CRYP2LPEN_W<RCC_MP_AHB3LPENCLRRrs> {
        CRYP2LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hash2lpen(&mut self) -> HASH2LPEN_W<RCC_MP_AHB3LPENCLRRrs> {
        HASH2LPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn rng2lpen(&mut self) -> RNG2LPEN_W<RCC_MP_AHB3LPENCLRRrs> {
        RNG2LPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn crc2lpen(&mut self) -> CRC2LPEN_W<RCC_MP_AHB3LPENCLRRrs> {
        CRC2LPEN_W::new(self, 7)
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsemlpen(&mut self) -> HSEMLPEN_W<RCC_MP_AHB3LPENCLRRrs> {
        HSEMLPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ipcclpen(&mut self) -> IPCCLPEN_W<RCC_MP_AHB3LPENCLRRrs> {
        IPCCLPEN_W::new(self, 12)
    }
}
#[doc = "This register is used by the MPU in order to clear the PERxLPEN bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb3lpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb3lpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_AHB3LPENCLRRrs;
impl crate::RegisterSpec for RCC_MP_AHB3LPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_ahb3lpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_AHB3LPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_ahb3lpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_AHB3LPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB3LPENCLRR to value 0x18f1"]
impl crate::Resettable for RCC_MP_AHB3LPENCLRRrs {
    const RESET_VALUE: u32 = 0x18f1;
}
