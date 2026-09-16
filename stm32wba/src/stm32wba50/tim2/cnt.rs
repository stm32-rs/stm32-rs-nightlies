///Register `CNT` reader
pub type R = crate::R<CNTrs>;
///Register `CNT` writer
pub type W = crate::W<CNTrs>;
///Field `CNT` reader - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32, crate::Safe>;
/**UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register

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
///Field `UIFCPY` reader - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
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
///Field `UIFCPY` writer - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
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
    ///Bits 0:30 - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("cnt", &self.cnt())
            .field("uifcpy", &self.uifcpy())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Least significant part of counter value Non-dithering mode (DITHEN = 0) The register holds the counter value. Dithering mode (DITHEN = 1) The register holds the non-dithered part in CNT\[30:0\]. The fractional part is not available.
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CNTrs> {
        CNT_W::new(self, 0)
    }
    ///Bit 31 - UIF copy or CNT bit 31 If TIMx_CR1.UIFREMAP = 0, thisbit means CNT\[31\], the most significant bit of counter value If TIMx_CRT1.UIFREMAP = 1, this bit means UIF Copy, and is a read-only copy of the UIF bit of the TIMx_ISR register
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W<'_, CNTrs> {
        UIFCPY_W::new(self, 31)
    }
}
/**TIM2 counter

You can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#TIM2:CNT)*/
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
