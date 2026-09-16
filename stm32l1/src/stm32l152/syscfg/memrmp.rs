///Register `MEMRMP` reader
pub type R = crate::R<MEMRMPrs>;
///Register `MEMRMP` writer
pub type W = crate::W<MEMRMPrs>;
///Field `MEM_MODE` reader - MEM_MODE
pub type MEM_MODE_R = crate::FieldReader;
///Field `MEM_MODE` writer - MEM_MODE
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BOOT_MODE` reader - BOOT_MODE
pub type BOOT_MODE_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - MEM_MODE
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - BOOT_MODE
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRMP")
            .field("mem_mode", &self.mem_mode())
            .field("boot_mode", &self.boot_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MEM_MODE
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, MEMRMPrs> {
        MEM_MODE_W::new(self, 0)
    }
}
/**memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#SYSCFG:MEMRMP)*/
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
