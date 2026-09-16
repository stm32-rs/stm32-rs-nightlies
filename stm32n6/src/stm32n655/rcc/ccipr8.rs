///Register `CCIPR8` reader
pub type R = crate::R<CCIPR8rs>;
///Register `CCIPR8` writer
pub type W = crate::W<CCIPR8rs>;
///Field `SDMMC1SEL` reader - Source selection for the SDMMC1 kernel clock
pub type SDMMC1SEL_R = crate::FieldReader;
///Field `SDMMC1SEL` writer - Source selection for the SDMMC1 kernel clock
pub type SDMMC1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SDMMC2SEL` reader - Source selection for the SDMMC2 kernel clock
pub type SDMMC2SEL_R = crate::FieldReader;
///Field `SDMMC2SEL` writer - Source selection for the SDMMC2 kernel clock
pub type SDMMC2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Source selection for the SDMMC1 kernel clock
    #[inline(always)]
    pub fn sdmmc1sel(&self) -> SDMMC1SEL_R {
        SDMMC1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Source selection for the SDMMC2 kernel clock
    #[inline(always)]
    pub fn sdmmc2sel(&self) -> SDMMC2SEL_R {
        SDMMC2SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR8")
            .field("sdmmc1sel", &self.sdmmc1sel())
            .field("sdmmc2sel", &self.sdmmc2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source selection for the SDMMC1 kernel clock
    #[inline(always)]
    pub fn sdmmc1sel(&mut self) -> SDMMC1SEL_W<'_, CCIPR8rs> {
        SDMMC1SEL_W::new(self, 0)
    }
    ///Bits 4:5 - Source selection for the SDMMC2 kernel clock
    #[inline(always)]
    pub fn sdmmc2sel(&mut self) -> SDMMC2SEL_W<'_, CCIPR8rs> {
        SDMMC2SEL_W::new(self, 4)
    }
}
/**RCC clock configuration for independent peripheral register8

You can [`read`](crate::Reg::read) this register and get [`ccipr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CCIPR8)*/
pub struct CCIPR8rs;
impl crate::RegisterSpec for CCIPR8rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr8::R`](R) reader structure
impl crate::Readable for CCIPR8rs {}
///`write(|w| ..)` method takes [`ccipr8::W`](W) writer structure
impl crate::Writable for CCIPR8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR8 to value 0
impl crate::Resettable for CCIPR8rs {}
