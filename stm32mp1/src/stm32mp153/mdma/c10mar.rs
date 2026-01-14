///Register `C10MAR` reader
pub type R = crate::R<C10MARrs>;
///Register `C10MAR` writer
pub type W = crate::W<C10MARrs>;
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
        f.debug_struct("C10MAR").field("mar", &self.mar()).finish()
    }
}
impl W {
    ///Bits 0:31 - MAR
    #[inline(always)]
    pub fn mar(&mut self) -> MAR_W<'_, C10MARrs> {
        MAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c10mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C10MAR)*/
pub struct C10MARrs;
impl crate::RegisterSpec for C10MARrs {
    type Ux = u32;
}
///`read()` method returns [`c10mar::R`](R) reader structure
impl crate::Readable for C10MARrs {}
///`write(|w| ..)` method takes [`c10mar::W`](W) writer structure
impl crate::Writable for C10MARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C10MAR to value 0
impl crate::Resettable for C10MARrs {}
