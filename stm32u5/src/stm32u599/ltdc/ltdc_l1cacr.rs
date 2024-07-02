///Register `LTDC_L1CACR` reader
pub type R = crate::R<LTDC_L1CACRrs>;
///Register `LTDC_L1CACR` writer
pub type W = crate::W<LTDC_L1CACRrs>;
///Field `CONSTA` reader - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
pub type CONSTA_R = crate::FieldReader;
///Field `CONSTA` writer - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
pub type CONSTA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_L1CACR")
            .field("consta", &self.consta())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - constant alpha These bits configure the constant alpha used for blending. The constant alpha is divided by 255 by hardware. Example: if the programmed constant alpha is 0xFF, the constant alpha value is 255 / 255 = 1.
    #[inline(always)]
    #[must_use]
    pub fn consta(&mut self) -> CONSTA_W<LTDC_L1CACRrs> {
        CONSTA_W::new(self, 0)
    }
}
/**LTDC layer 1 constant alpha configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LTDC_L1CACR)*/
pub struct LTDC_L1CACRrs;
impl crate::RegisterSpec for LTDC_L1CACRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l1cacr::R`](R) reader structure
impl crate::Readable for LTDC_L1CACRrs {}
///`write(|w| ..)` method takes [`ltdc_l1cacr::W`](W) writer structure
impl crate::Writable for LTDC_L1CACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L1CACR to value 0xff
impl crate::Resettable for LTDC_L1CACRrs {
    const RESET_VALUE: u32 = 0xff;
}
