#[doc = "Register `PUCR` reader"]
pub type R = crate::R<PUCRrs>;
#[doc = "Register `PUCR` writer"]
pub type W = crate::W<PUCRrs>;
#[doc = "Field `URCL` reader - ULPS request on clock lane"]
pub type URCL_R = crate::BitReader;
#[doc = "Field `URCL` writer - ULPS request on clock lane"]
pub type URCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UECL` reader - ULPS exit on clock lane"]
pub type UECL_R = crate::BitReader;
#[doc = "Field `UECL` writer - ULPS exit on clock lane"]
pub type UECL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URDL` reader - ULPS request on data lane"]
pub type URDL_R = crate::BitReader;
#[doc = "Field `URDL` writer - ULPS request on data lane"]
pub type URDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEDL` reader - ULPS exit on data lane"]
pub type UEDL_R = crate::BitReader;
#[doc = "Field `UEDL` writer - ULPS exit on data lane"]
pub type UEDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ULPS request on clock lane"]
    #[inline(always)]
    pub fn urcl(&self) -> URCL_R {
        URCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ULPS exit on clock lane"]
    #[inline(always)]
    pub fn uecl(&self) -> UECL_R {
        UECL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ULPS request on data lane"]
    #[inline(always)]
    pub fn urdl(&self) -> URDL_R {
        URDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULPS exit on data lane"]
    #[inline(always)]
    pub fn uedl(&self) -> UEDL_R {
        UEDL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ULPS request on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn urcl(&mut self) -> URCL_W<PUCRrs> {
        URCL_W::new(self, 0)
    }
    #[doc = "Bit 1 - ULPS exit on clock lane"]
    #[inline(always)]
    #[must_use]
    pub fn uecl(&mut self) -> UECL_W<PUCRrs> {
        UECL_W::new(self, 1)
    }
    #[doc = "Bit 2 - ULPS request on data lane"]
    #[inline(always)]
    #[must_use]
    pub fn urdl(&mut self) -> URDL_W<PUCRrs> {
        URDL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ULPS exit on data lane"]
    #[inline(always)]
    #[must_use]
    pub fn uedl(&mut self) -> UEDL_W<PUCRrs> {
        UEDL_W::new(self, 3)
    }
}
#[doc = "DSI Host PHY ULPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUCRrs;
impl crate::RegisterSpec for PUCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pucr::R`](R) reader structure"]
impl crate::Readable for PUCRrs {}
#[doc = "`write(|w| ..)` method takes [`pucr::W`](W) writer structure"]
impl crate::Writable for PUCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUCR to value 0"]
impl crate::Resettable for PUCRrs {
    const RESET_VALUE: u32 = 0;
}
