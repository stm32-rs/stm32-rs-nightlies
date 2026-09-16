///Register `HWLPCTL` reader
pub type R = crate::R<HWLPCTLrs>;
///Register `HWLPCTL` writer
pub type W = crate::W<HWLPCTLrs>;
///Field `HW_LP_EN` reader - HW_LP_EN
pub type HW_LP_EN_R = crate::BitReader;
///Field `HW_LP_EN` writer - HW_LP_EN
pub type HW_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HW_LP_EXIT_IDLE_EN` reader - HW_LP_EXIT_IDLE_EN
pub type HW_LP_EXIT_IDLE_EN_R = crate::BitReader;
///Field `HW_LP_EXIT_IDLE_EN` writer - HW_LP_EXIT_IDLE_EN
pub type HW_LP_EXIT_IDLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HW_LP_IDLE_X32` reader - HW_LP_IDLE_X32
pub type HW_LP_IDLE_X32_R = crate::FieldReader<u16>;
///Field `HW_LP_IDLE_X32` writer - HW_LP_IDLE_X32
pub type HW_LP_IDLE_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 0 - HW_LP_EN
    #[inline(always)]
    pub fn hw_lp_en(&self) -> HW_LP_EN_R {
        HW_LP_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HW_LP_EXIT_IDLE_EN
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&self) -> HW_LP_EXIT_IDLE_EN_R {
        HW_LP_EXIT_IDLE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:27 - HW_LP_IDLE_X32
    #[inline(always)]
    pub fn hw_lp_idle_x32(&self) -> HW_LP_IDLE_X32_R {
        HW_LP_IDLE_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWLPCTL")
            .field("hw_lp_en", &self.hw_lp_en())
            .field("hw_lp_exit_idle_en", &self.hw_lp_exit_idle_en())
            .field("hw_lp_idle_x32", &self.hw_lp_idle_x32())
            .finish()
    }
}
impl W {
    ///Bit 0 - HW_LP_EN
    #[inline(always)]
    pub fn hw_lp_en(&mut self) -> HW_LP_EN_W<'_, HWLPCTLrs> {
        HW_LP_EN_W::new(self, 0)
    }
    ///Bit 1 - HW_LP_EXIT_IDLE_EN
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&mut self) -> HW_LP_EXIT_IDLE_EN_W<'_, HWLPCTLrs> {
        HW_LP_EXIT_IDLE_EN_W::new(self, 1)
    }
    ///Bits 16:27 - HW_LP_IDLE_X32
    #[inline(always)]
    pub fn hw_lp_idle_x32(&mut self) -> HW_LP_IDLE_X32_W<'_, HWLPCTLrs> {
        HW_LP_IDLE_X32_W::new(self, 16)
    }
}
/**DDRCTRL hardware low power control register

You can [`read`](crate::Reg::read) this register and get [`hwlpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwlpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:HWLPCTL)*/
pub struct HWLPCTLrs;
impl crate::RegisterSpec for HWLPCTLrs {
    type Ux = u32;
}
///`read()` method returns [`hwlpctl::R`](R) reader structure
impl crate::Readable for HWLPCTLrs {}
///`write(|w| ..)` method takes [`hwlpctl::W`](W) writer structure
impl crate::Writable for HWLPCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWLPCTL to value 0x03
impl crate::Resettable for HWLPCTLrs {
    const RESET_VALUE: u32 = 0x03;
}
