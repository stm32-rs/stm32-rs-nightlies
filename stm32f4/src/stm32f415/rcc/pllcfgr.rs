///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
///Field `PLLM0` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM0_R = crate::BitReader;
///Field `PLLM0` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLM1` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM1_R = crate::BitReader;
///Field `PLLM1` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLM2` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM2_R = crate::BitReader;
///Field `PLLM2` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLM3` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM3_R = crate::BitReader;
///Field `PLLM3` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLM4` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM4_R = crate::BitReader;
///Field `PLLM4` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLM5` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM5_R = crate::BitReader;
///Field `PLLM5` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
pub type PLLM5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN0` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN0_R = crate::BitReader;
///Field `PLLN0` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN1` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN1_R = crate::BitReader;
///Field `PLLN1` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN2` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN2_R = crate::BitReader;
///Field `PLLN2` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN3` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN3_R = crate::BitReader;
///Field `PLLN3` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN4` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN4_R = crate::BitReader;
///Field `PLLN4` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN5` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN5_R = crate::BitReader;
///Field `PLLN5` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN6` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN6_R = crate::BitReader;
///Field `PLLN6` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN7` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN7_R = crate::BitReader;
///Field `PLLN7` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLN8` reader - Main PLL (PLL) multiplication factor for VCO
pub type PLLN8_R = crate::BitReader;
///Field `PLLN8` writer - Main PLL (PLL) multiplication factor for VCO
pub type PLLN8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLP0` reader - Main PLL (PLL) division factor for main system clock
pub type PLLP0_R = crate::BitReader;
///Field `PLLP0` writer - Main PLL (PLL) division factor for main system clock
pub type PLLP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLP1` reader - Main PLL (PLL) division factor for main system clock
pub type PLLP1_R = crate::BitReader;
///Field `PLLP1` writer - Main PLL (PLL) division factor for main system clock
pub type PLLP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSRC` reader - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
pub type PLLSRC_R = crate::BitReader;
///Field `PLLSRC` writer - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
pub type PLLSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLQ0` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ0_R = crate::BitReader;
///Field `PLLQ0` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLQ1` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ1_R = crate::BitReader;
///Field `PLLQ1` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLQ2` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ2_R = crate::BitReader;
///Field `PLLQ2` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLQ3` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ3_R = crate::BitReader;
///Field `PLLQ3` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
pub type PLLQ3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm0(&self) -> PLLM0_R {
        PLLM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm1(&self) -> PLLM1_R {
        PLLM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm2(&self) -> PLLM2_R {
        PLLM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm3(&self) -> PLLM3_R {
        PLLM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm4(&self) -> PLLM4_R {
        PLLM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm5(&self) -> PLLM5_R {
        PLLM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln0(&self) -> PLLN0_R {
        PLLN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln1(&self) -> PLLN1_R {
        PLLN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln2(&self) -> PLLN2_R {
        PLLN2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln3(&self) -> PLLN3_R {
        PLLN3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln4(&self) -> PLLN4_R {
        PLLN4_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln5(&self) -> PLLN5_R {
        PLLN5_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln6(&self) -> PLLN6_R {
        PLLN6_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln7(&self) -> PLLN7_R {
        PLLN7_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln8(&self) -> PLLN8_R {
        PLLN8_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp0(&self) -> PLLP0_R {
        PLLP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp1(&self) -> PLLP1_R {
        PLLP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq0(&self) -> PLLQ0_R {
        PLLQ0_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq1(&self) -> PLLQ1_R {
        PLLQ1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq2(&self) -> PLLQ2_R {
        PLLQ2_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq3(&self) -> PLLQ3_R {
        PLLQ3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pllq3", &self.pllq3())
            .field("pllq2", &self.pllq2())
            .field("pllq1", &self.pllq1())
            .field("pllq0", &self.pllq0())
            .field("pllsrc", &self.pllsrc())
            .field("pllp1", &self.pllp1())
            .field("pllp0", &self.pllp0())
            .field("plln8", &self.plln8())
            .field("plln7", &self.plln7())
            .field("plln6", &self.plln6())
            .field("plln5", &self.plln5())
            .field("plln4", &self.plln4())
            .field("plln3", &self.plln3())
            .field("plln2", &self.plln2())
            .field("plln1", &self.plln1())
            .field("plln0", &self.plln0())
            .field("pllm5", &self.pllm5())
            .field("pllm4", &self.pllm4())
            .field("pllm3", &self.pllm3())
            .field("pllm2", &self.pllm2())
            .field("pllm1", &self.pllm1())
            .field("pllm0", &self.pllm0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm0(&mut self) -> PLLM0_W<'_, PLLCFGRrs> {
        PLLM0_W::new(self, 0)
    }
    ///Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm1(&mut self) -> PLLM1_W<'_, PLLCFGRrs> {
        PLLM1_W::new(self, 1)
    }
    ///Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm2(&mut self) -> PLLM2_W<'_, PLLCFGRrs> {
        PLLM2_W::new(self, 2)
    }
    ///Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm3(&mut self) -> PLLM3_W<'_, PLLCFGRrs> {
        PLLM3_W::new(self, 3)
    }
    ///Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm4(&mut self) -> PLLM4_W<'_, PLLCFGRrs> {
        PLLM4_W::new(self, 4)
    }
    ///Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock
    #[inline(always)]
    pub fn pllm5(&mut self) -> PLLM5_W<'_, PLLCFGRrs> {
        PLLM5_W::new(self, 5)
    }
    ///Bit 6 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln0(&mut self) -> PLLN0_W<'_, PLLCFGRrs> {
        PLLN0_W::new(self, 6)
    }
    ///Bit 7 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln1(&mut self) -> PLLN1_W<'_, PLLCFGRrs> {
        PLLN1_W::new(self, 7)
    }
    ///Bit 8 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln2(&mut self) -> PLLN2_W<'_, PLLCFGRrs> {
        PLLN2_W::new(self, 8)
    }
    ///Bit 9 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln3(&mut self) -> PLLN3_W<'_, PLLCFGRrs> {
        PLLN3_W::new(self, 9)
    }
    ///Bit 10 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln4(&mut self) -> PLLN4_W<'_, PLLCFGRrs> {
        PLLN4_W::new(self, 10)
    }
    ///Bit 11 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln5(&mut self) -> PLLN5_W<'_, PLLCFGRrs> {
        PLLN5_W::new(self, 11)
    }
    ///Bit 12 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln6(&mut self) -> PLLN6_W<'_, PLLCFGRrs> {
        PLLN6_W::new(self, 12)
    }
    ///Bit 13 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln7(&mut self) -> PLLN7_W<'_, PLLCFGRrs> {
        PLLN7_W::new(self, 13)
    }
    ///Bit 14 - Main PLL (PLL) multiplication factor for VCO
    #[inline(always)]
    pub fn plln8(&mut self) -> PLLN8_W<'_, PLLCFGRrs> {
        PLLN8_W::new(self, 14)
    }
    ///Bit 16 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp0(&mut self) -> PLLP0_W<'_, PLLCFGRrs> {
        PLLP0_W::new(self, 16)
    }
    ///Bit 17 - Main PLL (PLL) division factor for main system clock
    #[inline(always)]
    pub fn pllp1(&mut self) -> PLLP1_W<'_, PLLCFGRrs> {
        PLLP1_W::new(self, 17)
    }
    ///Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<'_, PLLCFGRrs> {
        PLLSRC_W::new(self, 22)
    }
    ///Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq0(&mut self) -> PLLQ0_W<'_, PLLCFGRrs> {
        PLLQ0_W::new(self, 24)
    }
    ///Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq1(&mut self) -> PLLQ1_W<'_, PLLCFGRrs> {
        PLLQ1_W::new(self, 25)
    }
    ///Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq2(&mut self) -> PLLQ2_W<'_, PLLCFGRrs> {
        PLLQ2_W::new(self, 26)
    }
    ///Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks
    #[inline(always)]
    pub fn pllq3(&mut self) -> PLLQ3_W<'_, PLLCFGRrs> {
        PLLQ3_W::new(self, 27)
    }
}
/**PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#RCC:PLLCFGR)*/
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
///`reset()` method sets PLLCFGR to value 0x2400_3010
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x2400_3010;
}
