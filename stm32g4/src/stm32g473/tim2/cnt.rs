///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Register `CNT` writer
pub type W = crate::W<CNTrs>;
///Field `CNT` reader - counter value
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - counter value
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
/**UIFCPY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIFCPYR {
    ///0: No update occurred
    NoUpdateOccurred = 0,
    ///1: Update interrupt pending
    UpdatePending = 1,
}
impl From<UIFCPYR> for bool {
    #[inline(always)]
    fn from(variant: UIFCPYR) -> Self {
        variant as u8 != 0
    }
}
///Field `UIFCPY` reader - UIFCPY
pub type UIFCPY_R = crate::BitReader<UIFCPYR>;
impl UIFCPY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIFCPYR {
        match self.bits {
            false => UIFCPYR::NoUpdateOccurred,
            true => UIFCPYR::UpdatePending,
        }
    }
    ///No update occurred
    #[inline(always)]
    pub fn is_no_update_occurred(&self) -> bool {
        *self == UIFCPYR::NoUpdateOccurred
    }
    ///Update interrupt pending
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIFCPYR::UpdatePending
    }
}
///Field `UIFCPY` writer - UIFCPY
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG, UIFCPYR>;
impl<'a, REG> UIFCPY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No update occurred
    #[inline(always)]
    pub fn no_update_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(UIFCPYR::NoUpdateOccurred)
    }
    ///Update interrupt pending
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut crate::W<REG> {
        self.variant(UIFCPYR::UpdatePending)
    }
}
impl R {
    ///Bits 0:31 - counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
    ///Bit 31 - UIFCPY
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("uifcpy", &self.uifcpy())
            .field("cnt", &self.cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIFCPY
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<CNTrs> {
        UIFCPY_W::new(self, 31)
    }
}
/**counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#TIM2:CNT)*/
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
///`read()` method returns [`cnt::R`](R) reader structure
impl crate::Readable for CNTrs {}
///`write(|w| ..)` method takes [`cnt::W`](W) writer structure
impl crate::Writable for CNTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNTrs {}
