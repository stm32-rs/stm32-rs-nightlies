///Register `OPTSR2_PRG` reader
pub type R = crate::R<OPTSR2_PRGrs>;
///Register `OPTSR2_PRG` writer
pub type W = crate::W<OPTSR2_PRGrs>;
///Field `TCM_AXI_SHARED` reader - TCM RAM sharing status bit
pub type TCM_AXI_SHARED_R = crate::FieldReader;
///Field `TCM_AXI_SHARED` writer - TCM RAM sharing status bit
pub type TCM_AXI_SHARED_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPUFREQ_BOOST` reader - CPU frequency boost status bit
pub type CPUFREQ_BOOST_R = crate::BitReader;
///Field `CPUFREQ_BOOST` writer - CPU frequency boost status bit
pub type CPUFREQ_BOOST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - TCM RAM sharing status bit
    #[inline(always)]
    pub fn tcm_axi_shared(&self) -> TCM_AXI_SHARED_R {
        TCM_AXI_SHARED_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - CPU frequency boost status bit
    #[inline(always)]
    pub fn cpufreq_boost(&self) -> CPUFREQ_BOOST_R {
        CPUFREQ_BOOST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTSR2_PRG")
            .field("tcm_axi_shared", &self.tcm_axi_shared())
            .field("cpufreq_boost", &self.cpufreq_boost())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TCM RAM sharing status bit
    #[inline(always)]
    pub fn tcm_axi_shared(&mut self) -> TCM_AXI_SHARED_W<'_, OPTSR2_PRGrs> {
        TCM_AXI_SHARED_W::new(self, 0)
    }
    ///Bit 2 - CPU frequency boost status bit
    #[inline(always)]
    pub fn cpufreq_boost(&mut self) -> CPUFREQ_BOOST_W<'_, OPTSR2_PRGrs> {
        CPUFREQ_BOOST_W::new(self, 2)
    }
}
/**FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`optsr2_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr2_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FLASH:OPTSR2_PRG)*/
pub struct OPTSR2_PRGrs;
impl crate::RegisterSpec for OPTSR2_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`optsr2_prg::R`](R) reader structure
impl crate::Readable for OPTSR2_PRGrs {}
///`write(|w| ..)` method takes [`optsr2_prg::W`](W) writer structure
impl crate::Writable for OPTSR2_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTSR2_PRG to value 0
impl crate::Resettable for OPTSR2_PRGrs {}
