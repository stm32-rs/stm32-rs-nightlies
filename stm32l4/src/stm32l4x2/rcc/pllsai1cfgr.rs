///Register `PLLSAI1CFGR` reader
pub type R = crate::R<PLLSAI1CFGRrs>;
///Register `PLLSAI1CFGR` writer
pub type W = crate::W<PLLSAI1CFGRrs>;
///Field `PLLSAI1N` reader - SAI1PLL multiplication factor for VCO
pub type PLLSAI1N_R = crate::FieldReader;
///Field `PLLSAI1N` writer - SAI1PLL multiplication factor for VCO
pub type PLLSAI1N_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLLSAI1PEN` reader - SAI1PLL PLLSAI1CLK output enable
pub type PLLSAI1PEN_R = crate::BitReader;
///Field `PLLSAI1PEN` writer - SAI1PLL PLLSAI1CLK output enable
pub type PLLSAI1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1P` reader - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
pub type PLLSAI1P_R = crate::BitReader;
///Field `PLLSAI1P` writer - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
pub type PLLSAI1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1QEN` reader - SAI1PLL PLLUSB2CLK output enable
pub type PLLSAI1QEN_R = crate::BitReader;
///Field `PLLSAI1QEN` writer - SAI1PLL PLLUSB2CLK output enable
pub type PLLSAI1QEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1Q` reader - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
pub type PLLSAI1Q_R = crate::FieldReader;
///Field `PLLSAI1Q` writer - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
pub type PLLSAI1Q_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLLSAI1REN` reader - PLLSAI1 PLLADC1CLK output enable
pub type PLLSAI1REN_R = crate::BitReader;
///Field `PLLSAI1REN` writer - PLLSAI1 PLLADC1CLK output enable
pub type PLLSAI1REN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1R` reader - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
pub type PLLSAI1R_R = crate::FieldReader;
///Field `PLLSAI1R` writer - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
pub type PLLSAI1R_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLLSAI1PDIV` reader - PLLSAI1 division factor for PLLSAI1CLK
pub type PLLSAI1PDIV_R = crate::FieldReader;
///Field `PLLSAI1PDIV` writer - PLLSAI1 division factor for PLLSAI1CLK
pub type PLLSAI1PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 8:14 - SAI1PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai1n(&self) -> PLLSAI1N_R {
        PLLSAI1N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - SAI1PLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllsai1pen(&self) -> PLLSAI1PEN_R {
        PLLSAI1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai1p(&self) -> PLLSAI1P_R {
        PLLSAI1P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - SAI1PLL PLLUSB2CLK output enable
    #[inline(always)]
    pub fn pllsai1qen(&self) -> PLLSAI1QEN_R {
        PLLSAI1QEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
    #[inline(always)]
    pub fn pllsai1q(&self) -> PLLSAI1Q_R {
        PLLSAI1Q_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - PLLSAI1 PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllsai1ren(&self) -> PLLSAI1REN_R {
        PLLSAI1REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai1r(&self) -> PLLSAI1R_R {
        PLLSAI1R_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK
    #[inline(always)]
    pub fn pllsai1pdiv(&self) -> PLLSAI1PDIV_R {
        PLLSAI1PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSAI1CFGR")
            .field("pllsai1r", &self.pllsai1r())
            .field("pllsai1ren", &self.pllsai1ren())
            .field("pllsai1q", &self.pllsai1q())
            .field("pllsai1qen", &self.pllsai1qen())
            .field("pllsai1p", &self.pllsai1p())
            .field("pllsai1pen", &self.pllsai1pen())
            .field("pllsai1n", &self.pllsai1n())
            .field("pllsai1pdiv", &self.pllsai1pdiv())
            .finish()
    }
}
impl W {
    ///Bits 8:14 - SAI1PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai1n(&mut self) -> PLLSAI1N_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1N_W::new(self, 8)
    }
    ///Bit 16 - SAI1PLL PLLSAI1CLK output enable
    #[inline(always)]
    pub fn pllsai1pen(&mut self) -> PLLSAI1PEN_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1PEN_W::new(self, 16)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai1p(&mut self) -> PLLSAI1P_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1P_W::new(self, 17)
    }
    ///Bit 20 - SAI1PLL PLLUSB2CLK output enable
    #[inline(always)]
    pub fn pllsai1qen(&mut self) -> PLLSAI1QEN_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1QEN_W::new(self, 20)
    }
    ///Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)
    #[inline(always)]
    pub fn pllsai1q(&mut self) -> PLLSAI1Q_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1Q_W::new(self, 21)
    }
    ///Bit 24 - PLLSAI1 PLLADC1CLK output enable
    #[inline(always)]
    pub fn pllsai1ren(&mut self) -> PLLSAI1REN_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1REN_W::new(self, 24)
    }
    ///Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai1r(&mut self) -> PLLSAI1R_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1R_W::new(self, 25)
    }
    ///Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK
    #[inline(always)]
    pub fn pllsai1pdiv(&mut self) -> PLLSAI1PDIV_W<'_, PLLSAI1CFGRrs> {
        PLLSAI1PDIV_W::new(self, 27)
    }
}
/**PLLSAI1 configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsai1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#RCC:PLLSAI1CFGR)*/
pub struct PLLSAI1CFGRrs;
impl crate::RegisterSpec for PLLSAI1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllsai1cfgr::R`](R) reader structure
impl crate::Readable for PLLSAI1CFGRrs {}
///`write(|w| ..)` method takes [`pllsai1cfgr::W`](W) writer structure
impl crate::Writable for PLLSAI1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLSAI1CFGR to value 0x1000
impl crate::Resettable for PLLSAI1CFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
