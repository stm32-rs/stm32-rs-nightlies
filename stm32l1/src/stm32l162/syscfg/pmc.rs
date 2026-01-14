///Register `PMC` reader
pub type R = crate::R<PMCrs>;
///Register `PMC` writer
pub type W = crate::W<PMCrs>;
///Field `USB_PU` reader - USB pull-up
pub type USB_PU_R = crate::BitReader;
///Field `USB_PU` writer - USB pull-up
pub type USB_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCD_CAPA` reader - decoupling capacitance connection
pub type LCD_CAPA_R = crate::FieldReader;
///Field `LCD_CAPA` writer - decoupling capacitance connection
pub type LCD_CAPA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - USB pull-up
    #[inline(always)]
    pub fn usb_pu(&self) -> USB_PU_R {
        USB_PU_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:5 - decoupling capacitance connection
    #[inline(always)]
    pub fn lcd_capa(&self) -> LCD_CAPA_R {
        LCD_CAPA_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMC")
            .field("usb_pu", &self.usb_pu())
            .field("lcd_capa", &self.lcd_capa())
            .finish()
    }
}
impl W {
    ///Bit 0 - USB pull-up
    #[inline(always)]
    pub fn usb_pu(&mut self) -> USB_PU_W<'_, PMCrs> {
        USB_PU_W::new(self, 0)
    }
    ///Bits 1:5 - decoupling capacitance connection
    #[inline(always)]
    pub fn lcd_capa(&mut self) -> LCD_CAPA_W<'_, PMCrs> {
        LCD_CAPA_W::new(self, 1)
    }
}
/**peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#SYSCFG:PMC)*/
pub struct PMCrs;
impl crate::RegisterSpec for PMCrs {
    type Ux = u32;
}
///`read()` method returns [`pmc::R`](R) reader structure
impl crate::Readable for PMCrs {}
///`write(|w| ..)` method takes [`pmc::W`](W) writer structure
impl crate::Writable for PMCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMC to value 0
impl crate::Resettable for PMCrs {}
