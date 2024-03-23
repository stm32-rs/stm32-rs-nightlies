#[doc = "Register `ESUR` reader"]
pub type R = crate::R<ESURrs>;
#[doc = "Register `ESUR` writer"]
pub type W = crate::W<ESURrs>;
#[doc = "Field `FSU` reader - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter."]
pub type FSU_R = crate::FieldReader;
#[doc = "Field `FSU` writer - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter."]
pub type FSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LSU` reader - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter."]
pub type LSU_R = crate::FieldReader;
#[doc = "Field `LSU` writer - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter."]
pub type LSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LEU` reader - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter."]
pub type LEU_R = crate::FieldReader;
#[doc = "Field `LEU` writer - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter."]
pub type LEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FEU` reader - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter."]
pub type FEU_R = crate::FieldReader;
#[doc = "Field `FEU` writer - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter."]
pub type FEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter."]
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter."]
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter."]
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter."]
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter."]
    #[inline(always)]
    #[must_use]
    pub fn fsu(&mut self) -> FSU_W<ESURrs> {
        FSU_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter."]
    #[inline(always)]
    #[must_use]
    pub fn lsu(&mut self) -> LSU_W<ESURrs> {
        LSU_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter."]
    #[inline(always)]
    #[must_use]
    pub fn leu(&mut self) -> LEU_W<ESURrs> {
        LEU_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter."]
    #[inline(always)]
    #[must_use]
    pub fn feu(&mut self) -> FEU_W<ESURrs> {
        FEU_W::new(self, 24)
    }
}
#[doc = "DCMI embedded synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESURrs;
impl crate::RegisterSpec for ESURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esur::R`](R) reader structure"]
impl crate::Readable for ESURrs {}
#[doc = "`write(|w| ..)` method takes [`esur::W`](W) writer structure"]
impl crate::Writable for ESURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESUR to value 0"]
impl crate::Resettable for ESURrs {
    const RESET_VALUE: u32 = 0;
}
