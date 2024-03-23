#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - AES enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - AES enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DATATYPE_R = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - AES operating mode"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - AES operating mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHMOD` reader - AES chaining mode"]
pub type CHMOD_R = crate::FieldReader;
#[doc = "Field `CHMOD` writer - AES chaining mode"]
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCFC` reader - Computation Complete Flag Clear"]
pub type CCFC_R = crate::BitReader;
#[doc = "Field `CCFC` writer - Computation Complete Flag Clear"]
pub type CCFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` reader - Error clear"]
pub type ERRC_R = crate::BitReader;
#[doc = "Field `ERRC` writer - Error clear"]
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFIE` reader - CCF flag interrupt enable"]
pub type CCFIE_R = crate::BitReader;
#[doc = "Field `CCFIE` writer - CCF flag interrupt enable"]
pub type CCFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINEN` reader - Enable DMA management of data input phase"]
pub type DMAINEN_R = crate::BitReader;
#[doc = "Field `DMAINEN` writer - Enable DMA management of data input phase"]
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOUTEN` reader - Enable DMA management of data output phase"]
pub type DMAOUTEN_R = crate::BitReader;
#[doc = "Field `DMAOUTEN` writer - Enable DMA management of data output phase"]
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccfc(&mut self) -> CCFC_W<CRrs> {
        CCFC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<CRrs> {
        ERRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CCFIE_W<CRrs> {
        CCFIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
