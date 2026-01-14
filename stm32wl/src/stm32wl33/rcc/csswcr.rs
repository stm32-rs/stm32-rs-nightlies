///Register `CSSWCR` reader
pub type R = crate::R<CSSWCRrs>;
///Register `CSSWCR` writer
pub type W = crate::W<CSSWCRrs>;
///Field `LSISWTRIMEN` reader - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled
pub type LSISWTRIMEN_R = crate::BitReader;
///Field `LSISWTRIMEN` writer - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled
pub type LSISWTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSISWBW` reader - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn
pub type LSISWBW_R = crate::FieldReader;
///Field `LSISWBW` writer - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn
pub type LSISWBW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LSEDRV` reader - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability
pub type LSEDRV_R = crate::FieldReader;
///Field `LSEDRV` writer - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HSISWTRIMEN` reader - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled
pub type HSISWTRIMEN_R = crate::BitReader;
///Field `HSISWTRIMEN` writer - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled
pub type HSISWTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSITRIMSW` reader - High Speed Internal clock trimming value to set by SW.
pub type HSITRIMSW_R = crate::FieldReader;
///Field `HSITRIMSW` writer - High Speed Internal clock trimming value to set by SW.
pub type HSITRIMSW_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled
    #[inline(always)]
    pub fn lsiswtrimen(&self) -> LSISWTRIMEN_R {
        LSISWTRIMEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lsiswbw(&self) -> LSISWBW_R {
        LSISWBW_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bits 5:6 - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 23 - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled
    #[inline(always)]
    pub fn hsiswtrimen(&self) -> HSISWTRIMEN_R {
        HSISWTRIMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:29 - High Speed Internal clock trimming value to set by SW.
    #[inline(always)]
    pub fn hsitrimsw(&self) -> HSITRIMSW_R {
        HSITRIMSW_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSSWCR")
            .field("lsiswtrimen", &self.lsiswtrimen())
            .field("lsiswbw", &self.lsiswbw())
            .field("lsedrv", &self.lsedrv())
            .field("hsiswtrimen", &self.hsiswtrimen())
            .field("hsitrimsw", &self.hsitrimsw())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low Speed oscillator trimming by SW enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI oscillator Bias trimming by SW disabled 1: LSI oscillator Bias trimming by SW enabled
    #[inline(always)]
    pub fn lsiswtrimen(&mut self) -> LSISWTRIMEN_W<'_, CSSWCRrs> {
        LSISWTRIMEN_W::new(self, 0)
    }
    ///Bits 1:4 - Low Speed Internal clock trimming value to set by SW Reset source only for this field: PORESETn
    #[inline(always)]
    pub fn lsiswbw(&mut self) -> LSISWBW_W<'_, CSSWCRrs> {
        LSISWBW_W::new(self, 1)
    }
    ///Bits 5:6 - Maximum Crystal gm for Low Speed External XO (to connect to XTDRV of 32kHz LSE XO => into IO V33?) to amplify drinving capacity modulation Set by software. Reset source only for this field: PORESETn 00: 0.0, low drive capability 01: 0.1, medium low drive capability 10: 1.0, medium high drive capability 11: 1.1, highdrive capability
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, CSSWCRrs> {
        LSEDRV_W::new(self, 5)
    }
    ///Bit 23 - High Speed oscillator trimming by SW enable Set and reset by software. 0: HSI oscillator Bias trimming by SW disabled 1: HSI oscillator Bias trimming by SW enabled
    #[inline(always)]
    pub fn hsiswtrimen(&mut self) -> HSISWTRIMEN_W<'_, CSSWCRrs> {
        HSISWTRIMEN_W::new(self, 23)
    }
    ///Bits 24:29 - High Speed Internal clock trimming value to set by SW.
    #[inline(always)]
    pub fn hsitrimsw(&mut self) -> HSITRIMSW_W<'_, CSSWCRrs> {
        HSITRIMSW_W::new(self, 24)
    }
}
/**CSSWCR register

You can [`read`](crate::Reg::read) this register and get [`csswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:CSSWCR)*/
pub struct CSSWCRrs;
impl crate::RegisterSpec for CSSWCRrs {
    type Ux = u32;
}
///`read()` method returns [`csswcr::R`](R) reader structure
impl crate::Readable for CSSWCRrs {}
///`write(|w| ..)` method takes [`csswcr::W`](W) writer structure
impl crate::Writable for CSSWCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSSWCR to value 0
impl crate::Resettable for CSSWCRrs {}
