///Register `GPSR` reader
pub type R = crate::R<GPSRrs>;
/**Command FIFO empty This bit indicates the empty status of the generic command FIFO:

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDFE {
    ///0: Write payload FIFO not empty
    B0x0 = 0,
    ///1: Write payload FIFO empty
    B0x1 = 1,
}
impl From<CMDFE> for bool {
    #[inline(always)]
    fn from(variant: CMDFE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDFE` reader - Command FIFO empty This bit indicates the empty status of the generic command FIFO:
pub type CMDFE_R = crate::BitReader<CMDFE>;
impl CMDFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDFE {
        match self.bits {
            false => CMDFE::B0x0,
            true => CMDFE::B0x1,
        }
    }
    ///Write payload FIFO not empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CMDFE::B0x0
    }
    ///Write payload FIFO empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CMDFE::B0x1
    }
}
/**Command FIFO full This bit indicates the full status of the generic command FIFO:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDFF {
    ///0: Write payload FIFO not full
    B0x0 = 0,
    ///1: Write payload FIFO full
    B0x1 = 1,
}
impl From<CMDFF> for bool {
    #[inline(always)]
    fn from(variant: CMDFF) -> Self {
        variant as u8 != 0
    }
}
///Field `CMDFF` reader - Command FIFO full This bit indicates the full status of the generic command FIFO:
pub type CMDFF_R = crate::BitReader<CMDFF>;
impl CMDFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMDFF {
        match self.bits {
            false => CMDFF::B0x0,
            true => CMDFF::B0x1,
        }
    }
    ///Write payload FIFO not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CMDFF::B0x0
    }
    ///Write payload FIFO full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CMDFF::B0x1
    }
}
/**Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRFE {
    ///0: Write payload FIFO not empty
    B0x0 = 0,
    ///1: Write payload FIFO empty
    B0x1 = 1,
}
impl From<PWRFE> for bool {
    #[inline(always)]
    fn from(variant: PWRFE) -> Self {
        variant as u8 != 0
    }
}
///Field `PWRFE` reader - Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:
pub type PWRFE_R = crate::BitReader<PWRFE>;
impl PWRFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRFE {
        match self.bits {
            false => PWRFE::B0x0,
            true => PWRFE::B0x1,
        }
    }
    ///Write payload FIFO not empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRFE::B0x0
    }
    ///Write payload FIFO empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRFE::B0x1
    }
}
/**Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRFF {
    ///0: Write payload FIFO not full
    B0x0 = 0,
    ///1: Write payload FIFO full
    B0x1 = 1,
}
impl From<PWRFF> for bool {
    #[inline(always)]
    fn from(variant: PWRFF) -> Self {
        variant as u8 != 0
    }
}
///Field `PWRFF` reader - Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:
pub type PWRFF_R = crate::BitReader<PWRFF>;
impl PWRFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRFF {
        match self.bits {
            false => PWRFF::B0x0,
            true => PWRFF::B0x1,
        }
    }
    ///Write payload FIFO not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRFF::B0x0
    }
    ///Write payload FIFO full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRFF::B0x1
    }
}
/**Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRDFE {
    ///0: Read payload FIFO not empty
    B0x0 = 0,
    ///1: Read payload FIFO empty
    B0x1 = 1,
}
impl From<PRDFE> for bool {
    #[inline(always)]
    fn from(variant: PRDFE) -> Self {
        variant as u8 != 0
    }
}
///Field `PRDFE` reader - Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:
pub type PRDFE_R = crate::BitReader<PRDFE>;
impl PRDFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRDFE {
        match self.bits {
            false => PRDFE::B0x0,
            true => PRDFE::B0x1,
        }
    }
    ///Read payload FIFO not empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRDFE::B0x0
    }
    ///Read payload FIFO empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRDFE::B0x1
    }
}
/**Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRDFF {
    ///0: Read payload FIFO not full
    B0x0 = 0,
    ///1: Read payload FIFO ful.
    B0x1 = 1,
}
impl From<PRDFF> for bool {
    #[inline(always)]
    fn from(variant: PRDFF) -> Self {
        variant as u8 != 0
    }
}
///Field `PRDFF` reader - Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:
pub type PRDFF_R = crate::BitReader<PRDFF>;
impl PRDFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRDFF {
        match self.bits {
            false => PRDFF::B0x0,
            true => PRDFF::B0x1,
        }
    }
    ///Read payload FIFO not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRDFF::B0x0
    }
    ///Read payload FIFO ful.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRDFF::B0x1
    }
}
/**Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB {
    ///0: No read command on going
    B0x0 = 0,
    ///1: Read command on going
    B0x1 = 1,
}
impl From<RCB> for bool {
    #[inline(always)]
    fn from(variant: RCB) -> Self {
        variant as u8 != 0
    }
}
///Field `RCB` reader - Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:
pub type RCB_R = crate::BitReader<RCB>;
impl RCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCB {
        match self.bits {
            false => RCB::B0x0,
            true => RCB::B0x1,
        }
    }
    ///No read command on going
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RCB::B0x0
    }
    ///Read command on going
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RCB::B0x1
    }
}
impl R {
    ///Bit 0 - Command FIFO empty This bit indicates the empty status of the generic command FIFO:
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Command FIFO full This bit indicates the full status of the generic command FIFO:
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Payload write FIFO empty This bit indicates the empty status of the generic write payload FIFO:
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Payload write FIFO full This bit indicates the full status of the generic write payload FIFO:
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Payload read FIFO empty This bit indicates the empty status of the generic read payload FIFO:
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Payload read FIFO full This bit indicates the full status of the generic read payload FIFO:
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Read command busy This bit is set when a read command is issued and cleared when the entire response is stored in the FIFO:
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPSR")
            .field("cmdfe", &self.cmdfe())
            .field("cmdff", &self.cmdff())
            .field("pwrfe", &self.pwrfe())
            .field("pwrff", &self.pwrff())
            .field("prdfe", &self.prdfe())
            .field("prdff", &self.prdff())
            .field("rcb", &self.rcb())
            .finish()
    }
}
/**DSI Host generic packet status register

You can [`read`](crate::Reg::read) this register and get [`gpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:GPSR)*/
pub struct GPSRrs;
impl crate::RegisterSpec for GPSRrs {
    type Ux = u32;
}
///`read()` method returns [`gpsr::R`](R) reader structure
impl crate::Readable for GPSRrs {}
///`reset()` method sets GPSR to value 0x15
impl crate::Resettable for GPSRrs {
    const RESET_VALUE: u32 = 0x15;
}
