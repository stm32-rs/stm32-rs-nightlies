#[doc = "Register `DDRCTRL_INIT0` reader"]
pub type R = crate::R<DDRCTRL_INIT0rs>;
#[doc = "Register `DDRCTRL_INIT0` writer"]
pub type W = crate::W<DDRCTRL_INIT0rs>;
#[doc = "Field `PRE_CKE_X1024` reader - PRE_CKE_X1024"]
pub type PRE_CKE_X1024_R = crate::FieldReader<u16>;
#[doc = "Field `PRE_CKE_X1024` writer - PRE_CKE_X1024"]
pub type PRE_CKE_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `POST_CKE_X1024` reader - POST_CKE_X1024"]
pub type POST_CKE_X1024_R = crate::FieldReader<u16>;
#[doc = "Field `POST_CKE_X1024` writer - POST_CKE_X1024"]
pub type POST_CKE_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SKIP_DRAM_INIT` reader - SKIP_DRAM_INIT"]
pub type SKIP_DRAM_INIT_R = crate::FieldReader;
#[doc = "Field `SKIP_DRAM_INIT` writer - SKIP_DRAM_INIT"]
pub type SKIP_DRAM_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    pub fn pre_cke_x1024(&self) -> PRE_CKE_X1024_R {
        PRE_CKE_X1024_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    pub fn post_cke_x1024(&self) -> POST_CKE_X1024_R {
        POST_CKE_X1024_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    pub fn skip_dram_init(&self) -> SKIP_DRAM_INIT_R {
        SKIP_DRAM_INIT_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - PRE_CKE_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn pre_cke_x1024(&mut self) -> PRE_CKE_X1024_W<DDRCTRL_INIT0rs> {
        PRE_CKE_X1024_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - POST_CKE_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn post_cke_x1024(&mut self) -> POST_CKE_X1024_W<DDRCTRL_INIT0rs> {
        POST_CKE_X1024_W::new(self, 16)
    }
    #[doc = "Bits 30:31 - SKIP_DRAM_INIT"]
    #[inline(always)]
    #[must_use]
    pub fn skip_dram_init(&mut self) -> SKIP_DRAM_INIT_W<DDRCTRL_INIT0rs> {
        SKIP_DRAM_INIT_W::new(self, 30)
    }
}
#[doc = "DDRCTRL SDRAM initialization register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_INIT0rs;
impl crate::RegisterSpec for DDRCTRL_INIT0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_init0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_init0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT0 to value 0x0002_004e"]
impl crate::Resettable for DDRCTRL_INIT0rs {
    const RESET_VALUE: u32 = 0x0002_004e;
}
