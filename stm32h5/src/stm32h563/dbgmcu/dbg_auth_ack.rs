///Register `DBG_AUTH_ACK` reader
pub type R = crate::R<DBG_AUTH_ACKrs>;
///Register `DBG_AUTH_ACK` writer
pub type W = crate::W<DBG_AUTH_ACKrs>;
///Field `HOST_ACK` reader - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message
pub type HOST_ACK_R = crate::BitReader;
///Field `HOST_ACK` writer - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message
pub type HOST_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEV_ACK` reader - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message
pub type DEV_ACK_R = crate::BitReader;
///Field `DEV_ACK` writer - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message
pub type DEV_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message
    #[inline(always)]
    pub fn host_ack(&self) -> HOST_ACK_R {
        HOST_ACK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message
    #[inline(always)]
    pub fn dev_ack(&self) -> DEV_ACK_R {
        DEV_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_AUTH_ACK")
            .field("host_ack", &self.host_ack())
            .field("dev_ack", &self.dev_ack())
            .finish()
    }
}
impl W {
    ///Bit 0 - Host to device acknowledge. The device sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_DEVICE register. It should be reset by the host after reading the message
    #[inline(always)]
    pub fn host_ack(&mut self) -> HOST_ACK_W<'_, DBG_AUTH_ACKrs> {
        HOST_ACK_W::new(self, 0)
    }
    ///Bit 1 - Device to device acknowledge. The host sets this bit to indicate that it has placed a message in the DBGMCU_DBG_AUTH_HOST register. It is reset by the device after reading the message
    #[inline(always)]
    pub fn dev_ack(&mut self) -> DEV_ACK_W<'_, DBG_AUTH_ACKrs> {
        DEV_ACK_W::new(self, 1)
    }
}
/**DBGMCU debug authentication mailbox acknowledge register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_auth_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#DBGMCU:DBG_AUTH_ACK)*/
pub struct DBG_AUTH_ACKrs;
impl crate::RegisterSpec for DBG_AUTH_ACKrs {
    type Ux = u32;
}
///`read()` method returns [`dbg_auth_ack::R`](R) reader structure
impl crate::Readable for DBG_AUTH_ACKrs {}
///`write(|w| ..)` method takes [`dbg_auth_ack::W`](W) writer structure
impl crate::Writable for DBG_AUTH_ACKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBG_AUTH_ACK to value 0
impl crate::Resettable for DBG_AUTH_ACKrs {}
