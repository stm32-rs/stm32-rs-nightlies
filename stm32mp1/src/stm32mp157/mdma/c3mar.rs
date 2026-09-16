///Register `C3MAR` reader
pub type R = crate::R<C3MARrs>;
///Register `C3MAR` writer
pub type W = crate::W<C3MARrs>;
///Field `MAR` reader - MAR
pub type MAR_R = crate::FieldReader<u32>;
///Field `MAR` writer - MAR
pub type MAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MAR
    #[inline(always)]
    pub fn mar(&self) -> MAR_R {
        MAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C3MAR").field("mar", &self.mar()).finish()
    }
}
impl W {
    ///Bits 0:31 - MAR
    #[inline(always)]
    pub fn mar(&mut self) -> MAR_W<'_, C3MARrs> {
        MAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c3mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3MAR)*/
pub struct C3MARrs;
impl crate::RegisterSpec for C3MARrs {
    type Ux = u32;
}
///`read()` method returns [`c3mar::R`](R) reader structure
impl crate::Readable for C3MARrs {}
///`write(|w| ..)` method takes [`c3mar::W`](W) writer structure
impl crate::Writable for C3MARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C3MAR to value 0
impl crate::Resettable for C3MARrs {}
