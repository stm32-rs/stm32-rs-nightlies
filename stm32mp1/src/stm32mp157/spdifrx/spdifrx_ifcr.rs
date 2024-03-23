#[doc = "Register `SPDIFRX_IFCR` reader"]
pub type R = crate::R<SPDIFRX_IFCRrs>;
#[doc = "Register `SPDIFRX_IFCR` writer"]
pub type W = crate::W<SPDIFRX_IFCRrs>;
#[doc = "Field `PERRCF` writer - PERRCF"]
pub type PERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - OVRCF"]
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBDCF` writer - SBDCF"]
pub type SBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDCF` writer - SYNCDCF"]
pub type SYNCDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - PERRCF"]
    #[inline(always)]
    #[must_use]
    pub fn perrcf(&mut self) -> PERRCF_W<SPDIFRX_IFCRrs> {
        PERRCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - OVRCF"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<SPDIFRX_IFCRrs> {
        OVRCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - SBDCF"]
    #[inline(always)]
    #[must_use]
    pub fn sbdcf(&mut self) -> SBDCF_W<SPDIFRX_IFCRrs> {
        SBDCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYNCDCF"]
    #[inline(always)]
    #[must_use]
    pub fn syncdcf(&mut self) -> SYNCDCF_W<SPDIFRX_IFCRrs> {
        SYNCDCF_W::new(self, 5)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_ifcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdifrx_ifcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDIFRX_IFCRrs;
impl crate::RegisterSpec for SPDIFRX_IFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdifrx_ifcr::R`](R) reader structure"]
impl crate::Readable for SPDIFRX_IFCRrs {}
#[doc = "`write(|w| ..)` method takes [`spdifrx_ifcr::W`](W) writer structure"]
impl crate::Writable for SPDIFRX_IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIFRX_IFCR to value 0"]
impl crate::Resettable for SPDIFRX_IFCRrs {
    const RESET_VALUE: u32 = 0;
}
