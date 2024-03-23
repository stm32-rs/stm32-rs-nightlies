#[doc = "Register `RCC_USBCKSELR` reader"]
pub type R = crate::R<RCC_USBCKSELRrs>;
#[doc = "Register `RCC_USBCKSELR` writer"]
pub type W = crate::W<RCC_USBCKSELRrs>;
#[doc = "Field `USBPHYSRC` reader - USBPHYSRC"]
pub type USBPHYSRC_R = crate::FieldReader;
#[doc = "Field `USBPHYSRC` writer - USBPHYSRC"]
pub type USBPHYSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBOSRC` reader - USBOSRC"]
pub type USBOSRC_R = crate::BitReader;
#[doc = "Field `USBOSRC` writer - USBOSRC"]
pub type USBOSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&self) -> USBPHYSRC_R {
        USBPHYSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&self) -> USBOSRC_R {
        USBOSRC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    #[must_use]
    pub fn usbphysrc(&mut self) -> USBPHYSRC_W<RCC_USBCKSELRrs> {
        USBPHYSRC_W::new(self, 0)
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    #[must_use]
    pub fn usbosrc(&mut self) -> USBOSRC_W<RCC_USBCKSELRrs> {
        USBOSRC_W::new(self, 4)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_usbckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_usbckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_USBCKSELRrs;
impl crate::RegisterSpec for RCC_USBCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_usbckselr::R`](R) reader structure"]
impl crate::Readable for RCC_USBCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_usbckselr::W`](W) writer structure"]
impl crate::Writable for RCC_USBCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_USBCKSELR to value 0"]
impl crate::Resettable for RCC_USBCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
