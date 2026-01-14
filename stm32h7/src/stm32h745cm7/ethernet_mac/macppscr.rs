///Register `MACPPSCR` reader
pub type R = crate::R<MACPPSCRrs>;
///Register `MACPPSCR` writer
pub type W = crate::W<MACPPSCRrs>;
///Field `PPSCTRL` reader - Flexible PPS Output (ptp_pps_o\[0\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared
pub type PPSCTRL_R = crate::FieldReader;
///Field `PPSCTRL` writer - Flexible PPS Output (ptp_pps_o\[0\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared
pub type PPSCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPSEN0` reader - Flexible PPS Output Mode Enable
pub type PPSEN0_R = crate::BitReader;
///Field `PPSEN0` writer - Flexible PPS Output Mode Enable
pub type PPSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS Output
pub type TRGTMODSEL0_R = crate::FieldReader;
///Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS Output
pub type TRGTMODSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Flexible PPS Output (ptp_pps_o\[0\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Flexible PPS Output Mode Enable
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output
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
    ///Bits 0:3 - Flexible PPS Output (ptp_pps_o\[0\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared
    #[inline(always)]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<'_, MACPPSCRrs> {
        PPSCTRL_W::new(self, 0)
    }
    ///Bit 4 - Flexible PPS Output Mode Enable
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W<'_, MACPPSCRrs> {
        PPSEN0_W::new(self, 4)
    }
    ///Bits 5:6 - Target Time Register Mode for PPS Output
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<'_, MACPPSCRrs> {
        TRGTMODSEL0_W::new(self, 5)
    }
}
/**PPS control register

You can [`read`](crate::Reg::read) this register and get [`macppscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#Ethernet_MAC:MACPPSCR)*/
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
