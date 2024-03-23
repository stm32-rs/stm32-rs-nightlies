#[doc = "Register `PATT` reader"]
pub type R = crate::R<PATTrs>;
#[doc = "Register `PATT` writer"]
pub type W = crate::W<PATTrs>;
#[doc = "Field `ATTSETx` reader - ATTSETx"]
pub type ATTSETX_R = crate::FieldReader;
#[doc = "Field `ATTSETx` writer - ATTSETx"]
pub type ATTSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTWAITx` reader - ATTWAITx"]
pub type ATTWAITX_R = crate::FieldReader;
#[doc = "Field `ATTWAITx` writer - ATTWAITx"]
pub type ATTWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHOLDx` reader - ATTHOLDx"]
pub type ATTHOLDX_R = crate::FieldReader;
#[doc = "Field `ATTHOLDx` writer - ATTHOLDx"]
pub type ATTHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHIZx` reader - ATTHIZx"]
pub type ATTHIZX_R = crate::FieldReader;
#[doc = "Field `ATTHIZx` writer - ATTHIZx"]
pub type ATTHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    #[must_use]
    pub fn attsetx(&mut self) -> ATTSETX_W<PATTrs> {
        ATTSETX_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    #[must_use]
    pub fn attwaitx(&mut self) -> ATTWAITX_W<PATTrs> {
        ATTWAITX_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    #[must_use]
    pub fn attholdx(&mut self) -> ATTHOLDX_W<PATTrs> {
        ATTHOLDX_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    #[must_use]
    pub fn atthizx(&mut self) -> ATTHIZX_W<PATTrs> {
        ATTHIZX_W::new(self, 24)
    }
}
#[doc = "Attribute memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PATTrs;
impl crate::RegisterSpec for PATTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patt::R`](R) reader structure"]
impl crate::Readable for PATTrs {}
#[doc = "`write(|w| ..)` method takes [`patt::W`](W) writer structure"]
impl crate::Writable for PATTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PATT to value 0xfcfc_fcfc"]
impl crate::Resettable for PATTrs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
