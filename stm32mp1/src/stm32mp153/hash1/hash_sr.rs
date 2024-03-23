#[doc = "Register `HASH_SR` reader"]
pub type R = crate::R<HASH_SRrs>;
#[doc = "Register `HASH_SR` writer"]
pub type W = crate::W<HASH_SRrs>;
#[doc = "Field `DINIS` reader - DINIS"]
pub type DINIS_R = crate::BitReader;
#[doc = "Field `DINIS` writer - DINIS"]
pub type DINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIS` reader - DCIS"]
pub type DCIS_R = crate::BitReader;
#[doc = "Field `DCIS` writer - DCIS"]
pub type DCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAS` reader - DMAS"]
pub type DMAS_R = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DINIS"]
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCIS"]
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAS"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DINIS"]
    #[inline(always)]
    #[must_use]
    pub fn dinis(&mut self) -> DINIS_W<HASH_SRrs> {
        DINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCIS"]
    #[inline(always)]
    #[must_use]
    pub fn dcis(&mut self) -> DCIS_W<HASH_SRrs> {
        DCIS_W::new(self, 1)
    }
}
#[doc = "HASH status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_SRrs;
impl crate::RegisterSpec for HASH_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_sr::R`](R) reader structure"]
impl crate::Readable for HASH_SRrs {}
#[doc = "`write(|w| ..)` method takes [`hash_sr::W`](W) writer structure"]
impl crate::Writable for HASH_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_SR to value 0x01"]
impl crate::Resettable for HASH_SRrs {
    const RESET_VALUE: u32 = 0x01;
}
