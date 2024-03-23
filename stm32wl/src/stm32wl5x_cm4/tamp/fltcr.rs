#[doc = "Register `FLTCR` reader"]
pub type R = crate::R<FLTCRrs>;
#[doc = "Register `FLTCR` writer"]
pub type W = crate::W<FLTCRrs>;
#[doc = "TAMPFREQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFREQ {
    #[doc = "0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    Hz1 = 0,
    #[doc = "1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    Hz2 = 1,
    #[doc = "2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    Hz4 = 2,
    #[doc = "3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    Hz8 = 3,
    #[doc = "4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    Hz16 = 4,
    #[doc = "5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    Hz32 = 5,
    #[doc = "6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    Hz64 = 6,
    #[doc = "7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    Hz128 = 7,
}
impl From<TAMPFREQ> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFREQ {
    type Ux = u8;
}
#[doc = "Field `TAMPFREQ` reader - TAMPFREQ"]
pub type TAMPFREQ_R = crate::FieldReader<TAMPFREQ>;
impl TAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFREQ {
        match self.bits {
            0 => TAMPFREQ::Hz1,
            1 => TAMPFREQ::Hz2,
            2 => TAMPFREQ::Hz4,
            3 => TAMPFREQ::Hz8,
            4 => TAMPFREQ::Hz16,
            5 => TAMPFREQ::Hz32,
            6 => TAMPFREQ::Hz64,
            7 => TAMPFREQ::Hz128,
            _ => unreachable!(),
        }
    }
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_1(&self) -> bool {
        *self == TAMPFREQ::Hz1
    }
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_2(&self) -> bool {
        *self == TAMPFREQ::Hz2
    }
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_4(&self) -> bool {
        *self == TAMPFREQ::Hz4
    }
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_8(&self) -> bool {
        *self == TAMPFREQ::Hz8
    }
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_16(&self) -> bool {
        *self == TAMPFREQ::Hz16
    }
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_32(&self) -> bool {
        *self == TAMPFREQ::Hz32
    }
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_64(&self) -> bool {
        *self == TAMPFREQ::Hz64
    }
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn is_hz_128(&self) -> bool {
        *self == TAMPFREQ::Hz128
    }
}
#[doc = "Field `TAMPFREQ` writer - TAMPFREQ"]
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TAMPFREQ>;
impl<'a, REG> TAMPFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz1)
    }
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz2)
    }
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz4)
    }
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz8)
    }
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_16(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz16)
    }
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_32(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz32)
    }
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_64(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz64)
    }
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn hz_128(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz128)
    }
}
#[doc = "TAMPFLT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFLT {
    #[doc = "0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)\""]
    NoFilter = 0,
    #[doc = "1: Tamper event is activated after 2 consecutive samples at the active level\""]
    Filter2 = 1,
    #[doc = "2: Tamper event is activated after 4 consecutive samples at the active level\""]
    Filter4 = 2,
    #[doc = "3: Tamper event is activated after 8 consecutive samples at the active level\""]
    Filter8 = 3,
}
impl From<TAMPFLT> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFLT {
    type Ux = u8;
}
#[doc = "Field `TAMPFLT` reader - TAMPFLT"]
pub type TAMPFLT_R = crate::FieldReader<TAMPFLT>;
impl TAMPFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFLT {
        match self.bits {
            0 => TAMPFLT::NoFilter,
            1 => TAMPFLT::Filter2,
            2 => TAMPFLT::Filter4,
            3 => TAMPFLT::Filter8,
            _ => unreachable!(),
        }
    }
    #[doc = "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)\""]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == TAMPFLT::NoFilter
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level\""]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == TAMPFLT::Filter2
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level\""]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == TAMPFLT::Filter4
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level\""]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == TAMPFLT::Filter8
    }
}
#[doc = "Field `TAMPFLT` writer - TAMPFLT"]
pub type TAMPFLT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TAMPFLT>;
impl<'a, REG> TAMPFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)\""]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::NoFilter)
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level\""]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Filter2)
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level\""]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Filter4)
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level\""]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Filter8)
    }
}
#[doc = "TAMPPRCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPPRCH {
    #[doc = "0: 1 RTCCLK cycle"]
    Cycles1 = 0,
    #[doc = "1: 2 RTCCLK cycles"]
    Cycles2 = 1,
    #[doc = "2: 4 RTCCLK cycles"]
    Cycles4 = 2,
    #[doc = "3: 8 RTCCLK cycles"]
    Cycles8 = 3,
}
impl From<TAMPPRCH> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPPRCH {
    type Ux = u8;
}
#[doc = "Field `TAMPPRCH` reader - TAMPPRCH"]
pub type TAMPPRCH_R = crate::FieldReader<TAMPPRCH>;
impl TAMPPRCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPRCH {
        match self.bits {
            0 => TAMPPRCH::Cycles1,
            1 => TAMPPRCH::Cycles2,
            2 => TAMPPRCH::Cycles4,
            3 => TAMPPRCH::Cycles8,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        *self == TAMPPRCH::Cycles1
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TAMPPRCH::Cycles2
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TAMPPRCH::Cycles4
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TAMPPRCH::Cycles8
    }
}
#[doc = "Field `TAMPPRCH` writer - TAMPPRCH"]
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TAMPPRCH>;
impl<'a, REG> TAMPPRCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles1)
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles2)
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles4)
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles8)
    }
}
#[doc = "TAMPPUDIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPUDIS {
    #[doc = "0: Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    Enabled = 0,
    #[doc = "1: Disable precharge of TAMP_INx pins"]
    Disabled = 1,
}
impl From<TAMPPUDIS> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPPUDIS` reader - TAMPPUDIS"]
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS>;
impl TAMPPUDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPUDIS {
        match self.bits {
            false => TAMPPUDIS::Enabled,
            true => TAMPPUDIS::Disabled,
        }
    }
    #[doc = "Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPPUDIS::Enabled
    }
    #[doc = "Disable precharge of TAMP_INx pins"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPPUDIS::Disabled
    }
}
#[doc = "Field `TAMPPUDIS` writer - TAMPPUDIS"]
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPPUDIS>;
impl<'a, REG> TAMPPUDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Precharge TAMP_INx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Enabled)
    }
    #[doc = "Disable precharge of TAMP_INx pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TAMPFREQ"]
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<FLTCRrs> {
        TAMPFREQ_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - TAMPFLT"]
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<FLTCRrs> {
        TAMPFLT_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - TAMPPRCH"]
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<FLTCRrs> {
        TAMPPRCH_W::new(self, 5)
    }
    #[doc = "Bit 7 - TAMPPUDIS"]
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<FLTCRrs> {
        TAMPPUDIS_W::new(self, 7)
    }
}
#[doc = "TAMP filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTCRrs;
impl crate::RegisterSpec for FLTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltcr::R`](R) reader structure"]
impl crate::Readable for FLTCRrs {}
#[doc = "`write(|w| ..)` method takes [`fltcr::W`](W) writer structure"]
impl crate::Writable for FLTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FLTCRrs {
    const RESET_VALUE: u32 = 0;
}
