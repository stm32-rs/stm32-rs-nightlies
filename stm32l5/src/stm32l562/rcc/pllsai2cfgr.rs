///Register `PLLSAI2CFGR` reader
pub type R = crate::R<PLLSAI2CFGRrs>;
///Register `PLLSAI2CFGR` writer
pub type W = crate::W<PLLSAI2CFGRrs>;
///Field `PLLSAI2SRC` reader - PLLSAI2SRC
pub type PLLSAI2SRC_R = crate::FieldReader;
///Field `PLLSAI2SRC` writer - PLLSAI2SRC
pub type PLLSAI2SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PLLSAI2M` reader - Division factor for PLLSAI2 input clock
pub type PLLSAI2M_R = crate::FieldReader;
///Field `PLLSAI2M` writer - Division factor for PLLSAI2 input clock
pub type PLLSAI2M_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLLSAI2N` reader - SAI2PLL multiplication factor for VCO
pub type PLLSAI2N_R = crate::FieldReader;
///Field `PLLSAI2N` writer - SAI2PLL multiplication factor for VCO
pub type PLLSAI2N_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PLLSAI2PEN` reader - SAI2PLL PLLSAI2CLK output enable
pub type PLLSAI2PEN_R = crate::BitReader;
///Field `PLLSAI2PEN` writer - SAI2PLL PLLSAI2CLK output enable
pub type PLLSAI2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI2P` reader - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
pub type PLLSAI2P_R = crate::BitReader;
///Field `PLLSAI2P` writer - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
pub type PLLSAI2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI2PDIV` reader - PLLSAI2 division factor for PLLSAI2CLK
pub type PLLSAI2PDIV_R = crate::FieldReader;
///Field `PLLSAI2PDIV` writer - PLLSAI2 division factor for PLLSAI2CLK
pub type PLLSAI2PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - PLLSAI2SRC
    #[inline(always)]
    pub fn pllsai2src(&self) -> PLLSAI2SRC_R {
        PLLSAI2SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Division factor for PLLSAI2 input clock
    #[inline(always)]
    pub fn pllsai2m(&self) -> PLLSAI2M_R {
        PLLSAI2M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - SAI2PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLSAI2CFGR")
            .field("pllsai2pdiv", &self.pllsai2pdiv())
            .field("pllsai2p", &self.pllsai2p())
            .field("pllsai2pen", &self.pllsai2pen())
            .field("pllsai2n", &self.pllsai2n())
            .field("pllsai2m", &self.pllsai2m())
            .field("pllsai2src", &self.pllsai2src())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PLLSAI2SRC
    #[inline(always)]
    pub fn pllsai2src(&mut self) -> PLLSAI2SRC_W<'_, PLLSAI2CFGRrs> {
        PLLSAI2SRC_W::new(self, 0)
    }
    ///Bits 4:7 - Division factor for PLLSAI2 input clock
    #[inline(always)]
    pub fn pllsai2m(&mut self) -> PLLSAI2M_W<'_, PLLSAI2CFGRrs> {
        PLLSAI2M_W::new(self, 4)
    }
    ///Bits 8:14 - SAI2PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W<'_, PLLSAI2CFGRrs> {
        PLLSAI2N_W::new(self, 8)
    }
    ///Bit 16 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W<'_, PLLSAI2CFGRrs> {
        PLLSAI2PEN_W::new(self, 16)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W<'_, PLLSAI2CFGRrs> {
        PLLSAI2P_W::new(self, 17)
    }
    ///Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W<'_, PLLSAI2CFGRrs> {
        PLLSAI2PDIV_W::new(self, 27)
    }
}
/**PLLSAI2 configuration register

You can [`read`](crate::Reg::read) this register and get [`pllsai2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllsai2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:PLLSAI2CFGR)*/
pub struct PLLSAI2CFGRrs;
impl crate::RegisterSpec for PLLSAI2CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllsai2cfgr::R`](R) reader structure
impl crate::Readable for PLLSAI2CFGRrs {}
///`write(|w| ..)` method takes [`pllsai2cfgr::W`](W) writer structure
impl crate::Writable for PLLSAI2CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLSAI2CFGR to value 0x1000
impl crate::Resettable for PLLSAI2CFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
