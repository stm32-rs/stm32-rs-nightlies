#[doc = "Register `CACR` reader"]
pub type R = crate::R<CACRrs>;
#[doc = "Register `CACR` writer"]
pub type W = crate::W<CACRrs>;
#[doc = "Field `SIWT` reader - SIWT"]
pub type SIWT_R = crate::BitReader;
#[doc = "Field `SIWT` writer - SIWT"]
pub type SIWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEWT` reader - FORCEWT"]
pub type FORCEWT_R = crate::BitReader;
#[doc = "Field `FORCEWT` writer - FORCEWT"]
pub type FORCEWT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SIWT"]
    #[inline(always)]
    pub fn siwt(&self) -> SIWT_R {
        SIWT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FORCEWT"]
    #[inline(always)]
    pub fn forcewt(&self) -> FORCEWT_R {
        FORCEWT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SIWT"]
    #[inline(always)]
    #[must_use]
    pub fn siwt(&mut self) -> SIWT_W<CACRrs> {
        SIWT_W::new(self, 0)
    }
    #[doc = "Bit 1 - ECCEN"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<CACRrs> {
        ECCEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - FORCEWT"]
    #[inline(always)]
    #[must_use]
    pub fn forcewt(&mut self) -> FORCEWT_W<CACRrs> {
        FORCEWT_W::new(self, 2)
    }
}
#[doc = "Auxiliary Cache Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACRrs;
impl crate::RegisterSpec for CACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacr::R`](R) reader structure"]
impl crate::Readable for CACRrs {}
#[doc = "`write(|w| ..)` method takes [`cacr::W`](W) writer structure"]
impl crate::Writable for CACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACRrs {
    const RESET_VALUE: u32 = 0;
}
