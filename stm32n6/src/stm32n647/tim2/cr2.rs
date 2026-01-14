///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader;
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS` reader - MMS\[0\]: Master mode selection
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - MMS\[0\]: Master mode selection
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI1S` reader - tim_ti1 selection
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - tim_ti1 selection
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS_1` reader - MMS\[3\]
pub type MMS_1_R = crate::BitReader;
///Field `MMS_1` writer - MMS\[3\]
pub type MMS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADSYNC` reader - ADC synchronization
pub type ADSYNC_R = crate::BitReader;
///Field `ADSYNC` writer - ADC synchronization
pub type ADSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - MMS\[0\]: Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - tim_ti1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 25 - MMS\[3\]
    #[inline(always)]
    pub fn mms_1(&self) -> MMS_1_R {
        MMS_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - ADC synchronization
    #[inline(always)]
    pub fn adsync(&self) -> ADSYNC_R {
        ADSYNC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("mms_1", &self.mms_1())
            .field("adsync", &self.adsync())
            .finish()
    }
}
impl W {
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<'_, CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - MMS\[0\]: Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - tim_ti1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<'_, CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 25 - MMS\[3\]
    #[inline(always)]
    pub fn mms_1(&mut self) -> MMS_1_W<'_, CR2rs> {
        MMS_1_W::new(self, 25)
    }
    ///Bit 28 - ADC synchronization
    #[inline(always)]
    pub fn adsync(&mut self) -> ADSYNC_W<'_, CR2rs> {
        ADSYNC_W::new(self, 28)
    }
}
/**TIM2 control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM2:CR2)*/
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
