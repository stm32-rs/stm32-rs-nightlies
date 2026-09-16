///Register `PTPPPSCR` reader
pub type R = crate::R<PTPPPSCRrs>;
/**PPS frequency selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPSFREQ {
    ///0: 1 Hz with a pulse width of 125 ms for binary rollover and of 100 ms for digital rollover
    Hz1 = 0,
    ///1: 2 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz2 = 1,
    ///2: 4 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz4 = 2,
    ///3: 8 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz8 = 3,
    ///4: 16 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz16 = 4,
    ///5: 32 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz32 = 5,
    ///6: 64 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz64 = 6,
    ///7: 128 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz128 = 7,
    ///8: 256 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz256 = 8,
    ///9: 512 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz512 = 9,
    ///10: 1024 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz1024 = 10,
    ///11: 2048 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz2048 = 11,
    ///12: 4096 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz4096 = 12,
    ///13: 8192 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz8192 = 13,
    ///14: 16384 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz16384 = 14,
    ///15: 32768 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    Hz32768 = 15,
}
impl From<PPSFREQ> for u8 {
    #[inline(always)]
    fn from(variant: PPSFREQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPSFREQ {
    type Ux = u8;
}
impl crate::IsEnum for PPSFREQ {}
///Field `PPSFREQ` reader - PPS frequency selection
pub type PPSFREQ_R = crate::FieldReader<PPSFREQ>;
impl PPSFREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PPSFREQ {
        match self.bits {
            0 => PPSFREQ::Hz1,
            1 => PPSFREQ::Hz2,
            2 => PPSFREQ::Hz4,
            3 => PPSFREQ::Hz8,
            4 => PPSFREQ::Hz16,
            5 => PPSFREQ::Hz32,
            6 => PPSFREQ::Hz64,
            7 => PPSFREQ::Hz128,
            8 => PPSFREQ::Hz256,
            9 => PPSFREQ::Hz512,
            10 => PPSFREQ::Hz1024,
            11 => PPSFREQ::Hz2048,
            12 => PPSFREQ::Hz4096,
            13 => PPSFREQ::Hz8192,
            14 => PPSFREQ::Hz16384,
            15 => PPSFREQ::Hz32768,
            _ => unreachable!(),
        }
    }
    ///1 Hz with a pulse width of 125 ms for binary rollover and of 100 ms for digital rollover
    #[inline(always)]
    pub fn is_hz_1(&self) -> bool {
        *self == PPSFREQ::Hz1
    }
    ///2 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_2(&self) -> bool {
        *self == PPSFREQ::Hz2
    }
    ///4 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_4(&self) -> bool {
        *self == PPSFREQ::Hz4
    }
    ///8 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_8(&self) -> bool {
        *self == PPSFREQ::Hz8
    }
    ///16 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_16(&self) -> bool {
        *self == PPSFREQ::Hz16
    }
    ///32 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_32(&self) -> bool {
        *self == PPSFREQ::Hz32
    }
    ///64 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_64(&self) -> bool {
        *self == PPSFREQ::Hz64
    }
    ///128 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_128(&self) -> bool {
        *self == PPSFREQ::Hz128
    }
    ///256 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_256(&self) -> bool {
        *self == PPSFREQ::Hz256
    }
    ///512 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_512(&self) -> bool {
        *self == PPSFREQ::Hz512
    }
    ///1024 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_1024(&self) -> bool {
        *self == PPSFREQ::Hz1024
    }
    ///2048 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_2048(&self) -> bool {
        *self == PPSFREQ::Hz2048
    }
    ///4096 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_4096(&self) -> bool {
        *self == PPSFREQ::Hz4096
    }
    ///8192 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_8192(&self) -> bool {
        *self == PPSFREQ::Hz8192
    }
    ///16384 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_16384(&self) -> bool {
        *self == PPSFREQ::Hz16384
    }
    ///32768 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)
    #[inline(always)]
    pub fn is_hz_32768(&self) -> bool {
        *self == PPSFREQ::Hz32768
    }
}
impl R {
    ///Bits 0:3 - PPS frequency selection
    #[inline(always)]
    pub fn ppsfreq(&self) -> PPSFREQ_R {
        PPSFREQ_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPPPSCR")
            .field("ppsfreq", &self.ppsfreq())
            .finish()
    }
}
/**Ethernet PTP PPS control register

You can [`read`](crate::Reg::read) this register and get [`ptpppscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#Ethernet_PTP:PTPPPSCR)*/
pub struct PTPPPSCRrs;
impl crate::RegisterSpec for PTPPPSCRrs {
    type Ux = u32;
}
///`read()` method returns [`ptpppscr::R`](R) reader structure
impl crate::Readable for PTPPPSCRrs {}
///`reset()` method sets PTPPPSCR to value 0
impl crate::Resettable for PTPPPSCRrs {}
