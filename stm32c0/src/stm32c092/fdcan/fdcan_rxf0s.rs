///Register `FDCAN_RXF0S` reader
pub type R = crate::R<FDCAN_RXF0Srs>;
///Field `F0FL` reader - Rx FIFO 0 fill level
pub type F0FL_R = crate::FieldReader;
///Field `F0GI` reader - Rx FIFO 0 get index
pub type F0GI_R = crate::FieldReader;
///Field `F0PI` reader - Rx FIFO 0 put index
pub type F0PI_R = crate::FieldReader;
/**Rx FIFO 0 full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F0F {
    ///0: Rx FIFO 0 not full
    B0x0 = 0,
    ///1: Rx FIFO 0 full
    B0x1 = 1,
}
impl From<F0F> for bool {
    #[inline(always)]
    fn from(variant: F0F) -> Self {
        variant as u8 != 0
    }
}
///Field `F0F` reader - Rx FIFO 0 full
pub type F0F_R = crate::BitReader<F0F>;
impl F0F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> F0F {
        match self.bits {
            false => F0F::B0x0,
            true => F0F::B0x1,
        }
    }
    ///Rx FIFO 0 not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F0F::B0x0
    }
    ///Rx FIFO 0 full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F0F::B0x1
    }
}
/**Rx FIFO 0 message lost

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0L {
    ///0: No Rx FIFO 0 message lost
    B0x0 = 0,
    ///1: Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0
    B0x1 = 1,
}
impl From<RF0L> for bool {
    #[inline(always)]
    fn from(variant: RF0L) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0L` reader - Rx FIFO 0 message lost
pub type RF0L_R = crate::BitReader<RF0L>;
impl RF0L_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF0L {
        match self.bits {
            false => RF0L::B0x0,
            true => RF0L::B0x1,
        }
    }
    ///No Rx FIFO 0 message lost
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0L::B0x0
    }
    ///Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0L::B0x1
    }
}
impl R {
    ///Bits 0:3 - Rx FIFO 0 fill level
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - Rx FIFO 0 get index
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Rx FIFO 0 put index
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Rx FIFO 0 full
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Rx FIFO 0 message lost
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0S")
            .field("f0fl", &self.f0fl())
            .field("f0gi", &self.f0gi())
            .field("f0pi", &self.f0pi())
            .field("f0f", &self.f0f())
            .field("rf0l", &self.rf0l())
            .finish()
    }
}
/**FDCAN Rx FIFO 0 status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_RXF0S)*/
pub struct FDCAN_RXF0Srs;
impl crate::RegisterSpec for FDCAN_RXF0Srs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxf0s::R`](R) reader structure
impl crate::Readable for FDCAN_RXF0Srs {}
///`reset()` method sets FDCAN_RXF0S to value 0
impl crate::Resettable for FDCAN_RXF0Srs {}
