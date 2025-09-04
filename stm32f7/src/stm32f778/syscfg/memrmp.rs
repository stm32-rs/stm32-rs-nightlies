///Register `MEMRMP` reader
pub type R = crate::R<MEMRMPrs>;
///Register `MEMRMP` writer
pub type W = crate::W<MEMRMPrs>;
///Field `MEM_BOOT` reader - Memory mapping selection
pub type MEM_BOOT_R = crate::BitReader;
///Field `MEM_BOOT` writer - Memory mapping selection
pub type MEM_BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FB_MODE` reader - Flash bank mode selection
pub type FB_MODE_R = crate::BitReader;
///Field `FB_MODE` writer - Flash bank mode selection
pub type FB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWP_FMC` reader - FMC memory mapping swap
pub type SWP_FMC_R = crate::FieldReader;
///Field `SWP_FMC` writer - FMC memory mapping swap
pub type SWP_FMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Memory mapping selection
    #[inline(always)]
    pub fn mem_boot(&self) -> MEM_BOOT_R {
        MEM_BOOT_R::new((self.bits & 1) != 0)
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
        f.debug_struct("MEMRMP")
            .field("mem_boot", &self.mem_boot())
            .field("fb_mode", &self.fb_mode())
            .field("swp_fmc", &self.swp_fmc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Memory mapping selection
    #[inline(always)]
    pub fn mem_boot(&mut self) -> MEM_BOOT_W<MEMRMPrs> {
        MEM_BOOT_W::new(self, 0)
    }
    ///Bit 8 - Flash bank mode selection
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FB_MODE_W<MEMRMPrs> {
        FB_MODE_W::new(self, 8)
    }
    ///Bits 10:11 - FMC memory mapping swap
    #[inline(always)]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W<MEMRMPrs> {
        SWP_FMC_W::new(self, 10)
    }
}
/**memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#SYSCFG:MEMRMP)*/
pub struct MEMRMPrs;
impl crate::RegisterSpec for MEMRMPrs {
    type Ux = u32;
}
///`read()` method returns [`memrmp::R`](R) reader structure
impl crate::Readable for MEMRMPrs {}
///`write(|w| ..)` method takes [`memrmp::W`](W) writer structure
impl crate::Writable for MEMRMPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMRMP to value 0
impl crate::Resettable for MEMRMPrs {}
