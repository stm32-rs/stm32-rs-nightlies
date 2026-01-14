///Register `FDCAN_TXBRP` reader
pub type R = crate::R<FDCAN_TXBRPrs>;
/**Transmission request pending

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRP {
    ///0: No transmission request pending
    B0x0 = 0,
    ///1: Transmission request pending
    B0x1 = 1,
}
impl From<TRP> for u8 {
    #[inline(always)]
    fn from(variant: TRP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRP {
    type Ux = u8;
}
impl crate::IsEnum for TRP {}
///Field `TRP` reader - Transmission request pending
pub type TRP_R = crate::FieldReader<TRP>;
impl TRP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRP> {
        match self.bits {
            0 => Some(TRP::B0x0),
            1 => Some(TRP::B0x1),
            _ => None,
        }
    }
    ///No transmission request pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TRP::B0x0
    }
    ///Transmission request pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TRP::B0x1
    }
}
impl R {
    ///Bits 0:2 - Transmission request pending
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBRP")
            .field("trp", &self.trp())
            .finish()
    }
}
/**FDCAN Tx buffer request pending register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXBRP)*/
pub struct FDCAN_TXBRPrs;
impl crate::RegisterSpec for FDCAN_TXBRPrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbrp::R`](R) reader structure
impl crate::Readable for FDCAN_TXBRPrs {}
///`reset()` method sets FDCAN_TXBRP to value 0
impl crate::Resettable for FDCAN_TXBRPrs {}
