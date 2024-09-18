///Register `MDMA_C18MAR` reader
pub type R = crate::R<MDMA_C18MARrs>;
///Register `MDMA_C18MAR` writer
pub type W = crate::W<MDMA_C18MARrs>;
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
        f.debug_struct("MDMA_C18MAR")
            .field("mar", &self.mar())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MAR
    #[inline(always)]
    #[must_use]
    pub fn mar(&mut self) -> MAR_W<MDMA_C18MARrs> {
        MAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:MDMA_C18MAR)*/
pub struct MDMA_C18MARrs;
impl crate::RegisterSpec for MDMA_C18MARrs {
    type Ux = u32;
}
///`read()` method returns [`mdma_c18mar::R`](R) reader structure
impl crate::Readable for MDMA_C18MARrs {}
///`write(|w| ..)` method takes [`mdma_c18mar::W`](W) writer structure
impl crate::Writable for MDMA_C18MARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDMA_C18MAR to value 0
impl crate::Resettable for MDMA_C18MARrs {
    const RESET_VALUE: u32 = 0;
}
