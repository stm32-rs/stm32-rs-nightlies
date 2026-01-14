///Register `CCIPR6` reader
pub type R = crate::R<CCIPR6rs>;
///Register `CCIPR6` writer
pub type W = crate::W<CCIPR6rs>;
///Field `XSPI1SEL` reader - Source selection for the XSPI1 kernel clock
pub type XSPI1SEL_R = crate::FieldReader;
///Field `XSPI1SEL` writer - Source selection for the XSPI1 kernel clock
pub type XSPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `XSPI2SEL` reader - Source selection for the XSPI2 kernel clock
pub type XSPI2SEL_R = crate::FieldReader;
///Field `XSPI2SEL` writer - Source selection for the XSPI2 kernel clock
pub type XSPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `XSPI3SEL` reader - Source selection for the XSPI3 kernel clock
pub type XSPI3SEL_R = crate::FieldReader;
///Field `XSPI3SEL` writer - Source selection for the XSPI3 kernel clock
pub type XSPI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OTGPHY1SEL` reader - Source selection for the OTGPHY1 kernel clock
pub type OTGPHY1SEL_R = crate::FieldReader;
///Field `OTGPHY1SEL` writer - Source selection for the OTGPHY1 kernel clock
pub type OTGPHY1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OTGPHY1CKREFSEL` reader - Set and reset by software
pub type OTGPHY1CKREFSEL_R = crate::BitReader;
///Field `OTGPHY1CKREFSEL` writer - Set and reset by software
pub type OTGPHY1CKREFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGPHY2SEL` reader - Source selection for the OTGPHY2 kernel clock
pub type OTGPHY2SEL_R = crate::FieldReader;
///Field `OTGPHY2SEL` writer - Source selection for the OTGPHY2 kernel clock
pub type OTGPHY2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OTGPHY2CKREFSEL` reader - Set and reset by software
pub type OTGPHY2CKREFSEL_R = crate::BitReader;
///Field `OTGPHY2CKREFSEL` writer - Set and reset by software
pub type OTGPHY2CKREFSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Source selection for the XSPI1 kernel clock
    #[inline(always)]
    pub fn xspi1sel(&self) -> XSPI1SEL_R {
        XSPI1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Source selection for the XSPI2 kernel clock
    #[inline(always)]
    pub fn xspi2sel(&self) -> XSPI2SEL_R {
        XSPI2SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Source selection for the XSPI3 kernel clock
    #[inline(always)]
    pub fn xspi3sel(&self) -> XSPI3SEL_R {
        XSPI3SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Source selection for the OTGPHY1 kernel clock
    #[inline(always)]
    pub fn otgphy1sel(&self) -> OTGPHY1SEL_R {
        OTGPHY1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 16 - Set and reset by software
    #[inline(always)]
    pub fn otgphy1ckrefsel(&self) -> OTGPHY1CKREFSEL_R {
        OTGPHY1CKREFSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Source selection for the OTGPHY2 kernel clock
    #[inline(always)]
    pub fn otgphy2sel(&self) -> OTGPHY2SEL_R {
        OTGPHY2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 24 - Set and reset by software
    #[inline(always)]
    pub fn otgphy2ckrefsel(&self) -> OTGPHY2CKREFSEL_R {
        OTGPHY2CKREFSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR6")
            .field("xspi1sel", &self.xspi1sel())
            .field("xspi2sel", &self.xspi2sel())
            .field("xspi3sel", &self.xspi3sel())
            .field("otgphy1sel", &self.otgphy1sel())
            .field("otgphy1ckrefsel", &self.otgphy1ckrefsel())
            .field("otgphy2sel", &self.otgphy2sel())
            .field("otgphy2ckrefsel", &self.otgphy2ckrefsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source selection for the XSPI1 kernel clock
    #[inline(always)]
    pub fn xspi1sel(&mut self) -> XSPI1SEL_W<'_, CCIPR6rs> {
        XSPI1SEL_W::new(self, 0)
    }
    ///Bits 4:5 - Source selection for the XSPI2 kernel clock
    #[inline(always)]
    pub fn xspi2sel(&mut self) -> XSPI2SEL_W<'_, CCIPR6rs> {
        XSPI2SEL_W::new(self, 4)
    }
    ///Bits 8:9 - Source selection for the XSPI3 kernel clock
    #[inline(always)]
    pub fn xspi3sel(&mut self) -> XSPI3SEL_W<'_, CCIPR6rs> {
        XSPI3SEL_W::new(self, 8)
    }
    ///Bits 12:13 - Source selection for the OTGPHY1 kernel clock
    #[inline(always)]
    pub fn otgphy1sel(&mut self) -> OTGPHY1SEL_W<'_, CCIPR6rs> {
        OTGPHY1SEL_W::new(self, 12)
    }
    ///Bit 16 - Set and reset by software
    #[inline(always)]
    pub fn otgphy1ckrefsel(&mut self) -> OTGPHY1CKREFSEL_W<'_, CCIPR6rs> {
        OTGPHY1CKREFSEL_W::new(self, 16)
    }
    ///Bits 20:21 - Source selection for the OTGPHY2 kernel clock
    #[inline(always)]
    pub fn otgphy2sel(&mut self) -> OTGPHY2SEL_W<'_, CCIPR6rs> {
        OTGPHY2SEL_W::new(self, 20)
    }
    ///Bit 24 - Set and reset by software
    #[inline(always)]
    pub fn otgphy2ckrefsel(&mut self) -> OTGPHY2CKREFSEL_W<'_, CCIPR6rs> {
        OTGPHY2CKREFSEL_W::new(self, 24)
    }
}
/**RCC clock configuration for independent peripheral register6

You can [`read`](crate::Reg::read) this register and get [`ccipr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:CCIPR6)*/
pub struct CCIPR6rs;
impl crate::RegisterSpec for CCIPR6rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr6::R`](R) reader structure
impl crate::Readable for CCIPR6rs {}
///`write(|w| ..)` method takes [`ccipr6::W`](W) writer structure
impl crate::Writable for CCIPR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR6 to value 0
impl crate::Resettable for CCIPR6rs {}
