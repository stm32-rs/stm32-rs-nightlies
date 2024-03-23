#[doc = "Register `DBG_AUTH_ACK` reader"]
pub type R = crate::R<DBG_AUTH_ACKrs>;
#[doc = "Register `DBG_AUTH_ACK` writer"]
pub type W = crate::W<DBG_AUTH_ACKrs>;
#[doc = "Field `HOST_ACK` reader - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
pub type HOST_ACK_R = crate::BitReader;
#[doc = "Field `HOST_ACK` writer - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
pub type HOST_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_ACK` reader - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
pub type DEV_ACK_R = crate::BitReader;
#[doc = "Field `DEV_ACK` writer - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
pub type DEV_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
    #[inline(always)]
    pub fn host_ack(&self) -> HOST_ACK_R {
        HOST_ACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
    #[inline(always)]
    pub fn dev_ack(&self) -> DEV_ACK_R {
        DEV_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message"]
    #[inline(always)]
    #[must_use]
    pub fn host_ack(&mut self) -> HOST_ACK_W<DBG_AUTH_ACKrs> {
        HOST_ACK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message"]
    #[inline(always)]
    #[must_use]
    pub fn dev_ack(&mut self) -> DEV_ACK_W<DBG_AUTH_ACKrs> {
        DEV_ACK_W::new(self, 1)
    }
}
#[doc = "DBGMCU debug authentication mailbox acknowledge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_auth_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_auth_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_AUTH_ACKrs;
impl crate::RegisterSpec for DBG_AUTH_ACKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_auth_ack::R`](R) reader structure"]
impl crate::Readable for DBG_AUTH_ACKrs {}
#[doc = "`write(|w| ..)` method takes [`dbg_auth_ack::W`](W) writer structure"]
impl crate::Writable for DBG_AUTH_ACKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_AUTH_ACK to value 0"]
impl crate::Resettable for DBG_AUTH_ACKrs {
    const RESET_VALUE: u32 = 0;
}
