#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - PKA Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - PKA Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start the operation"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start the operation"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - PKA operation code"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - PKA operation code"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PROCENDIE` reader - End of operation interrupt enable"]
pub type PROCENDIE_R = crate::BitReader;
#[doc = "Field `PROCENDIE` writer - End of operation interrupt enable"]
pub type PROCENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERRIE` reader - RAM error interrupt enable"]
pub type RAMERRIE_R = crate::BitReader;
#[doc = "Field `RAMERRIE` writer - RAM error interrupt enable"]
pub type RAMERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRERRIE` reader - Address error interrupt enable"]
pub type ADDRERRIE_R = crate::BitReader;
#[doc = "Field `ADDRERRIE` writer - Address error interrupt enable"]
pub type ADDRERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PKA Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start the operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn procendie(&self) -> PROCENDIE_R {
        PROCENDIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    pub fn ramerrie(&self) -> RAMERRIE_R {
        RAMERRIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    pub fn addrerrie(&self) -> ADDRERRIE_R {
        ADDRERRIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start the operation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 1)
    }
    #[doc = "Bits 8:13 - PKA operation code"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 8)
    }
    #[doc = "Bit 17 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn procendie(&mut self) -> PROCENDIE_W<CRrs> {
        PROCENDIE_W::new(self, 17)
    }
    #[doc = "Bit 19 - RAM error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrie(&mut self) -> RAMERRIE_W<CRrs> {
        RAMERRIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Address error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrie(&mut self) -> ADDRERRIE_W<CRrs> {
        ADDRERRIE_W::new(self, 20)
    }
}
#[doc = "PKA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
