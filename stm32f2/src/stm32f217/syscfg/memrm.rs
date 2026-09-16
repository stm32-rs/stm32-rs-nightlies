///Register `MEMRM` reader
pub type R = crate::R<MEMRMrs>;
///Register `MEMRM` writer
pub type W = crate::W<MEMRMrs>;
///Field `MEM_MODE` reader - MEM_MODE
pub type MEM_MODE_R = crate::FieldReader;
///Field `MEM_MODE` writer - MEM_MODE
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - MEM_MODE
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRM")
            .field("mem_mode", &self.mem_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MEM_MODE
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, MEMRMrs> {
        MEM_MODE_W::new(self, 0)
    }
}
/**memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#SYSCFG:MEMRM)*/
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
