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
#[doc = "Field `MSIRGSEL` writer - MSI clock range selection"]
pub type MSIRGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIRANGE {
    #[doc = "0: range 0 around 100 kHz"]
    Range100k = 0,
    #[doc = "1: range 1 around 200 kHz"]
    Range200k = 1,
    #[doc = "2: range 2 around 400 kHz"]
    Range400k = 2,
    #[doc = "3: range 3 around 800 kHz"]
    Range800k = 3,
    #[doc = "4: range 4 around 1 MHz"]
    Range1m = 4,
    #[doc = "5: range 5 around 2 MHz"]
    Range2m = 5,
    #[doc = "6: range 6 around 4 MHz"]
    Range4m = 6,
    #[doc = "7: range 7 around 8 MHz"]
    Range8m = 7,
    #[doc = "8: range 8 around 16 MHz"]
    Range16m = 8,
    #[doc = "9: range 9 around 24 MHz"]
    Range24m = 9,
    #[doc = "10: range 10 around 32 MHz"]
    Range32m = 10,
    #[doc = "11: range 11 around 48 MHz"]
    Range48m = 11,
}
impl From<MSIRANGE> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSIRANGE {
    type Ux = u8;
}
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader<MSIRANGE>;
impl MSIRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSIRANGE> {
        match self.bits {
            0 => Some(MSIRANGE::Range100k),
            1 => Some(MSIRANGE::Range200k),
            2 => Some(MSIRANGE::Range400k),
            3 => Some(MSIRANGE::Range800k),
            4 => Some(MSIRANGE::Range1m),
            5 => Some(MSIRANGE::Range2m),
            6 => Some(MSIRANGE::Range4m),
            7 => Some(MSIRANGE::Range8m),
            8 => Some(MSIRANGE::Range16m),
            9 => Some(MSIRANGE::Range24m),
            10 => Some(MSIRANGE::Range32m),
            11 => Some(MSIRANGE::Range48m),
            _ => None,
        }
    }
    #[doc = "range 0 around 100 kHz"]
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGE::Range100k
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGE::Range200k
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGE::Range400k
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGE::Range800k
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGE::Range1m
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGE::Range2m
    }
    #[doc = "range 6 around 4 MHz"]
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGE::Range4m
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGE::Range8m
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGE::Range16m
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGE::Range24m
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGE::Range32m
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGE::Range48m
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MSIRANGE>;
impl<'a, REG> MSIRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "range 0 around 100 kHz"]
    #[inline(always)]
    pub fn range100k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range100k)
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline(always)]
    pub fn range200k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range200k)
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline(always)]
    pub fn range400k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range400k)
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline(always)]
    pub fn range800k(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range800k)
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn range1m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range1m)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn range2m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range2m)
    }
    #[doc = "range 6 around 4 MHz"]
    #[inline(always)]
    pub fn range4m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range4m)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn range8m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range8m)
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline(always)]
    pub fn range16m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range16m)
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline(always)]
    pub fn range24m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range24m)
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline(always)]
    pub fn range32m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range32m)
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline(always)]
    pub fn range48m(self) -> &'a mut crate::W<REG> {
        self.variant(MSIRANGE::Range48m)
    }
}
#[doc = "Field `HSION` reader - HSI clock enable"]
pub type HSION_R = crate::BitReader;
#[doc = "Field `HSION` writer - HSI clock enable"]
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
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HSEON_R = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
pub type HSEBYP_R = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PLLSAI2ON` reader - SAI2 PLL enable"]
pub type PLLSAI2ON_R = crate::BitReader;
#[doc = "Field `PLLSAI2ON` writer - SAI2 PLL enable"]
pub type PLLSAI2ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSAI2RDY` reader - SAI2 PLL clock ready flag"]
pub type PLLSAI2RDY_R = crate::BitReader;
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
    #[doc = "Bit 8 - HSI clock enable"]
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
    #[doc = "Bit 16 - HSE clock enable"]
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
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    pub fn pllsai2on(&self) -> PLLSAI2ON_R {
        PLLSAI2ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SAI2 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai2rdy(&self) -> PLLSAI2RDY_R {
        PLLSAI2RDY_R::new(((self.bits >> 29) & 1) != 0)
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
    #[doc = "Bit 3 - MSI clock range selection"]
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<CRrs> {
        MSIRGSEL_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<CRrs> {
        MSIRANGE_W::new(self, 4)
    }
    #[doc = "Bit 8 - HSI clock enable"]
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
    #[doc = "Bit 16 - HSE clock enable"]
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
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
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
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2on(&mut self) -> PLLSAI2ON_W<CRrs> {
        PLLSAI2ON_W::new(self, 28)
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
#[doc = "`reset()` method sets CR to value 0x63"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x63;
}
