#[doc = "Register `VDSL` reader"]
pub type R = crate::R<VDSLrs>;
#[doc = "Register `VDSL` writer"]
pub type W = crate::W<VDSLrs>;
#[doc = "Field `LENG` reader - Non-volatile data segment length"]
pub type LENG_R = crate::FieldReader<u16>;
#[doc = "Field `LENG` writer - Non-volatile data segment length"]
pub type LENG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    #[must_use]
    pub fn leng(&mut self) -> LENG_W<VDSLrs> {
        LENG_W::new(self, 6)
    }
}
#[doc = "Volatile data segment length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdsl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdsl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDSLrs;
impl crate::RegisterSpec for VDSLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdsl::R`](R) reader structure"]
impl crate::Readable for VDSLrs {}
#[doc = "`write(|w| ..)` method takes [`vdsl::W`](W) writer structure"]
impl crate::Writable for VDSLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDSL to value 0"]
impl crate::Resettable for VDSLrs {
    const RESET_VALUE: u32 = 0;
}
