#[doc = "Register `RCC_CSICFGR` reader"]
pub type R = crate::R<RCC_CSICFGRrs>;
#[doc = "Register `RCC_CSICFGR` writer"]
pub type W = crate::W<RCC_CSICFGRrs>;
#[doc = "Field `CSITRIM` reader - CSITRIM"]
pub type CSITRIM_R = crate::FieldReader;
#[doc = "Field `CSITRIM` writer - CSITRIM"]
pub type CSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CSICAL` reader - CSICAL"]
pub type CSICAL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 8:12 - CSITRIM"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - CSICAL"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - CSITRIM"]
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<RCC_CSICFGRrs> {
        CSITRIM_W::new(self, 8)
    }
}
#[doc = "This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_csicfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_csicfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_CSICFGRrs;
impl crate::RegisterSpec for RCC_CSICFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_csicfgr::R`](R) reader structure"]
impl crate::Readable for RCC_CSICFGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_csicfgr::W`](W) writer structure"]
impl crate::Writable for RCC_CSICFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CSICFGR to value 0x1000"]
impl crate::Resettable for RCC_CSICFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
