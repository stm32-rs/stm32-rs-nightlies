///Register `MDMA_C2SAR` reader
pub type R = crate::R<MDMA_C2SARrs>;
///Register `MDMA_C2SAR` writer
pub type W = crate::W<MDMA_C2SARrs>;
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
        f.debug_struct("MDMA_C2SAR")
            .field("sar", &self.sar())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SAR
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<MDMA_C2SARrs> {
        SAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:MDMA_C2SAR)*/
pub struct MDMA_C2SARrs;
impl crate::RegisterSpec for MDMA_C2SARrs {
    type Ux = u32;
}
///`read()` method returns [`mdma_c2sar::R`](R) reader structure
impl crate::Readable for MDMA_C2SARrs {}
///`write(|w| ..)` method takes [`mdma_c2sar::W`](W) writer structure
impl crate::Writable for MDMA_C2SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDMA_C2SAR to value 0
impl crate::Resettable for MDMA_C2SARrs {
    const RESET_VALUE: u32 = 0;
}
