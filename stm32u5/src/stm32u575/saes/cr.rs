#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - SAES enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - SAES enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATYPE` reader - DATATYPE"]
pub type DATATYPE_R = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - DATATYPE"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHMOD` reader - CHMOD"]
pub type CHMOD_R = crate::FieldReader;
#[doc = "Field `CHMOD` writer - CHMOD"]
pub type CHMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMAINEN` reader - DMAINEN"]
pub type DMAINEN_R = crate::BitReader;
#[doc = "Field `DMAINEN` writer - DMAINEN"]
pub type DMAINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOUTEN` reader - DMAOUTEN"]
pub type DMAOUTEN_R = crate::BitReader;
#[doc = "Field `DMAOUTEN` writer - DMAOUTEN"]
pub type DMAOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYSIZE` reader - KEYSIZE"]
pub type KEYSIZE_R = crate::BitReader;
#[doc = "Field `KEYSIZE` writer - KEYSIZE"]
pub type KEYSIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYPROT` reader - KEYPROT"]
pub type KEYPROT_R = crate::BitReader;
#[doc = "Field `KEYPROT` writer - KEYPROT"]
pub type KEYPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KMOD` reader - KMOD"]
pub type KMOD_R = crate::FieldReader;
#[doc = "Field `KMOD` writer - KMOD"]
pub type KMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KSHAREID` reader - KSHAREID"]
pub type KSHAREID_R = crate::FieldReader;
#[doc = "Field `KSHAREID` writer - KSHAREID"]
pub type KSHAREID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYSEL` reader - KEYSEL"]
pub type KEYSEL_R = crate::FieldReader;
#[doc = "Field `KEYSEL` writer - KEYSEL"]
pub type KEYSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IPRST` reader - IPRST"]
pub type IPRST_R = crate::BitReader;
#[doc = "Field `IPRST` writer - IPRST"]
pub type IPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SAES enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - DATATYPE"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - CHMOD"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 11 - DMAINEN"]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAOUTEN"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - KEYSIZE"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - KEYPROT"]
    #[inline(always)]
    pub fn keyprot(&self) -> KEYPROT_R {
        KEYPROT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:25 - KMOD"]
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - KSHAREID"]
    #[inline(always)]
    pub fn kshareid(&self) -> KSHAREID_R {
        KSHAREID_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - KEYSEL"]
    #[inline(always)]
    pub fn keysel(&self) -> KEYSEL_R {
        KEYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - IPRST"]
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAES enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - DATATYPE"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - CHMOD"]
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> CHMOD_W<CRrs> {
        CHMOD_W::new(self, 5)
    }
    #[doc = "Bit 11 - DMAINEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DMAINEN_W<CRrs> {
        DMAINEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - DMAOUTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<CRrs> {
        DMAOUTEN_W::new(self, 12)
    }
    #[doc = "Bit 18 - KEYSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRrs> {
        KEYSIZE_W::new(self, 18)
    }
    #[doc = "Bit 19 - KEYPROT"]
    #[inline(always)]
    #[must_use]
    pub fn keyprot(&mut self) -> KEYPROT_W<CRrs> {
        KEYPROT_W::new(self, 19)
    }
    #[doc = "Bits 24:25 - KMOD"]
    #[inline(always)]
    #[must_use]
    pub fn kmod(&mut self) -> KMOD_W<CRrs> {
        KMOD_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - KSHAREID"]
    #[inline(always)]
    #[must_use]
    pub fn kshareid(&mut self) -> KSHAREID_W<CRrs> {
        KSHAREID_W::new(self, 26)
    }
    #[doc = "Bits 28:30 - KEYSEL"]
    #[inline(always)]
    #[must_use]
    pub fn keysel(&mut self) -> KEYSEL_W<CRrs> {
        KEYSEL_W::new(self, 28)
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
