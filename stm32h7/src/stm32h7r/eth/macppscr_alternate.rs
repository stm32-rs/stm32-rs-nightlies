///Register `MACPPSCR_alternate` reader
pub type R = crate::R<MACPPSCR_ALTERNATErs>;
///Register `MACPPSCR_alternate` writer
pub type W = crate::W<MACPPSCR_ALTERNATErs>;
///Field `PPSCMD` reader - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are all zero. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used
pub type PPSCMD_R = crate::FieldReader;
///Field `PPSCMD` writer - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are all zero. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used
pub type PPSCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPSEN0` reader - Flexible PPS Output Mode Enable When this bit is set, Bits\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, Bits\[3:0\] function as PPSCTRL (Fixed PPS mode).
pub type PPSEN0_R = crate::BitReader;
///Field `PPSEN0` writer - Flexible PPS Output Mode Enable When this bit is set, Bits\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, Bits\[3:0\] function as PPSCTRL (Fixed PPS mode).
pub type PPSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS Output This field indicates the Target Time registers (MAC registers 96 and 97) mode for PPS output signal:
pub type TRGTMODSEL0_R = crate::FieldReader;
///Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS Output This field indicates the Target Time registers (MAC registers 96 and 97) mode for PPS output signal:
pub type TRGTMODSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are all zero. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used
    #[inline(always)]
    pub fn ppscmd(&self) -> PPSCMD_R {
        PPSCMD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Flexible PPS Output Mode Enable When this bit is set, Bits\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, Bits\[3:0\] function as PPSCTRL (Fixed PPS mode).
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output This field indicates the Target Time registers (MAC registers 96 and 97) mode for PPS output signal:
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSCR_alternate")
            .field("ppscmd", &self.ppscmd())
            .field("ppsen0", &self.ppsen0())
            .field("trgtmodsel0", &self.trgtmodsel0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are all zero. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\[3:0\] = 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used
    #[inline(always)]
    pub fn ppscmd(&mut self) -> PPSCMD_W<'_, MACPPSCR_ALTERNATErs> {
        PPSCMD_W::new(self, 0)
    }
    ///Bit 4 - Flexible PPS Output Mode Enable When this bit is set, Bits\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, Bits\[3:0\] function as PPSCTRL (Fixed PPS mode).
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W<'_, MACPPSCR_ALTERNATErs> {
        PPSEN0_W::new(self, 4)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output This field indicates the Target Time registers (MAC registers 96 and 97) mode for PPS output signal:
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<'_, MACPPSCR_ALTERNATErs> {
        TRGTMODSEL0_W::new(self, 5)
    }
}
/**PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:MACPPSCR_alternate)*/
pub struct MACPPSCR_ALTERNATErs;
impl crate::RegisterSpec for MACPPSCR_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`macppscr_alternate::R`](R) reader structure
impl crate::Readable for MACPPSCR_ALTERNATErs {}
///`write(|w| ..)` method takes [`macppscr_alternate::W`](W) writer structure
impl crate::Writable for MACPPSCR_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSCR_alternate to value 0
impl crate::Resettable for MACPPSCR_ALTERNATErs {}
