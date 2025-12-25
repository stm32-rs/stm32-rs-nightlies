///Register `CCIPR9` reader
pub type R = crate::R<CCIPR9rs>;
///Register `CCIPR9` writer
pub type W = crate::W<CCIPR9rs>;
///Field `SPDIFRX1SEL` reader - Source selection for the SPDIFRX1 kernel clock
pub type SPDIFRX1SEL_R = crate::FieldReader;
///Field `SPDIFRX1SEL` writer - Source selection for the SPDIFRX1 kernel clock
pub type SPDIFRX1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI1SEL` reader - Source selection for the SPI1 kernel clock
pub type SPI1SEL_R = crate::FieldReader;
///Field `SPI1SEL` writer - Source selection for the SPI1 kernel clock
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI2SEL` reader - Source selection for the SPI2 kernel clock
pub type SPI2SEL_R = crate::FieldReader;
///Field `SPI2SEL` writer - Source selection for the SPI2 kernel clock
pub type SPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI3SEL` reader - Source selection for the SPI3 kernel clock
pub type SPI3SEL_R = crate::FieldReader;
///Field `SPI3SEL` writer - Source selection for the SPI3 kernel clock
pub type SPI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI4SEL` reader - Source selection for the SPI4 kernel clock
pub type SPI4SEL_R = crate::FieldReader;
///Field `SPI4SEL` writer - Source selection for the SPI4 kernel clock
pub type SPI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI5SEL` reader - Source selection for the SPI5 kernel clock
pub type SPI5SEL_R = crate::FieldReader;
///Field `SPI5SEL` writer - Source selection for the SPI5 kernel clock
pub type SPI5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPI6SEL` reader - Source selection for the SPI6 kernel clock
pub type SPI6SEL_R = crate::FieldReader;
///Field `SPI6SEL` writer - Source selection for the SPI6 kernel clock
pub type SPI6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Source selection for the SPDIFRX1 kernel clock
    #[inline(always)]
    pub fn spdifrx1sel(&self) -> SPDIFRX1SEL_R {
        SPDIFRX1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Source selection for the SPI1 kernel clock
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Source selection for the SPI2 kernel clock
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Source selection for the SPI3 kernel clock
    #[inline(always)]
    pub fn spi3sel(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - Source selection for the SPI4 kernel clock
    #[inline(always)]
    pub fn spi4sel(&self) -> SPI4SEL_R {
        SPI4SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Source selection for the SPI5 kernel clock
    #[inline(always)]
    pub fn spi5sel(&self) -> SPI5SEL_R {
        SPI5SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Source selection for the SPI6 kernel clock
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR9")
            .field("spdifrx1sel", &self.spdifrx1sel())
            .field("spi1sel", &self.spi1sel())
            .field("spi2sel", &self.spi2sel())
            .field("spi3sel", &self.spi3sel())
            .field("spi4sel", &self.spi4sel())
            .field("spi5sel", &self.spi5sel())
            .field("spi6sel", &self.spi6sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Source selection for the SPDIFRX1 kernel clock
    #[inline(always)]
    pub fn spdifrx1sel(&mut self) -> SPDIFRX1SEL_W<'_, CCIPR9rs> {
        SPDIFRX1SEL_W::new(self, 0)
    }
    ///Bits 4:6 - Source selection for the SPI1 kernel clock
    #[inline(always)]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<'_, CCIPR9rs> {
        SPI1SEL_W::new(self, 4)
    }
    ///Bits 8:10 - Source selection for the SPI2 kernel clock
    #[inline(always)]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<'_, CCIPR9rs> {
        SPI2SEL_W::new(self, 8)
    }
    ///Bits 12:14 - Source selection for the SPI3 kernel clock
    #[inline(always)]
    pub fn spi3sel(&mut self) -> SPI3SEL_W<'_, CCIPR9rs> {
        SPI3SEL_W::new(self, 12)
    }
    ///Bits 16:18 - Source selection for the SPI4 kernel clock
    #[inline(always)]
    pub fn spi4sel(&mut self) -> SPI4SEL_W<'_, CCIPR9rs> {
        SPI4SEL_W::new(self, 16)
    }
    ///Bits 20:22 - Source selection for the SPI5 kernel clock
    #[inline(always)]
    pub fn spi5sel(&mut self) -> SPI5SEL_W<'_, CCIPR9rs> {
        SPI5SEL_W::new(self, 20)
    }
    ///Bits 24:26 - Source selection for the SPI6 kernel clock
    #[inline(always)]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<'_, CCIPR9rs> {
        SPI6SEL_W::new(self, 24)
    }
}
/**RCC clock configuration for independent peripheral register9

You can [`read`](crate::Reg::read) this register and get [`ccipr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:CCIPR9)*/
pub struct CCIPR9rs;
impl crate::RegisterSpec for CCIPR9rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr9::R`](R) reader structure
impl crate::Readable for CCIPR9rs {}
///`write(|w| ..)` method takes [`ccipr9::W`](W) writer structure
impl crate::Writable for CCIPR9rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR9 to value 0
impl crate::Resettable for CCIPR9rs {}
