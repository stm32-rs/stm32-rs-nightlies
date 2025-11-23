///Register `C26LAR` reader
pub type R = crate::R<C26LARrs>;
///Register `C26LAR` writer
pub type W = crate::W<C26LARrs>;
///Field `LAR` reader - LAR
pub type LAR_R = crate::FieldReader<u32>;
///Field `LAR` writer - LAR
pub type LAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - LAR
    #[inline(always)]
    pub fn lar(&self) -> LAR_R {
        LAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C26LAR").field("lar", &self.lar()).finish()
    }
}
impl W {
    ///Bits 0:31 - LAR
    #[inline(always)]
    pub fn lar(&mut self) -> LAR_W<'_, C26LARrs> {
        LAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c26lar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26lar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C26LAR)*/
pub struct C26LARrs;
impl crate::RegisterSpec for C26LARrs {
    type Ux = u32;
}
///`read()` method returns [`c26lar::R`](R) reader structure
impl crate::Readable for C26LARrs {}
///`write(|w| ..)` method takes [`c26lar::W`](W) writer structure
impl crate::Writable for C26LARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C26LAR to value 0
impl crate::Resettable for C26LARrs {}
