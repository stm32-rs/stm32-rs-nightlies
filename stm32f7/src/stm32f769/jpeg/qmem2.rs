///Register `QMEM2%s` reader
pub type R = crate::R<QMEM2rs>;
///Register `QMEM2%s` writer
pub type W = crate::W<QMEM2rs>;
///Field `QMem_RAM` reader - QMem RAM
pub type QMEM_RAM_R = crate::FieldReader<u32>;
///Field `QMem_RAM` writer - QMem RAM
pub type QMEM_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - QMem RAM
    #[inline(always)]
    pub fn qmem_ram(&self) -> QMEM_RAM_R {
        QMEM_RAM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QMEM2")
            .field("qmem_ram", &self.qmem_ram())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - QMem RAM
    #[inline(always)]
    pub fn qmem_ram(&mut self) -> QMEM_RAM_W<QMEM2rs> {
        QMEM_RAM_W::new(self, 0)
    }
}
/**JPEG quantization tables

You can [`read`](crate::Reg::read) this register and get [`qmem2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#JPEG:QMEM2[0])*/
pub struct QMEM2rs;
impl crate::RegisterSpec for QMEM2rs {
    type Ux = u32;
}
///`read()` method returns [`qmem2::R`](R) reader structure
impl crate::Readable for QMEM2rs {}
///`write(|w| ..)` method takes [`qmem2::W`](W) writer structure
impl crate::Writable for QMEM2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets QMEM2%s to value 0
impl crate::Resettable for QMEM2rs {}
