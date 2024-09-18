///Register `MDMA_C2MDR` reader
pub type R = crate::R<MDMA_C2MDRrs>;
///Register `MDMA_C2MDR` writer
pub type W = crate::W<MDMA_C2MDRrs>;
///Field `MDR` reader - MDR
pub type MDR_R = crate::FieldReader<u32>;
///Field `MDR` writer - MDR
pub type MDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MDR
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMA_C2MDR")
            .field("mdr", &self.mdr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MDR
    #[inline(always)]
    #[must_use]
    pub fn mdr(&mut self) -> MDR_W<MDMA_C2MDRrs> {
        MDR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2MDR)*/
pub struct MDMA_C2MDRrs;
impl crate::RegisterSpec for MDMA_C2MDRrs {
    type Ux = u32;
}
///`read()` method returns [`mdma_c2mdr::R`](R) reader structure
impl crate::Readable for MDMA_C2MDRrs {}
///`write(|w| ..)` method takes [`mdma_c2mdr::W`](W) writer structure
impl crate::Writable for MDMA_C2MDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDMA_C2MDR to value 0
impl crate::Resettable for MDMA_C2MDRrs {
    const RESET_VALUE: u32 = 0;
}
