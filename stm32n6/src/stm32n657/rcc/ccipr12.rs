///Register `CCIPR12` reader
pub type R = crate::R<CCIPR12rs>;
///Register `CCIPR12` writer
pub type W = crate::W<CCIPR12rs>;
///Field `LPTIM1SEL` reader - Source selection for the LPTIM1 kernel clock
pub type LPTIM1SEL_R = crate::FieldReader;
///Field `LPTIM1SEL` writer - Source selection for the LPTIM1 kernel clock
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM2SEL` reader - Source selection for the LPTIM2 kernel clock
pub type LPTIM2SEL_R = crate::FieldReader;
///Field `LPTIM2SEL` writer - Source selection for the LPTIM2 kernel clock
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM3SEL` reader - Source selection for the LPTIM3 kernel clock
pub type LPTIM3SEL_R = crate::FieldReader;
///Field `LPTIM3SEL` writer - Source selection for the LPTIM3 kernel clock
pub type LPTIM3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM4SEL` reader - Source selection for the LPTIM4 kernel clock
pub type LPTIM4SEL_R = crate::FieldReader;
///Field `LPTIM4SEL` writer - Source selection for the LPTIM4 kernel clock
pub type LPTIM4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPTIM5SEL` reader - Source selection for the LPTIM5 kernel clock
pub type LPTIM5SEL_R = crate::FieldReader;
///Field `LPTIM5SEL` writer - Source selection for the LPTIM5 kernel clock
pub type LPTIM5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 8:10 - Source selection for the LPTIM1 kernel clock
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Source selection for the LPTIM2 kernel clock
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - Source selection for the LPTIM3 kernel clock
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Source selection for the LPTIM4 kernel clock
    #[inline(always)]
    pub fn lptim4sel(&self) -> LPTIM4SEL_R {
        LPTIM4SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Source selection for the LPTIM5 kernel clock
    #[inline(always)]
    pub fn lptim5sel(&self) -> LPTIM5SEL_R {
        LPTIM5SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR12")
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("lptim3sel", &self.lptim3sel())
            .field("lptim4sel", &self.lptim4sel())
            .field("lptim5sel", &self.lptim5sel())
            .finish()
    }
}
impl W {
    ///Bits 8:10 - Source selection for the LPTIM1 kernel clock
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, CCIPR12rs> {
        LPTIM1SEL_W::new(self, 8)
    }
    ///Bits 12:14 - Source selection for the LPTIM2 kernel clock
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<'_, CCIPR12rs> {
        LPTIM2SEL_W::new(self, 12)
    }
    ///Bits 16:18 - Source selection for the LPTIM3 kernel clock
    #[inline(always)]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<'_, CCIPR12rs> {
        LPTIM3SEL_W::new(self, 16)
    }
    ///Bits 20:22 - Source selection for the LPTIM4 kernel clock
    #[inline(always)]
    pub fn lptim4sel(&mut self) -> LPTIM4SEL_W<'_, CCIPR12rs> {
        LPTIM4SEL_W::new(self, 20)
    }
    ///Bits 24:26 - Source selection for the LPTIM5 kernel clock
    #[inline(always)]
    pub fn lptim5sel(&mut self) -> LPTIM5SEL_W<'_, CCIPR12rs> {
        LPTIM5SEL_W::new(self, 24)
    }
}
/**RCC clock configuration for independent peripheral register12

You can [`read`](crate::Reg::read) this register and get [`ccipr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:CCIPR12)*/
pub struct CCIPR12rs;
impl crate::RegisterSpec for CCIPR12rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr12::R`](R) reader structure
impl crate::Readable for CCIPR12rs {}
///`write(|w| ..)` method takes [`ccipr12::W`](W) writer structure
impl crate::Writable for CCIPR12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR12 to value 0
impl crate::Resettable for CCIPR12rs {}
