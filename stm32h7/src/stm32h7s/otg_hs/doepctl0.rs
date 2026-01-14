///Register `DOEPCTL0` reader
pub type R = crate::R<DOEPCTL0rs>;
///Register `DOEPCTL0` writer
pub type W = crate::W<DOEPCTL0rs>;
///Field `MPSIZ` reader - Maximum packet size The maximum packet size for control OUT endpoint 0 is the same as what is programmed in control IN endpoint 0.
pub type MPSIZ_R = crate::FieldReader;
///Field `USBAEP` reader - USB active endpoint This bit is always set to 1, indicating that a control endpoint 0 is always active in all configurations and interfaces.
pub type USBAEP_R = crate::BitReader;
///Field `NAKSTS` reader - NAK status Indicates the following: When either the application or the core sets this bit, the core stops receiving data, even if there is space in the Rx FIFO to accommodate the incoming packet. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - Endpoint type Hardcoded to 00 for control.
pub type EPTYP_R = crate::FieldReader;
///Field `SNPM` reader - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
pub type SNPM_R = crate::BitReader;
///Field `SNPM` writer - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
pub type SNPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALL` reader - STALL handshake The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer - Clear NAK A write to this bit clears the NAK bit for the endpoint.
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - Set NAK A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit on a transfer completed interrupt, or after a SETUP is received on the endpoint.
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - Endpoint disable The application cannot disable control OUT endpoint 0.
pub type EPDIS_R = crate::BitReader;
///Field `EPENA` writer - Endpoint enable The application sets this bit to start transmitting data on endpoint 0. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Maximum packet size The maximum packet size for control OUT endpoint 0 is the same as what is programmed in control IN endpoint 0.
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    ///Bit 15 - USB active endpoint This bit is always set to 1, indicating that a control endpoint 0 is always active in all configurations and interfaces.
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - NAK status Indicates the following: When either the application or the core sets this bit, the core stops receiving data, even if there is space in the Rx FIFO to accommodate the incoming packet. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type Hardcoded to 00 for control.
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - STALL handshake The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - Endpoint disable The application cannot disable control OUT endpoint 0.
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL0")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("snpm", &self.snpm())
            .field("stall", &self.stall())
            .field("epdis", &self.epdis())
            .finish()
    }
}
impl W {
    ///Bit 20 - Snoop mode This bit configures the endpoint to Snoop mode. In Snoop mode, the core does not check the correctness of OUT packets before transferring them to application memory.
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<'_, DOEPCTL0rs> {
        SNPM_W::new(self, 20)
    }
    ///Bit 21 - STALL handshake The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bits setting, the core always responds to SETUP data packets with an ACK handshake.
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, DOEPCTL0rs> {
        STALL_W::new(self, 21)
    }
    ///Bit 26 - Clear NAK A write to this bit clears the NAK bit for the endpoint.
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, DOEPCTL0rs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - Set NAK A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit on a transfer completed interrupt, or after a SETUP is received on the endpoint.
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, DOEPCTL0rs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 31 - Endpoint enable The application sets this bit to start transmitting data on endpoint 0. The core clears this bit before setting any of the following interrupts on this endpoint: SETUP phase done Endpoint disabled Transfer completed
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, DOEPCTL0rs> {
        EPENA_W::new(self, 31)
    }
}
/**OTG device control OUT endpoint 0 control register

You can [`read`](crate::Reg::read) this register and get [`doepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:DOEPCTL0)*/
pub struct DOEPCTL0rs;
impl crate::RegisterSpec for DOEPCTL0rs {
    type Ux = u32;
}
///`read()` method returns [`doepctl0::R`](R) reader structure
impl crate::Readable for DOEPCTL0rs {}
///`write(|w| ..)` method takes [`doepctl0::W`](W) writer structure
impl crate::Writable for DOEPCTL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPCTL0 to value 0x8000
impl crate::Resettable for DOEPCTL0rs {
    const RESET_VALUE: u32 = 0x8000;
}
