///Register `GRSTCTL` reader
pub type R = crate::R<GRSTCTLrs>;
///Register `GRSTCTL` writer
pub type W = crate::W<GRSTCTLrs>;
///Field `CSRST` reader - Core soft reset Resets the HCLK and PHY clock domains as follows: Clears the interrupts and all the CSR register bits except for the following bits: GATEHCLK bit in OTG_PCGCCTL STPPCLK bit in OTG_PCGCCTL FSLSPCS bits in OTG_HCFG DSPD bit in OTG_DCFG SDIS bit in OTG_DCTL OTG_GCCFG register All module state machines (except for the AHB slave unit) are reset to the Idle state, and all the transmit FIFOs and the receive FIFO are flushed. Any transactions on the AHB Master are terminated as soon as possible, after completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. The application can write to this bit any time it wants to reset the core. This is a self-clearing bit and the core clears this bit after all the necessary logic is reset in the core, which can take several clocks, depending on the current state of the core. Once this bit has been cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). The software must also check that bit 31 in this register is set to 1 (AHB Master is Idle) before starting any operation. Typically, the software reset is used during software development and also when the user dynamically changes the PHY selection bits in the above listed USB configuration registers. When the user changes the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation. Note: Accessible in both device and host modes.
pub type CSRST_R = crate::BitReader;
///Field `CSRST` writer - Core soft reset Resets the HCLK and PHY clock domains as follows: Clears the interrupts and all the CSR register bits except for the following bits: GATEHCLK bit in OTG_PCGCCTL STPPCLK bit in OTG_PCGCCTL FSLSPCS bits in OTG_HCFG DSPD bit in OTG_DCFG SDIS bit in OTG_DCTL OTG_GCCFG register All module state machines (except for the AHB slave unit) are reset to the Idle state, and all the transmit FIFOs and the receive FIFO are flushed. Any transactions on the AHB Master are terminated as soon as possible, after completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. The application can write to this bit any time it wants to reset the core. This is a self-clearing bit and the core clears this bit after all the necessary logic is reset in the core, which can take several clocks, depending on the current state of the core. Once this bit has been cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). The software must also check that bit 31 in this register is set to 1 (AHB Master is Idle) before starting any operation. Typically, the software reset is used during software development and also when the user dynamically changes the PHY selection bits in the above listed USB configuration registers. When the user changes the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation. Note: Accessible in both device and host modes.
pub type CSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSRST` reader - Partial soft reset Resets the internal state machines but keeps the enumeration info. Could be used to recover some specific PHY errors. Note: Accessible in both device and host modes.
pub type PSRST_R = crate::BitReader;
///Field `PSRST` writer - Partial soft reset Resets the internal state machines but keeps the enumeration info. Could be used to recover some specific PHY errors. Note: Accessible in both device and host modes.
pub type PSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCRST` reader - Host frame counter reset The application writes this bit to reset the (micro-)frame number counter inside the core. When the (micro-)frame counter is reset, the subsequent SOF sent out by the core has a frame number of 0. When application writes '1' to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles. Note: Only accessible in host mode.
pub type FCRST_R = crate::BitReader;
///Field `FCRST` writer - Host frame counter reset The application writes this bit to reset the (micro-)frame number counter inside the core. When the (micro-)frame counter is reset, the subsequent SOF sent out by the core has a frame number of 0. When application writes '1' to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles. Note: Only accessible in host mode.
pub type FCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFFLSH` reader - Rx FIFO flush The application can flush the entire Rx FIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the core is neither reading from the Rx FIFO nor writing to the Rx FIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires 8 clocks (slowest of PHY or AHB clock) to clear. Note: Accessible in both device and host modes.
pub type RXFFLSH_R = crate::BitReader;
///Field `RXFFLSH` writer - Rx FIFO flush The application can flush the entire Rx FIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the core is neither reading from the Rx FIFO nor writing to the Rx FIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires 8 clocks (slowest of PHY or AHB clock) to clear. Note: Accessible in both device and host modes.
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFFLSH` reader - Tx FIFO flush
pub type TXFFLSH_R = crate::BitReader;
///Field `TXFFLSH` writer - Tx FIFO flush
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - Tx FIFO number This is the FIFO number that must be flushed using the Tx FIFO Flush bit. This field must not be changed until the core clears the Tx FIFO Flush bit. ... Note: Accessible in both device and host modes.
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - Tx FIFO number This is the FIFO number that must be flushed using the Tx FIFO Flush bit. This field must not be changed until the core clears the Tx FIFO Flush bit. ... Note: Accessible in both device and host modes.
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DMAREQ` reader - DMA request signal enabled This bit indicates that the DMA request is in progress. Used for debug.
pub type DMAREQ_R = crate::BitReader;
///Field `AHBIDL` reader - AHB master idle Indicates that the AHB master state machine is in the Idle condition. Note: Accessible in both device and host modes.
pub type AHBIDL_R = crate::BitReader;
impl R {
    ///Bit 0 - Core soft reset Resets the HCLK and PHY clock domains as follows: Clears the interrupts and all the CSR register bits except for the following bits: GATEHCLK bit in OTG_PCGCCTL STPPCLK bit in OTG_PCGCCTL FSLSPCS bits in OTG_HCFG DSPD bit in OTG_DCFG SDIS bit in OTG_DCTL OTG_GCCFG register All module state machines (except for the AHB slave unit) are reset to the Idle state, and all the transmit FIFOs and the receive FIFO are flushed. Any transactions on the AHB Master are terminated as soon as possible, after completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. The application can write to this bit any time it wants to reset the core. This is a self-clearing bit and the core clears this bit after all the necessary logic is reset in the core, which can take several clocks, depending on the current state of the core. Once this bit has been cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). The software must also check that bit 31 in this register is set to 1 (AHB Master is Idle) before starting any operation. Typically, the software reset is used during software development and also when the user dynamically changes the PHY selection bits in the above listed USB configuration registers. When the user changes the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Partial soft reset Resets the internal state machines but keeps the enumeration info. Could be used to recover some specific PHY errors. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn psrst(&self) -> PSRST_R {
        PSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Host frame counter reset The application writes this bit to reset the (micro-)frame number counter inside the core. When the (micro-)frame counter is reset, the subsequent SOF sent out by the core has a frame number of 0. When application writes '1' to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn fcrst(&self) -> FCRST_R {
        FCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO flush The application can flush the entire Rx FIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the core is neither reading from the Rx FIFO nor writing to the Rx FIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires 8 clocks (slowest of PHY or AHB clock) to clear. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Tx FIFO flush
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - Tx FIFO number This is the FIFO number that must be flushed using the Tx FIFO Flush bit. This field must not be changed until the core clears the Tx FIFO Flush bit. ... Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 30 - DMA request signal enabled This bit indicates that the DMA request is in progress. Used for debug.
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AHB master idle Indicates that the AHB master state machine is in the Idle condition. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csrst", &self.csrst())
            .field("psrst", &self.psrst())
            .field("fcrst", &self.fcrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("dmareq", &self.dmareq())
            .field("ahbidl", &self.ahbidl())
            .finish()
    }
}
impl W {
    ///Bit 0 - Core soft reset Resets the HCLK and PHY clock domains as follows: Clears the interrupts and all the CSR register bits except for the following bits: GATEHCLK bit in OTG_PCGCCTL STPPCLK bit in OTG_PCGCCTL FSLSPCS bits in OTG_HCFG DSPD bit in OTG_DCFG SDIS bit in OTG_DCTL OTG_GCCFG register All module state machines (except for the AHB slave unit) are reset to the Idle state, and all the transmit FIFOs and the receive FIFO are flushed. Any transactions on the AHB Master are terminated as soon as possible, after completing the last data phase of an AHB transfer. Any transactions on the USB are terminated immediately. The application can write to this bit any time it wants to reset the core. This is a self-clearing bit and the core clears this bit after all the necessary logic is reset in the core, which can take several clocks, depending on the current state of the core. Once this bit has been cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). The software must also check that bit 31 in this register is set to 1 (AHB Master is Idle) before starting any operation. Typically, the software reset is used during software development and also when the user dynamically changes the PHY selection bits in the above listed USB configuration registers. When the user changes the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain has to be reset for proper operation. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn csrst(&mut self) -> CSRST_W<'_, GRSTCTLrs> {
        CSRST_W::new(self, 0)
    }
    ///Bit 1 - Partial soft reset Resets the internal state machines but keeps the enumeration info. Could be used to recover some specific PHY errors. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn psrst(&mut self) -> PSRST_W<'_, GRSTCTLrs> {
        PSRST_W::new(self, 1)
    }
    ///Bit 2 - Host frame counter reset The application writes this bit to reset the (micro-)frame number counter inside the core. When the (micro-)frame counter is reset, the subsequent SOF sent out by the core has a frame number of 0. When application writes '1' to the bit, it might not be able to read back the value as it gets cleared by the core in a few clock cycles. Note: Only accessible in host mode.
    #[inline(always)]
    pub fn fcrst(&mut self) -> FCRST_W<'_, GRSTCTLrs> {
        FCRST_W::new(self, 2)
    }
    ///Bit 4 - Rx FIFO flush The application can flush the entire Rx FIFO using this bit, but must first ensure that the core is not in the middle of a transaction. The application must only write to this bit after checking that the core is neither reading from the Rx FIFO nor writing to the Rx FIFO. The application must wait until the bit is cleared before performing any other operations. This bit requires 8 clocks (slowest of PHY or AHB clock) to clear. Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<'_, GRSTCTLrs> {
        RXFFLSH_W::new(self, 4)
    }
    ///Bit 5 - Tx FIFO flush
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<'_, GRSTCTLrs> {
        TXFFLSH_W::new(self, 5)
    }
    ///Bits 6:10 - Tx FIFO number This is the FIFO number that must be flushed using the Tx FIFO Flush bit. This field must not be changed until the core clears the Tx FIFO Flush bit. ... Note: Accessible in both device and host modes.
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, GRSTCTLrs> {
        TXFNUM_W::new(self, 6)
    }
}
/**OTG reset register

You can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:GRSTCTL)*/
pub struct GRSTCTLrs;
impl crate::RegisterSpec for GRSTCTLrs {
    type Ux = u32;
}
///`read()` method returns [`grstctl::R`](R) reader structure
impl crate::Readable for GRSTCTLrs {}
///`write(|w| ..)` method takes [`grstctl::W`](W) writer structure
impl crate::Writable for GRSTCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GRSTCTL to value 0x8000_0000
impl crate::Resettable for GRSTCTLrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
