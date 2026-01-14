///Register `FDCAN_TXBTO` reader
pub type R = crate::R<FDCAN_TXBTOrs>;
/**Transmission occurred.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TO {
    ///0: No transmission occurred
    B0x0 = 0,
    ///1: Transmission occurred
    B0x1 = 1,
}
impl From<TO> for u8 {
    #[inline(always)]
    fn from(variant: TO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TO {
    type Ux = u8;
}
impl crate::IsEnum for TO {}
///Field `TO` reader - Transmission occurred.
pub type TO_R = crate::FieldReader<TO>;
impl TO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TO> {
        match self.bits {
            0 => Some(TO::B0x0),
            1 => Some(TO::B0x1),
            _ => None,
        }
    }
    ///No transmission occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TO::B0x0
    }
    ///Transmission occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TO::B0x1
    }
}
impl R {
    ///Bits 0:2 - Transmission occurred.
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBTO")
            .field("to", &self.to())
            .finish()
    }
}
/**FDCAN Tx buffer transmission occurred register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXBTO)*/
pub struct FDCAN_TXBTOrs;
impl crate::RegisterSpec for FDCAN_TXBTOrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbto::R`](R) reader structure
impl crate::Readable for FDCAN_TXBTOrs {}
///`reset()` method sets FDCAN_TXBTO to value 0
impl crate::Resettable for FDCAN_TXBTOrs {}
