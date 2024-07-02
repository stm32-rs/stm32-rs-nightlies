///Register `PLLSYSCFGR` reader
pub type R = crate::R<PLLSYSCFGRrs>;
///Register `PLLSYSCFGR` writer
pub type W = crate::W<PLLSYSCFGRrs>;
///Field `PLLSRC` reader - PLL input clock source
pub type PLLSRC_R = crate::FieldReader;
///Field `PLLSRC` writer - PLL input clock source
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLLM` reader - Division factor M of the PLL input clock divider
pub type PLLM_R = crate::FieldReader;
///Field `PLLM` writer - Division factor M of the PLL input clock divider
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLLN` reader - PLL frequency multiplication factor N
pub type PLLN_R = crate::FieldReader;
///Field `PLLN` writer - PLL frequency multiplication factor N
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLLPEN` reader - PLLPCLK clock output enable
pub type PLLPEN_R = crate::BitReader;
///Field `PLLPEN` writer - PLLPCLK clock output enable
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLP` reader - PLL VCO division factor P for PLLPCLK clock output
pub type PLLP_R = crate::FieldReader;
///Field `PLLP` writer - PLL VCO division factor P for PLLPCLK clock output
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PLLQEN` reader - PLLQCLK clock output enable
pub type PLLQEN_R = crate::BitReader;
///Field `PLLQEN` writer - PLLQCLK clock output enable
pub type PLLQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLQ` reader - PLL VCO division factor Q for PLLQCLK clock output
pub type PLLQ_R = crate::FieldReader;
///Field `PLLQ` writer - PLL VCO division factor Q for PLLQCLK clock output
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLLREN` reader - PLLRCLK clock output enable
pub type PLLREN_R = crate::BitReader;
///Field `PLLREN` writer - PLLRCLK clock output enable
pub type PLLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLR` reader - PLL VCO division factor R for PLLRCLK clock output
pub type PLLR_R = crate::FieldReader;
///Field `PLLR` writer - PLL VCO division factor R for PLLRCLK clock output
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:1 - PLL input clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Division factor M of the PLL input clock divider
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:14 - PLL frequency multiplication factor N
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - PLLPCLK clock output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 24 - PLLQCLK clock output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - PLLRCLK clock output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSYSCFGR")
            .field("pllsrc", &self.pllsrc())
            .field("pllm", &self.pllm())
            .field("plln", &self.plln())
            .field("pllpen", &self.pllpen())
            .field("pllp", &self.pllp())
            .field("pllqen", &self.pllqen())
            .field("pllq", &self.pllq())
            .field("pllren", &self.pllren())
            .field("pllr", &self.pllr())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLL input clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLSYSCFGRrs> {
        PLLSRC_W::new(self, 0)
    }
    ///Bits 4:6 - Division factor M of the PLL input clock divider
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<PLLSYSCFGRrs> {
        PLLM_W::new(self, 4)
    }
    ///Bits 8:14 - PLL frequency multiplication factor N
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<PLLSYSCFGRrs> {
        PLLN_W::new(self, 8)
    }
    ///Bit 16 - PLLPCLK clock output enable
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<PLLSYSCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    ///Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<PLLSYSCFGRrs> {
        PLLP_W::new(self, 17)
    }
    ///Bit 24 - PLLQCLK clock output enable
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<PLLSYSCFGRrs> {
        PLLQEN_W::new(self, 24)
    }
    ///Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<PLLSYSCFGRrs> {
        PLLQ_W::new(self, 25)
    }
    ///Bit 28 - PLLRCLK clock output enable
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<PLLSYSCFGRrs> {
        PLLREN_W::new(self, 28)
    }
    ///Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<PLLSYSCFGRrs> {
        PLLR_W::new(self, 29)
    }
}
/**PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsyscfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsyscfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#RCC:PLLSYSCFGR)*/
pub struct PLLSYSCFGRrs;
impl crate::RegisterSpec for PLLSYSCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllsyscfgr::R`](R) reader structure
impl crate::Readable for PLLSYSCFGRrs {}
///`write(|w| ..)` method takes [`pllsyscfgr::W`](W) writer structure
impl crate::Writable for PLLSYSCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLLSYSCFGR to value 0x1000
impl crate::Resettable for PLLSYSCFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
