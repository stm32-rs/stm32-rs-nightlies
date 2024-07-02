///Register `LTDC_L2CFBAR` reader
pub type R = crate::R<LTDC_L2CFBARrs>;
///Register `LTDC_L2CFBAR` writer
pub type W = crate::W<LTDC_L2CFBARrs>;
///Field `CFBADD` reader - color frame buffer start address These bits define the color frame buffer start address.
pub type CFBADD_R = crate::FieldReader<u32>;
///Field `CFBADD` writer - color frame buffer start address These bits define the color frame buffer start address.
pub type CFBADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - color frame buffer start address These bits define the color frame buffer start address.
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_L2CFBAR")
            .field("cfbadd", &self.cfbadd())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - color frame buffer start address These bits define the color frame buffer start address.
    #[inline(always)]
    #[must_use]
    pub fn cfbadd(&mut self) -> CFBADD_W<LTDC_L2CFBARrs> {
        CFBADD_W::new(self, 0)
    }
}
/**LTDC layer 2 color frame buffer address register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cfbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cfbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LTDC_L2CFBAR)*/
pub struct LTDC_L2CFBARrs;
impl crate::RegisterSpec for LTDC_L2CFBARrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l2cfbar::R`](R) reader structure
impl crate::Readable for LTDC_L2CFBARrs {}
///`write(|w| ..)` method takes [`ltdc_l2cfbar::W`](W) writer structure
impl crate::Writable for LTDC_L2CFBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L2CFBAR to value 0
impl crate::Resettable for LTDC_L2CFBARrs {
    const RESET_VALUE: u32 = 0;
}
