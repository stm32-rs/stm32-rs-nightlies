///Register `MACPPSCR` reader
pub type R = crate::R<MACPPSCRrs>;
///Register `MACPPSCR` writer
pub type W = crate::W<MACPPSCRrs>;
///Field `PPSCTRL` reader - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register.
pub type PPSCTRL_R = crate::FieldReader;
///Field `PPSCTRL` writer - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register.
pub type PPSCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPSEN0` reader - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, PPSCTRL\[3:0\] function as PPSCTRL (Fixed PPS mode).
pub type PPSEN0_R = crate::BitReader;
///Field `PPSEN0` writer - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, PPSCTRL\[3:0\] function as PPSCTRL (Fixed PPS mode).
pub type PPSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:
pub type TRGTMODSEL0_R = crate::FieldReader;
///Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:
pub type TRGTMODSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register.
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, PPSCTRL\[3:0\] function as PPSCTRL (Fixed PPS mode).
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSCR")
            .field("ppsctrl", &self.ppsctrl())
            .field("ppsen0", &self.ppsen0())
            .field("trgtmodsel0", &self.trgtmodsel0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register.
    #[inline(always)]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<'_, MACPPSCRrs> {
        PPSCTRL_W::new(self, 0)
    }
    ///Bit 4 - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\[3:0\] function as PPSCMD\[3:0\]. When this bit is reset, PPSCTRL\[3:0\] function as PPSCTRL (Fixed PPS mode).
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W<'_, MACPPSCRrs> {
        PPSEN0_W::new(self, 4)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<'_, MACPPSCRrs> {
        TRGTMODSEL0_W::new(self, 5)
    }
}
/**PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:MACPPSCR)*/
pub struct MACPPSCRrs;
impl crate::RegisterSpec for MACPPSCRrs {
    type Ux = u32;
}
///`read()` method returns [`macppscr::R`](R) reader structure
impl crate::Readable for MACPPSCRrs {}
///`write(|w| ..)` method takes [`macppscr::W`](W) writer structure
impl crate::Writable for MACPPSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSCR to value 0
impl crate::Resettable for MACPPSCRrs {}
