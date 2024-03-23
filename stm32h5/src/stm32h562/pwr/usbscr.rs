#[doc = "Register `USBSCR` reader"]
pub type R = crate::R<USBSCRrs>;
#[doc = "Register `USBSCR` writer"]
pub type W = crate::W<USBSCRrs>;
#[doc = "Field `USB33DEN` reader - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable"]
pub type USB33DEN_R = crate::BitReader;
#[doc = "Field `USB33DEN` writer - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable"]
pub type USB33DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB33SV` reader - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not."]
pub type USB33SV_R = crate::BitReader;
#[doc = "Field `USB33SV` writer - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not."]
pub type USB33SV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable"]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not."]
    #[inline(always)]
    pub fn usb33sv(&self) -> USB33SV_R {
        USB33SV_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb33den(&mut self) -> USB33DEN_W<USBSCRrs> {
        USB33DEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not."]
    #[inline(always)]
    #[must_use]
    pub fn usb33sv(&mut self) -> USB33SV_W<USBSCRrs> {
        USB33SV_W::new(self, 25)
    }
}
#[doc = "PWR USB supply control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBSCRrs;
impl crate::RegisterSpec for USBSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbscr::R`](R) reader structure"]
impl crate::Readable for USBSCRrs {}
#[doc = "`write(|w| ..)` method takes [`usbscr::W`](W) writer structure"]
impl crate::Writable for USBSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBSCR to value 0"]
impl crate::Resettable for USBSCRrs {
    const RESET_VALUE: u32 = 0;
}
