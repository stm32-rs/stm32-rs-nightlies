///Register `FDCAN_TXBCF` reader
pub type R = crate::R<FDCAN_TXBCFrs>;
/**Cancellation finished

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CF {
    ///0: No transmit buffer cancellation
    B0x0 = 0,
    ///1: Transmit buffer cancellation finished
    B0x1 = 1,
}
impl From<CF> for u8 {
    #[inline(always)]
    fn from(variant: CF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CF {
    type Ux = u8;
}
impl crate::IsEnum for CF {}
///Field `CF` reader - Cancellation finished
pub type CF_R = crate::FieldReader<CF>;
impl CF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CF> {
        match self.bits {
            0 => Some(CF::B0x0),
            1 => Some(CF::B0x1),
            _ => None,
        }
    }
    ///No transmit buffer cancellation
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CF::B0x0
    }
    ///Transmit buffer cancellation finished
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CF::B0x1
    }
}
impl R {
    ///Bits 0:2 - Cancellation finished
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCF")
            .field("cf", &self.cf())
            .finish()
    }
}
/**FDCAN Tx buffer cancellation finished register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXBCF)*/
pub struct FDCAN_TXBCFrs;
impl crate::RegisterSpec for FDCAN_TXBCFrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbcf::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCFrs {}
///`reset()` method sets FDCAN_TXBCF to value 0
impl crate::Resettable for FDCAN_TXBCFrs {}
