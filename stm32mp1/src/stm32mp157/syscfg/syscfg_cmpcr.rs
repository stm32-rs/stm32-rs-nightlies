#[doc = "Register `SYSCFG_CMPCR` reader"]
pub type R = crate::R<SYSCFG_CMPCRrs>;
#[doc = "Register `SYSCFG_CMPCR` writer"]
pub type W = crate::W<SYSCFG_CMPCRrs>;
#[doc = "Field `SW_CTRL` reader - SW_CTRL"]
pub type SW_CTRL_R = crate::BitReader;
#[doc = "Field `SW_CTRL` writer - SW_CTRL"]
pub type SW_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY` reader - READY"]
pub type READY_R = crate::BitReader;
#[doc = "Field `RANSRC` reader - RANSRC"]
pub type RANSRC_R = crate::FieldReader;
#[doc = "Field `RANSRC` writer - RANSRC"]
pub type RANSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RAPSRC` reader - RAPSRC"]
pub type RAPSRC_R = crate::FieldReader;
#[doc = "Field `RAPSRC` writer - RAPSRC"]
pub type RAPSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ANSRC` reader - ANSRC"]
pub type ANSRC_R = crate::FieldReader;
#[doc = "Field `APSRC` reader - APSRC"]
pub type APSRC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    pub fn sw_ctrl(&self) -> SW_CTRL_R {
        SW_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    pub fn ransrc(&self) -> RANSRC_R {
        RANSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    pub fn rapsrc(&self) -> RAPSRC_R {
        RAPSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ANSRC"]
    #[inline(always)]
    pub fn ansrc(&self) -> ANSRC_R {
        ANSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - APSRC"]
    #[inline(always)]
    pub fn apsrc(&self) -> APSRC_R {
        APSRC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - SW_CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ctrl(&mut self) -> SW_CTRL_W<SYSCFG_CMPCRrs> {
        SW_CTRL_W::new(self, 1)
    }
    #[doc = "Bits 16:19 - RANSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ransrc(&mut self) -> RANSRC_W<SYSCFG_CMPCRrs> {
        RANSRC_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - RAPSRC"]
    #[inline(always)]
    #[must_use]
    pub fn rapsrc(&mut self) -> RAPSRC_W<SYSCFG_CMPCRrs> {
        RAPSRC_W::new(self, 20)
    }
}
#[doc = "SYSCFG compensation cell control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cmpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cmpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_CMPCRrs;
impl crate::RegisterSpec for SYSCFG_CMPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cmpcr::R`](R) reader structure"]
impl crate::Readable for SYSCFG_CMPCRrs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cmpcr::W`](W) writer structure"]
impl crate::Writable for SYSCFG_CMPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_CMPCR to value 0x0087_0000"]
impl crate::Resettable for SYSCFG_CMPCRrs {
    const RESET_VALUE: u32 = 0x0087_0000;
}
