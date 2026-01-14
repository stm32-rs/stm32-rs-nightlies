///Register `ADDITIONAL_CTRL` reader
pub type R = crate::R<ADDITIONAL_CTRLrs>;
///Register `ADDITIONAL_CTRL` writer
pub type W = crate::W<ADDITIONAL_CTRLrs>;
///Field `CH_NUM` reader - Channel number.
pub type CH_NUM_R = crate::FieldReader;
///Field `CH_NUM` writer - Channel number.
pub type CH_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CH_SPACING` reader - Channel spacing.
pub type CH_SPACING_R = crate::FieldReader;
///Field `PA_FC` reader - Power control bandwidth selection according data rate
pub type PA_FC_R = crate::FieldReader;
///Field `PA_FC` writer - Power control bandwidth selection according data rate
pub type PA_FC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIME_CAPTURESEL` reader - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\[31:0\] register
pub type TIME_CAPTURESEL_R = crate::FieldReader;
///Field `TIME_CAPTURESEL` writer - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\[31:0\] register
pub type TIME_CAPTURESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AS_ENABLE` reader - Enable the antenna switching feature.
pub type AS_ENABLE_R = crate::BitReader;
///Field `AS_ENABLE` writer - Enable the antenna switching feature.
pub type AS_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Channel number.
    #[inline(always)]
    pub fn ch_num(&self) -> CH_NUM_R {
        CH_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Channel spacing.
    #[inline(always)]
    pub fn ch_spacing(&self) -> CH_SPACING_R {
        CH_SPACING_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Power control bandwidth selection according data rate
    #[inline(always)]
    pub fn pa_fc(&self) -> PA_FC_R {
        PA_FC_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:22 - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\[31:0\] register
    #[inline(always)]
    pub fn time_capturesel(&self) -> TIME_CAPTURESEL_R {
        TIME_CAPTURESEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 31 - Enable the antenna switching feature.
    #[inline(always)]
    pub fn as_enable(&self) -> AS_ENABLE_R {
        AS_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDITIONAL_CTRL")
            .field("ch_num", &self.ch_num())
            .field("ch_spacing", &self.ch_spacing())
            .field("pa_fc", &self.pa_fc())
            .field("time_capturesel", &self.time_capturesel())
            .field("as_enable", &self.as_enable())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Channel number.
    #[inline(always)]
    pub fn ch_num(&mut self) -> CH_NUM_W<'_, ADDITIONAL_CTRLrs> {
        CH_NUM_W::new(self, 0)
    }
    ///Bits 16:17 - Power control bandwidth selection according data rate
    #[inline(always)]
    pub fn pa_fc(&mut self) -> PA_FC_W<'_, ADDITIONAL_CTRLrs> {
        PA_FC_W::new(self, 16)
    }
    ///Bits 20:22 - Select the trigger event to capture the interpolated absolute time in the TIME_CAPTURE\[31:0\] register
    #[inline(always)]
    pub fn time_capturesel(&mut self) -> TIME_CAPTURESEL_W<'_, ADDITIONAL_CTRLrs> {
        TIME_CAPTURESEL_W::new(self, 20)
    }
    ///Bit 31 - Enable the antenna switching feature.
    #[inline(always)]
    pub fn as_enable(&mut self) -> AS_ENABLE_W<'_, ADDITIONAL_CTRLrs> {
        AS_ENABLE_W::new(self, 31)
    }
}
/**ADDITIONAL_CTRL register

You can [`read`](crate::Reg::read) this register and get [`additional_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`additional_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:ADDITIONAL_CTRL)*/
pub struct ADDITIONAL_CTRLrs;
impl crate::RegisterSpec for ADDITIONAL_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`additional_ctrl::R`](R) reader structure
impl crate::Readable for ADDITIONAL_CTRLrs {}
///`write(|w| ..)` method takes [`additional_ctrl::W`](W) writer structure
impl crate::Writable for ADDITIONAL_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDITIONAL_CTRL to value 0x0003_8800
impl crate::Resettable for ADDITIONAL_CTRLrs {
    const RESET_VALUE: u32 = 0x0003_8800;
}
