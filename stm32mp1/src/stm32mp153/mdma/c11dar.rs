///Register `C11DAR` reader
pub type R = crate::R<C11DARrs>;
///Register `C11DAR` writer
pub type W = crate::W<C11DARrs>;
///Field `DAR` reader - DAR
pub type DAR_R = crate::FieldReader<u32>;
///Field `DAR` writer - DAR
pub type DAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DAR
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C11DAR").field("dar", &self.dar()).finish()
    }
}
impl W {
    ///Bits 0:31 - DAR
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<'_, C11DARrs> {
        DAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c11dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C11DAR)*/
pub struct C11DARrs;
impl crate::RegisterSpec for C11DARrs {
    type Ux = u32;
}
///`read()` method returns [`c11dar::R`](R) reader structure
impl crate::Readable for C11DARrs {}
///`write(|w| ..)` method takes [`c11dar::W`](W) writer structure
impl crate::Writable for C11DARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C11DAR to value 0
impl crate::Resettable for C11DARrs {}
