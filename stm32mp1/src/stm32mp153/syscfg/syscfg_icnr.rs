#[doc = "Register `SYSCFG_ICNR` reader"]
pub type R = crate::R<SYSCFG_ICNRrs>;
#[doc = "Register `SYSCFG_ICNR` writer"]
pub type W = crate::W<SYSCFG_ICNRrs>;
#[doc = "Field `AXI_M0` reader - AXI_M0"]
pub type AXI_M0_R = crate::BitReader;
#[doc = "Field `AXI_M0` writer - AXI_M0"]
pub type AXI_M0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M1` reader - AXI_M1"]
pub type AXI_M1_R = crate::BitReader;
#[doc = "Field `AXI_M1` writer - AXI_M1"]
pub type AXI_M1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M2` reader - AXI_M2"]
pub type AXI_M2_R = crate::BitReader;
#[doc = "Field `AXI_M2` writer - AXI_M2"]
pub type AXI_M2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M3` reader - AXI_M3"]
pub type AXI_M3_R = crate::BitReader;
#[doc = "Field `AXI_M3` writer - AXI_M3"]
pub type AXI_M3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M5` reader - AXI_M5"]
pub type AXI_M5_R = crate::BitReader;
#[doc = "Field `AXI_M5` writer - AXI_M5"]
pub type AXI_M5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M6` reader - AXI_M6"]
pub type AXI_M6_R = crate::BitReader;
#[doc = "Field `AXI_M6` writer - AXI_M6"]
pub type AXI_M6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M7` reader - AXI_M7"]
pub type AXI_M7_R = crate::BitReader;
#[doc = "Field `AXI_M7` writer - AXI_M7"]
pub type AXI_M7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M8` reader - AXI_M8"]
pub type AXI_M8_R = crate::BitReader;
#[doc = "Field `AXI_M8` writer - AXI_M8"]
pub type AXI_M8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M9` reader - AXI_M9"]
pub type AXI_M9_R = crate::BitReader;
#[doc = "Field `AXI_M9` writer - AXI_M9"]
pub type AXI_M9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_M10` reader - AXI_M10"]
pub type AXI_M10_R = crate::BitReader;
#[doc = "Field `AXI_M10` writer - AXI_M10"]
pub type AXI_M10_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AXI_M0"]
    #[inline(always)]
    pub fn axi_m0(&self) -> AXI_M0_R {
        AXI_M0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI_M1"]
    #[inline(always)]
    pub fn axi_m1(&self) -> AXI_M1_R {
        AXI_M1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI_M2"]
    #[inline(always)]
    pub fn axi_m2(&self) -> AXI_M2_R {
        AXI_M2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI_M3"]
    #[inline(always)]
    pub fn axi_m3(&self) -> AXI_M3_R {
        AXI_M3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - AXI_M5"]
    #[inline(always)]
    pub fn axi_m5(&self) -> AXI_M5_R {
        AXI_M5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AXI_M6"]
    #[inline(always)]
    pub fn axi_m6(&self) -> AXI_M6_R {
        AXI_M6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AXI_M7"]
    #[inline(always)]
    pub fn axi_m7(&self) -> AXI_M7_R {
        AXI_M7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AXI_M8"]
    #[inline(always)]
    pub fn axi_m8(&self) -> AXI_M8_R {
        AXI_M8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AXI_M9"]
    #[inline(always)]
    pub fn axi_m9(&self) -> AXI_M9_R {
        AXI_M9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AXI_M10"]
    #[inline(always)]
    pub fn axi_m10(&self) -> AXI_M10_R {
        AXI_M10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXI_M0"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m0(&mut self) -> AXI_M0_W<SYSCFG_ICNRrs> {
        AXI_M0_W::new(self, 0)
    }
    #[doc = "Bit 1 - AXI_M1"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m1(&mut self) -> AXI_M1_W<SYSCFG_ICNRrs> {
        AXI_M1_W::new(self, 1)
    }
    #[doc = "Bit 2 - AXI_M2"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m2(&mut self) -> AXI_M2_W<SYSCFG_ICNRrs> {
        AXI_M2_W::new(self, 2)
    }
    #[doc = "Bit 3 - AXI_M3"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m3(&mut self) -> AXI_M3_W<SYSCFG_ICNRrs> {
        AXI_M3_W::new(self, 3)
    }
    #[doc = "Bit 5 - AXI_M5"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m5(&mut self) -> AXI_M5_W<SYSCFG_ICNRrs> {
        AXI_M5_W::new(self, 5)
    }
    #[doc = "Bit 6 - AXI_M6"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m6(&mut self) -> AXI_M6_W<SYSCFG_ICNRrs> {
        AXI_M6_W::new(self, 6)
    }
    #[doc = "Bit 7 - AXI_M7"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m7(&mut self) -> AXI_M7_W<SYSCFG_ICNRrs> {
        AXI_M7_W::new(self, 7)
    }
    #[doc = "Bit 8 - AXI_M8"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m8(&mut self) -> AXI_M8_W<SYSCFG_ICNRrs> {
        AXI_M8_W::new(self, 8)
    }
    #[doc = "Bit 9 - AXI_M9"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m9(&mut self) -> AXI_M9_W<SYSCFG_ICNRrs> {
        AXI_M9_W::new(self, 9)
    }
    #[doc = "Bit 10 - AXI_M10"]
    #[inline(always)]
    #[must_use]
    pub fn axi_m10(&mut self) -> AXI_M10_W<SYSCFG_ICNRrs> {
        AXI_M10_W::new(self, 10)
    }
}
#[doc = "SYSCFG interconnect control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_icnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_icnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ICNRrs;
impl crate::RegisterSpec for SYSCFG_ICNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_icnr::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ICNRrs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_icnr::W`](W) writer structure"]
impl crate::Writable for SYSCFG_ICNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_ICNR to value 0"]
impl crate::Resettable for SYSCFG_ICNRrs {
    const RESET_VALUE: u32 = 0;
}
