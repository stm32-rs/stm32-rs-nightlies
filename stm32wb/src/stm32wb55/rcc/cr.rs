///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `MSION` reader - MSI clock enable
pub type MSION_R = crate::BitReader;
///Field `MSION` writer - MSI clock enable
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRDY` reader - MSI clock ready flag
pub type MSIRDY_R = crate::BitReader;
///Field `MSIPLLEN` reader - MSI clock PLL enable
pub type MSIPLLEN_R = crate::BitReader;
///Field `MSIPLLEN` writer - MSI clock PLL enable
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSIRANGE` reader - MSI clock ranges
pub type MSIRANGE_R = crate::FieldReader;
///Field `MSIRANGE` writer - MSI clock ranges
pub type MSIRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HSION` reader - HSI clock enabled
pub type HSION_R = crate::BitReader;
///Field `HSION` writer - HSI clock enabled
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIKERON` reader - HSI always enable for peripheral kernels
pub type HSIKERON_R = crate::BitReader;
///Field `HSIKERON` writer - HSI always enable for peripheral kernels
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDY` reader - HSI clock ready flag
pub type HSIRDY_R = crate::BitReader;
///Field `HSIASFS` reader - HSI automatic start from Stop
pub type HSIASFS_R = crate::BitReader;
///Field `HSIASFS` writer - HSI automatic start from Stop
pub type HSIASFS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIKERDY` reader - HSI kernel clock ready flag for peripherals requests
pub type HSIKERDY_R = crate::BitReader;
///Field `HSEON` reader - HSE clock enabled
pub type HSEON_R = crate::BitReader;
///Field `HSEON` writer - HSE clock enabled
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDY` reader - HSE clock ready flag
pub type HSERDY_R = crate::BitReader;
///Field `HSEBYP` reader - HSE crystal oscillator bypass
pub type HSEBYP_R = crate::BitReader;
///Field `HSEBYP` writer - HSE crystal oscillator bypass
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSON` writer - HSE Clock security system enable
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEPRE` reader - HSE sysclk and PLL M divider prescaler
pub type HSEPRE_R = crate::BitReader;
///Field `HSEPRE` writer - HSE sysclk and PLL M divider prescaler
pub type HSEPRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLON` reader - Main PLL enable
pub type PLLON_R = crate::BitReader;
///Field `PLLON` writer - Main PLL enable
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLRDY` reader - Main PLL clock ready flag
pub type PLLRDY_R = crate::BitReader;
///Field `PLLSAI1ON` reader - SAI1 PLL enable
pub type PLLSAI1ON_R = crate::BitReader;
///Field `PLLSAI1ON` writer - SAI1 PLL enable
pub type PLLSAI1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag
pub type PLLSAI1RDY_R = crate::BitReader;
impl R {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - HSI clock enabled
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HSI kernel clock ready flag for peripherals requests
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - HSE clock enabled
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - HSE sysclk and PLL M divider prescaler
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Main PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SAI1 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pllsai1rdy", &self.pllsai1rdy())
            .field("pllsai1on", &self.pllsai1on())
            .field("pllrdy", &self.pllrdy())
            .field("pllon", &self.pllon())
            .field("hsepre", &self.hsepre())
            .field("hsebyp", &self.hsebyp())
            .field("hserdy", &self.hserdy())
            .field("hseon", &self.hseon())
            .field("hsikerdy", &self.hsikerdy())
            .field("hsiasfs", &self.hsiasfs())
            .field("hsirdy", &self.hsirdy())
            .field("hsikeron", &self.hsikeron())
            .field("hsion", &self.hsion())
            .field("msirange", &self.msirange())
            .field("msipllen", &self.msipllen())
            .field("msirdy", &self.msirdy())
            .field("msion", &self.msion())
            .finish()
    }
}
impl W {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<CRrs> {
        MSION_W::new(self, 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<CRrs> {
        MSIPLLEN_W::new(self, 2)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    ///Bit 8 - HSI clock enabled
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 8)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 9)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    #[must_use]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<CRrs> {
        HSIASFS_W::new(self, 11)
    }
    ///Bit 16 - HSE clock enabled
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    ///Bit 19 - HSE Clock security system enable
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    ///Bit 20 - HSE sysclk and PLL M divider prescaler
    #[inline(always)]
    #[must_use]
    pub fn hsepre(&mut self) -> HSEPRE_W<CRrs> {
        HSEPRE_W::new(self, 20)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<CRrs> {
        PLLON_W::new(self, 24)
    }
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W<CRrs> {
        PLLSAI1ON_W::new(self, 26)
    }
}
/**Clock control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0x61
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x61;
}
