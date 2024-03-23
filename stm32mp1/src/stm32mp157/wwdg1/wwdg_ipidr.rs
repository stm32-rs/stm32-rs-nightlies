#[doc = "Register `WWDG_IPIDR` reader"]
pub type R = crate::R<WWDG_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "WWDG ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_IPIDRrs;
impl crate::RegisterSpec for WWDG_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_ipidr::R`](R) reader structure"]
impl crate::Readable for WWDG_IPIDRrs {}
#[doc = "`reset()` method sets WWDG_IPIDR to value 0x0012_0051"]
impl crate::Resettable for WWDG_IPIDRrs {
    const RESET_VALUE: u32 = 0x0012_0051;
}
