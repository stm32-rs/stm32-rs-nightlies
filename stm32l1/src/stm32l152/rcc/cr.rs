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
///Field `MSION` reader - MSI clock enable
pub type MSION_R = crate::BitReader;
///Field `MSION` writer - MSI clock enable
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDY` reader - MSI clock ready flag
pub type MSIRDY_R = crate::BitReader;
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
///Field `PLLON` reader - PLL enable
pub type PLLON_R = crate::BitReader;
///Field `PLLON` writer - PLL enable
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDY` reader - PLL clock ready flag
pub type PLLRDY_R = crate::BitReader;
///Field `CSSON` reader - Clock security system enable
pub type CSSON_R = crate::BitReader;
///Field `CSSON` writer - Clock security system enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCPRE0` reader - RTCPRE0
pub type RTCPRE0_R = crate::BitReader;
///Field `RTCPRE0` writer - RTCPRE0
pub type RTCPRE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCPRE1` reader - TC/LCD prescaler
pub type RTCPRE1_R = crate::BitReader;
///Field `RTCPRE1` writer - TC/LCD prescaler
pub type RTCPRE1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 1) != 0)
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
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RTCPRE0
    #[inline(always)]
    pub fn rtcpre0(&self) -> RTCPRE0_R {
        RTCPRE0_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TC/LCD prescaler
    #[inline(always)]
    pub fn rtcpre1(&self) -> RTCPRE1_R {
        RTCPRE1_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rtcpre1", &self.rtcpre1())
            .field("rtcpre0", &self.rtcpre0())
            .field("csson", &self.csson())
            .field("pllrdy", &self.pllrdy())
            .field("pllon", &self.pllon())
            .field("hsebyp", &self.hsebyp())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("msirdy", &self.msirdy())
            .field("msion", &self.msion())
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
    ///Bit 8 - MSI clock enable
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W<'_, CRrs> {
        MSION_W::new(self, 8)
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
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<'_, CRrs> {
        PLLON_W::new(self, 24)
    }
    ///Bit 28 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<'_, CRrs> {
        CSSON_W::new(self, 28)
    }
    ///Bit 29 - RTCPRE0
    #[inline(always)]
    pub fn rtcpre0(&mut self) -> RTCPRE0_W<'_, CRrs> {
        RTCPRE0_W::new(self, 29)
    }
    ///Bit 30 - TC/LCD prescaler
    #[inline(always)]
    pub fn rtcpre1(&mut self) -> RTCPRE1_W<'_, CRrs> {
        RTCPRE1_W::new(self, 30)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#RCC:CR)*/
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
///`reset()` method sets CR to value 0x0300
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0300;
}
