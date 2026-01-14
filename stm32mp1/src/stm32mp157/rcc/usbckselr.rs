///Register `USBCKSELR` reader
pub type R = crate::R<USBCKSELRrs>;
///Register `USBCKSELR` writer
pub type W = crate::W<USBCKSELRrs>;
///Field `USBPHYSRC` reader - USBPHYSRC
pub type USBPHYSRC_R = crate::FieldReader;
///Field `USBPHYSRC` writer - USBPHYSRC
pub type USBPHYSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USBOSRC` reader - USBOSRC
pub type USBOSRC_R = crate::BitReader;
///Field `USBOSRC` writer - USBOSRC
pub type USBOSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - USBPHYSRC
    #[inline(always)]
    pub fn usbphysrc(&self) -> USBPHYSRC_R {
        USBPHYSRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - USBOSRC
    #[inline(always)]
    pub fn usbosrc(&self) -> USBOSRC_R {
        USBOSRC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCKSELR")
            .field("usbphysrc", &self.usbphysrc())
            .field("usbosrc", &self.usbosrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USBPHYSRC
    #[inline(always)]
    pub fn usbphysrc(&mut self) -> USBPHYSRC_W<'_, USBCKSELRrs> {
        USBPHYSRC_W::new(self, 0)
    }
    ///Bit 4 - USBOSRC
    #[inline(always)]
    pub fn usbosrc(&mut self) -> USBOSRC_W<'_, USBCKSELRrs> {
        USBOSRC_W::new(self, 4)
    }
}
/**This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG

You can [`read`](crate::Reg::read) this register and get [`usbckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:USBCKSELR)*/
pub struct USBCKSELRrs;
impl crate::RegisterSpec for USBCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`usbckselr::R`](R) reader structure
impl crate::Readable for USBCKSELRrs {}
///`write(|w| ..)` method takes [`usbckselr::W`](W) writer structure
impl crate::Writable for USBCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBCKSELR to value 0
impl crate::Resettable for USBCKSELRrs {}
