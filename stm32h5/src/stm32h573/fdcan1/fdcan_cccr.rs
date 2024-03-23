#[doc = "Register `FDCAN_CCCR` reader"]
pub type R = crate::R<FDCAN_CCCRrs>;
#[doc = "Register `FDCAN_CCCR` writer"]
pub type W = crate::W<FDCAN_CCCRrs>;
#[doc = "Field `INIT` reader - Initialization"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Configuration change enable"]
pub type CCE_R = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration change enable"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASM` reader - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type ASM_R = crate::BitReader;
#[doc = "Field `ASM` writer - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
pub type ASM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSA` reader - Clock stop acknowledge"]
pub type CSA_R = crate::BitReader;
#[doc = "Field `CSR` reader - Clock stop request"]
pub type CSR_R = crate::BitReader;
#[doc = "Field `CSR` writer - Clock stop request"]
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MON_R = crate::BitReader;
#[doc = "Field `MON` writer - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
pub type MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Disable automatic retransmission"]
pub type DAR_R = crate::BitReader;
#[doc = "Field `DAR` writer - Disable automatic retransmission"]
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - Test mode enable"]
pub type TEST_R = crate::BitReader;
#[doc = "Field `TEST` writer - Test mode enable"]
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOE` reader - FD operation enable"]
pub type FDOE_R = crate::BitReader;
#[doc = "Field `FDOE` writer - FD operation enable"]
pub type FDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSE` reader - FDCAN bit rate switching"]
pub type BRSE_R = crate::BitReader;
#[doc = "Field `BRSE` writer - FDCAN bit rate switching"]
pub type BRSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXHD` reader - Protocol exception handling disable"]
pub type PXHD_R = crate::BitReader;
#[doc = "Field `PXHD` writer - Protocol exception handling disable"]
pub type PXHD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - Edge filtering during bus integration"]
pub type EFBI_R = crate::BitReader;
#[doc = "Field `EFBI` writer - Edge filtering during bus integration"]
pub type EFBI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TXP_R = crate::BitReader;
#[doc = "Field `TXP` writer - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
pub type TXP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NISO_R = crate::BitReader;
#[doc = "Field `NISO` writer - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
pub type NISO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock stop acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<FDCAN_CCCRrs> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration change enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<FDCAN_CCCRrs> {
        CCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted operation Mode after it has received a valid frame. In the optional Restricted operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time."]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<FDCAN_CCCRrs> {
        ASM_W::new(self, 2)
    }
    #[doc = "Bit 4 - Clock stop request"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<FDCAN_CCCRrs> {
        CSR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time."]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<FDCAN_CCCRrs> {
        MON_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable automatic retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<FDCAN_CCCRrs> {
        DAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<FDCAN_CCCRrs> {
        TEST_W::new(self, 7)
    }
    #[doc = "Bit 8 - FD operation enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<FDCAN_CCCRrs> {
        FDOE_W::new(self, 8)
    }
    #[doc = "Bit 9 - FDCAN bit rate switching"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<FDCAN_CCCRrs> {
        BRSE_W::new(self, 9)
    }
    #[doc = "Bit 12 - Protocol exception handling disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<FDCAN_CCCRrs> {
        PXHD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<FDCAN_CCCRrs> {
        EFBI_W::new(self, 13)
    }
    #[doc = "Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame."]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<FDCAN_CCCRrs> {
        TXP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0."]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<FDCAN_CCCRrs> {
        NISO_W::new(self, 15)
    }
}
#[doc = "FDCAN CC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_CCCRrs;
impl crate::RegisterSpec for FDCAN_CCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_cccr::R`](R) reader structure"]
impl crate::Readable for FDCAN_CCCRrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_cccr::W`](W) writer structure"]
impl crate::Writable for FDCAN_CCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_CCCR to value 0x01"]
impl crate::Resettable for FDCAN_CCCRrs {
    const RESET_VALUE: u32 = 0x01;
}
