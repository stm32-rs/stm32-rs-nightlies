#[doc = "Register `ETH_MACPCSR` reader"]
pub type R = crate::R<ETH_MACPCSRrs>;
#[doc = "Register `ETH_MACPCSR` writer"]
pub type W = crate::W<ETH_MACPCSRrs>;
#[doc = "Field `PWRDWN` reader - PWRDWN"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - PWRDWN"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPKTEN` reader - MGKPKTEN"]
pub type MGKPKTEN_R = crate::BitReader;
#[doc = "Field `MGKPKTEN` writer - MGKPKTEN"]
pub type MGKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPKTEN` reader - RWKPKTEN"]
pub type RWKPKTEN_R = crate::BitReader;
#[doc = "Field `RWKPKTEN` writer - RWKPKTEN"]
pub type RWKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPRCVD` reader - MGKPRCVD"]
pub type MGKPRCVD_R = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - RWKPRCVD"]
pub type RWKPRCVD_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - GLBLUCAST"]
pub type GLBLUCAST_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` writer - GLBLUCAST"]
pub type GLBLUCAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPFE` reader - RWKPFE"]
pub type RWKPFE_R = crate::BitReader;
#[doc = "Field `RWKPFE` writer - RWKPFE"]
pub type RWKPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPTR` reader - RWKPTR"]
pub type RWKPTR_R = crate::FieldReader;
#[doc = "Field `RWKFILTRST` reader - RWKFILTRST"]
pub type RWKFILTRST_R = crate::BitReader;
#[doc = "Field `RWKFILTRST` writer - RWKFILTRST"]
pub type RWKFILTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - MGKPRCVD"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWKPRCVD"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:28 - RWKPTR"]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<ETH_MACPCSRrs> {
        PWRDWN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<ETH_MACPCSRrs> {
        MGKPKTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<ETH_MACPCSRrs> {
        RWKPKTEN_W::new(self, 2)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<ETH_MACPCSRrs> {
        GLBLUCAST_W::new(self, 9)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<ETH_MACPCSRrs> {
        RWKPFE_W::new(self, 10)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<ETH_MACPCSRrs> {
        RWKFILTRST_W::new(self, 31)
    }
}
#[doc = "The PMT Control and Status Register is present only when you select the PMT module in coreConsultant.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macpcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macpcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPCSRrs;
impl crate::RegisterSpec for ETH_MACPCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macpcsr::R`](R) reader structure"]
impl crate::Readable for ETH_MACPCSRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macpcsr::W`](W) writer structure"]
impl crate::Writable for ETH_MACPCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPCSR to value 0"]
impl crate::Resettable for ETH_MACPCSRrs {
    const RESET_VALUE: u32 = 0;
}
