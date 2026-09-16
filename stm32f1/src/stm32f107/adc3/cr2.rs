///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///A/D converter ON / OFF
pub use crate::stm32f107::adc1::cr2::ADON;
///Field `ADON` reader - A/D converter ON / OFF
pub use crate::stm32f107::adc1::cr2::ADON_R;
///Field `ADON` writer - A/D converter ON / OFF
pub use crate::stm32f107::adc1::cr2::ADON_W;
///Data alignment
pub use crate::stm32f107::adc1::cr2::ALIGN;
///Field `ALIGN` reader - Data alignment
pub use crate::stm32f107::adc1::cr2::ALIGN_R;
///Field `ALIGN` writer - Data alignment
pub use crate::stm32f107::adc1::cr2::ALIGN_W;
///A/D calibration
pub use crate::stm32f107::adc1::cr2::CALR;
///A/D calibration
pub use crate::stm32f107::adc1::cr2::CALW;
///Field `CAL` reader - A/D calibration
pub use crate::stm32f107::adc1::cr2::CAL_R;
///Field `CAL` writer - A/D calibration
pub use crate::stm32f107::adc1::cr2::CAL_W;
///Continuous conversion
pub use crate::stm32f107::adc1::cr2::CONT;
///Field `CONT` reader - Continuous conversion
pub use crate::stm32f107::adc1::cr2::CONT_R;
///Field `CONT` writer - Continuous conversion
pub use crate::stm32f107::adc1::cr2::CONT_W;
///Direct memory access mode
pub use crate::stm32f107::adc1::cr2::DMA;
///Field `DMA` reader - Direct memory access mode
pub use crate::stm32f107::adc1::cr2::DMA_R;
///Field `DMA` writer - Direct memory access mode
pub use crate::stm32f107::adc1::cr2::DMA_W;
///Reset calibration
pub use crate::stm32f107::adc1::cr2::RSTCALR;
///Reset calibration
pub use crate::stm32f107::adc1::cr2::RSTCALW;
///Field `RSTCAL` reader - Reset calibration
pub use crate::stm32f107::adc1::cr2::RSTCAL_R;
///Field `RSTCAL` writer - Reset calibration
pub use crate::stm32f107::adc1::cr2::RSTCAL_W;
/**External event select for injected group

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL {
    ///0: Timer 1 TRGO event
    Tim1trgo = 0,
    ///1: Timer 1 CC4 event
    Tim1cc4 = 1,
    ///2: Timer 4 CC3 event
    Tim4cc3 = 2,
    ///3: Timer 8 CC2 event
    Tim8cc2 = 3,
    ///4: Timer 8 CC4 event
    Tim8cc4 = 4,
    ///5: Timer 5 TRGO event
    Tim5trgo = 5,
    ///6: Timer 5 CC4 event
    Tim5cc4 = 6,
    ///7: JSWSTART
    Jswstart = 7,
}
impl From<JEXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for JEXTSEL {}
///Field `JEXTSEL` reader - External event select for injected group
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL>;
impl JEXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEXTSEL {
        match self.bits {
            0 => JEXTSEL::Tim1trgo,
            1 => JEXTSEL::Tim1cc4,
            2 => JEXTSEL::Tim4cc3,
            3 => JEXTSEL::Tim8cc2,
            4 => JEXTSEL::Tim8cc4,
            5 => JEXTSEL::Tim5trgo,
            6 => JEXTSEL::Tim5cc4,
            7 => JEXTSEL::Jswstart,
            _ => unreachable!(),
        }
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL::Tim1trgo
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSEL::Tim1cc4
    }
    ///Timer 4 CC3 event
    #[inline(always)]
    pub fn is_tim4cc3(&self) -> bool {
        *self == JEXTSEL::Tim4cc3
    }
    ///Timer 8 CC2 event
    #[inline(always)]
    pub fn is_tim8cc2(&self) -> bool {
        *self == JEXTSEL::Tim8cc2
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn is_tim8cc4(&self) -> bool {
        *self == JEXTSEL::Tim8cc4
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == JEXTSEL::Tim5trgo
    }
    ///Timer 5 CC4 event
    #[inline(always)]
    pub fn is_tim5cc4(&self) -> bool {
        *self == JEXTSEL::Tim5cc4
    }
    ///JSWSTART
    #[inline(always)]
    pub fn is_jswstart(&self) -> bool {
        *self == JEXTSEL::Jswstart
    }
}
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, JEXTSEL, crate::Safe>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1trgo)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1cc4)
    }
    ///Timer 4 CC3 event
    #[inline(always)]
    pub fn tim4cc3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4cc3)
    }
    ///Timer 8 CC2 event
    #[inline(always)]
    pub fn tim8cc2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8cc2)
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn tim8cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8cc4)
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim5trgo)
    }
    ///Timer 5 CC4 event
    #[inline(always)]
    pub fn tim5cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim5cc4)
    }
    ///JSWSTART
    #[inline(always)]
    pub fn jswstart(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Jswstart)
    }
}
///External trigger conversion mode for injected channels
pub use crate::stm32f107::adc1::cr2::JEXTTRIG;
///Field `JEXTTRIG` reader - External trigger conversion mode for injected channels
pub use crate::stm32f107::adc1::cr2::JEXTTRIG_R;
///Field `JEXTTRIG` writer - External trigger conversion mode for injected channels
pub use crate::stm32f107::adc1::cr2::JEXTTRIG_W;
/**External event select for regular group

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: Timer 3 CC1 event
    Tim3cc1 = 0,
    ///1: Timer 2 CC3 event
    Tim2cc3 = 1,
    ///2: Timer 1 CC3 event
    Tim1cc3 = 2,
    ///3: Timer 8 CC1 event
    Tim8cc1 = 3,
    ///4: Timer 8 TRGO event
    Tim8trgo = 4,
    ///5: Timer 5 CC1 event
    Tim5cc1 = 5,
    ///6: Timer 5 CC3 event
    Tim5cc3 = 6,
    ///7: SWSTART
    Swstart = 7,
}
impl From<EXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for EXTSEL {}
///Field `EXTSEL` reader - External event select for regular group
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTSEL {
        match self.bits {
            0 => EXTSEL::Tim3cc1,
            1 => EXTSEL::Tim2cc3,
            2 => EXTSEL::Tim1cc3,
            3 => EXTSEL::Tim8cc1,
            4 => EXTSEL::Tim8trgo,
            5 => EXTSEL::Tim5cc1,
            6 => EXTSEL::Tim5cc3,
            7 => EXTSEL::Swstart,
            _ => unreachable!(),
        }
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn is_tim3cc1(&self) -> bool {
        *self == EXTSEL::Tim3cc1
    }
    ///Timer 2 CC3 event
    #[inline(always)]
    pub fn is_tim2cc3(&self) -> bool {
        *self == EXTSEL::Tim2cc3
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn is_tim1cc3(&self) -> bool {
        *self == EXTSEL::Tim1cc3
    }
    ///Timer 8 CC1 event
    #[inline(always)]
    pub fn is_tim8cc1(&self) -> bool {
        *self == EXTSEL::Tim8cc1
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == EXTSEL::Tim8trgo
    }
    ///Timer 5 CC1 event
    #[inline(always)]
    pub fn is_tim5cc1(&self) -> bool {
        *self == EXTSEL::Tim5cc1
    }
    ///Timer 5 CC3 event
    #[inline(always)]
    pub fn is_tim5cc3(&self) -> bool {
        *self == EXTSEL::Tim5cc3
    }
    ///SWSTART
    #[inline(always)]
    pub fn is_swstart(&self) -> bool {
        *self == EXTSEL::Swstart
    }
}
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTSEL, crate::Safe>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn tim3cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3cc1)
    }
    ///Timer 2 CC3 event
    #[inline(always)]
    pub fn tim2cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2cc3)
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn tim1cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1cc3)
    }
    ///Timer 8 CC1 event
    #[inline(always)]
    pub fn tim8cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8cc1)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim8trgo)
    }
    ///Timer 5 CC1 event
    #[inline(always)]
    pub fn tim5cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim5cc1)
    }
    ///Timer 5 CC3 event
    #[inline(always)]
    pub fn tim5cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim5cc3)
    }
    ///SWSTART
    #[inline(always)]
    pub fn swstart(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Swstart)
    }
}
///External trigger conversion mode for regular channels
pub use crate::stm32f107::adc1::cr2::EXTTRIG;
///Field `EXTTRIG` reader - External trigger conversion mode for regular channels
pub use crate::stm32f107::adc1::cr2::EXTTRIG_R;
///Field `EXTTRIG` writer - External trigger conversion mode for regular channels
pub use crate::stm32f107::adc1::cr2::EXTTRIG_W;
///Start conversion of injected channels
pub use crate::stm32f107::adc1::cr2::JSWSTARTR;
///Start conversion of injected channels
pub use crate::stm32f107::adc1::cr2::JSWSTARTW;
///Field `JSWSTART` reader - Start conversion of injected channels
pub use crate::stm32f107::adc1::cr2::JSWSTART_R;
///Field `JSWSTART` writer - Start conversion of injected channels
pub use crate::stm32f107::adc1::cr2::JSWSTART_W;
///Start conversion of regular channels
pub use crate::stm32f107::adc1::cr2::SWSTARTR;
///Start conversion of regular channels
pub use crate::stm32f107::adc1::cr2::SWSTARTW;
///Field `SWSTART` reader - Start conversion of regular channels
pub use crate::stm32f107::adc1::cr2::SWSTART_R;
///Field `SWSTART` writer - Start conversion of regular channels
pub use crate::stm32f107::adc1::cr2::SWSTART_W;
///Temperature sensor and VREFINT enable
pub use crate::stm32f107::adc1::cr2::TSVREFE;
///Field `TSVREFE` reader - Temperature sensor and VREFINT enable
pub use crate::stm32f107::adc1::cr2::TSVREFE_R;
///Field `TSVREFE` writer - Temperature sensor and VREFINT enable
pub use crate::stm32f107::adc1::cr2::TSVREFE_W;
impl R {
    ///Bit 0 - A/D converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D calibration
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reset calibration
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - External trigger conversion mode for injected channels
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - External trigger conversion mode for regular channels
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tsvrefe", &self.tsvrefe())
            .field("swstart", &self.swstart())
            .field("jswstart", &self.jswstart())
            .field("exttrig", &self.exttrig())
            .field("extsel", &self.extsel())
            .field("jexttrig", &self.jexttrig())
            .field("jextsel", &self.jextsel())
            .field("align", &self.align())
            .field("dma", &self.dma())
            .field("rstcal", &self.rstcal())
            .field("cal", &self.cal())
            .field("cont", &self.cont())
            .field("adon", &self.adon())
            .finish()
    }
}
impl W {
    ///Bit 0 - A/D converter ON / OFF
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W<'_, CR2rs> {
        ADON_W::new(self, 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CR2rs> {
        CONT_W::new(self, 1)
    }
    ///Bit 2 - A/D calibration
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W<'_, CR2rs> {
        CAL_W::new(self, 2)
    }
    ///Bit 3 - Reset calibration
    #[inline(always)]
    pub fn rstcal(&mut self) -> RSTCAL_W<'_, CR2rs> {
        RSTCAL_W::new(self, 3)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<'_, CR2rs> {
        DMA_W::new(self, 8)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CR2rs> {
        ALIGN_W::new(self, 11)
    }
    ///Bits 12:14 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, CR2rs> {
        JEXTSEL_W::new(self, 12)
    }
    ///Bit 15 - External trigger conversion mode for injected channels
    #[inline(always)]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W<'_, CR2rs> {
        JEXTTRIG_W::new(self, 15)
    }
    ///Bits 17:19 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CR2rs> {
        EXTSEL_W::new(self, 17)
    }
    ///Bit 20 - External trigger conversion mode for regular channels
    #[inline(always)]
    pub fn exttrig(&mut self) -> EXTTRIG_W<'_, CR2rs> {
        EXTTRIG_W::new(self, 20)
    }
    ///Bit 21 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<'_, CR2rs> {
        JSWSTART_W::new(self, 21)
    }
    ///Bit 22 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&mut self) -> SWSTART_W<'_, CR2rs> {
        SWSTART_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<'_, CR2rs> {
        TSVREFE_W::new(self, 23)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#ADC3:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
