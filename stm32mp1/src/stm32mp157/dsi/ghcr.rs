#[doc = "Register `GHCR` reader"]
pub type R = crate::R<GHCRrs>;
#[doc = "Register `GHCR` writer"]
pub type W = crate::W<GHCRrs>;
#[doc = "Field `DT` reader - DT"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - DT"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VCID` reader - VCID"]
pub type VCID_R = crate::FieldReader;
#[doc = "Field `VCID` writer - VCID"]
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WCLSB` reader - WCLSB"]
pub type WCLSB_R = crate::FieldReader;
#[doc = "Field `WCLSB` writer - WCLSB"]
pub type WCLSB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WCMSB` reader - WCMSB"]
pub type WCMSB_R = crate::FieldReader;
#[doc = "Field `WCMSB` writer - WCMSB"]
pub type WCMSB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - WCLSB"]
    #[inline(always)]
    pub fn wclsb(&self) -> WCLSB_R {
        WCLSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - WCMSB"]
    #[inline(always)]
    pub fn wcmsb(&self) -> WCMSB_R {
        WCMSB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DT"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<GHCRrs> {
        DT_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - VCID"]
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<GHCRrs> {
        VCID_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - WCLSB"]
    #[inline(always)]
    #[must_use]
    pub fn wclsb(&mut self) -> WCLSB_W<GHCRrs> {
        WCLSB_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - WCMSB"]
    #[inline(always)]
    #[must_use]
    pub fn wcmsb(&mut self) -> WCMSB_W<GHCRrs> {
        WCMSB_W::new(self, 16)
    }
}
#[doc = "DSI Host generic header configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ghcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GHCRrs;
impl crate::RegisterSpec for GHCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghcr::R`](R) reader structure"]
impl crate::Readable for GHCRrs {}
#[doc = "`write(|w| ..)` method takes [`ghcr::W`](W) writer structure"]
impl crate::Writable for GHCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GHCR to value 0"]
impl crate::Resettable for GHCRrs {
    const RESET_VALUE: u32 = 0;
}
