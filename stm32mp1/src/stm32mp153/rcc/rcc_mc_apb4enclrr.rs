#[doc = "Register `RCC_MC_APB4ENCLRR` reader"]
pub type R = crate::R<RCC_MC_APB4ENCLRRrs>;
#[doc = "Register `RCC_MC_APB4ENCLRR` writer"]
pub type W = crate::W<RCC_MC_APB4ENCLRRrs>;
#[doc = "Field `LTDCEN` reader - LTDCEN"]
pub type LTDCEN_R = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LTDCEN"]
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIEN` reader - DSIEN"]
pub type DSIEN_R = crate::BitReader;
#[doc = "Field `DSIEN` writer - DSIEN"]
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRPERFMEN` reader - DDRPERFMEN"]
pub type DDRPERFMEN_R = crate::BitReader;
#[doc = "Field `DDRPERFMEN` writer - DDRPERFMEN"]
pub type DDRPERFMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPHYEN` reader - USBPHYEN"]
pub type USBPHYEN_R = crate::BitReader;
#[doc = "Field `USBPHYEN` writer - USBPHYEN"]
pub type USBPHYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STGENROEN` reader - STGENROEN"]
pub type STGENROEN_R = crate::BitReader;
#[doc = "Field `STGENROEN` writer - STGENROEN"]
pub type STGENROEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMEN"]
    #[inline(always)]
    pub fn ddrperfmen(&self) -> DDRPERFMEN_R {
        DDRPERFMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - USBPHYEN"]
    #[inline(always)]
    pub fn usbphyen(&self) -> USBPHYEN_R {
        USBPHYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENROEN"]
    #[inline(always)]
    pub fn stgenroen(&self) -> STGENROEN_R {
        STGENROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<RCC_MC_APB4ENCLRRrs> {
        LTDCEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - DSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<RCC_MC_APB4ENCLRRrs> {
        DSIEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - DDRPERFMEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrperfmen(&mut self) -> DDRPERFMEN_W<RCC_MC_APB4ENCLRRrs> {
        DDRPERFMEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - USBPHYEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbphyen(&mut self) -> USBPHYEN_W<RCC_MC_APB4ENCLRRrs> {
        USBPHYEN_W::new(self, 16)
    }
    #[doc = "Bit 20 - STGENROEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenroen(&mut self) -> STGENROEN_W<RCC_MC_APB4ENCLRRrs> {
        STGENROEN_W::new(self, 20)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mc_apb4enclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mc_apb4enclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MC_APB4ENCLRRrs;
impl crate::RegisterSpec for RCC_MC_APB4ENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mc_apb4enclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MC_APB4ENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mc_apb4enclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MC_APB4ENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MC_APB4ENCLRR to value 0"]
impl crate::Resettable for RCC_MC_APB4ENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
