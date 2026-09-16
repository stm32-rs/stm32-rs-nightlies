///Register `CCIPR4` reader
pub type R = crate::R<CCIPR4rs>;
///Register `CCIPR4` writer
pub type W = crate::W<CCIPR4rs>;
///Field `I2C1SEL` reader - Source selection for the I2C1 kernel clock
pub type I2C1SEL_R = crate::FieldReader;
///Field `I2C1SEL` writer - Source selection for the I2C1 kernel clock
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `I2C2SEL` reader - Source selection for the I2C2 kernel clock
pub type I2C2SEL_R = crate::FieldReader;
///Field `I2C2SEL` writer - Source selection for the I2C2 kernel clock
pub type I2C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `I2C3SEL` reader - Source selection for the I2C3 kernel clock
pub type I2C3SEL_R = crate::FieldReader;
///Field `I2C3SEL` writer - Source selection for the I2C3 kernel clock
pub type I2C3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `I2C4SEL` reader - Source selection for the I2C4 kernel clock
pub type I2C4SEL_R = crate::FieldReader;
///Field `I2C4SEL` writer - Source selection for the I2C4 kernel clock
pub type I2C4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `I3C1SEL` reader - Source selection for the I3C1 kernel clock
pub type I3C1SEL_R = crate::FieldReader;
///Field `I3C1SEL` writer - Source selection for the I3C1 kernel clock
pub type I3C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `I3C2SEL` reader - Source selection for the I3C2 kernel clock
pub type I3C2SEL_R = crate::FieldReader;
///Field `I3C2SEL` writer - Source selection for the I3C2 kernel clock
pub type I3C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LTDCSEL` reader - Source selection for the LTDC kernel clock
pub type LTDCSEL_R = crate::FieldReader;
///Field `LTDCSEL` writer - Source selection for the LTDC kernel clock
pub type LTDCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Source selection for the I2C1 kernel clock
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Source selection for the I2C2 kernel clock
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Source selection for the I2C3 kernel clock
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Source selection for the I2C4 kernel clock
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - Source selection for the I3C1 kernel clock
    #[inline(always)]
    pub fn i3c1sel(&self) -> I3C1SEL_R {
        I3C1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Source selection for the I3C2 kernel clock
    #[inline(always)]
    pub fn i3c2sel(&self) -> I3C2SEL_R {
        I3C2SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:25 - Source selection for the LTDC kernel clock
    #[inline(always)]
    pub fn ltdcsel(&self) -> LTDCSEL_R {
        LTDCSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR4")
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c2sel", &self.i2c2sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("i2c4sel", &self.i2c4sel())
            .field("i3c1sel", &self.i3c1sel())
            .field("i3c2sel", &self.i3c2sel())
            .field("ltdcsel", &self.ltdcsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Source selection for the I2C1 kernel clock
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, CCIPR4rs> {
        I2C1SEL_W::new(self, 0)
    }
    ///Bits 4:6 - Source selection for the I2C2 kernel clock
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<'_, CCIPR4rs> {
        I2C2SEL_W::new(self, 4)
    }
    ///Bits 8:10 - Source selection for the I2C3 kernel clock
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<'_, CCIPR4rs> {
        I2C3SEL_W::new(self, 8)
    }
    ///Bits 12:14 - Source selection for the I2C4 kernel clock
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<'_, CCIPR4rs> {
        I2C4SEL_W::new(self, 12)
    }
    ///Bits 16:18 - Source selection for the I3C1 kernel clock
    #[inline(always)]
    pub fn i3c1sel(&mut self) -> I3C1SEL_W<'_, CCIPR4rs> {
        I3C1SEL_W::new(self, 16)
    }
    ///Bits 20:22 - Source selection for the I3C2 kernel clock
    #[inline(always)]
    pub fn i3c2sel(&mut self) -> I3C2SEL_W<'_, CCIPR4rs> {
        I3C2SEL_W::new(self, 20)
    }
    ///Bits 24:25 - Source selection for the LTDC kernel clock
    #[inline(always)]
    pub fn ltdcsel(&mut self) -> LTDCSEL_W<'_, CCIPR4rs> {
        LTDCSEL_W::new(self, 24)
    }
}
/**RCC clock configuration for independent peripheral register4

You can [`read`](crate::Reg::read) this register and get [`ccipr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:CCIPR4)*/
pub struct CCIPR4rs;
impl crate::RegisterSpec for CCIPR4rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr4::R`](R) reader structure
impl crate::Readable for CCIPR4rs {}
///`write(|w| ..)` method takes [`ccipr4::W`](W) writer structure
impl crate::Writable for CCIPR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR4 to value 0
impl crate::Resettable for CCIPR4rs {}
