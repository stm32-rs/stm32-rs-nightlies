#[doc = "Register `CLCR` reader"]
pub type R = crate::R<CLCRrs>;
#[doc = "Register `CLCR` writer"]
pub type W = crate::W<CLCRrs>;
#[doc = "Field `DPCC` reader - D-PHY Clock Control"]
pub type DPCC_R = crate::BitReader;
#[doc = "Field `DPCC` writer - D-PHY Clock Control"]
pub type DPCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACR` reader - Automatic Clock lane Control"]
pub type ACR_R = crate::BitReader;
#[doc = "Field `ACR` writer - Automatic Clock lane Control"]
pub type ACR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - D-PHY Clock Control"]
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic Clock lane Control"]
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D-PHY Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn dpcc(&mut self) -> DPCC_W<CLCRrs> {
        DPCC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Automatic Clock lane Control"]
    #[inline(always)]
    #[must_use]
    pub fn acr(&mut self) -> ACR_W<CLCRrs> {
        ACR_W::new(self, 1)
    }
}
#[doc = "DSI Host Clock Lane Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLCRrs;
impl crate::RegisterSpec for CLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clcr::R`](R) reader structure"]
impl crate::Readable for CLCRrs {}
#[doc = "`write(|w| ..)` method takes [`clcr::W`](W) writer structure"]
impl crate::Writable for CLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLCR to value 0"]
impl crate::Resettable for CLCRrs {
    const RESET_VALUE: u32 = 0;
}
