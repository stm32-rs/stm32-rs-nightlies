///Register `FDCAN_HPMS` reader
pub type R = crate::R<FDCAN_HPMSrs>;
///Field `BIDX` reader - Buffer index
pub type BIDX_R = crate::FieldReader;
/**Message storage indicator

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSI {
    ///0: No FIFO selected
    B0x0 = 0,
    ///1: FIFO overrun
    B0x1 = 1,
    ///2: Message stored in FIFO 0
    B0x2 = 2,
    ///3: Message stored in FIFO 1
    B0x3 = 3,
}
impl From<MSI> for u8 {
    #[inline(always)]
    fn from(variant: MSI) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSI {
    type Ux = u8;
}
impl crate::IsEnum for MSI {}
///Field `MSI` reader - Message storage indicator
pub type MSI_R = crate::FieldReader<MSI>;
impl MSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSI {
        match self.bits {
            0 => MSI::B0x0,
            1 => MSI::B0x1,
            2 => MSI::B0x2,
            3 => MSI::B0x3,
            _ => unreachable!(),
        }
    }
    ///No FIFO selected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSI::B0x0
    }
    ///FIFO overrun
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSI::B0x1
    }
    ///Message stored in FIFO 0
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MSI::B0x2
    }
    ///Message stored in FIFO 1
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MSI::B0x3
    }
}
///Field `FIDX` reader - Filter index
pub type FIDX_R = crate::FieldReader;
/**Filter list

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLST {
    ///0: Standard filter list
    B0x0 = 0,
    ///1: Extended filter list
    B0x1 = 1,
}
impl From<FLST> for bool {
    #[inline(always)]
    fn from(variant: FLST) -> Self {
        variant as u8 != 0
    }
}
///Field `FLST` reader - Filter list
pub type FLST_R = crate::BitReader<FLST>;
impl FLST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLST {
        match self.bits {
            false => FLST::B0x0,
            true => FLST::B0x1,
        }
    }
    ///Standard filter list
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLST::B0x0
    }
    ///Extended filter list
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLST::B0x1
    }
}
impl R {
    ///Bits 0:2 - Buffer index
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 7) as u8)
    }
    ///Bits 6:7 - Message storage indicator
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:12 - Filter index
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 15 - Filter list
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_HPMS")
            .field("bidx", &self.bidx())
            .field("msi", &self.msi())
            .field("fidx", &self.fidx())
            .field("flst", &self.flst())
            .finish()
    }
}
/**FDCAN high-priority message status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_HPMS)*/
pub struct FDCAN_HPMSrs;
impl crate::RegisterSpec for FDCAN_HPMSrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_hpms::R`](R) reader structure
impl crate::Readable for FDCAN_HPMSrs {}
///`reset()` method sets FDCAN_HPMS to value 0
impl crate::Resettable for FDCAN_HPMSrs {}
