///Register `ICNR` reader
pub type R = crate::R<ICNRrs>;
///Register `ICNR` writer
pub type W = crate::W<ICNRrs>;
///Field `AXI_M0` reader - AXI_M0
pub type AXI_M0_R = crate::BitReader;
///Field `AXI_M0` writer - AXI_M0
pub type AXI_M0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M1` reader - AXI_M1
pub type AXI_M1_R = crate::BitReader;
///Field `AXI_M1` writer - AXI_M1
pub type AXI_M1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M2` reader - AXI_M2
pub type AXI_M2_R = crate::BitReader;
///Field `AXI_M2` writer - AXI_M2
pub type AXI_M2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M3` reader - AXI_M3
pub type AXI_M3_R = crate::BitReader;
///Field `AXI_M3` writer - AXI_M3
pub type AXI_M3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M5` reader - AXI_M5
pub type AXI_M5_R = crate::BitReader;
///Field `AXI_M5` writer - AXI_M5
pub type AXI_M5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M6` reader - AXI_M6
pub type AXI_M6_R = crate::BitReader;
///Field `AXI_M6` writer - AXI_M6
pub type AXI_M6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M7` reader - AXI_M7
pub type AXI_M7_R = crate::BitReader;
///Field `AXI_M7` writer - AXI_M7
pub type AXI_M7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M8` reader - AXI_M8
pub type AXI_M8_R = crate::BitReader;
///Field `AXI_M8` writer - AXI_M8
pub type AXI_M8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M9` reader - AXI_M9
pub type AXI_M9_R = crate::BitReader;
///Field `AXI_M9` writer - AXI_M9
pub type AXI_M9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_M10` reader - AXI_M10
pub type AXI_M10_R = crate::BitReader;
///Field `AXI_M10` writer - AXI_M10
pub type AXI_M10_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AXI_M0
    #[inline(always)]
    pub fn axi_m0(&self) -> AXI_M0_R {
        AXI_M0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AXI_M1
    #[inline(always)]
    pub fn axi_m1(&self) -> AXI_M1_R {
        AXI_M1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AXI_M2
    #[inline(always)]
    pub fn axi_m2(&self) -> AXI_M2_R {
        AXI_M2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXI_M3
    #[inline(always)]
    pub fn axi_m3(&self) -> AXI_M3_R {
        AXI_M3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - AXI_M5
    #[inline(always)]
    pub fn axi_m5(&self) -> AXI_M5_R {
        AXI_M5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AXI_M6
    #[inline(always)]
    pub fn axi_m6(&self) -> AXI_M6_R {
        AXI_M6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AXI_M7
    #[inline(always)]
    pub fn axi_m7(&self) -> AXI_M7_R {
        AXI_M7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AXI_M8
    #[inline(always)]
    pub fn axi_m8(&self) -> AXI_M8_R {
        AXI_M8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AXI_M9
    #[inline(always)]
    pub fn axi_m9(&self) -> AXI_M9_R {
        AXI_M9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AXI_M10
    #[inline(always)]
    pub fn axi_m10(&self) -> AXI_M10_R {
        AXI_M10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICNR")
            .field("axi_m0", &self.axi_m0())
            .field("axi_m1", &self.axi_m1())
            .field("axi_m2", &self.axi_m2())
            .field("axi_m3", &self.axi_m3())
            .field("axi_m5", &self.axi_m5())
            .field("axi_m6", &self.axi_m6())
            .field("axi_m7", &self.axi_m7())
            .field("axi_m8", &self.axi_m8())
            .field("axi_m9", &self.axi_m9())
            .field("axi_m10", &self.axi_m10())
            .finish()
    }
}
impl W {
    ///Bit 0 - AXI_M0
    #[inline(always)]
    pub fn axi_m0(&mut self) -> AXI_M0_W<'_, ICNRrs> {
        AXI_M0_W::new(self, 0)
    }
    ///Bit 1 - AXI_M1
    #[inline(always)]
    pub fn axi_m1(&mut self) -> AXI_M1_W<'_, ICNRrs> {
        AXI_M1_W::new(self, 1)
    }
    ///Bit 2 - AXI_M2
    #[inline(always)]
    pub fn axi_m2(&mut self) -> AXI_M2_W<'_, ICNRrs> {
        AXI_M2_W::new(self, 2)
    }
    ///Bit 3 - AXI_M3
    #[inline(always)]
    pub fn axi_m3(&mut self) -> AXI_M3_W<'_, ICNRrs> {
        AXI_M3_W::new(self, 3)
    }
    ///Bit 5 - AXI_M5
    #[inline(always)]
    pub fn axi_m5(&mut self) -> AXI_M5_W<'_, ICNRrs> {
        AXI_M5_W::new(self, 5)
    }
    ///Bit 6 - AXI_M6
    #[inline(always)]
    pub fn axi_m6(&mut self) -> AXI_M6_W<'_, ICNRrs> {
        AXI_M6_W::new(self, 6)
    }
    ///Bit 7 - AXI_M7
    #[inline(always)]
    pub fn axi_m7(&mut self) -> AXI_M7_W<'_, ICNRrs> {
        AXI_M7_W::new(self, 7)
    }
    ///Bit 8 - AXI_M8
    #[inline(always)]
    pub fn axi_m8(&mut self) -> AXI_M8_W<'_, ICNRrs> {
        AXI_M8_W::new(self, 8)
    }
    ///Bit 9 - AXI_M9
    #[inline(always)]
    pub fn axi_m9(&mut self) -> AXI_M9_W<'_, ICNRrs> {
        AXI_M9_W::new(self, 9)
    }
    ///Bit 10 - AXI_M10
    #[inline(always)]
    pub fn axi_m10(&mut self) -> AXI_M10_W<'_, ICNRrs> {
        AXI_M10_W::new(self, 10)
    }
}
/**SYSCFG interconnect control register

You can [`read`](crate::Reg::read) this register and get [`icnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:ICNR)*/
pub struct ICNRrs;
impl crate::RegisterSpec for ICNRrs {
    type Ux = u32;
}
///`read()` method returns [`icnr::R`](R) reader structure
impl crate::Readable for ICNRrs {}
///`write(|w| ..)` method takes [`icnr::W`](W) writer structure
impl crate::Writable for ICNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICNR to value 0
impl crate::Resettable for ICNRrs {}
