#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2rs>;
#[doc = "Low-power regulator started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPS {
    #[doc = "0: The low-power regulator is not ready"]
    NotReady = 0,
    #[doc = "1: The low-power regulator is ready"]
    Ready = 1,
}
impl From<REGLPS> for bool {
    #[inline(always)]
    fn from(variant: REGLPS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPS` reader - Low-power regulator started"]
pub type REGLPS_R = crate::BitReader<REGLPS>;
impl REGLPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REGLPS {
        match self.bits {
            false => REGLPS::NotReady,
            true => REGLPS::Ready,
        }
    }
    #[doc = "The low-power regulator is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPS::NotReady
    }
    #[doc = "The low-power regulator is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPS::Ready
    }
}
#[doc = "Low-power regulator flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPF {
    #[doc = "0: The regulator is ready in main mode (MR)"]
    Mr = 0,
    #[doc = "1: The regulator is in low-power mode (LPR)"]
    Lpr = 1,
}
impl From<REGLPF> for bool {
    #[inline(always)]
    fn from(variant: REGLPF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPF` reader - Low-power regulator flag"]
pub type REGLPF_R = crate::BitReader<REGLPF>;
impl REGLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REGLPF {
        match self.bits {
            false => REGLPF::Mr,
            true => REGLPF::Lpr,
        }
    }
    #[doc = "The regulator is ready in main mode (MR)"]
    #[inline(always)]
    pub fn is_mr(&self) -> bool {
        *self == REGLPF::Mr
    }
    #[doc = "The regulator is in low-power mode (LPR)"]
    #[inline(always)]
    pub fn is_lpr(&self) -> bool {
        *self == REGLPF::Lpr
    }
}
#[doc = "Voltage scaling flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSF {
    #[doc = "0: The regulator is ready in the selected voltage range"]
    Ready = 0,
    #[doc = "1: The regulator output voltage is changing to the required voltage level"]
    NotReady = 1,
}
impl From<VOSF> for bool {
    #[inline(always)]
    fn from(variant: VOSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub type VOSF_R = crate::BitReader<VOSF>;
impl VOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOSF {
        match self.bits {
            false => VOSF::Ready,
            true => VOSF::NotReady,
        }
    }
    #[doc = "The regulator is ready in the selected voltage range"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSF::Ready
    }
    #[doc = "The regulator output voltage is changing to the required voltage level"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSF::NotReady
    }
}
#[doc = "Power voltage detector output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO {
    #[doc = "0: VDD is above the selected PVD threshold"]
    Above = 0,
    #[doc = "1: VDD is below the selected PVD threshold"]
    Below = 1,
}
impl From<PVDO> for bool {
    #[inline(always)]
    fn from(variant: PVDO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDO` reader - Power voltage detector output"]
pub type PVDO_R = crate::BitReader<PVDO>;
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDO {
        match self.bits {
            false => PVDO::Above,
            true => PVDO::Below,
        }
    }
    #[doc = "VDD is above the selected PVD threshold"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVDO::Above
    }
    #[doc = "VDD is below the selected PVD threshold"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVDO::Below
    }
}
#[doc = "Peripheral voltage monitoring output: VDDUSB vs. 1.2 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO1 {
    #[doc = "0: VDDUSB voltage is above PVM1 threshold (around 1.2 V)"]
    Above = 0,
    #[doc = "1: VDDUSB voltage is below PVM1 threshold (around 1.2 V)"]
    Below = 1,
}
impl From<PVMO1> for bool {
    #[inline(always)]
    fn from(variant: PVMO1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO1` reader - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
pub type PVMO1_R = crate::BitReader<PVMO1>;
impl PVMO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVMO1 {
        match self.bits {
            false => PVMO1::Above,
            true => PVMO1::Below,
        }
    }
    #[doc = "VDDUSB voltage is above PVM1 threshold (around 1.2 V)"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO1::Above
    }
    #[doc = "VDDUSB voltage is below PVM1 threshold (around 1.2 V)"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO1::Below
    }
}
#[doc = "Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO2 {
    #[doc = "0: VDDIO2 voltage is above PVM2 threshold (around 0.9 V)"]
    Above = 0,
    #[doc = "1: VDDIO2 voltage is below PVM2 threshold (around 0.9 V)"]
    Below = 1,
}
impl From<PVMO2> for bool {
    #[inline(always)]
    fn from(variant: PVMO2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO2` reader - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
pub type PVMO2_R = crate::BitReader<PVMO2>;
impl PVMO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVMO2 {
        match self.bits {
            false => PVMO2::Above,
            true => PVMO2::Below,
        }
    }
    #[doc = "VDDIO2 voltage is above PVM2 threshold (around 0.9 V)"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO2::Above
    }
    #[doc = "VDDIO2 voltage is below PVM2 threshold (around 0.9 V)"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO2::Below
    }
}
#[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO3 {
    #[doc = "0: VDDA voltage is above PVM3 threshold (around 1.62 V)"]
    Above = 0,
    #[doc = "1: VDDA voltage is below PVM3 threshold (around 1.62 V)"]
    Below = 1,
}
impl From<PVMO3> for bool {
    #[inline(always)]
    fn from(variant: PVMO3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
pub type PVMO3_R = crate::BitReader<PVMO3>;
impl PVMO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVMO3 {
        match self.bits {
            false => PVMO3::Above,
            true => PVMO3::Below,
        }
    }
    #[doc = "VDDA voltage is above PVM3 threshold (around 1.62 V)"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO3::Above
    }
    #[doc = "VDDA voltage is below PVM3 threshold (around 1.62 V)"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO3::Below
    }
}
#[doc = "Peripheral voltage monitoring output: VDDA vs. 2.2 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO4 {
    #[doc = "0: VDDA voltage is above PVM4 threshold (around 2.2 V)"]
    Above = 0,
    #[doc = "1: VDDA voltage is below PVM4 threshold (around 2.2 V)"]
    Below = 1,
}
impl From<PVMO4> for bool {
    #[inline(always)]
    fn from(variant: PVMO4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO4` reader - Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
pub type PVMO4_R = crate::BitReader<PVMO4>;
impl PVMO4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVMO4 {
        match self.bits {
            false => PVMO4::Above,
            true => PVMO4::Below,
        }
    }
    #[doc = "VDDA voltage is above PVM4 threshold (around 2.2 V)"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO4::Above
    }
    #[doc = "VDDA voltage is below PVM4 threshold (around 2.2 V)"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO4::Below
    }
}
impl R {
    #[doc = "Bit 8 - Low-power regulator started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
    #[inline(always)]
    pub fn pvmo2(&self) -> PVMO2_R {
        PVMO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
    #[inline(always)]
    pub fn pvmo4(&self) -> PVMO4_R {
        PVMO4_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2rs {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0;
}
