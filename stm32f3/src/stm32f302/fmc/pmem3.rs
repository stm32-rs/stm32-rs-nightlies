#[doc = "Register `PMEM3` reader"]
pub type R = crate::R<PMEM3rs>;
#[doc = "Register `PMEM3` writer"]
pub type W = crate::W<PMEM3rs>;
#[doc = "Field `MEMSETx` reader - MEMSETx"]
pub type MEMSETX_R = crate::FieldReader;
#[doc = "Field `MEMSETx` writer - MEMSETx"]
pub type MEMSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMWAITx` reader - MEMWAITx"]
pub type MEMWAITX_R = crate::FieldReader;
#[doc = "Field `MEMWAITx` writer - MEMWAITx"]
pub type MEMWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHOLDx` reader - MEMHOLDx"]
pub type MEMHOLDX_R = crate::FieldReader;
#[doc = "Field `MEMHOLDx` writer - MEMHOLDx"]
pub type MEMHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHIZx` reader - MEMHIZx"]
pub type MEMHIZX_R = crate::FieldReader;
#[doc = "Field `MEMHIZx` writer - MEMHIZx"]
pub type MEMHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    #[must_use]
    pub fn memsetx(&mut self) -> MEMSETX_W<PMEM3rs> {
        MEMSETX_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    #[must_use]
    pub fn memwaitx(&mut self) -> MEMWAITX_W<PMEM3rs> {
        MEMWAITX_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    #[must_use]
    pub fn memholdx(&mut self) -> MEMHOLDX_W<PMEM3rs> {
        MEMHOLDX_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    #[must_use]
    pub fn memhizx(&mut self) -> MEMHIZX_W<PMEM3rs> {
        MEMHIZX_W::new(self, 24)
    }
}
#[doc = "Common memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMEM3rs;
impl crate::RegisterSpec for PMEM3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmem3::R`](R) reader structure"]
impl crate::Readable for PMEM3rs {}
#[doc = "`write(|w| ..)` method takes [`pmem3::W`](W) writer structure"]
impl crate::Writable for PMEM3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMEM3 to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM3rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
