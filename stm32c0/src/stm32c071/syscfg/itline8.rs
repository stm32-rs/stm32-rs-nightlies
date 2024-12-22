///Register `ITLINE8` reader
pub type R = crate::R<ITLINE8rs>;
///Field `USB` reader - USB interrupt request pending
pub type USB_R = crate::BitReader;
impl R {
    ///Bit 0 - USB interrupt request pending
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE8").field("usb", &self.usb()).finish()
    }
}
/**SYSCFG interrupt line 8 status register

You can [`read`](crate::Reg::read) this register and get [`itline8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#SYSCFG:ITLINE8)*/
pub struct ITLINE8rs;
impl crate::RegisterSpec for ITLINE8rs {
    type Ux = u32;
}
///`read()` method returns [`itline8::R`](R) reader structure
impl crate::Readable for ITLINE8rs {}
///`reset()` method sets ITLINE8 to value 0
impl crate::Resettable for ITLINE8rs {
    const RESET_VALUE: u32 = 0;
}
