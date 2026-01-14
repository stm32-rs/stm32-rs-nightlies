///Register `FDCAN_TXBAR` reader
pub type R = crate::R<FDCAN_TXBARrs>;
///Register `FDCAN_TXBAR` writer
pub type W = crate::W<FDCAN_TXBARrs>;
/**Add request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AR {
    ///0: No transmission request added
    B0x0 = 0,
    ///1: Transmission requested added.
    B0x1 = 1,
}
impl From<AR> for u8 {
    #[inline(always)]
    fn from(variant: AR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AR {
    type Ux = u8;
}
impl crate::IsEnum for AR {}
///Field `AR` reader - Add request
pub type AR_R = crate::FieldReader<AR>;
impl AR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AR> {
        match self.bits {
            0 => Some(AR::B0x0),
            1 => Some(AR::B0x1),
            _ => None,
        }
    }
    ///No transmission request added
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AR::B0x0
    }
    ///Transmission requested added.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AR::B0x1
    }
}
///Field `AR` writer - Add request
pub type AR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, AR>;
impl<'a, REG> AR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No transmission request added
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AR::B0x0)
    }
    ///Transmission requested added.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AR::B0x1)
    }
}
impl R {
    ///Bits 0:2 - Add request
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBAR")
            .field("ar", &self.ar())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Add request
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W<'_, FDCAN_TXBARrs> {
        AR_W::new(self, 0)
    }
}
/**FDCAN Tx buffer add request register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXBAR)*/
pub struct FDCAN_TXBARrs;
impl crate::RegisterSpec for FDCAN_TXBARrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbar::R`](R) reader structure
impl crate::Readable for FDCAN_TXBARrs {}
///`write(|w| ..)` method takes [`fdcan_txbar::W`](W) writer structure
impl crate::Writable for FDCAN_TXBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TXBAR to value 0
impl crate::Resettable for FDCAN_TXBARrs {}
