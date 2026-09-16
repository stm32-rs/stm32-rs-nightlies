///Register `C28SAR` reader
pub type R = crate::R<C28SARrs>;
///Register `C28SAR` writer
pub type W = crate::W<C28SARrs>;
///Field `SAR` reader - SAR
pub type SAR_R = crate::FieldReader<u32>;
///Field `SAR` writer - SAR
pub type SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SAR
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C28SAR").field("sar", &self.sar()).finish()
    }
}
impl W {
    ///Bits 0:31 - SAR
    #[inline(always)]
    pub fn sar(&mut self) -> SAR_W<'_, C28SARrs> {
        SAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c28sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28SAR)*/
pub struct C28SARrs;
impl crate::RegisterSpec for C28SARrs {
    type Ux = u32;
}
///`read()` method returns [`c28sar::R`](R) reader structure
impl crate::Readable for C28SARrs {}
///`write(|w| ..)` method takes [`c28sar::W`](W) writer structure
impl crate::Writable for C28SARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C28SAR to value 0
impl crate::Resettable for C28SARrs {}
