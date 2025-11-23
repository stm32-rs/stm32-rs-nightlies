///Register `C25MDR` reader
pub type R = crate::R<C25MDRrs>;
///Register `C25MDR` writer
pub type W = crate::W<C25MDRrs>;
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
        f.debug_struct("C25MDR").field("mdr", &self.mdr()).finish()
    }
}
impl W {
    ///Bits 0:31 - MDR
    #[inline(always)]
    pub fn mdr(&mut self) -> MDR_W<'_, C25MDRrs> {
        MDR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c25mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25MDR)*/
pub struct C25MDRrs;
impl crate::RegisterSpec for C25MDRrs {
    type Ux = u32;
}
///`read()` method returns [`c25mdr::R`](R) reader structure
impl crate::Readable for C25MDRrs {}
///`write(|w| ..)` method takes [`c25mdr::W`](W) writer structure
impl crate::Writable for C25MDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C25MDR to value 0
impl crate::Resettable for C25MDRrs {}
