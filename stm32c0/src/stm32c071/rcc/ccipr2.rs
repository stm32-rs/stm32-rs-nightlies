///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
///Field `USBSEL` reader - USB clock source selection Set and cleared by software.
pub type USBSEL_R = crate::BitReader;
///Field `USBSEL` writer - USB clock source selection Set and cleared by software.
pub type USBSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12 - USB clock source selection Set and cleared by software.
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("usbsel", &self.usbsel())
            .finish()
    }
}
impl W {
    ///Bit 12 - USB clock source selection Set and cleared by software.
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W<CCIPR2rs> {
        USBSEL_W::new(self, 12)
    }
}
/**RCC peripherals independent clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {
    const RESET_VALUE: u32 = 0;
}
