#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMRrs>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMRrs>;
#[doc = "Field `DINIE` reader - Data input interrupt enable"]
pub type DINIE_R = crate::BitReader;
#[doc = "Field `DINIE` writer - Data input interrupt enable"]
pub type DINIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIE` reader - Digest calculation completion interrupt enable"]
pub type DCIE_R = crate::BitReader;
#[doc = "Field `DCIE` writer - Digest calculation completion interrupt enable"]
pub type DCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn dinie(&self) -> DINIE_R {
        DINIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    pub fn dcie(&self) -> DCIE_R {
        DCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dinie(&mut self) -> DINIE_W<IMRrs> {
        DINIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcie(&mut self) -> DCIE_W<IMRrs> {
        DCIE_W::new(self, 1)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMRrs {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMRrs {
    const RESET_VALUE: u32 = 0;
}
