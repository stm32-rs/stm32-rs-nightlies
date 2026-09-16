///Register `C4MDR` reader
pub type R = crate::R<C4MDRrs>;
///Register `C4MDR` writer
pub type W = crate::W<C4MDRrs>;
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
        f.debug_struct("C4MDR").field("mdr", &self.mdr()).finish()
    }
}
impl W {
    ///Bits 0:31 - MDR
    #[inline(always)]
    pub fn mdr(&mut self) -> MDR_W<'_, C4MDRrs> {
        MDR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c4mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4MDR)*/
pub struct C4MDRrs;
impl crate::RegisterSpec for C4MDRrs {
    type Ux = u32;
}
///`read()` method returns [`c4mdr::R`](R) reader structure
impl crate::Readable for C4MDRrs {}
///`write(|w| ..)` method takes [`c4mdr::W`](W) writer structure
impl crate::Writable for C4MDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C4MDR to value 0
impl crate::Resettable for C4MDRrs {}
