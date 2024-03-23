#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `MSION` reader - MSI clock enable"]
pub type MSION_R = crate::BitReader;
#[doc = "Field `MSION` writer - MSI clock enable"]
pub type MSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRDY` reader - MSI clock ready flag"]
pub type MSIRDY_R = crate::BitReader;
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable"]
pub type MSIPLLEN_R = crate::BitReader;
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable"]
pub type MSIPLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader;
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSION` reader - HSI clock enabled"]
pub type HSION_R = crate::BitReader;
#[doc = "Field `HSION` writer - HSI clock enabled"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERON` reader - HSI always enable for peripheral kernels"]
pub type HSIKERON_R = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSI always enable for peripheral kernels"]
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HSIRDY_R = crate::BitReader;
#[doc = "Field `HSIASFS` reader - HSI automatic start from Stop"]
pub type HSIASFS_R = crate::BitReader;
#[doc = "Field `HSIASFS` writer - HSI automatic start from Stop"]
pub type HSIASFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERDY` reader - HSI kernel clock ready flag for peripherals requests"]
pub type HSIKERDY_R = crate::BitReader;
#[doc = "Field `HSEON` reader - HSE clock enabled"]
pub type HSEON_R = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enabled"]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
pub type HSEBYP_R = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` writer - HSE Clock security system enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEPRE` reader - HSE sysclk and PLL M divider prescaler"]
pub type HSEPRE_R = crate::BitReader;
#[doc = "Field `HSEPRE` writer - HSE sysclk and PLL M divider prescaler"]
pub type HSEPRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub type PLLON_R = crate::BitReader;
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader;
#[doc = "Field `PLLSAI1ON` reader - SAI1 PLL enable"]
pub type PLLSAI1ON_R = crate::BitReader;
#[doc = "Field `PLLSAI1ON` writer - SAI1 PLL enable"]
pub type PLLSAI1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag"]
pub type PLLSAI1RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HSI clock enabled"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HSI kernel clock ready flag for peripherals requests"]
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enabled"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - HSE sysclk and PLL M divider prescaler"]
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SAI1 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<CRrs> {
        MSION_W::new(self, 0)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<CRrs> {
        MSIPLLEN_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    #[doc = "Bit 8 - HSI clock enabled"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 8)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 9)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    #[must_use]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<CRrs> {
        HSIASFS_W::new(self, 11)
    }
    #[doc = "Bit 16 - HSE clock enabled"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSE Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    #[doc = "Bit 20 - HSE sysclk and PLL M divider prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hsepre(&mut self) -> HSEPRE_W<CRrs> {
        HSEPRE_W::new(self, 20)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<CRrs> {
        PLLON_W::new(self, 24)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W<CRrs> {
        PLLSAI1ON_W::new(self, 26)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x61"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x61;
}
