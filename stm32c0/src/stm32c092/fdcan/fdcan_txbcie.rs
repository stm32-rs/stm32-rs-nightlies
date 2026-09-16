///Register `FDCAN_TXBCIE` reader
pub type R = crate::R<FDCAN_TXBCIErs>;
///Register `FDCAN_TXBCIE` writer
pub type W = crate::W<FDCAN_TXBCIErs>;
/**Cancellation finished interrupt enable.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFIE {
    ///0: Cancellation finished interrupt disabled
    B0x0 = 0,
    ///1: Cancellation finished interrupt enabled
    B0x1 = 1,
}
impl From<CFIE> for u8 {
    #[inline(always)]
    fn from(variant: CFIE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFIE {
    type Ux = u8;
}
impl crate::IsEnum for CFIE {}
///Field `CFIE` reader - Cancellation finished interrupt enable.
pub type CFIE_R = crate::FieldReader<CFIE>;
impl CFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CFIE> {
        match self.bits {
            0 => Some(CFIE::B0x0),
            1 => Some(CFIE::B0x1),
            _ => None,
        }
    }
    ///Cancellation finished interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CFIE::B0x0
    }
    ///Cancellation finished interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CFIE::B0x1
    }
}
///Field `CFIE` writer - Cancellation finished interrupt enable.
pub type CFIE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CFIE>;
impl<'a, REG> CFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Cancellation finished interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CFIE::B0x0)
    }
    ///Cancellation finished interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CFIE::B0x1)
    }
}
impl R {
    ///Bits 0:2 - Cancellation finished interrupt enable.
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCIE")
            .field("cfie", &self.cfie())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Cancellation finished interrupt enable.
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W<'_, FDCAN_TXBCIErs> {
        CFIE_W::new(self, 0)
    }
}
/**FDCAN Tx buffer cancellation finished interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXBCIE)*/
pub struct FDCAN_TXBCIErs;
impl crate::RegisterSpec for FDCAN_TXBCIErs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbcie::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCIErs {}
///`write(|w| ..)` method takes [`fdcan_txbcie::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCIErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TXBCIE to value 0
impl crate::Resettable for FDCAN_TXBCIErs {}
