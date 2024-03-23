#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `DINIS` reader - Data input interrupt status"]
pub type DINIS_R = crate::BitReader;
#[doc = "Field `DINIS` writer - Data input interrupt status"]
pub type DINIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIS` reader - Digest calculation completion interrupt status"]
pub type DCIS_R = crate::BitReader;
#[doc = "Field `DCIS` writer - Digest calculation completion interrupt status"]
pub type DCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAS` reader - DMA Status"]
pub type DMAS_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data input interrupt status"]
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status"]
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Status"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn dinis(&mut self) -> DINIS_W<SRrs> {
        DINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn dcis(&mut self) -> DCIS_W<SRrs> {
        DCIS_W::new(self, 1)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
