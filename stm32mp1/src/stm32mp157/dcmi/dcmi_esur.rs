#[doc = "Register `DCMI_ESUR` reader"]
pub type R = crate::R<DCMI_ESURrs>;
#[doc = "Register `DCMI_ESUR` writer"]
pub type W = crate::W<DCMI_ESURrs>;
#[doc = "Field `FSU` reader - FSU"]
pub type FSU_R = crate::FieldReader;
#[doc = "Field `FSU` writer - FSU"]
pub type FSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LSU` reader - LSU"]
pub type LSU_R = crate::FieldReader;
#[doc = "Field `LSU` writer - LSU"]
pub type LSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LEU` reader - LEU"]
pub type LEU_R = crate::FieldReader;
#[doc = "Field `LEU` writer - LEU"]
pub type LEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FEU` reader - FEU"]
pub type FEU_R = crate::FieldReader;
#[doc = "Field `FEU` writer - FEU"]
pub type FEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FSU"]
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - LSU"]
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LEU"]
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FEU"]
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FSU"]
    #[inline(always)]
    #[must_use]
    pub fn fsu(&mut self) -> FSU_W<DCMI_ESURrs> {
        FSU_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - LSU"]
    #[inline(always)]
    #[must_use]
    pub fn lsu(&mut self) -> LSU_W<DCMI_ESURrs> {
        LSU_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - LEU"]
    #[inline(always)]
    #[must_use]
    pub fn leu(&mut self) -> LEU_W<DCMI_ESURrs> {
        LEU_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - FEU"]
    #[inline(always)]
    #[must_use]
    pub fn feu(&mut self) -> FEU_W<DCMI_ESURrs> {
        FEU_W::new(self, 24)
    }
}
#[doc = "DCMI embedded synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_esur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_esur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMI_ESURrs;
impl crate::RegisterSpec for DCMI_ESURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmi_esur::R`](R) reader structure"]
impl crate::Readable for DCMI_ESURrs {}
#[doc = "`write(|w| ..)` method takes [`dcmi_esur::W`](W) writer structure"]
impl crate::Writable for DCMI_ESURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCMI_ESUR to value 0"]
impl crate::Resettable for DCMI_ESURrs {
    const RESET_VALUE: u32 = 0;
}
