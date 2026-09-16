///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `HSION` reader - Internal high-speed clock enable
pub type HSION_R = crate::BitReader;
///Field `HSION` writer - Internal high-speed clock enable
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDY` reader - Internal high-speed clock ready flag
pub type HSIRDY_R = crate::BitReader;
///Field `HSITRIM` reader - Internal high-speed clock trimming
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - Internal high-speed clock trimming
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `HSICAL` reader - Internal high-speed clock calibration
pub type HSICAL_R = crate::FieldReader;
///Field `HSEON` reader - HSE clock enable
pub type HSEON_R = crate::BitReader;
///Field `HSEON` writer - HSE clock enable
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDY` reader - HSE clock ready flag
pub type HSERDY_R = crate::BitReader;
///Field `HSEBYP` reader - HSE clock bypass
pub type HSEBYP_R = crate::BitReader;
///Field `HSEBYP` writer - HSE clock bypass
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSON` reader - Clock security system enable
pub type CSSON_R = crate::BitReader;
///Field `CSSON` writer - Clock security system enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLON` reader - Main PLL (PLL) enable
pub type PLLON_R = crate::BitReader;
///Field `PLLON` writer - Main PLL (PLL) enable
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDY` reader - Main PLL (PLL) clock ready flag
pub type PLLRDY_R = crate::BitReader;
///Field `PLLI2SON` reader - PLLI2S enable
pub type PLLI2SON_R = crate::BitReader;
///Field `PLLI2SON` writer - PLLI2S enable
pub type PLLI2SON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLI2SRDY` reader - PLLI2S clock ready flag
pub type PLLI2SRDY_R = crate::BitReader;
impl R {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal high-speed clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 3:7 - Internal high-speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:15 - Internal high-speed clock calibration
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - Main PLL (PLL) enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Main PLL (PLL) clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PLLI2S enable
    #[inline(always)]
    pub fn plli2son(&self) -> PLLI2SON_R {
        PLLI2SON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PLLI2S clock ready flag
    #[inline(always)]
    pub fn plli2srdy(&self) -> PLLI2SRDY_R {
        PLLI2SRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("plli2srdy", &self.plli2srdy())
            .field("plli2son", &self.plli2son())
            .field("pllrdy", &self.pllrdy())
            .field("pllon", &self.pllon())
            .field("csson", &self.csson())
            .field("hsebyp", &self.hsebyp())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("hsical", &self.hsical())
            .field("hsitrim", &self.hsitrim())
            .field("hsirdy", &self.hsirdy())
            .field("hsion", &self.hsion())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, CRrs> {
        HSION_W::new(self, 0)
    }
    ///Bits 3:7 - Internal high-speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<'_, CRrs> {
        HSITRIM_W::new(self, 3)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<'_, CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 24 - Main PLL (PLL) enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
    ///Bit 26 - PLLI2S enable
    #[inline(always)]
    pub fn plli2son(&mut self) -> PLLI2SON_W<'_, CRrs> {
        PLLI2SON_W::new(self, 26)
    }
}
/**clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x83
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x83;
}
