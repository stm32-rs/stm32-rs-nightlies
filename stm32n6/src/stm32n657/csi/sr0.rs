///Register `SR0` reader
pub type R = crate::R<SR0rs>;
///Field `LB0F` reader - Line/byte counter 0 flag
pub type LB0F_R = crate::BitReader;
///Field `LB1F` reader - Line/byte counter 1 flag
pub type LB1F_R = crate::BitReader;
///Field `LB2F` reader - Line/byte counter 2 flag
pub type LB2F_R = crate::BitReader;
///Field `LB3F` reader - Line/byte counter 3 flag
pub type LB3F_R = crate::BitReader;
///Field `TIM0F` reader - Timer 0 flag
pub type TIM0F_R = crate::BitReader;
///Field `TIM1F` reader - Timer 1 flag
pub type TIM1F_R = crate::BitReader;
///Field `TIM2F` reader - Timer 2 flag
pub type TIM2F_R = crate::BitReader;
///Field `TIM3F` reader - Timer 3 flag
pub type TIM3F_R = crate::BitReader;
///Field `SOF0F` reader - SOF flag for virtual channel 0
pub type SOF0F_R = crate::BitReader;
///Field `SOF1F` reader - SOF flag for virtual channel 1
pub type SOF1F_R = crate::BitReader;
///Field `SOF2F` reader - SOF flag for virtual channel 2
pub type SOF2F_R = crate::BitReader;
///Field `SOF3F` reader - SOF flag for virtual channel 3
pub type SOF3F_R = crate::BitReader;
///Field `EOF0F` reader - EOF flag for virtual channel 0
pub type EOF0F_R = crate::BitReader;
///Field `EOF1F` reader - EOF flag for virtual channel 1
pub type EOF1F_R = crate::BitReader;
///Field `EOF2F` reader - EOF flag for virtual channel 2
pub type EOF2F_R = crate::BitReader;
///Field `EOF3F` reader - EOF flag for virtual channel 3
pub type EOF3F_R = crate::BitReader;
///Field `SPKTF` reader - Short packet flag
pub type SPKTF_R = crate::BitReader;
///Field `VC0STATEF` reader - Virtual channel 0 state flag
pub type VC0STATEF_R = crate::BitReader;
///Field `VC1STATEF` reader - Virtual channel 1 state flag
pub type VC1STATEF_R = crate::BitReader;
///Field `VC2STATEF` reader - Virtual channel 2 state flag
pub type VC2STATEF_R = crate::BitReader;
///Field `VC3STATEF` reader - Virtual channel 3 state flag
pub type VC3STATEF_R = crate::BitReader;
///Field `CCFIFOFF` reader - Clock changer FIFO full flag
pub type CCFIFOFF_R = crate::BitReader;
///Field `CRCERRF` reader - CRC error flag
pub type CRCERRF_R = crate::BitReader;
///Field `ECCERRF` reader - ECC error flag
pub type ECCERRF_R = crate::BitReader;
///Field `CECCERRF` reader - Corrected ECC error flag
pub type CECCERRF_R = crate::BitReader;
///Field `IDERRF` reader - Data type ID error flag
pub type IDERRF_R = crate::BitReader;
///Field `SPKTERRF` reader - Short packet error flag
pub type SPKTERRF_R = crate::BitReader;
///Field `WDERRF` reader - Watchdog error flag
pub type WDERRF_R = crate::BitReader;
///Field `SYNCERRF` reader - Invalid synchronization error flag
pub type SYNCERRF_R = crate::BitReader;
impl R {
    ///Bit 0 - Line/byte counter 0 flag
    #[inline(always)]
    pub fn lb0f(&self) -> LB0F_R {
        LB0F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Line/byte counter 1 flag
    #[inline(always)]
    pub fn lb1f(&self) -> LB1F_R {
        LB1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Line/byte counter 2 flag
    #[inline(always)]
    pub fn lb2f(&self) -> LB2F_R {
        LB2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Line/byte counter 3 flag
    #[inline(always)]
    pub fn lb3f(&self) -> LB3F_R {
        LB3F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer 0 flag
    #[inline(always)]
    pub fn tim0f(&self) -> TIM0F_R {
        TIM0F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 1 flag
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer 2 flag
    #[inline(always)]
    pub fn tim2f(&self) -> TIM2F_R {
        TIM2F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer 3 flag
    #[inline(always)]
    pub fn tim3f(&self) -> TIM3F_R {
        TIM3F_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SOF flag for virtual channel 0
    #[inline(always)]
    pub fn sof0f(&self) -> SOF0F_R {
        SOF0F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SOF flag for virtual channel 1
    #[inline(always)]
    pub fn sof1f(&self) -> SOF1F_R {
        SOF1F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SOF flag for virtual channel 2
    #[inline(always)]
    pub fn sof2f(&self) -> SOF2F_R {
        SOF2F_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SOF flag for virtual channel 3
    #[inline(always)]
    pub fn sof3f(&self) -> SOF3F_R {
        SOF3F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EOF flag for virtual channel 0
    #[inline(always)]
    pub fn eof0f(&self) -> EOF0F_R {
        EOF0F_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EOF flag for virtual channel 1
    #[inline(always)]
    pub fn eof1f(&self) -> EOF1F_R {
        EOF1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - EOF flag for virtual channel 2
    #[inline(always)]
    pub fn eof2f(&self) -> EOF2F_R {
        EOF2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EOF flag for virtual channel 3
    #[inline(always)]
    pub fn eof3f(&self) -> EOF3F_R {
        EOF3F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Short packet flag
    #[inline(always)]
    pub fn spktf(&self) -> SPKTF_R {
        SPKTF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Virtual channel 0 state flag
    #[inline(always)]
    pub fn vc0statef(&self) -> VC0STATEF_R {
        VC0STATEF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Virtual channel 1 state flag
    #[inline(always)]
    pub fn vc1statef(&self) -> VC1STATEF_R {
        VC1STATEF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Virtual channel 2 state flag
    #[inline(always)]
    pub fn vc2statef(&self) -> VC2STATEF_R {
        VC2STATEF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Virtual channel 3 state flag
    #[inline(always)]
    pub fn vc3statef(&self) -> VC3STATEF_R {
        VC3STATEF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Clock changer FIFO full flag
    #[inline(always)]
    pub fn ccfifoff(&self) -> CCFIFOFF_R {
        CCFIFOFF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - CRC error flag
    #[inline(always)]
    pub fn crcerrf(&self) -> CRCERRF_R {
        CRCERRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ECC error flag
    #[inline(always)]
    pub fn eccerrf(&self) -> ECCERRF_R {
        ECCERRF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Corrected ECC error flag
    #[inline(always)]
    pub fn ceccerrf(&self) -> CECCERRF_R {
        CECCERRF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Data type ID error flag
    #[inline(always)]
    pub fn iderrf(&self) -> IDERRF_R {
        IDERRF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Short packet error flag
    #[inline(always)]
    pub fn spkterrf(&self) -> SPKTERRF_R {
        SPKTERRF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Watchdog error flag
    #[inline(always)]
    pub fn wderrf(&self) -> WDERRF_R {
        WDERRF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Invalid synchronization error flag
    #[inline(always)]
    pub fn syncerrf(&self) -> SYNCERRF_R {
        SYNCERRF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR0")
            .field("lb0f", &self.lb0f())
            .field("lb1f", &self.lb1f())
            .field("lb2f", &self.lb2f())
            .field("lb3f", &self.lb3f())
            .field("tim0f", &self.tim0f())
            .field("tim1f", &self.tim1f())
            .field("tim2f", &self.tim2f())
            .field("tim3f", &self.tim3f())
            .field("sof0f", &self.sof0f())
            .field("sof1f", &self.sof1f())
            .field("sof2f", &self.sof2f())
            .field("sof3f", &self.sof3f())
            .field("eof0f", &self.eof0f())
            .field("eof1f", &self.eof1f())
            .field("eof2f", &self.eof2f())
            .field("eof3f", &self.eof3f())
            .field("spktf", &self.spktf())
            .field("vc0statef", &self.vc0statef())
            .field("vc1statef", &self.vc1statef())
            .field("vc2statef", &self.vc2statef())
            .field("vc3statef", &self.vc3statef())
            .field("ccfifoff", &self.ccfifoff())
            .field("crcerrf", &self.crcerrf())
            .field("eccerrf", &self.eccerrf())
            .field("ceccerrf", &self.ceccerrf())
            .field("iderrf", &self.iderrf())
            .field("spkterrf", &self.spkterrf())
            .field("wderrf", &self.wderrf())
            .field("syncerrf", &self.syncerrf())
            .finish()
    }
}
/**CSI-2 Host status register 0

You can [`read`](crate::Reg::read) this register and get [`sr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:SR0)*/
pub struct SR0rs;
impl crate::RegisterSpec for SR0rs {
    type Ux = u32;
}
///`read()` method returns [`sr0::R`](R) reader structure
impl crate::Readable for SR0rs {}
///`reset()` method sets SR0 to value 0
impl crate::Resettable for SR0rs {}
