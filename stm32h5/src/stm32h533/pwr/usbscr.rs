///Register `USBSCR` reader
pub type R = crate::R<USBSCRrs>;
///Register `USBSCR` writer
pub type W = crate::W<USBSCRrs>;
///Field `USB33DEN` reader - Vless thansub>DDUSBless than/sub> voltage level detector enable
pub type USB33DEN_R = crate::BitReader;
///Field `USB33DEN` writer - Vless thansub>DDUSBless than/sub> voltage level detector enable
pub type USB33DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB33SV` reader - independent USB supply valid
pub type USB33SV_R = crate::BitReader;
///Field `USB33SV` writer - independent USB supply valid
pub type USB33SV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 24 - Vless thansub>DDUSBless than/sub> voltage level detector enable
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - independent USB supply valid
    #[inline(always)]
    pub fn usb33sv(&self) -> USB33SV_R {
        USB33SV_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBSCR")
            .field("usb33den", &self.usb33den())
            .field("usb33sv", &self.usb33sv())
            .finish()
    }
}
impl W {
    ///Bit 24 - Vless thansub>DDUSBless than/sub> voltage level detector enable
    #[inline(always)]
    pub fn usb33den(&mut self) -> USB33DEN_W<'_, USBSCRrs> {
        USB33DEN_W::new(self, 24)
    }
    ///Bit 25 - independent USB supply valid
    #[inline(always)]
    pub fn usb33sv(&mut self) -> USB33SV_W<'_, USBSCRrs> {
        USB33SV_W::new(self, 25)
    }
}
/**PWR USB supply control register

You can [`read`](crate::Reg::read) this register and get [`usbscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#PWR:USBSCR)*/
pub struct USBSCRrs;
impl crate::RegisterSpec for USBSCRrs {
    type Ux = u32;
}
///`read()` method returns [`usbscr::R`](R) reader structure
impl crate::Readable for USBSCRrs {}
///`write(|w| ..)` method takes [`usbscr::W`](W) writer structure
impl crate::Writable for USBSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBSCR to value 0
impl crate::Resettable for USBSCRrs {}
