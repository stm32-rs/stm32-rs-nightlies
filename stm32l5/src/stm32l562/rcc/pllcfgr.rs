///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
///Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_R = crate::FieldReader;
///Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLLM` reader - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_R = crate::FieldReader;
///Field `PLLM` writer - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLLN` reader - Main PLL multiplication factor for VCO
pub type PLLN_R = crate::FieldReader;
///Field `PLLN` writer - Main PLL multiplication factor for VCO
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLLPEN` reader - Main PLL PLLSAI3CLK output enable
pub type PLLPEN_R = crate::BitReader;
///Field `PLLPEN` writer - Main PLL PLLSAI3CLK output enable
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLP` reader - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_R = crate::BitReader;
///Field `PLLP` writer - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
pub type PLLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLQEN` reader - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_R = crate::BitReader;
///Field `PLLQEN` writer - Main PLL PLLUSB1CLK output enable
pub type PLLQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLQ` reader - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_R = crate::FieldReader;
///Field `PLLQ` writer - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLLREN` reader - Main PLL PLLCLK output enable
pub type PLLREN_R = crate::BitReader;
///Field `PLLREN` writer - Main PLL PLLCLK output enable
pub type PLLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLR` reader - Main PLL division factor for PLLCLK (system clock)
pub type PLLR_R = crate::FieldReader;
///Field `PLLR` writer - Main PLL division factor for PLLCLK (system clock)
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLLPDIV` reader - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_R = crate::FieldReader;
///Field `PLLPDIV` writer - Main PLL division factor for PLLSAI2CLK
pub type PLLPDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pllpdiv", &self.pllpdiv())
            .field("pllr", &self.pllr())
            .field("pllren", &self.pllren())
            .field("pllq", &self.pllq())
            .field("pllqen", &self.pllqen())
            .field("pllp", &self.pllp())
            .field("pllpen", &self.pllpen())
            .field("plln", &self.plln())
            .field("pllm", &self.pllm())
            .field("pllsrc", &self.pllsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<'_, PLLCFGRrs> {
        PLLSRC_W::new(self, 0)
    }
    ///Bits 4:7 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W<'_, PLLCFGRrs> {
        PLLM_W::new(self, 4)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W<'_, PLLCFGRrs> {
        PLLN_W::new(self, 8)
    }
    ///Bit 16 - Main PLL PLLSAI3CLK output enable
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W<'_, PLLCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    ///Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W<'_, PLLCFGRrs> {
        PLLP_W::new(self, 17)
    }
    ///Bit 20 - Main PLL PLLUSB1CLK output enable
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W<'_, PLLCFGRrs> {
        PLLQEN_W::new(self, 20)
    }
    ///Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<'_, PLLCFGRrs> {
        PLLQ_W::new(self, 21)
    }
    ///Bit 24 - Main PLL PLLCLK output enable
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W<'_, PLLCFGRrs> {
        PLLREN_W::new(self, 24)
    }
    ///Bits 25:26 - Main PLL division factor for PLLCLK (system clock)
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W<'_, PLLCFGRrs> {
        PLLR_W::new(self, 25)
    }
    ///Bits 27:31 - Main PLL division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W<'_, PLLCFGRrs> {
        PLLPDIV_W::new(self, 27)
    }
}
/**PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:PLLCFGR)*/
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllcfgr::R`](R) reader structure
impl crate::Readable for PLLCFGRrs {}
///`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCFGR to value 0x1000
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
