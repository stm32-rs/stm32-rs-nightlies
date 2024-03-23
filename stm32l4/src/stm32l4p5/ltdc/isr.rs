#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Line Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIF {
    #[doc = "0: Programmed line not reached"]
    NotReached = 0,
    #[doc = "1: Line interrupt generated when a programmed line is reached"]
    Reached = 1,
}
impl From<LIF> for bool {
    #[inline(always)]
    fn from(variant: LIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIF` reader - Line Interrupt flag"]
pub type LIF_R = crate::BitReader<LIF>;
impl LIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LIF {
        match self.bits {
            false => LIF::NotReached,
            true => LIF::Reached,
        }
    }
    #[doc = "Programmed line not reached"]
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == LIF::NotReached
    }
    #[doc = "Line interrupt generated when a programmed line is reached"]
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == LIF::Reached
    }
}
#[doc = "FIFO Underrun Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUIF {
    #[doc = "0: No FIFO underrun"]
    NoUnderrun = 0,
    #[doc = "1: FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO"]
    Underrun = 1,
}
impl From<FUIF> for bool {
    #[inline(always)]
    fn from(variant: FUIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUIF` reader - FIFO Underrun Interrupt flag"]
pub type FUIF_R = crate::BitReader<FUIF>;
impl FUIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FUIF {
        match self.bits {
            false => FUIF::NoUnderrun,
            true => FUIF::Underrun,
        }
    }
    #[doc = "No FIFO underrun"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == FUIF::NoUnderrun
    }
    #[doc = "FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == FUIF::Underrun
    }
}
#[doc = "Transfer Error interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIF {
    #[doc = "0: No transfer error"]
    NoError = 0,
    #[doc = "1: Transfer error interrupt generated when a bus error occurs"]
    Error = 1,
}
impl From<TERRIF> for bool {
    #[inline(always)]
    fn from(variant: TERRIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRIF` reader - Transfer Error interrupt flag"]
pub type TERRIF_R = crate::BitReader<TERRIF>;
impl TERRIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TERRIF {
        match self.bits {
            false => TERRIF::NoError,
            true => TERRIF::Error,
        }
    }
    #[doc = "No transfer error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TERRIF::NoError
    }
    #[doc = "Transfer error interrupt generated when a bus error occurs"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TERRIF::Error
    }
}
#[doc = "Register Reload Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIF {
    #[doc = "0: No register reload"]
    NoReload = 0,
    #[doc = "1: Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)"]
    Reload = 1,
}
impl From<RRIF> for bool {
    #[inline(always)]
    fn from(variant: RRIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRIF` reader - Register Reload Interrupt Flag"]
pub type RRIF_R = crate::BitReader<RRIF>;
impl RRIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RRIF {
        match self.bits {
            false => RRIF::NoReload,
            true => RRIF::Reload,
        }
    }
    #[doc = "No register reload"]
    #[inline(always)]
    pub fn is_no_reload(&self) -> bool {
        *self == RRIF::NoReload
    }
    #[doc = "Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == RRIF::Reload
    }
}
impl R {
    #[doc = "Bit 0 - Line Interrupt flag"]
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt flag"]
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Error interrupt flag"]
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register Reload Interrupt Flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "LTDC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
