///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `LSIRDY` reader - LSI clock ready flag
pub type LSIRDY_R = crate::BitReader;
///Field `LSERDY` reader - LSE clock ready flag
pub type LSERDY_R = crate::BitReader;
///Field `MSIRDY` reader - MSI clock ready flag
pub type MSIRDY_R = crate::BitReader;
///Field `HSIRDY` reader - HSI clock ready flag
pub type HSIRDY_R = crate::BitReader;
///Field `HSERDY` reader - HSE clock ready flag
pub type HSERDY_R = crate::BitReader;
///Field `PLL1RDY` reader - PLL1 clock ready flag
pub type PLL1RDY_R = crate::BitReader;
///Field `PLL2RDY` reader - PLL2 clock ready flag
pub type PLL2RDY_R = crate::BitReader;
///Field `PLL3RDY` reader - PLL3 clock ready flag
pub type PLL3RDY_R = crate::BitReader;
///Field `PLL4RDY` reader - PLL4 clock ready flag
pub type PLL4RDY_R = crate::BitReader;
impl R {
    ///Bit 0 - LSI clock ready flag
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE clock ready flag
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - PLL1 clock ready flag
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL2 clock ready flag
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL3 clock ready flag
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PLL4 clock ready flag
    #[inline(always)]
    pub fn pll4rdy(&self) -> PLL4RDY_R {
        PLL4RDY_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("lsirdy", &self.lsirdy())
            .field("lserdy", &self.lserdy())
            .field("msirdy", &self.msirdy())
            .field("hsirdy", &self.hsirdy())
            .field("hserdy", &self.hserdy())
            .field("pll1rdy", &self.pll1rdy())
            .field("pll2rdy", &self.pll2rdy())
            .field("pll3rdy", &self.pll3rdy())
            .field("pll4rdy", &self.pll4rdy())
            .finish()
    }
}
/**RCC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x08
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x08;
}
