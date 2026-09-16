///Register `C29SAR` reader
pub type R = crate::R<C29SARrs>;
///Register `C29SAR` writer
pub type W = crate::W<C29SARrs>;
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
        f.debug_struct("C29SAR").field("sar", &self.sar()).finish()
    }
}
impl W {
    ///Bits 0:31 - SAR
    #[inline(always)]
    pub fn sar(&mut self) -> SAR_W<'_, C29SARrs> {
        SAR_W::new(self, 0)
    }
}
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c29sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29SAR)*/
pub struct C29SARrs;
impl crate::RegisterSpec for C29SARrs {
    type Ux = u32;
}
///`read()` method returns [`c29sar::R`](R) reader structure
impl crate::Readable for C29SARrs {}
///`write(|w| ..)` method takes [`c29sar::W`](W) writer structure
impl crate::Writable for C29SARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C29SAR to value 0
impl crate::Resettable for C29SARrs {}
