///Register `C15BRUR` reader
pub type R = crate::R<C15BRURrs>;
///Register `C15BRUR` writer
pub type W = crate::W<C15BRURrs>;
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
        f.debug_struct("C15BRUR")
            .field("suv", &self.suv())
            .field("duv", &self.duv())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - SUV
    #[inline(always)]
    pub fn suv(&mut self) -> SUV_W<'_, C15BRURrs> {
        SUV_W::new(self, 0)
    }
    ///Bits 16:31 - DUV
    #[inline(always)]
    pub fn duv(&mut self) -> DUV_W<'_, C15BRURrs> {
        DUV_W::new(self, 16)
    }
}
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c15brur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15brur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C15BRUR)*/
pub struct C15BRURrs;
impl crate::RegisterSpec for C15BRURrs {
    type Ux = u32;
}
///`read()` method returns [`c15brur::R`](R) reader structure
impl crate::Readable for C15BRURrs {}
///`write(|w| ..)` method takes [`c15brur::W`](W) writer structure
impl crate::Writable for C15BRURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C15BRUR to value 0
impl crate::Resettable for C15BRURrs {}
