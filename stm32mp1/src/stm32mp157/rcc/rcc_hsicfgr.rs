#[doc = "Register `RCC_HSICFGR` reader"]
pub type R = crate::R<RCC_HSICFGRrs>;
#[doc = "Register `RCC_HSICFGR` writer"]
pub type W = crate::W<RCC_HSICFGRrs>;
#[doc = "Field `HSIDIV` reader - HSIDIV"]
pub type HSIDIV_R = crate::FieldReader;
#[doc = "Field `HSIDIV` writer - HSIDIV"]
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSITRIM` reader - HSITRIM"]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSITRIM"]
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HSICAL` reader - HSICAL"]
pub type HSICAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - HSICAL"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    #[must_use]
    pub fn hsidiv(&mut self) -> HSIDIV_W<RCC_HSICFGRrs> {
        HSIDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<RCC_HSICFGRrs> {
        HSITRIM_W::new(self, 8)
    }
}
#[doc = "This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_hsicfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_hsicfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_HSICFGRrs;
impl crate::RegisterSpec for RCC_HSICFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_hsicfgr::R`](R) reader structure"]
impl crate::Readable for RCC_HSICFGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_hsicfgr::W`](W) writer structure"]
impl crate::Writable for RCC_HSICFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_HSICFGR to value 0"]
impl crate::Resettable for RCC_HSICFGRrs {
    const RESET_VALUE: u32 = 0;
}
