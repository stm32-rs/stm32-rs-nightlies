#[doc = "Register `PMC` reader"]
pub type R = crate::R<PMCrs>;
#[doc = "Register `PMC` writer"]
pub type W = crate::W<PMCrs>;
#[doc = "Field `USB_PU` reader - USB pull-up"]
pub type USB_PU_R = crate::BitReader;
#[doc = "Field `USB_PU` writer - USB pull-up"]
pub type USB_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CAPA` reader - decoupling capacitance connection"]
pub type LCD_CAPA_R = crate::FieldReader;
#[doc = "Field `LCD_CAPA` writer - decoupling capacitance connection"]
pub type LCD_CAPA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    pub fn usb_pu(&self) -> USB_PU_R {
        USB_PU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    pub fn lcd_capa(&self) -> LCD_CAPA_R {
        LCD_CAPA_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pu(&mut self) -> USB_PU_W<PMCrs> {
        USB_PU_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_capa(&mut self) -> LCD_CAPA_W<PMCrs> {
        LCD_CAPA_W::new(self, 1)
    }
}
#[doc = "peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCrs;
impl crate::RegisterSpec for PMCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc::R`](R) reader structure"]
impl crate::Readable for PMCrs {}
#[doc = "`write(|w| ..)` method takes [`pmc::W`](W) writer structure"]
impl crate::Writable for PMCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMCrs {
    const RESET_VALUE: u32 = 0;
}
