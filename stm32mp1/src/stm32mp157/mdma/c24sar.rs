///Register `C24SAR` reader
pub type R = crate::R<C24SARrs>;
///Register `C24SAR` writer
pub type W = crate::W<C24SARrs>;
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
        f.debug_struct("C24SAR").field("sar", &self.sar()).finish()
    }
}
impl W {
    ///Bits 0:31 - SAR
    #[inline(always)]
    pub fn sar(&mut self) -> SAR_W<C24SARrs> {
        SAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c24sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24SAR)*/
pub struct C24SARrs;
impl crate::RegisterSpec for C24SARrs {
    type Ux = u32;
}
///`read()` method returns [`c24sar::R`](R) reader structure
impl crate::Readable for C24SARrs {}
///`write(|w| ..)` method takes [`c24sar::W`](W) writer structure
impl crate::Writable for C24SARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C24SAR to value 0
impl crate::Resettable for C24SARrs {}
