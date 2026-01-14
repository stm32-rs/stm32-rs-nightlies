///Register `C6BRUR` reader
pub type R = crate::R<C6BRURrs>;
///Register `C6BRUR` writer
pub type W = crate::W<C6BRURrs>;
///Field `SUV` reader - SUV
pub type SUV_R = crate::FieldReader<u16>;
///Field `SUV` writer - SUV
pub type SUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DUV` reader - DUV
pub type DUV_R = crate::FieldReader<u16>;
///Field `DUV` writer - DUV
pub type DUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - SUV
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - DUV
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C6BRUR")
            .field("suv", &self.suv())
            .field("duv", &self.duv())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - SUV
    #[inline(always)]
    pub fn suv(&mut self) -> SUV_W<'_, C6BRURrs> {
        SUV_W::new(self, 0)
    }
    ///Bits 16:31 - DUV
    #[inline(always)]
    pub fn duv(&mut self) -> DUV_W<'_, C6BRURrs> {
        DUV_W::new(self, 16)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c6brur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6brur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6BRUR)*/
pub struct C6BRURrs;
impl crate::RegisterSpec for C6BRURrs {
    type Ux = u32;
}
///`read()` method returns [`c6brur::R`](R) reader structure
impl crate::Readable for C6BRURrs {}
///`write(|w| ..)` method takes [`c6brur::W`](W) writer structure
impl crate::Writable for C6BRURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C6BRUR to value 0
impl crate::Resettable for C6BRURrs {}
