///Register `DBG_AUTH_ACK` reader
pub type R = crate::R<DBG_AUTH_ACKrs>;
///Field `HOST_ACK` reader - Access status to DBG_AUTH_HOST register
pub type HOST_ACK_R = crate::BitReader;
///Field `DEVICE_ACK` reader - Access status to DBG_AUTH_DEV register
pub type DEVICE_ACK_R = crate::BitReader;
impl R {
    ///Bit 0 - Access status to DBG_AUTH_HOST register
    #[inline(always)]
    pub fn host_ack(&self) -> HOST_ACK_R {
        HOST_ACK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Access status to DBG_AUTH_DEV register
    #[inline(always)]
    pub fn device_ack(&self) -> DEVICE_ACK_R {
        DEVICE_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_AUTH_ACK")
            .field("host_ack", &self.host_ack())
            .field("device_ack", &self.device_ack())
            .finish()
    }
}
/**DBGMCU message read acknowledge authentication register

You can [`read`](crate::Reg::read) this register and get [`dbg_auth_ack::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DBGMCU:DBG_AUTH_ACK)*/
pub struct DBG_AUTH_ACKrs;
impl crate::RegisterSpec for DBG_AUTH_ACKrs {
    type Ux = u32;
}
///`read()` method returns [`dbg_auth_ack::R`](R) reader structure
impl crate::Readable for DBG_AUTH_ACKrs {}
///`reset()` method sets DBG_AUTH_ACK to value 0
impl crate::Resettable for DBG_AUTH_ACKrs {}
