///Register `FDCAN_RXF1S` reader
pub type R = crate::R<FDCAN_RXF1Srs>;
///Field `F1FL` reader - Rx FIFO 1 fill level
pub type F1FL_R = crate::FieldReader;
///Field `F1GI` reader - Rx FIFO 1 get index
pub type F1GI_R = crate::FieldReader;
///Field `F1PI` reader - Rx FIFO 1 put index
pub type F1PI_R = crate::FieldReader;
/**Rx FIFO 1 full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F1F {
    ///0: Rx FIFO 1 not full
    B0x0 = 0,
    ///1: Rx FIFO 1 full
    B0x1 = 1,
}
impl From<F1F> for bool {
    #[inline(always)]
    fn from(variant: F1F) -> Self {
        variant as u8 != 0
    }
}
///Field `F1F` reader - Rx FIFO 1 full
pub type F1F_R = crate::BitReader<F1F>;
impl F1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> F1F {
        match self.bits {
            false => F1F::B0x0,
            true => F1F::B0x1,
        }
    }
    ///Rx FIFO 1 not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F1F::B0x0
    }
    ///Rx FIFO 1 full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F1F::B0x1
    }
}
/**Rx FIFO 1 message lost

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1L {
    ///0: No Rx FIFO 1 message lost
    B0x0 = 0,
    ///1: Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0
    B0x1 = 1,
}
impl From<RF1L> for bool {
    #[inline(always)]
    fn from(variant: RF1L) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1L` reader - Rx FIFO 1 message lost
pub type RF1L_R = crate::BitReader<RF1L>;
impl RF1L_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF1L {
        match self.bits {
            false => RF1L::B0x0,
            true => RF1L::B0x1,
        }
    }
    ///No Rx FIFO 1 message lost
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1L::B0x0
    }
    ///Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1L::B0x1
    }
}
impl R {
    ///Bits 0:3 - Rx FIFO 1 fill level
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - Rx FIFO 1 get index
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Rx FIFO 1 put index
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Rx FIFO 1 full
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Rx FIFO 1 message lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1S")
            .field("f1fl", &self.f1fl())
            .field("f1gi", &self.f1gi())
            .field("f1pi", &self.f1pi())
            .field("f1f", &self.f1f())
            .field("rf1l", &self.rf1l())
            .finish()
    }
}
/**FDCAN Rx FIFO 1 status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_RXF1S)*/
pub struct FDCAN_RXF1Srs;
impl crate::RegisterSpec for FDCAN_RXF1Srs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_rxf1s::R`](R) reader structure
impl crate::Readable for FDCAN_RXF1Srs {}
///`reset()` method sets FDCAN_RXF1S to value 0
impl crate::Resettable for FDCAN_RXF1Srs {}
