#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DMACRrs>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DMACRrs>;
#[doc = "Field `DIEN` reader - DMA input enable"]
pub type DIEN_R = crate::BitReader;
#[doc = "Field `DIEN` writer - DMA input enable"]
pub type DIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOEN` reader - DMA output enable"]
pub type DOEN_R = crate::BitReader;
#[doc = "Field `DOEN` writer - DMA output enable"]
pub type DOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA input enable"]
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA output enable"]
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA input enable"]
    #[inline(always)]
    #[must_use]
    pub fn dien(&mut self) -> DIEN_W<DMACRrs> {
        DIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA output enable"]
    #[inline(always)]
    #[must_use]
    pub fn doen(&mut self) -> DOEN_W<DMACRrs> {
        DOEN_W::new(self, 1)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRrs;
impl crate::RegisterSpec for DMACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DMACRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DMACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DMACRrs {
    const RESET_VALUE: u32 = 0;
}
