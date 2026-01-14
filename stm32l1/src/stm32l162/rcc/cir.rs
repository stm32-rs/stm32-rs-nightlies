///Register `CIR` reader
pub type R = crate::R<CIRrs>;
///Register `CIR` writer
pub type W = crate::W<CIRrs>;
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub type LSIRDYF_R = crate::BitReader;
///Field `LSERDYF` reader - LSE ready interrupt flag
pub type LSERDYF_R = crate::BitReader;
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub type HSIRDYF_R = crate::BitReader;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub type HSERDYF_R = crate::BitReader;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub type PLLRDYF_R = crate::BitReader;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub type MSIRDYF_R = crate::BitReader;
///Field `CSSF` reader - Clock security system interrupt flag
pub type CSSF_R = crate::BitReader;
///Field `LSIRDYIE` reader - LSI ready interrupt enable
pub type LSIRDYIE_R = crate::BitReader;
///Field `LSIRDYIE` writer - LSI ready interrupt enable
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIE` reader - LSE ready interrupt enable
pub type LSERDYIE_R = crate::BitReader;
///Field `LSERDYIE` writer - LSE ready interrupt enable
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIE` reader - HSI ready interrupt enable
pub type HSIRDYIE_R = crate::BitReader;
///Field `HSIRDYIE` writer - HSI ready interrupt enable
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIE` reader - HSE ready interrupt enable
pub type HSERDYIE_R = crate::BitReader;
///Field `HSERDYIE` writer - HSE ready interrupt enable
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDYIE` reader - PLL ready interrupt enable
pub type PLLRDYIE_R = crate::BitReader;
///Field `PLLRDYIE` writer - PLL ready interrupt enable
pub type PLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDYIE` reader - MSI ready interrupt enable
pub type MSIRDYIE_R = crate::BitReader;
///Field `MSIRDYIE` writer - MSI ready interrupt enable
pub type MSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDYC` writer - LSI ready interrupt clear
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYC` writer - LSE ready interrupt clear
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYC` writer - HSI ready interrupt clear
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYC` writer - HSE ready interrupt clear
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub type PLLRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub type MSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSC` writer - Clock security system interrupt clear
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIR")
            .field("msirdyie", &self.msirdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("lsirdyie", &self.lsirdyie())
            .field("cssf", &self.cssf())
            .field("msirdyf", &self.msirdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("hserdyf", &self.hserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("lsirdyf", &self.lsirdyf())
            .finish()
    }
}
impl W {
    ///Bit 8 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIRrs> {
        LSIRDYIE_W::new(self, 8)
    }
    ///Bit 9 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIRrs> {
        LSERDYIE_W::new(self, 9)
    }
    ///Bit 10 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIRrs> {
        HSIRDYIE_W::new(self, 10)
    }
    ///Bit 11 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIRrs> {
        HSERDYIE_W::new(self, 11)
    }
    ///Bit 12 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<'_, CIRrs> {
        PLLRDYIE_W::new(self, 12)
    }
    ///Bit 13 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<'_, CIRrs> {
        MSIRDYIE_W::new(self, 13)
    }
    ///Bit 16 - LSI ready interrupt clear
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<'_, CIRrs> {
        LSIRDYC_W::new(self, 16)
    }
    ///Bit 17 - LSE ready interrupt clear
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<'_, CIRrs> {
        LSERDYC_W::new(self, 17)
    }
    ///Bit 18 - HSI ready interrupt clear
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<'_, CIRrs> {
        HSIRDYC_W::new(self, 18)
    }
    ///Bit 19 - HSE ready interrupt clear
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<'_, CIRrs> {
        HSERDYC_W::new(self, 19)
    }
    ///Bit 20 - PLL ready interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<'_, CIRrs> {
        PLLRDYC_W::new(self, 20)
    }
    ///Bit 21 - MSI ready interrupt clear
    #[inline(always)]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<'_, CIRrs> {
        MSIRDYC_W::new(self, 21)
    }
    ///Bit 23 - Clock security system interrupt clear
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<'_, CIRrs> {
        CSSC_W::new(self, 23)
    }
}
/**Clock interrupt register

You can [`read`](crate::Reg::read) this register and get [`cir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RCC:CIR)*/
pub struct CIRrs;
impl crate::RegisterSpec for CIRrs {
    type Ux = u32;
}
///`read()` method returns [`cir::R`](R) reader structure
impl crate::Readable for CIRrs {}
///`write(|w| ..)` method takes [`cir::W`](W) writer structure
impl crate::Writable for CIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIR to value 0
impl crate::Resettable for CIRrs {}
