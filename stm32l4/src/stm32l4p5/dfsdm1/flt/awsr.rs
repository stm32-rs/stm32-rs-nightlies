///Register `AWSR` reader
pub type R = crate::R<AWSRrs>;
/**Analog watchdog low threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWLTF0 {
    ///0: No low threshold error
    NoError = 0,
    ///1: A low threshold error on channel y
    Error = 1,
}
impl From<AWLTF0> for bool {
    #[inline(always)]
    fn from(variant: AWLTF0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWLTF(0-7)` reader - Analog watchdog low threshold flag
pub type AWLTF_R = crate::BitReader<AWLTF0>;
impl AWLTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWLTF0 {
        match self.bits {
            false => AWLTF0::NoError,
            true => AWLTF0::Error,
        }
    }
    ///No low threshold error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AWLTF0::NoError
    }
    ///A low threshold error on channel y
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AWLTF0::Error
    }
}
/**Analog watchdog high threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWHTF0 {
    ///0: No high threshold error
    NoError = 0,
    ///1: A high threshold error on channel y
    Error = 1,
}
impl From<AWHTF0> for bool {
    #[inline(always)]
    fn from(variant: AWHTF0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWHTF(0-7)` reader - Analog watchdog high threshold flag
pub type AWHTF_R = crate::BitReader<AWHTF0>;
impl AWHTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWHTF0 {
        match self.bits {
            false => AWHTF0::NoError,
            true => AWHTF0::Error,
        }
    }
    ///No high threshold error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AWHTF0::NoError
    }
    ///A high threshold error on channel y
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AWHTF0::Error
    }
}
impl R {
    ///Analog watchdog low threshold flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWLTF0` field.</div>
    #[inline(always)]
    pub fn awltf(&self, n: u8) -> AWLTF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AWLTF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf_iter(&self) -> impl Iterator<Item = AWLTF_R> + '_ {
        (0..8).map(move |n| AWLTF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf0(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf1(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf2(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf3(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf4(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf5(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf6(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf7(&self) -> AWLTF_R {
        AWLTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Analog watchdog high threshold flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWHTF0` field.</div>
    #[inline(always)]
    pub fn awhtf(&self, n: u8) -> AWHTF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AWHTF_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf_iter(&self) -> impl Iterator<Item = AWHTF_R> + '_ {
        (0..8).map(move |n| AWHTF_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf0(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf1(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf2(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf3(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf4(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf5(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf6(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf7(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWSR")
            .field("awhtf0", &self.awhtf0())
            .field("awhtf1", &self.awhtf1())
            .field("awhtf2", &self.awhtf2())
            .field("awhtf3", &self.awhtf3())
            .field("awhtf4", &self.awhtf4())
            .field("awhtf5", &self.awhtf5())
            .field("awhtf6", &self.awhtf6())
            .field("awhtf7", &self.awhtf7())
            .field("awltf0", &self.awltf0())
            .field("awltf1", &self.awltf1())
            .field("awltf2", &self.awltf2())
            .field("awltf3", &self.awltf3())
            .field("awltf4", &self.awltf4())
            .field("awltf5", &self.awltf5())
            .field("awltf6", &self.awltf6())
            .field("awltf7", &self.awltf7())
            .finish()
    }
}
/**analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`awsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWSRrs;
impl crate::RegisterSpec for AWSRrs {
    type Ux = u32;
}
///`read()` method returns [`awsr::R`](R) reader structure
impl crate::Readable for AWSRrs {}
///`reset()` method sets AWSR to value 0
impl crate::Resettable for AWSRrs {}
