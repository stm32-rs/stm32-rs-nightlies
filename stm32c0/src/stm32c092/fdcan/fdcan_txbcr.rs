///Register `FDCAN_TXBCR` reader
pub type R = crate::R<FDCAN_TXBCRrs>;
///Register `FDCAN_TXBCR` writer
pub type W = crate::W<FDCAN_TXBCRrs>;
/**Cancellation request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CR {
    ///0: No cancellation pending
    B0x0 = 0,
    ///1: Cancellation pending
    B0x1 = 1,
}
impl From<CR> for u8 {
    #[inline(always)]
    fn from(variant: CR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CR {
    type Ux = u8;
}
impl crate::IsEnum for CR {}
///Field `CR` reader - Cancellation request
pub type CR_R = crate::FieldReader<CR>;
impl CR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CR> {
        match self.bits {
            0 => Some(CR::B0x0),
            1 => Some(CR::B0x1),
            _ => None,
        }
    }
    ///No cancellation pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CR::B0x0
    }
    ///Cancellation pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CR::B0x1
    }
}
///Field `CR` writer - Cancellation request
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CR>;
impl<'a, REG> CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No cancellation pending
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CR::B0x0)
    }
    ///Cancellation pending
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CR::B0x1)
    }
}
impl R {
    ///Bits 0:2 - Cancellation request
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCR")
            .field("cr", &self.cr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Cancellation request
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, FDCAN_TXBCRrs> {
        CR_W::new(self, 0)
    }
}
/**FDCAN Tx buffer cancellation request register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXBCR)*/
pub struct FDCAN_TXBCRrs;
impl crate::RegisterSpec for FDCAN_TXBCRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbcr::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCRrs {}
///`write(|w| ..)` method takes [`fdcan_txbcr::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TXBCR to value 0
impl crate::Resettable for FDCAN_TXBCRrs {}
