///Register `IWUF` reader
pub type R = crate::R<IWUFrs>;
///Register `IWUF` writer
pub type W = crate::W<IWUFrs>;
///Field `IWUF0` reader - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF0_R = crate::BitReader;
///Field `IWUF0` writer - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUF1` reader - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred
pub type IWUF1_R = crate::BitReader;
///Field `IWUF1` writer - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred
pub type IWUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUF2` reader - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF2_R = crate::BitReader;
///Field `IWUF2` writer - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUF3` reader - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF3_R = crate::BitReader;
///Field `IWUF3` writer - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWUF4` reader - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF4_R = crate::BitReader;
///Field `IWUF4` writer - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit.
pub type IWUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMRSUBGF` reader - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt
pub type WMRSUBGF_R = crate::BitReader;
///Field `WMRSUBGF` writer - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt
pub type WMRSUBGF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMRSUBGHCPUF` reader - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt
pub type WMRSUBGHCPUF_R = crate::BitReader;
///Field `WMRSUBGHCPUF` writer - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt
pub type WMRSUBGHCPUF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WLPAWURF` reader - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt
pub type WLPAWURF_R = crate::BitReader;
///Field `WLPAWURF` writer - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt
pub type WLPAWURF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf0(&self) -> IWUF0_R {
        IWUF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred
    #[inline(always)]
    pub fn iwuf1(&self) -> IWUF1_R {
        IWUF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf2(&self) -> IWUF2_R {
        IWUF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf3(&self) -> IWUF3_R {
        IWUF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf4(&self) -> IWUF4_R {
        IWUF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt
    #[inline(always)]
    pub fn wmrsubgf(&self) -> WMRSUBGF_R {
        WMRSUBGF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt
    #[inline(always)]
    pub fn wmrsubghcpuf(&self) -> WMRSUBGHCPUF_R {
        WMRSUBGHCPUF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt
    #[inline(always)]
    pub fn wlpawurf(&self) -> WLPAWURF_R {
        WLPAWURF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWUF")
            .field("iwuf0", &self.iwuf0())
            .field("iwuf1", &self.iwuf1())
            .field("iwuf2", &self.iwuf2())
            .field("iwuf3", &self.iwuf3())
            .field("iwuf4", &self.iwuf4())
            .field("wmrsubgf", &self.wmrsubgf())
            .field("wmrsubghcpuf", &self.wmrsubghcpuf())
            .field("wlpawurf", &self.wlpawurf())
            .finish()
    }
}
impl W {
    ///Bit 0 - IWUF0: Internal wakeup flag (LPUART). - 0: no wakeup from LPUART occurred since last clear. - 1: a wakeup from LPUART occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf0(&mut self) -> IWUF0_W<'_, IWUFrs> {
        IWUF0_W::new(self, 0)
    }
    ///Bit 1 - IWUF1: Internal wakeup flag (RTC). - 0: no wakeup from RTC occurred since last clear. - 1: a wakeup from RTC occurred
    #[inline(always)]
    pub fn iwuf1(&mut self) -> IWUF1_W<'_, IWUFrs> {
        IWUF1_W::new(self, 1)
    }
    ///Bit 2 - IWUF2: Internal wakeup flag (LCD). - 0: no wakeup from LCD occurred since last clear. - 1: a wakeup from LCD occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf2(&mut self) -> IWUF2_W<'_, IWUFrs> {
        IWUF2_W::new(self, 2)
    }
    ///Bit 3 - IWUF3: Internal wakeup flag (COMP). - 0: no wakeup from COMP occurred since last clear. - 1: a wakeup from COMP occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf3(&mut self) -> IWUF3_W<'_, IWUFrs> {
        IWUF3_W::new(self, 3)
    }
    ///Bit 4 - IWUF4: Internal wakeup flag (LCSC). - 0: no wakeup from LCSC occurred since last clear. - 1: a wakeup from LCSC occurred since last clear. Cleared by writing 1 in this bit.
    #[inline(always)]
    pub fn iwuf4(&mut self) -> IWUF4_W<'_, IWUFrs> {
        IWUF4_W::new(self, 4)
    }
    ///Bit 8 - WMRSUBGF Wakeup MRSUBG Flag This bit is set by hardware when a MRSUBG wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Wakeup detected - 1: MRSUBG Wakeup detected writting 1 in this bit, clears the interrupt
    #[inline(always)]
    pub fn wmrsubgf(&mut self) -> WMRSUBGF_W<'_, IWUFrs> {
        WMRSUBGF_W::new(self, 8)
    }
    ///Bit 9 - WMRSUBGHCPUF Wakeup MRSUBG HOST CPU Flag (cf. user manual) This bit is set by hardware when a MRSUBG HOST CPU wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No MRSUBG Host CPU wakeup detected - 1: MRSUBG Host CPU wakeup detected writting 1 in this bit, clears the interrupt
    #[inline(always)]
    pub fn wmrsubghcpuf(&mut self) -> WMRSUBGHCPUF_W<'_, IWUFrs> {
        WMRSUBGHCPUF_W::new(self, 9)
    }
    ///Bit 10 - WLPAWURF Wakeup LPAWUR Flag (cf. user manual) This bit is set by hardware when a LPAWUR wakeup is detected It is cleared by a reset pad or by software writing 1 in this bit field. - 0: No LPAWUR wakeup detected - 1: LPAWUR wakeup detected writting 1 in this bit, clears the interrupt
    #[inline(always)]
    pub fn wlpawurf(&mut self) -> WLPAWURF_W<'_, IWUFrs> {
        WLPAWURF_W::new(self, 10)
    }
}
/**IWUF register

You can [`read`](crate::Reg::read) this register and get [`iwuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:IWUF)*/
pub struct IWUFrs;
impl crate::RegisterSpec for IWUFrs {
    type Ux = u32;
}
///`read()` method returns [`iwuf::R`](R) reader structure
impl crate::Readable for IWUFrs {}
///`write(|w| ..)` method takes [`iwuf::W`](W) writer structure
impl crate::Writable for IWUFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IWUF to value 0
impl crate::Resettable for IWUFrs {}
