///Register `MACPCSR` reader
pub type R = crate::R<MACPCSRrs>;
///Register `MACPCSR` writer
pub type W = crate::W<MACPCSRrs>;
///Field `PWRDWN` reader - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit.
pub type PWRDWN_R = crate::BitReader;
///Field `PWRDWN` writer - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit.
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MGKPKTEN` reader - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet.
pub type MGKPKTEN_R = crate::BitReader;
///Field `MGKPKTEN` writer - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet.
pub type MGKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPKTEN` reader - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet.
pub type RWKPKTEN_R = crate::BitReader;
///Field `RWKPKTEN` writer - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet.
pub type RWKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `MGKPRCVD` reader - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type MGKPRCVD_R = crate::BitReader;
///Field `MGKPRCVD` writer - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
pub type MGKPRCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPRCVD` reader - Remote wakeup Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a remote wakeup packet. This bit is cleared when this register is read.
pub type RWKPRCVD_R = crate::BitReader;
///Field `GLBLUCAST` reader - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet.
pub type GLBLUCAST_R = crate::BitReader;
///Field `GLBLUCAST` writer - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet.
pub type GLBLUCAST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPFE` reader - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application.
pub type RWKPFE_R = crate::BitReader;
///Field `RWKPFE` writer - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application.
pub type RWKPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWKPTR` reader - Remote wakeup FIFO Pointer This field gives the current value (0 to 7) of the Remote wakeup Packet Filter register pointer. When the value of this pointer is equal to 7, the contents of the Remote wakeup Packet Filter Register are transferred to the eth_mii_rx_clk domain when a Write occurs to that register.
pub type RWKPTR_R = crate::FieldReader;
///Field `RWKFILTRST` reader - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle.
pub type RWKFILTRST_R = crate::BitReader;
///Field `RWKFILTRST` writer - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle.
pub type RWKFILTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit.
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet.
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet.
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Remote wakeup Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a remote wakeup packet. This bit is cleared when this register is read.
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet.
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application.
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 24:28 - Remote wakeup FIFO Pointer This field gives the current value (0 to 7) of the Remote wakeup Packet Filter register pointer. When the value of this pointer is equal to 7, the contents of the Remote wakeup Packet Filter Register are transferred to the eth_mii_rx_clk domain when a Write occurs to that register.
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 31 - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle.
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPCSR")
            .field("pwrdwn", &self.pwrdwn())
            .field("mgkpkten", &self.mgkpkten())
            .field("rwkpkten", &self.rwkpkten())
            .field("rwkprcvd", &self.rwkprcvd())
            .field("glblucast", &self.glblucast())
            .field("rwkpfe", &self.rwkpfe())
            .field("rwkptr", &self.rwkptr())
            .field("rwkfiltrst", &self.rwkfiltrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power Down When this bit is set, the MAC receiver drops all received packets until it receives the expected magic packet or remote wakeup packet. This bit is then self-cleared and the power-down mode is disabled. The software can clear this bit before the expected magic packet or remote wakeup packet is received. The packets received by the MAC after this bit is cleared are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote wakeup Packet Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit.
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<'_, MACPCSRrs> {
        PWRDWN_W::new(self, 0)
    }
    ///Bit 1 - Magic Packet Enable When this bit is set, a power management event is generated when the MAC receives a magic packet.
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<'_, MACPCSRrs> {
        MGKPKTEN_W::new(self, 1)
    }
    ///Bit 2 - Remote wakeup Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wakeup packet.
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<'_, MACPCSRrs> {
        RWKPKTEN_W::new(self, 2)
    }
    ///Bit 5 - Magic Packet Received When this bit is set, it indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared when this register is read (or written to 1 when RCWE bit in CSR software control register (ETH_MACCSRSWCR) is set).
    #[inline(always)]
    pub fn mgkprcvd(&mut self) -> MGKPRCVD_W<'_, MACPCSRrs> {
        MGKPRCVD_W::new(self, 5)
    }
    ///Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wakeup packet.
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<'_, MACPCSRrs> {
        GLBLUCAST_W::new(self, 9)
    }
    ///Bit 10 - Remote wakeup Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wakeup frame. All frames after that event including the received wakeup frame are forwarded to application. This bit is then self-cleared on receiving the wakeup packet. The application can also clear this bit before the expected wakeup frame is received. In such cases, the MAC reverts to the default behavior where packets received are forwarded to the application. This bit must only be set when RWKPKTEN is set high and PWRDWN is set low. The setting of this bit has no effect when PWRDWN is set high. Note: If Magic Packet Enable and wakeup Frame Enable are both set along with setting of this bit and Magic Packet is received prior to wakeup frame, this bit is self-cleared on receiving Magic Packet, the received Magic packet is dropped, and all frames after received Magic Packet are forwarded to application.
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<'_, MACPCSRrs> {
        RWKPFE_W::new(self, 10)
    }
    ///Bit 31 - Remote wakeup Packet Filter Register Pointer Reset When this bit is set, the remote wakeup packet filter register pointer is reset to 0. It is automatically cleared after 1 clock cycle.
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<'_, MACPCSRrs> {
        RWKFILTRST_W::new(self, 31)
    }
}
/**PMT control status register

You can [`read`](crate::Reg::read) this register and get [`macpcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MACPCSR)*/
pub struct MACPCSRrs;
impl crate::RegisterSpec for MACPCSRrs {
    type Ux = u32;
}
///`read()` method returns [`macpcsr::R`](R) reader structure
impl crate::Readable for MACPCSRrs {}
///`write(|w| ..)` method takes [`macpcsr::W`](W) writer structure
impl crate::Writable for MACPCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPCSR to value 0
impl crate::Resettable for MACPCSRrs {}
