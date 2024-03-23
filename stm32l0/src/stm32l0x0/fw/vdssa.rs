#[doc = "Register `VDSSA` reader"]
pub type R = crate::R<VDSSArs>;
#[doc = "Register `VDSSA` writer"]
pub type W = crate::W<VDSSArs>;
#[doc = "Field `ADD` reader - Volatile data segment start address"]
pub type ADD_R = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - Volatile data segment start address"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<VDSSArs> {
        ADD_W::new(self, 6)
    }
}
#[doc = "Volatile data segment start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdssa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdssa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDSSArs;
impl crate::RegisterSpec for VDSSArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdssa::R`](R) reader structure"]
impl crate::Readable for VDSSArs {}
#[doc = "`write(|w| ..)` method takes [`vdssa::W`](W) writer structure"]
impl crate::Writable for VDSSArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDSSA to value 0"]
impl crate::Resettable for VDSSArs {
    const RESET_VALUE: u32 = 0;
}
