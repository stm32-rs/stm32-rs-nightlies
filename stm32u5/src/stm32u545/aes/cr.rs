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
#[doc = "Field `DMAINEN` reader - Enable DMA management of data input phase"]
pub type DMAINEN_R = crate::BitReader;
#[doc = "Field `DMAINEN` writer - Enable DMA management of data input phase"]
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOUTEN` reader - Enable DMA management of data output phase"]
pub type DMAOUTEN_R = crate::BitReader;
#[doc = "Field `DMAOUTEN` writer - Enable DMA management of data output phase"]
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCMPH` reader - GCMPH"]
pub type GCMPH_R = crate::FieldReader;
#[doc = "Field `GCMPH` writer - GCMPH"]
pub type GCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHMOD_2` reader - CHMOD_2"]
pub type CHMOD_2_R = crate::BitReader;
#[doc = "Field `CHMOD_2` writer - CHMOD_2"]
pub type CHMOD_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYSIZE` reader - KEYSIZE"]
pub type KEYSIZE_R = crate::BitReader;
#[doc = "Field `KEYSIZE` writer - KEYSIZE"]
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPBLB` reader - NPBLB"]
pub type NPBLB_R = crate::FieldReader;
#[doc = "Field `NPBLB` writer - NPBLB"]
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KMOD` reader - KMOD"]
pub type KMOD_R = crate::FieldReader;
#[doc = "Field `KMOD` writer - KMOD"]
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IPRST` reader - IPRST"]
pub type IPRST_R = crate::BitReader;
#[doc = "Field `IPRST` writer - IPRST"]
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 13:14 - GCMPH"]
    #[inline(always)]
    pub fn gcmph(&self) -> GCMPH_R {
        GCMPH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - CHMOD_2"]
    #[inline(always)]
    pub fn chmod_2(&self) -> CHMOD_2_R {
        CHMOD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - KEYSIZE"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - NPBLB"]
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - KMOD"]
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - IPRST"]
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
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
    #[doc = "Bits 13:14 - GCMPH"]
    #[inline(always)]
    #[must_use]
    pub fn gcmph(&mut self) -> GCMPH_W<CRrs> {
        GCMPH_W::new(self, 13)
    }
    #[doc = "Bit 16 - CHMOD_2"]
    #[inline(always)]
    #[must_use]
    pub fn chmod_2(&mut self) -> CHMOD_2_W<CRrs> {
        CHMOD_2_W::new(self, 16)
    }
    #[doc = "Bit 18 - KEYSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - NPBLB"]
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NPBLB_W<CRrs> {
        NPBLB_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - KMOD"]
    #[inline(always)]
    #[must_use]
    pub fn kmod(&mut self) -> KMOD_W<CRrs> {
        KMOD_W::new(self, 24)
    }
    #[doc = "Bit 31 - IPRST"]
    #[inline(always)]
    #[must_use]
    pub fn iprst(&mut self) -> IPRST_W<CRrs> {
        IPRST_W::new(self, 31)
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
