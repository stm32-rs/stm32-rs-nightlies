///Register `MEMRM` reader
pub type R = crate::R<MEMRMrs>;
///Register `MEMRM` writer
pub type W = crate::W<MEMRMrs>;
///Field `MEM_MODE` reader - Memory mapping selection
pub type MEM_MODE_R = crate::FieldReader;
///Field `MEM_MODE` writer - Memory mapping selection
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FB_MODE` reader - Flash bank mode selection
pub type FB_MODE_R = crate::BitReader;
///Field `FB_MODE` writer - Flash bank mode selection
pub type FB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWP_FMC` reader - FMC memory mapping swap
pub type SWP_FMC_R = crate::FieldReader;
///Field `SWP_FMC` writer - FMC memory mapping swap
pub type SWP_FMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Memory mapping selection
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - Flash bank mode selection
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 10:11 - FMC memory mapping swap
    #[inline(always)]
    pub fn swp_fmc(&self) -> SWP_FMC_R {
        SWP_FMC_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRM")
            .field("mem_mode", &self.mem_mode())
            .field("fb_mode", &self.fb_mode())
            .field("swp_fmc", &self.swp_fmc())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Memory mapping selection
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, MEMRMrs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 8 - Flash bank mode selection
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FB_MODE_W<'_, MEMRMrs> {
        FB_MODE_W::new(self, 8)
    }
    ///Bits 10:11 - FMC memory mapping swap
    #[inline(always)]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W<'_, MEMRMrs> {
        SWP_FMC_W::new(self, 10)
    }
}
/**memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SYSCFG:MEMRM)*/
pub struct MEMRMrs;
impl crate::RegisterSpec for MEMRMrs {
    type Ux = u32;
}
///`read()` method returns [`memrm::R`](R) reader structure
impl crate::Readable for MEMRMrs {}
///`write(|w| ..)` method takes [`memrm::W`](W) writer structure
impl crate::Writable for MEMRMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMRM to value 0
impl crate::Resettable for MEMRMrs {}
