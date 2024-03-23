#[doc = "Register `MACPCSR` reader"]
pub type R = crate::R<MACPCSRrs>;
#[doc = "Register `MACPCSR` writer"]
pub type W = crate::W<MACPCSRrs>;
#[doc = "Field `PWRDWN` reader - Power Down"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power Down"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable"]
pub type MGKPKTEN_R = crate::BitReader;
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable"]
pub type MGKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPKTEN` reader - Remote wakeup Packet Enable"]
pub type RWKPKTEN_R = crate::BitReader;
#[doc = "Field `RWKPKTEN` writer - Remote wakeup Packet Enable"]
pub type RWKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received"]
pub type MGKPRCVD_R = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - Remote wakeup Packet Received"]
pub type RWKPRCVD_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - Global Unicast"]
pub type GLBLUCAST_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` writer - Global Unicast"]
pub type GLBLUCAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPFE` reader - Remote wakeup Packet Forwarding Enable"]
pub type RWKPFE_R = crate::BitReader;
#[doc = "Field `RWKPFE` writer - Remote wakeup Packet Forwarding Enable"]
pub type RWKPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPTR` reader - Remote wakeup FIFO Pointer"]
pub type RWKPTR_R = crate::FieldReader;
#[doc = "Field `RWKPTR` writer - Remote wakeup FIFO Pointer"]
pub type RWKPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RWKFILTRST` reader - Remote wakeup Packet Filter Register Pointer Reset"]
pub type RWKFILTRST_R = crate::BitReader;
#[doc = "Field `RWKFILTRST` writer - Remote wakeup Packet Filter Register Pointer Reset"]
pub type RWKFILTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remote wakeup Packet Enable"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remote wakeup Packet Received"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remote wakeup Packet Forwarding Enable"]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Remote wakeup FIFO Pointer"]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Remote wakeup Packet Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<MACPCSRrs> {
        PWRDWN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<MACPCSRrs> {
        MGKPKTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Remote wakeup Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<MACPCSRrs> {
        RWKPKTEN_W::new(self, 2)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<MACPCSRrs> {
        GLBLUCAST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Remote wakeup Packet Forwarding Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<MACPCSRrs> {
        RWKPFE_W::new(self, 10)
    }
    #[doc = "Bits 24:28 - Remote wakeup FIFO Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rwkptr(&mut self) -> RWKPTR_W<MACPCSRrs> {
        RWKPTR_W::new(self, 24)
    }
    #[doc = "Bit 31 - Remote wakeup Packet Filter Register Pointer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<MACPCSRrs> {
        RWKFILTRST_W::new(self, 31)
    }
}
#[doc = "PMT control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPCSRrs;
impl crate::RegisterSpec for MACPCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpcsr::R`](R) reader structure"]
impl crate::Readable for MACPCSRrs {}
#[doc = "`write(|w| ..)` method takes [`macpcsr::W`](W) writer structure"]
impl crate::Writable for MACPCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPCSR to value 0"]
impl crate::Resettable for MACPCSRrs {
    const RESET_VALUE: u32 = 0;
}
