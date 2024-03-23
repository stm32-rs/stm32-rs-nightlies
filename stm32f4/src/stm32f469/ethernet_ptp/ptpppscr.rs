#[doc = "Register `PTPPPSCR` reader"]
pub type R = crate::R<PTPPPSCRrs>;
#[doc = "PPS frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPSFREQ {
    #[doc = "0: 1 Hz with a pulse width of 125 ms for binary rollover and of 100 ms for digital rollover"]
    Hz1 = 0,
    #[doc = "1: 2 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz2 = 1,
    #[doc = "2: 4 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz4 = 2,
    #[doc = "3: 8 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz8 = 3,
    #[doc = "4: 16 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz16 = 4,
    #[doc = "5: 32 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz32 = 5,
    #[doc = "6: 64 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz64 = 6,
    #[doc = "7: 128 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz128 = 7,
    #[doc = "8: 256 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz256 = 8,
    #[doc = "9: 512 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz512 = 9,
    #[doc = "10: 1024 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz1024 = 10,
    #[doc = "11: 2048 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz2048 = 11,
    #[doc = "12: 4096 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz4096 = 12,
    #[doc = "13: 8192 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz8192 = 13,
    #[doc = "14: 16384 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    Hz16384 = 14,
    #[doc = "15: 32768 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
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
#[doc = "Field `PPSFREQ` reader - PPS frequency selection"]
pub type PPSFREQ_R = crate::FieldReader<PPSFREQ>;
impl PPSFREQ_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "1 Hz with a pulse width of 125 ms for binary rollover and of 100 ms for digital rollover"]
    #[inline(always)]
    pub fn is_hz_1(&self) -> bool {
        *self == PPSFREQ::Hz1
    }
    #[doc = "2 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_2(&self) -> bool {
        *self == PPSFREQ::Hz2
    }
    #[doc = "4 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_4(&self) -> bool {
        *self == PPSFREQ::Hz4
    }
    #[doc = "8 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_8(&self) -> bool {
        *self == PPSFREQ::Hz8
    }
    #[doc = "16 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_16(&self) -> bool {
        *self == PPSFREQ::Hz16
    }
    #[doc = "32 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_32(&self) -> bool {
        *self == PPSFREQ::Hz32
    }
    #[doc = "64 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_64(&self) -> bool {
        *self == PPSFREQ::Hz64
    }
    #[doc = "128 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_128(&self) -> bool {
        *self == PPSFREQ::Hz128
    }
    #[doc = "256 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_256(&self) -> bool {
        *self == PPSFREQ::Hz256
    }
    #[doc = "512 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_512(&self) -> bool {
        *self == PPSFREQ::Hz512
    }
    #[doc = "1024 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_1024(&self) -> bool {
        *self == PPSFREQ::Hz1024
    }
    #[doc = "2048 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_2048(&self) -> bool {
        *self == PPSFREQ::Hz2048
    }
    #[doc = "4096 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_4096(&self) -> bool {
        *self == PPSFREQ::Hz4096
    }
    #[doc = "8192 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_8192(&self) -> bool {
        *self == PPSFREQ::Hz8192
    }
    #[doc = "16384 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_16384(&self) -> bool {
        *self == PPSFREQ::Hz16384
    }
    #[doc = "32768 Hz with 50% duty cycle for binary rollover (digital rollover not recommended)"]
    #[inline(always)]
    pub fn is_hz_32768(&self) -> bool {
        *self == PPSFREQ::Hz32768
    }
}
impl R {
    #[doc = "Bits 0:3 - PPS frequency selection"]
    #[inline(always)]
    pub fn ppsfreq(&self) -> PPSFREQ_R {
        PPSFREQ_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Ethernet PTP PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpppscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPPPSCRrs;
impl crate::RegisterSpec for PTPPPSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpppscr::R`](R) reader structure"]
impl crate::Readable for PTPPPSCRrs {}
#[doc = "`reset()` method sets PTPPPSCR to value 0"]
impl crate::Resettable for PTPPPSCRrs {
    const RESET_VALUE: u32 = 0;
}
