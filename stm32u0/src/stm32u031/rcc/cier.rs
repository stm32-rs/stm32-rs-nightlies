///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
///Field `LSIRDYIE` reader - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
pub type LSIRDYIE_R = crate::BitReader;
///Field `LSIRDYIE` writer - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
pub type LSERDYIE_R = crate::BitReader;
///Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDYIE` reader - MSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSI oscillator stabilization.
pub type MSIRDYIE_R = crate::BitReader;
///Field `MSIRDYIE` writer - MSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSI oscillator stabilization.
pub type MSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIE` reader - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
pub type HSIRDYIE_R = crate::BitReader;
///Field `HSIRDYIE` writer - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIE` reader - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
pub type HSERDYIE_R = crate::BitReader;
///Field `HSERDYIE` writer - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDYIE` reader - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock:
pub type PLLRDYIE_R = crate::BitReader;
///Field `PLLRDYIE` writer - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock:
pub type PLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSIE` reader - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE.
pub type LSECSSIE_R = crate::BitReader;
///Field `LSECSSIE` writer - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE.
pub type LSECSSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator.
pub type HSI48RDYIE_R = crate::BitReader;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator.
pub type HSI48RDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSI oscillator stabilization.
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock:
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE.
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator.
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("msirdyie", &self.msirdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("lsecssie", &self.lsecssie())
            .field("hsi48rdyie", &self.hsi48rdyie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 2 - MSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSI oscillator stabilization.
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<'_, CIERrs> {
        MSIRDYIE_W::new(self, 2)
    }
    ///Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 5 - PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL lock:
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<'_, CIERrs> {
        PLLRDYIE_W::new(self, 5)
    }
    ///Bit 9 - LSE clock security system interrupt enable Set and cleared by software to enable/disable interrupt caused by the clock security system on LSE.
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<'_, CIERrs> {
        LSECSSIE_W::new(self, 9)
    }
    ///Bit 10 - HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the internal HSI48 oscillator.
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<'_, CIERrs> {
        HSI48RDYIE_W::new(self, 10)
    }
}
/**Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#RCC:CIER)*/
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
///`read()` method returns [`cier::R`](R) reader structure
impl crate::Readable for CIERrs {}
///`write(|w| ..)` method takes [`cier::W`](W) writer structure
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIERrs {}
