///Register `MDMA_C28BRUR` reader
pub type R = crate::R<MDMA_C28BRURrs>;
///Register `MDMA_C28BRUR` writer
pub type W = crate::W<MDMA_C28BRURrs>;
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
        f.debug_struct("MDMA_C28BRUR")
            .field("suv", &self.suv())
            .field("duv", &self.duv())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - SUV
    #[inline(always)]
    #[must_use]
    pub fn suv(&mut self) -> SUV_W<MDMA_C28BRURrs> {
        SUV_W::new(self, 0)
    }
    ///Bits 16:31 - DUV
    #[inline(always)]
    #[must_use]
    pub fn duv(&mut self) -> DUV_W<MDMA_C28BRURrs> {
        DUV_W::new(self, 16)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28brur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28brur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28BRUR)*/
pub struct MDMA_C28BRURrs;
impl crate::RegisterSpec for MDMA_C28BRURrs {
    type Ux = u32;
}
///`read()` method returns [`mdma_c28brur::R`](R) reader structure
impl crate::Readable for MDMA_C28BRURrs {}
///`write(|w| ..)` method takes [`mdma_c28brur::W`](W) writer structure
impl crate::Writable for MDMA_C28BRURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDMA_C28BRUR to value 0
impl crate::Resettable for MDMA_C28BRURrs {
    const RESET_VALUE: u32 = 0;
}
